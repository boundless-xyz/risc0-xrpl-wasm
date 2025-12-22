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

#[allow(unused)]
#[link(wasm_import_module = "host_lib")]
unsafe extern "C" {

    // ########################################
    // Host Function Category: BN254 operations
    // ########################################

    /// Adds two BN254 curve points together.
    ///
    /// # Parameters
    /// * `p1_ptr` - Pointer to the first point (in uncompressed form, 64 bytes, in big-endian byte order)
    /// * `p1_len` - Length of the first point                  
    /// * `p2_ptr` - Pointer to the second point (in uncompressed form, 64 bytes, in big-endian byte order)
    /// * `p2_len` - Length of the second point
    /// * `out_buff_ptr` - Pointer to output buffer where result will be written
    /// * `out_buff_len` - Length of the output buffer (should be at least 64 bytes, in big-endian byte order)
    ///
    /// # Returns
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    ///
    /// # Note
    /// This function is intended for advanced cryptographic operations and should be used with caution.
    /// Improper use can lead to security vulnerabilities.
    pub fn bn254_add_helper(
        p1_ptr: *const u8,
        p1_len: usize,
        p2_ptr: *const u8,
        p2_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Multiplies a BN254 curve point by a scalar.
    ///
    /// # Parameters
    /// * `p1_ptr` - Pointer to the point (in uncompressed form, 64 bytes, in big-endian byte order)
    /// * `p1_len` - Length of the point                  
    /// * `scalar_ptr` - Pointer to the scalar (in big-endian byte order)
    /// * `scalar_len` - Length of the scalar (should be at least 1 and at most 32 bytes)
    /// * `out_buff_ptr` - Pointer to output buffer where result will be written
    /// * `out_buff_len` - Length of the output buffer (should be at least 64 bytes, in big-endian byte order)
    ///
    /// # Returns
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    ///
    /// # Note
    /// This function is intended for advanced cryptographic operations and should be used with caution.
    /// Improper use can lead to security vulnerabilities.  
    pub fn bn254_mul_helper(
        p1_ptr: *const u8,
        p1_len: usize,
        scalar_ptr: *const u8,
        scalar_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Negates a BN254 curve point.
    ///
    /// # Parameters
    /// * `p_ptr` - Pointer to the point (in uncompressed form, 64 bytes, in big-endian byte order)
    /// * `p_len` - Length of the point                  
    /// * `out_buff_ptr` - Pointer to output buffer where result will be written
    /// * `out_buff_len` - Length of the output buffer (should be at least 64 bytes, in big-endian byte order)
    ///
    /// # Returns
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    ///
    /// # Note
    /// This function is intended for advanced cryptographic operations and should be used with caution.
    /// Improper use can lead to security vulnerabilities.
    pub fn bn254_neg_helper(
        p_ptr: *const u8,
        p_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Conducts the pairing check of BN254 curve points.
    ///
    /// # Parameters
    /// * `affine_ptr` - Pointer to the input data (should be 768 bytes representing 4 pairs of (G1, G2) points)
    /// * `affine_len` - Length of the input data
    ///
    /// # Returns
    /// - Returns 1 if the pairing check passes
    /// - Returns -1 if the pairing check fails
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    ///
    /// # Note
    /// This function is intended for advanced cryptographic operations and should be used with caution.
    /// Improper use can lead to security vulnerabilities.
    pub fn bn254_pairing_helper(affine_ptr: *const u8, affine_len: usize) -> i32;
}
