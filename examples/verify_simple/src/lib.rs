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

/// Verify a known-good RISC0 proof from the prime factorization example
#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let image_id = [
        129, 187, 16, 5, 197, 19, 82, 142, 40, 46, 60, 84, 5, 154, 125, 12, 209, 38, 228, 90, 15,
        171, 134, 179, 31, 28, 90, 111, 251, 111, 113, 144,
    ];
    let seal = [
        22, 224, 88, 191, 0, 167, 255, 190, 246, 108, 211, 219, 238, 225, 0, 176, 84, 219, 145,
        111, 209, 32, 111, 170, 134, 65, 152, 134, 122, 95, 135, 90, 18, 44, 69, 217, 146, 79, 91,
        59, 86, 5, 20, 174, 67, 108, 243, 77, 87, 200, 78, 194, 237, 241, 92, 24, 93, 151, 36, 92,
        2, 201, 61, 247, 38, 45, 122, 172, 230, 85, 127, 84, 216, 139, 185, 71, 12, 10, 96, 240,
        130, 115, 37, 67, 105, 126, 120, 97, 192, 102, 41, 247, 230, 190, 33, 95, 2, 47, 212, 117,
        59, 234, 140, 60, 17, 42, 229, 239, 159, 84, 98, 153, 52, 123, 64, 240, 37, 180, 23, 99,
        98, 23, 96, 197, 195, 111, 70, 138, 26, 107, 165, 20, 131, 77, 204, 162, 176, 235, 199, 27,
        69, 253, 0, 215, 146, 141, 165, 77, 14, 11, 86, 199, 92, 156, 36, 6, 206, 139, 129, 67, 23,
        152, 158, 0, 234, 192, 111, 101, 180, 101, 185, 177, 245, 111, 34, 206, 216, 16, 161, 249,
        46, 135, 163, 175, 33, 52, 7, 70, 80, 79, 226, 68, 17, 99, 82, 200, 209, 166, 6, 233, 151,
        70, 137, 189, 225, 11, 112, 236, 196, 10, 106, 147, 12, 253, 253, 168, 121, 122, 183, 207,
        4, 76, 60, 232, 42, 202, 19, 41, 168, 149, 171, 228, 196, 223, 96, 61, 177, 178, 15, 213,
        45, 243, 50, 90, 107, 139, 33, 248, 160, 181, 38, 27, 132, 124, 157, 242,
    ];

    let proof = Proof::from_seal_bytes(&seal).unwrap();

    let journal = 33u32.to_le_bytes();
    let journal_digest = risc0::hash_journal(&journal);

    risc0::verify(&proof, &image_id, &journal_digest).unwrap();

    1
}
