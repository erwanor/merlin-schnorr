use curve25519_dalek_ng::{self, constants, ristretto::RistrettoPoint, scalar::Scalar};
use merlin::Transcript;
use rand_core::OsRng;
fn main() {
    let transcript = Transcript::new(b"schnorrid");

    println!("Hello, world!");
    let mut rng = OsRng;

    // make-believe generator for our example
    // better to use the standard ristretto base point
    let g = curve25519_dalek_ng::ristretto::RistrettoPoint::random(&mut rng);
    println!("generator: {:?}", g.compress());

    // our secret
    // we'll create the interactive proof that we "know"
    // the discrete log of g^x in G
    let secret = Scalar::random(&mut rng);
    println!("secret: {:?}", secret);

    let h = g * secret;

    /*
     * interactive pok-dl proof
     */

    // publish g st <g> = G, and h = g^x

    let r = Scalar::random(&mut rng);
    let u = g * r;

    // verifier challenge
    let challenge = Scalar::random(&mut rng);

    // prover response
    let z = r + challenge * secret;

    // verify proof
    let target = g * z;
    let ver = u + h * challenge;
    println!("checks out? {}", target == ver);

    // NI version
    let mut transcript = Transcript::new(b"schnorrid");

    transcript.append_message(b"generator", &g.compress().to_bytes());
    let secret = Scalar::random(&mut rng);
    let h = g * secret;

    let r = Scalar::random(&mut rng);
    let u = g * r;

    transcript.append_message(b"field order", &constants::BASEPOINT_ORDER.to_bytes());
    transcript.append_message(b"committed secret", &h.compress().to_bytes());
    transcript.append_message(b"randomness commitment", &u.compress().to_bytes());

    let mut transcript_rng = transcript.build_rng().finalize(&mut rng);
    let challenge = Scalar::random(&mut transcript_rng);
    let semi_proof = secret * challenge;
    let proof = r + semi_proof;

    let proof_eval = proof * g;
    let eval_commit_semi = challenge * h;
    let eval_commit = eval_commit_semi + u;
    println!("checks out? {}", proof_eval == eval_commit);
}
