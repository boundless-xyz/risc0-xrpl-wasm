// Copyright 2026 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{env, process::Command};

use anyhow::{Context, bail};
use serde_json::json;

pub const GENESIS_ACCOUNT: &str = "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh";
pub const GENESIS_SECRET: &str = "snoPBrXtMeMyMHUVTgbuqAfg1SUTb";

/// A handle to a running rippled docker container
/// Stops the container when dropped so if using this in tests it must be kept in scope
/// for the duration of the test
pub struct RippledHandle {
    container_id: String,
}

impl RippledHandle {
    pub fn start(cfg_path: &str) -> anyhow::Result<Self> {
        check_docker()?;

        let image = env::var("RIPPLED_DOCKER_IMAGE")
            .unwrap_or_else(|_| "ghcr.io/boundless-xyz/risc0-xrpl-wasm/rippled:f615764e8a5eb100381d25fb97b10fe49a963128".to_string());

        let output = Command::new("docker")
            .args([
                "run",
                "--rm",
                "-d",
                "-p",
                "5005:5005",
                "-p",
                "6006:6006",
                "-p",
                "51235:51235",
                "-v",
                &format!("{}:/var/lib/rippled/rippled.cfg:ro", cfg_path),
                &image,
                "--standalone",
                "--conf",
                "/var/lib/rippled/rippled.cfg",
            ])
            .output()
            .context("Failed to run docker command to start rippled")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("Failed to start rippled docker container: {}", stderr);
        }

        let container_id = String::from_utf8(output.stdout)
            .context("Failed to parse docker run output")?
            .trim()
            .to_string();

        if container_id.is_empty() {
            bail!("docker run returned empty container id");
        }

        // wait a bit for rippled to start
        std::thread::sleep(std::time::Duration::from_secs(2));

        Ok(Self { container_id })
    }

    fn call_rippled(&self, args: &[&str]) -> anyhow::Result<serde_json::Value> {
        let output = Command::new("docker")
            .args(["exec", &self.container_id, "rippled"])
            .args(args)
            .output()
            .context("Failed to run docker command to call rippled")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("Failed to call rippled: {}", stderr);
        }

        let r: serde_json::Value = serde_json::from_slice(&output.stdout)
            .context("Failed to parse rippled output as JSON")?;
        Ok(r["result"].clone())
    }

    /// Advance the ledger by one (e.g. mine a block)
    pub fn ledger_accept(&self) -> anyhow::Result<()> {
        self.call_rippled(&["ledger_accept"])?;
        Ok(())
    }

    pub fn validated_ledger(&self) -> anyhow::Result<serde_json::Value> {
        let r = self.call_rippled(&["ledger", "validated"])?;
        Ok(r)
    }

    /// Create a new account and fund it with 1000 XRP from the genesis account
    /// Note this will automatically advance the ledger to confirm the payment
    pub fn new_account(&self) -> anyhow::Result<(String, String)> {
        let r = self.call_rippled(&["wallet_propose"])?;
        let account_id = r["account_id"]
            .as_str()
            .context("Failed to parse account_id from wallet_propose output")?
            .to_string();
        let secret = r["master_seed"]
            .as_str()
            .context("Failed to parse secret from wallet_propose output")?
            .to_string();

        self.submit(
            json!({
                "TransactionType": "Payment",
                "Account": GENESIS_ACCOUNT,
                "Destination": account_id,
                "Amount": "1000000000",
            }),
            GENESIS_SECRET,
        )?;

        self.ledger_accept()?;

        Ok((account_id, secret))
    }

    /// Submit a transaction from the genesis account
    pub fn submit(&self, tx: serde_json::Value, secret: &str) -> anyhow::Result<serde_json::Value> {
        let r = self.call_rippled(&["submit", secret, tx.to_string().as_str()])?;
        println!("Submit result: {}", r);
        match r["engine_result_code"].as_u64() {
            Some(0) => Ok(r),
            Some(e) => {
                bail!(
                    "Transaction failed with engine_result_code {}: {}",
                    e,
                    r["engine_result_message"]
                );
            }
            None => bail!(
                "Failed to parse engine_result_code from submit result {}",
                r
            ),
        }
    }
}

impl Drop for RippledHandle {
    fn drop(&mut self) {
        let _ = Command::new("docker")
            .args(["stop", &self.container_id])
            .status();
    }
}

fn check_docker() -> anyhow::Result<()> {
    if !Command::new("docker")
        .arg("--version")
        .status()
        .context("Could not find or execute docker")?
        .success()
    {
        bail!("`docker --version` failed");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_start_stop_rippled() {
        let _handle =
            RippledHandle::start("../tests/rippled.cfg").expect("Failed to start rippled");
    }

    #[test]
    #[serial]
    fn test_ledger_accept() {
        let handle = RippledHandle::start("../tests/rippled.cfg").expect("Failed to start rippled");
        handle.ledger_accept().expect("Failed to advance ledger");
    }

    #[test]
    #[serial]
    fn test_new_account() {
        let handle = RippledHandle::start("../tests/rippled.cfg").expect("Failed to start rippled");
        let (account_id, secret) = handle.new_account().expect("Failed to create new account");
        println!(
            "New account created: {} with secret: {}",
            account_id, secret
        );
    }

    #[test]
    #[serial]
    fn test_ledger_submit() {
        let handle = RippledHandle::start("../tests/rippled.cfg").expect("Failed to start rippled");
        let (acc_1, secret) = handle.new_account().expect("Failed to create new account");
        let (acc_2, _) = handle.new_account().expect("Failed to create new account");

        let tx = serde_json::json!({
            "TransactionType": "Payment",
            "Account": acc_1,
            "Destination": acc_2,
            "Amount": "1000",
            "Fee": "10",
            "Sequence": 1,
        });
        handle
            .submit(tx, &secret)
            .expect("Failed to submit transaction");
    }
}
