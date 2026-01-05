// Copyright 2025 RISC Zero, Inc.
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

use serde_json::json;
use std::{
    path::{Path, PathBuf},
    process::Command,
};
use test_utils::RippledHandle;
// use wasm_host_simulator::mock_data::MockData;
// use wasm_host_simulator::vm_wamr::run_func;

fn build_example(example: &str) -> PathBuf {
    let status = Command::new("cargo")
        .args([
            "build",
            "--manifest-path",
            format!("examples/{example}/Cargo.toml").as_str(),
            "--release",
            "--target",
            "wasm32v1-none",
        ])
        .status()
        .expect("failed to run cargo build");

    if !status.success() {
        panic!("cargo build failed with status: {status}");
    }

    Path::new(&format!("target/wasm32v1-none/release/{example}.wasm")).to_path_buf()
}

fn create_escrow(
    handle: &RippledHandle,
    wasm_path: &PathBuf,
    from: &str,
    from_secret: &str,
    to: &str,
) -> anyhow::Result<u64> {
    let wasm = std::fs::read(wasm_path).expect("Failed to read wasm file");

    let val_ledger = handle
        .validated_ledger()
        .expect("Failed to get validated ledger");
    let close_time = val_ledger["ledger"]["close_time"]
        .as_u64()
        .expect("Failed to parse close_time from validated ledger");

    let tx = json!({
        "TransactionType": "EscrowCreate",
        "Account": from,
        "Destination": to,
        "Amount": "10000",
        "CancelAfter": close_time + 2000,
        "FinishFunction": hex::encode(&wasm),
    });
    println!("Submitting escrow create: {}", tx);

    let res = handle.submit(tx, from_secret)?;
    Ok(res["tx_json"]["Sequence"].as_u64().unwrap())
}

fn finish_escrow(
    handle: &RippledHandle,
    escrow_index: u32,
    from: &str,
    to: &str,
    secret: &str,
) -> anyhow::Result<()> {
    let tx = json!({
        "TransactionType": "EscrowFinish",
        "Owner": from,
        "Account": to,
        "OfferSequence": escrow_index,
        "ComputationAllowance": 1000000,
    });
    let res = handle.submit(tx, secret)?;
    println!("Submit result: {}", res);
    Ok(())
}

#[test]
fn run_groth16_verify_simple() -> anyhow::Result<()> {
    let path = build_example("groth16_verify_simple");

    let handle = RippledHandle::start().expect("Failed to start rippled");
    let (acc_1, secret_1) = handle.new_account().expect("Failed to create new account");

    let seq = create_escrow(&handle, &path, &acc_1, &secret_1, &acc_1)?;
    println!("Created escrow with sequence: {}", seq);
    handle.ledger_accept().expect("Failed to advance ledger");

    finish_escrow(&handle, seq as u32, &acc_1, &acc_1, &secret_1)?;

    Ok(())
    // assert!(
    //     run_func(
    //         path.to_str().unwrap().to_string(),
    //         Some("finish"),
    //         None,
    //         MockData::new("{}", "{}", "{}", "[]", "[]"),
    //     )
    //     .expect("Executing test wasm function failed")
    // );
}

// #[test]
// fn run_risc0_verify_simple() {
//     let path = build_example("risc0_verify_simple");
//     assert!(
//         run_func(
//             path.to_str().unwrap().to_string(),
//             Some("finish"),
//             None,
//             MockData::new("{}", "{}", "{}", "[]", "[]"),
//         )
//         .expect("Executing test wasm function failed")
//     );
// }

// #[test]
// fn run_risc0_verify_proof_from_memo() {
//     let path = build_example("risc0_verify_proof_from_memo");

//     let tx_data = &json!({"Memos": [
//     {
//       "Memo": {
//         "MemoData": "16e058bf00a7ffbef66cd3dbeee100b054db916fd1206faa864198867a5f875a122c45d9924f5b3b560514ae436cf34d57c84ec2edf15c185d97245c02c93df7262d7aace6557f54d88bb9470c0a60f082732543697e7861c06629f7e6be215f022fd4753bea8c3c112ae5ef9f546299347b40f025b41763621760c5c36f468a1a6ba514834dcca2b0ebc71b45fd00d7928da54d0e0b56c75c9c2406ce8b814317989e00eac06f65b465b9b1f56f22ced810a1f92e87a3af21340746504fe244116352c8d1a606e9974689bde10b70ecc40a6a930cfdfda8797ab7cf044c3ce82aca1329a895abe4c4df603db1b20fd52df3325a6b8b21f8a0b5261b847c9df2",
//       }
//     }
//   ],});

//     assert!(
//         run_func(
//             path.to_str().unwrap().to_string(),
//             Some("finish"),
//             None,
//             MockData::new(&tx_data.to_string(), "{}", "{}", "[]", "[]"),
//         )
//         .expect("Executing test wasm function failed")
//     );
// }
