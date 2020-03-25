use ed25519_dalek::{PublicKey, SecretKey, SECRET_KEY_LENGTH, PUBLIC_KEY_LENGTH, Keypair, Signature, Digest};
use rand::rngs::OsRng;
use rand::thread_rng;
use hex::FromHex;
use sha2::Sha512;
fn main() {
//    println!("Hello, world!");
//    let sk = "f105fd915a8ecd7f81a4865bb47b80c2384be5a6c5d5a1b7758891ab7a1f73ec09dc2585308709375013f3c2d0e7ce384467dae85f7a47335f6303dcf6b24b9d";
//    let pk = "09dc2585308709375013f3c2d0e7ce384467dae85f7a47335f6303dcf6b24b9d";
//
//    let public_key: PublicKey = PublicKey::from_bytes(pk.as_bytes()).unwrap();
//    let secret_key: SecretKey = SecretKey::from_bytes(sk.as_bytes()).unwrap();
    let secret_key: &[u8] = b"566a2f7806b92a02dbb08d64f28fe827b0e9a7d302f51ee6190ae9e9748985f9";
    let public_key: &[u8] = b"6ab7c20ea0817ec8232829cda98598bbc60662bf8c6a0306da0981e3f04d49a4";
    let message: &[u8] = b"616263";
    let signature: &[u8] = b"98a70222f0b8121aa9d30f813d683f809e462b469c7ff87639499bb94e6dae4131f85042463c2a355a2003d062adf5aaa10b8c61e636062aaad11c2a26083406";

    let sec_bytes: Vec<u8> = FromHex::from_hex(secret_key).unwrap();
    let pub_bytes: Vec<u8> = FromHex::from_hex(public_key).unwrap();
    let msg_bytes: Vec<u8> = FromHex::from_hex(message).unwrap();
    let sig_bytes: Vec<u8> = FromHex::from_hex(signature).unwrap();

    let secret: SecretKey = SecretKey::from_bytes(&sec_bytes[..SECRET_KEY_LENGTH]).unwrap();
    let public: PublicKey = PublicKey::from_bytes(&pub_bytes[..PUBLIC_KEY_LENGTH]).unwrap();
    let keypair: Keypair  = Keypair{ secret: secret, public: public };
    let sig1: Signature = Signature::from_bytes(&sig_bytes[..]).unwrap();

    let mut prehash_for_signing: Sha512 = Sha512::default();
    let mut prehash_for_verifying: Sha512 = Sha512::default();

    prehash_for_signing.input(&msg_bytes[..]);
    prehash_for_verifying.input(&msg_bytes[..]);

    let sig2: Signature = keypair.sign_prehashed(prehash_for_signing, None);

    println!("{:?}", sig1.eq(&sig2));
    assert_eq!(sig1, sig2, "Original signature from test vectors doesn't equal signature produced:\
                \noriginal:\n{:?}\nproduced:\n{:?}", sig1, sig2);
    assert!(keypair.verify_prehashed(prehash_for_verifying, None, &sig2).is_ok(),
            "Could not verify ed25519ph signature!");
}
