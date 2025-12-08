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

pub use proof::Proof;
pub use verify::verify_groth16;

use crate::groth16::bn254::{G1Point, G2Point, Scalar};

mod bn254;
mod proof;
mod verify;

/// Groth16 proof verification key on BN254 curve
pub struct VerificationKey<const N: usize> {
    pub vk_alpha_g1: G1Point,
    pub vk_beta_g2: G2Point,
    pub vk_gamma_g2: G2Point,
    pub vk_delta_g2: G2Point,
    /// Zeroth element of the IC array
    pub vk_ic_0: G1Point,
    /// Rest of the IC array
    pub vk_ic: [G1Point; N],
}

pub struct PublicInputs<const N: usize> {
    pub inputs: [Scalar; N],
}
