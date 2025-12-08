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

use crate::groth16::bn254::{BASE_FIELD_MODULUS_Q, G1_LEN, G1Point, G2_LEN, G2Point};
use crate::{Error, Result};

/// Groth16 proof elements on BN254 curve
/// - neg_pi_a must be a point in G1 (Note this the negated version of pi_a)
/// - pi_b must be a point in G2
/// - pi_c must be a point in G1
pub struct Proof {
    pub neg_pi_a: G1Point,
    pub pi_b: G2Point,
    pub pi_c: G1Point,
}

impl Proof {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != 256 {
            return Err(Error::InvalidProof);
        }
        Ok(Proof {
            neg_pi_a: bytes[..G1_LEN].try_into().unwrap(),
            pi_b: bytes[G1_LEN..G1_LEN + G2_LEN].try_into().unwrap(),
            pi_c: bytes[G1_LEN + G2_LEN..].try_into().unwrap(),
        })
    }

    pub fn to_bytes(&self) -> [u8; 256] {
        let mut bytes = [0u8; 256];
        bytes[0..G1_LEN].copy_from_slice(&self.neg_pi_a);
        bytes[G1_LEN..G1_LEN + G2_LEN].copy_from_slice(&self.pi_b);
        bytes[G1_LEN + G2_LEN..].copy_from_slice(&self.pi_c);
        bytes
    }

    /// Create a Proof from the Risc0 Groth16 seal bytes
    /// This takes care of negating pi_a to get neg_pi_a
    pub fn from_seal_bytes(seal: &[u8]) -> Result<Self> {
        if seal.len() != 256 {
            return Err(Error::InvalidProof);
        }
        let mut proof = Proof::from_bytes(seal)?;
        proof.neg_pi_a = negate_g1(&proof.neg_pi_a);
        Ok(proof)
    }
}

/// Negate a BN254 G1 curve point
fn negate_g1(point: &G1Point) -> G1Point {
    let mut negated_point = [0u8; G1_LEN];
    negated_point[..32].copy_from_slice(&point[..32]);

    let mut y = [0u8; 32];
    y.copy_from_slice(&point[32..]);

    let mut modulus = BASE_FIELD_MODULUS_Q;
    subtract_be_bytes(&mut modulus, &y);
    negated_point[32..].copy_from_slice(&modulus);

    negated_point
}

/// Subtract big-endian numbers
fn subtract_be_bytes(a: &mut [u8; 32], b: &[u8; 32]) {
    let mut borrow: u32 = 0;
    for (ai, bi) in a.iter_mut().zip(b.iter()).rev() {
        let result = (*ai as u32).wrapping_sub(*bi as u32).wrapping_sub(borrow);
        *ai = result as u8;
        borrow = (result >> 31) & 1;
    }
}
