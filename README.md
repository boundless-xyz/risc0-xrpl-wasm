# RISC Zero Verifier for XRPL Wasm

This crate implements a stateless RISC Zero proof verifier, and generic Groth16 verifier to be used within the XRPL Wasm runtime

## Usage

Note the examples below will only work in an XRPL Wasm runtime environment or similar environment that provides host functions for BN254 operations.

> [!IMPORTANT]
> These precompiles are not supported by the current version of the xrpl-wasm runtime. Examples must currently be run against the host simulator 
> from [this fork](https://github.com/ripple/craft/tree/zhang/groth16) of the (now deprecated) Craft repo

### Verify RISC0 proof

To verify a RISC0 proof:

```rust,ignore
use xrpl_risc0_verifier::{risc0, Proof};

// first obtain seal (proof), and journal bytes from chain state or transaction memo

let proof = Proof::from_seal_bytes(&seal).unwrap();
let journal_digest = risc0::hash_journal(&journal);

risc0::verify(&proof, &image_id, &journal_digest).unwrap();
```

`verify` will return `Ok(())` if verification is successful or an error variant otherwise.

### Verify arbitrary Groth16 proof

Verifying an arbitrary Groth16 proof requires specifying the full view key, proof and public inputs.

```rust,ignore
use xrpl_risc0_verifier::groth16::{verify_groth16, Proof, PublicInputs, VerificationKey};

let vk = VerificationKey {
    vk_alpha_g1: [..],
    vk_beta_g2: [..],
    vk_gamma_g2: [..],
    vk_delta_g2: [..],
    vk_ic_0: [..],
    vk_ic: [..],
};
let proof = Proof {
    // Note this is negated, negation should be performed off-chain
    neg_pi_a: [..],
    pi_b: [..],
    pi_c: [..],
};

let inputs = PublicInputs {
    inputs: [...],
};

verify_groth16(&vk, &proof, &inputs).expect("Failed to verify proof");
```

`verify` will return `Ok(())` if verification is successful or an error variant otherwise.

## License

Licensed under the Apache-2.0 License (SPDX: Apache-2.0). See the LICENSE file for full terms.
