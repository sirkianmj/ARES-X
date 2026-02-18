use aes_gcm::{Aes256Gcm, Key, Nonce, KeyInit, aead::Aead};
use argon2::{Argon2, Algorithm, Version, Params};
use zeroize::Zeroize;

pub struct SovereignShield {
    key: [u8; 32],
}

impl SovereignShield {
    pub fn new(password: &str, salt: &[u8]) -> Self {
        let mut derived_key = [0u8; 32];
        let argon2 = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::default()
        );
        
        // Derive the key into the buffer
        argon2.hash_password_into(password.as_bytes(), salt, &mut derived_key)
            .expect("KDF Failed");

        Self { key: derived_key }
    }

    pub fn encrypt(&self, data: &[u8], nonce_slice: &[u8; 12]) -> Vec<u8> {
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&self.key));
        let nonce = Nonce::from_slice(nonce_slice);
        cipher.encrypt(nonce, data).expect("Encryption failed")
    }

    pub fn trigger_kill_switch(&mut self) {
        // Zeroize the RAM-resident key immediately
        self.key.zeroize();
        println!("CRITICAL: Forgex4 Kill-Switch triggered. RAM keys zeroized.");
    }
}