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

use anyhow::{Result, bail};
use risc0_zkvm::InnerReceipt;

/// Encode the seal of the given receipt for use with XRPL verifiers.
///
/// This is a helper to be used when building a transaction on the client and should not be used inside an XRPL Wasm function
pub fn encode_seal(receipt: &risc0_zkvm::Receipt) -> Result<Vec<u8>> {
    let seal = match receipt.inner.clone() {
        InnerReceipt::Groth16(receipt) => receipt.seal,
        _ => bail!("Unsupported receipt type"),
    };
    Ok(seal)
}
