use ark_r1cs_std::prelude::*;
use ark_r1cs_std::fields::fp::FpVar;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_bls12_381::Fr;

#[derive(Copy, Clone)]
pub struct SquareCircuit {
    pub x: Option<Fr>,
    pub y: Option<Fr>,
}

impl ConstraintSynthesizer<Fr> for SquareCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        // Allocate x as a witness (private input)
        let x = FpVar::<Fr>::new_witness(cs.clone(), || self.x.ok_or(SynthesisError::AssignmentMissing))?;
        
        // Allocate y as a public input
        let y = FpVar::<Fr>::new_input(cs.clone(), || self.y.ok_or(SynthesisError::AssignmentMissing))?;
        
        // Enforce x * x = y
        x.square()?.enforce_equal(&y)?;
        
        Ok(())
    }
}
