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

#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use risc0_verifier_xrpl_wasm::{risc0, Proof};
#[allow(unused_imports)]
use xrpl_wasm_stdlib;

const IMAGE_ID: [u8; 32] =
    hex_literal::hex!("81bb1005c513528e282e3c54059a7d0cd126e45a0fab86b31f1c5a6ffb6f7190");

const SEAL: &[u8] = &hex_literal::hex!("16e058bf00a7ffbef66cd3dbeee100b054db916fd1206faa864198867a5f875a122c45d9924f5b3b560514ae436cf34d57c84ec2edf15c185d97245c02c93df7262d7aace6557f54d88bb9470c0a60f082732543697e7861c06629f7e6be215f022fd4753bea8c3c112ae5ef9f546299347b40f025b41763621760c5c36f468a1a6ba514834dcca2b0ebc71b45fd00d7928da54d0e0b56c75c9c2406ce8b814317989e00eac06f65b465b9b1f56f22ced810a1f92e87a3af21340746504fe244116352c8d1a606e9974689bde10b70ecc40a6a930cfdfda8797ab7cf044c3ce82aca1329a895abe4c4df603db1b20fd52df3325a6b8b21f8a0b5261b847c9df2");

/// Verify a known-good RISC0 proof from the prime factorization example
#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let proof = Proof::from_seal_bytes(SEAL).unwrap();

    let journal = 33u32.to_le_bytes();
    let journal_digest = risc0::hash_journal(&journal);

    risc0::verify(&proof, &IMAGE_ID, &journal_digest).unwrap();

    1
}
