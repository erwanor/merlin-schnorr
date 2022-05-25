use curve25519_dalek_ng::{self, constants, ristretto, scalar::Scalar};
use rand_core::OsRng;

fn main() {
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
}
