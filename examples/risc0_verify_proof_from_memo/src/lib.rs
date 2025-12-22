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
use xrpl_wasm_stdlib::host::trace::{trace_data, DataRepr};
use xrpl_wasm_stdlib::host::{Error, Result, Result::Err, Result::Ok};
use xrpl_wasm_stdlib::{core::locator::Locator, host::get_tx_nested_field, sfield};

// Commit to the image ID inside the wasm code. This constrains the zkVM program that this escrow will allow proofs from.
// NOTE: This must be updated if you change/rebuild the RISC0 zkVM guest code
const IMAGE_ID: [u8; 32] =
    hex_literal::hex!("81bb1005c513528e282e3c54059a7d0cd126e45a0fab86b31f1c5a6ffb6f7190");

/// Verify a RISC0 proof retrieved from the data of the first memo in the transaction
#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let seal: [u8; 256] = get_memo(0).unwrap();
    trace_data("received seal", &seal, DataRepr::AsHex).unwrap();

    let proof = Proof::from_seal_bytes(&seal).unwrap();

    let journal = 33u32.to_le_bytes();
    let journal_digest = risc0::hash_journal(&journal);

    risc0::verify(&proof, &IMAGE_ID, &journal_digest).unwrap();

    1
}

/// Retrieves the idx indexed memo's MemoData field from the transaction.
/// The calling code must know the expected length of the MemoData field in advance
pub fn get_memo<const LEN: usize>(idx: i32) -> Result<[u8; LEN]> {
    let mut buffer = [0; LEN];
    let mut locator = Locator::new();
    locator.pack(sfield::Memos);
    locator.pack(idx);
    locator.pack(sfield::Memo);
    locator.pack(sfield::MemoData);
    let result_code = unsafe {
        get_tx_nested_field(
            locator.as_ptr(),
            locator.num_packed_bytes(),
            buffer.as_mut_ptr(),
            buffer.len(),
        )
    };

    match result_code {
        result_code if result_code > 0 => Ok(buffer),
        0 => Err(Error::InternalError),
        result_code => Err(Error::from_code(result_code)),
    }
}
