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

use sha2::{Digest, Sha256};

mod constants;

#[cfg(feature = "std")]
mod encode_seal;

/// Generate SHA256 hash of the journal bytes. Apply this to raw bytes to obtain a journal_digest
/// that can be passed to `risc0::verify`
pub fn hash_journal(journal: &[u8]) -> [u8; 32] {
    Sha256::digest(journal).into()
}

#[cfg(feature = "std")]
pub use encode_seal::encode_seal;
pub use verify::*;

mod verify {
    use super::constants::*;
    use crate::groth16::{Proof, PublicInputs, verify_groth16};
    use crate::{Error, Result};
    use sha2::{Digest, Sha256};

    /// Verify a RISC0 proof against an image ID and journal digest
    ///
    /// This is most likely the function you want to call if you are verifying a proof
    /// for a particular RISC0 program (identified by the image ID).
    ///
    /// Use `hash_journal` on the journal bytes to generate the journal digest
    pub fn verify(proof: &Proof, image_id: &[u8; 32], journal_digest: &[u8; 32]) -> Result<()> {
        let claim_digest = hash_claim(image_id, journal_digest);
        verify_integrity(proof, &claim_digest)
    }

    /// Verify integrity of a RISC0 proof against a given claim digest.
    pub fn verify_integrity(proof: &Proof, claim_digest: &[u8; 32]) -> Result<()> {
        let public_inputs = public_inputs(*claim_digest)?;
        verify_groth16(&RISC0_VERIFICATION_KEY, proof, &public_inputs)
    }

    /// Generate a receipt claim digest
    fn hash_claim(image_id: &[u8; 32], journal_digest: &[u8; 32]) -> [u8; 32] {
        let input_digest = [0u8; 32];
        let pre_digest = image_id;
        let post_digest = SYSTEM_STATE_ZERO_DIGEST;
        let output_digest = hash_output(journal_digest, &[0u8; 32]);
        let system_exit = 0;
        let user_exit = 0;

        hash_receipt_claim(
            &input_digest,
            pre_digest,
            &post_digest,
            &output_digest,
            system_exit,
            user_exit,
        )
    }

    /// Generate ReceiptClaim digest
    /// SHA256('risc0.ReceiptClaim' || input || preState || postState || output || system_code || user_code || 0x0400)
    fn hash_receipt_claim(
        input_digest: &[u8; 32],
        pre_state_digest: &[u8; 32],
        post_state_digest: &[u8; 32],
        output_digest: &[u8; 32],
        system_exit_code: u32,
        user_exit_code: u32,
    ) -> [u8; 32] {
        let system_bytes = (system_exit_code << 24).to_be_bytes();
        let user_bytes = (user_exit_code << 24).to_be_bytes();
        let down_len = (4u16 << 8).to_be_bytes();

        let mut hasher = Sha256::new();
        hasher.update(RECEIPT_CLAIM_TAG);
        hasher.update(input_digest);
        hasher.update(pre_state_digest);
        hasher.update(post_state_digest);
        hasher.update(output_digest);
        hasher.update(system_bytes);
        hasher.update(user_bytes);
        hasher.update(down_len);
        hasher.finalize().into()
    }

    /// Generate Output digest
    /// SHA256('risc0.Output' || journal_digest || assumptions_digest || 0x0200)
    fn hash_output(journal_digest: &[u8; 32], assumptions_digest: &[u8; 32]) -> [u8; 32] {
        let down_len = (2u16 << 8).to_be_bytes();
        let mut hasher = Sha256::new();
        hasher.update(OUTPUT_TAG);
        hasher.update(journal_digest);
        hasher.update(assumptions_digest);
        hasher.update(down_len);
        hasher.finalize().into()
    }

    /// Process claim digest into Groth16 public inputs
    /// Generates five field elements:
    /// - (a0,a1): Allowed control root split into two field elements
    /// - (c0,c1): Claim digest split into two field elements
    /// - id: BN254 identity control ID
    fn public_inputs(claim_digest: [u8; 32]) -> Result<PublicInputs<5>> {
        if claim_digest == [0u8; 32] {
            return Err(Error::InvalidClaimDigest);
        }

        let (a0, a1) = split_digest(ALLOWED_CONTROL_ROOT);
        let (c0, c1) = split_digest(claim_digest);

        let mut id = BN254_IDENTITY_CONTROL_ID;
        id.reverse();

        Ok(PublicInputs {
            inputs: [a0, a1, c0, c1, to_field_element(&id)],
        })
    }

    /// Split Digest into two 32-byte field elements
    fn split_digest(bytes: [u8; 32]) -> ([u8; 32], [u8; 32]) {
        let mut big_endian = bytes;
        big_endian.reverse();
        let (b, a) = big_endian.split_at(big_endian.len() / 2);
        (to_field_element(a), to_field_element(b))
    }

    /// Convert arbitrary bytes to 32-byte field element
    fn to_field_element(input: &[u8]) -> [u8; 32] {
        let mut fixed_array = [0u8; 32];
        let start_index = 32 - input.len();
        fixed_array[start_index..].copy_from_slice(input);
        fixed_array
    }
}
