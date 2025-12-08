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

use crate::groth16::{
    Proof, PublicInputs, VerificationKey, bn254,
    bn254::{BASE_FIELD_MODULUS_Q, Pair},
};
use crate::{Error, Result};

/// Verify an arbitrary Groth16 proof given a proof, verification key, and public inputs
/// This is const-generic over the number of public inputs
pub fn verify_groth16<const N_PUBLIC: usize>(
    vk: &VerificationKey<N_PUBLIC>,
    proof: &Proof,
    public: &PublicInputs<N_PUBLIC>,
) -> Result<()> {
    // compute vkx as linear combination of the public inputs and IC points
    let vkx =
        public
            .inputs
            .iter()
            .zip(vk.vk_ic.iter())
            .try_fold(vk.vk_ic_0, |vkx, (input, ic)| {
                verify_scalar_in_field(input)?;
                Ok(bn254::add_g1(vkx, bn254::mult_g1(*ic, *input)))
            })?;

    // use ec_pairing_check_helper_bn254 to check
    // e(−A,B)⋅e(α,β)⋅e(vkx​,γ)⋅e(C,δ) == 1
    if bn254::pairing_check(&[
        Pair(proof.neg_pi_a, proof.pi_b),
        Pair(vk.vk_alpha_g1, vk.vk_beta_g2),
        Pair(vkx, vk.vk_gamma_g2),
        Pair(proof.pi_c, vk.vk_delta_g2),
    ]) {
        Ok(())
    } else {
        Err(Error::PairingCheckFailed)
    }
}

/// Verify that a scalar is properly reduced modulo BN254 base field
fn verify_scalar_in_field(x: &[u8; 32]) -> Result<()> {
    if x.iter().cmp(BASE_FIELD_MODULUS_Q.iter()) != core::cmp::Ordering::Less {
        Err(Error::ScalarNotInField)
    } else {
        Ok(())
    }
}
