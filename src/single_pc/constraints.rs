use algebra::Field;
use r1cs_core::{ConstraintSystem, SynthesisError};
use r1cs_std::prelude::*;

use crate::single_pc::SinglePolynomialCommitment;

/// Describes the interface for a gadget for a `SinglePolynomialCommitment` 
/// verifier.
pub trait SinglePCCheckGadget<PCF: Field, PC: SinglePolynomialCommitment<PCF>, ConstraintF: Field> {
    /// An allocated version of `PC::VerifierKey`.
    type VerifierKey: AllocGadget<PC::VerifierKey, ConstraintF> + ToBytesGadget<ConstraintF>;
    /// An allocated version of `PC::Commitment`.
    type Commitment: AllocGadget<PC::Proof, ConstraintF>;
    /// An allocated version of `PC::Proof`.
    type Proof: AllocGadget<PC::Proof, ConstraintF>;


    /// Add to `CS` new constraints that check that `proof` is a valid evaluation
    /// proof at `point` for the polynomial in `commitment`.
    // TODO: Provide an option to the gadget to avoid allocating
    // the key, and to instead just use a hardcoded key.
    // Maybe by making the concrete struct a 
    fn check_evaluation<CS: ConstraintSystem<ConstraintF>>(
        cs: CS,
        verification_key: &Self::VerifierKey,
        commitment: &Self::Commitment,
        point: impl ToBitsGadget<ConstraintF> + ?Sized,
        value: impl ToBitsGadget<ConstraintF> + ?Sized,
        proof: &Self::Proof,
    ) -> Result<(), SynthesisError>;

    /// Add to `CS` new constraints that check that `proof_i` is a valid evaluation
    /// proof at `point_i` for the polynomial in `commitment_i`.
    // TODO: add rng.
    fn batch_check_evaluations<CS: ConstraintSystem<ConstraintF>>(
        cs: CS,
        verification_key: &Self::VerifierKey,
        commitments: &[Self::Commitment],
        points: &[impl ToBitsGadget<ConstraintF> + Sized],
        values: &[impl ToBitsGadget<ConstraintF> + Sized],
        proofs: &[Self::Proof],
    ) -> Result<(), SynthesisError>;
}
