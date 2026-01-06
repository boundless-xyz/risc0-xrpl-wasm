use anyhow::{Result, bail};
use risc0_zkvm::InnerReceipt;

/// Encode the seal of the given receipt for use with XRPL verifiers.
///
/// This is just a helper to extract the seal from the receipt and return it as a byte vector.
pub fn encode_seal(receipt: &risc0_zkvm::Receipt) -> Result<Vec<u8>> {
    let seal = match receipt.inner.clone() {
        InnerReceipt::Groth16(receipt) => receipt.seal,
        _ => bail!("Unsupported receipt type"),
    };
    Ok(seal)
}
