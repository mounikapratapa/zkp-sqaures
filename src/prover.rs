use ark_bls12_381::{Bls12_381, Fr};
use ark_groth16::{Groth16, Proof, ProvingKey};
use ark_std::rand::thread_rng;

use crate::circuit::SquareCircuit;

pub fn generate_proving_key() -> ProvingKey<Bls12_381> {
    let rng = &mut thread_rng();
    let circuit = SquareCircuit {
        x: None,
        y: None,
    };
    Groth16::<Bls12_381>::generate_random_parameters_with_reduction(circuit, rng).unwrap()
}

pub fn create_proof(x: Fr, pk: &ProvingKey<Bls12_381>) -> (Proof<Bls12_381>, Fr) {
    let rng = &mut thread_rng();
    let y = x * x; // Compute y = x^2
    
    let circuit = SquareCircuit {
        x: Some(x),
        y: Some(y),
    };
    
    let proof = Groth16::<Bls12_381>::create_random_proof_with_reduction(circuit, pk, rng).unwrap();
    (proof, y)
} 