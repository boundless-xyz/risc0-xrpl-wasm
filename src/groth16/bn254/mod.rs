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

/// This module provides an implementation of g1 addition, g1 multiplication by a scalar, and pairing checks for BN256
/// The actual implementation is selected based on the target. When building for xrpl-wasm it will use host function precompiles
/// and when building natively will use arkworks directly
use hex_literal::hex;

// Base field modulus 'q' for BN254
// https://docs.rs/ark-bn254/latest/ark_bn254/
pub const BASE_FIELD_MODULUS_Q: [u8; 32] =
    hex!("30644E72E131A029B85045B68181585D97816A916871CA8D3C208C16D87CFD47");

pub const SCALAR_LEN: usize = 32;
pub const G1_LEN: usize = 64;
pub const G2_LEN: usize = 128;

pub type G1Point = [u8; G1_LEN];
pub type G2Point = [u8; G2_LEN];
pub type Scalar = [u8; SCALAR_LEN];

#[repr(C)]
pub(crate) struct Pair(pub G1Point, pub G2Point);

#[cfg(target_arch = "wasm32")]
#[path = "precompiles.rs"]
mod ops;

#[cfg(not(target_arch = "wasm32"))]
#[path = "native.rs"]
mod ops;

pub(crate) use ops::*;
