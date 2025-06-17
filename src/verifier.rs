use ark_bls12_381::{Bls12_381, Fr};
use ark_groth16::{Groth16, Proof, ProvingKey, VerifyingKey, PreparedVerifyingKey};
use ark_snark::SNARK;

pub fn generate_verifying_key(pk: &ProvingKey<Bls12_381>) -> VerifyingKey<Bls12_381> {
    pk.vk.clone()
}

pub fn verify_proof(y: Fr, proof: &Proof<Bls12_381>, vk: &VerifyingKey<Bls12_381>) -> bool {
    let pvk = PreparedVerifyingKey::from(vk.clone());
    let public_inputs = vec![y];
    Groth16::<Bls12_381>::verify_proof(&pvk, proof, &public_inputs).unwrap()
} 