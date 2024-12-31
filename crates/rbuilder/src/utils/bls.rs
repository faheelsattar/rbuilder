
use blst::min_pk::SecretKey;
use rand::{self, RngCore};

pub fn generate_random_bls_address() -> [u8; 48] {
    let mut rng = rand::thread_rng();
    let mut ikm = [0u8; 32];
    rng.fill_bytes(&mut ikm);
    
    let sk = SecretKey::key_gen(&ikm, &[]).unwrap();
    let pk = sk.sk_to_pk();
    return pk.compress();
}