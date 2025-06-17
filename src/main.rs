use ark_bls12_381::Fr;
use ark_std::UniformRand;

mod circuit;
mod prover;
mod verifier;

fn main() {
    // Generate a random value for x
    let rng = &mut ark_std::rand::thread_rng();
    let x = Fr::rand(rng);
    println!("Secret value x: {}", x);

    // Generate proving and verifying keys
    let pk = prover::generate_proving_key();
    let vk = verifier::generate_verifying_key(&pk);

    // Create a proof
    let (proof, y) = prover::create_proof(x, &pk);
    println!("Public value y: {}", y);

    // Verify the proof
    let is_valid = verifier::verify_proof(y, &proof, &vk);
    println!("Proof is valid: {}", is_valid);

    // Try to verify with wrong y
    let wrong_y = y + Fr::from(1u32);
    let is_valid_wrong = verifier::verify_proof(wrong_y, &proof, &vk);
    println!("Proof with wrong y is valid: {}", is_valid_wrong);
}
