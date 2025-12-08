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

use xrpl_wasm_std::host::{bn254_add_helper, bn254_mul_helper, bn254_pairing_helper};

use crate::groth16::bn254::{G1_LEN, G1Point, G2_LEN, Pair, Scalar};

pub(crate) fn add_g1(p1: G1Point, p2: G1Point) -> G1Point {
    let mut add_res = [0u8; G1_LEN];
    unsafe {
        bn254_add_helper(
            p1.as_ptr(),
            p1.len(),
            p2.as_ptr(),
            p2.len(),
            add_res.as_mut_ptr(),
            add_res.len(),
        );
    }
    add_res
}

pub(crate) fn mult_g1(p1: G1Point, scalar: Scalar) -> G1Point {
    let mut mul_res = [0u8; G1_LEN];
    unsafe {
        bn254_mul_helper(
            p1.as_ptr(),
            p1.len(),
            scalar.as_ptr(),
            scalar.len(),
            mul_res.as_mut_ptr(),
            mul_res.len(),
        );
    }
    mul_res
}

/// Evaluates prod [ e(pairs[i].0, pairs[i].1) == 1 ]
pub(crate) fn pairing_check(pairs: &[Pair]) -> bool {
    let result = unsafe {
        bn254_pairing_helper(pairs.as_ptr() as *const u8, pairs.len() * (G1_LEN + G2_LEN))
    };
    result == 1
}
