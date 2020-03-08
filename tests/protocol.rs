extern crate clsag;
extern crate curve25519_dalek;

use clsag::clsag::Clsag;
use clsag::tests_helper::*;

#[test]
fn test_protocol() {
    // Define setup parameters
    let num_keys = 2;
    let num_decoys = 11;
    let msg = b"hello world";

    // Define a clsag object which will be used to create a signature
    let mut clsag = Clsag::new();

    // Generate and add decoys
    let decoys = generate_decoys(num_decoys, num_keys);
    for decoy in decoys {
        clsag.add_member(decoy);
    }

    // Generate and add signer
    let signer = generate_signer(num_keys);
    clsag.add_member(signer);

    let signature = clsag.sign(msg).unwrap();
    let res = signature.verify(&mut clsag.public_keys(), msg);

    assert!(res.is_ok())
}
