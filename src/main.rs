// use std::intrinsics::const_allocate;

use curve25519_dalek_ng::{self, constants, ristretto::RistrettoPoint, scalar::Scalar};
use merlin::Transcript;
use rand_core::OsRng;
fn main() {
    let mut rng = OsRng;

    let generator = constants::RISTRETTO_BASEPOINT_POINT;

    println!("generator: {:?}", generator.compress());

    let mut transcript = Transcript::new(b"schnorrid");
    // public parameters:
    // field_order, generator, secret_commit, random_commit
    transcript.append_message(
        b"generator",
        &constants::ED25519_BASEPOINT_POINT.compress().to_bytes(),
    );

    transcript.append_message(b"field order", &constants::BASEPOINT_ORDER.to_bytes());

    let secret = Scalar::random(&mut rng);
    let secret_commit = generator * secret;

    transcript.append_message(b"secret_commit", &secret_commit.compress().to_bytes());

    let r = Scalar::random(&mut rng);
    let random_commit = generator * r;

    transcript.append_message(
        b"randomness commitment",
        &random_commit.compress().to_bytes(),
    );

    let mut transcript_rng = transcript.build_rng().finalize(&mut rng);
    let challenge = Scalar::random(&mut transcript_rng);
    let semi_proof = secret * challenge;
    let proof = r + semi_proof;

    let proof_eval = proof * generator;
    let eval_commit_semi = challenge * secret_commit;
    let eval_commit = eval_commit_semi + random_commit;
    println!("checks out? {}", proof_eval == eval_commit);
}
