
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


#[cfg(test)]
mod tests {
    use crate::utils::bls::generate_random_bls_address;

    #[test]
    fn test_generate_random_bls_address() {
        let bls_address = generate_random_bls_address();
        assert_eq!(bls_address.len(), 48, "BLS address should be of 48 length");
    }
}