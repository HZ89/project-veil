use serde::{Serialize, Deserialize};
use aes_gcm::{Aes256Gcm, aead::{Aead, KeyInit, Nonce}};
use x25519_dalek::{EphemeralSecret, PublicKey};
use rand::RngCore;
use rand;
use std::convert::TryInto;


#[derive(Serialize, Deserialize, Debug)]
pub enum ProtocolMessage {
    JoinRequest {
        username: String,
        // Client’s X25519 encryption public key (32 bytes)
        encryption_pub: Vec<u8>,
        // Client’s Ed25519 signing public key (32 bytes)
        signing_pub: Vec<u8>,
    },
    JoinResponse {
        // Group key (K0) encrypted for the client.
        encrypted_group_key: Vec<u8>,
        // Admin’s signing public key (Ed25519)
        admin_signing_pub: Vec<u8>,
        // Admin’s encryption public key (X25519)
        admin_encryption_pub: Vec<u8>,
    },
    ChatMessage {
        username: String,
        msg_id: String,
        // In a full implementation, this would be the ciphertext.
        ciphertext: Vec<u8>,
        // Digital signature over (ciphertext || msg_id)
        signature: Vec<u8>,
    },
}


pub trait VecToArray32 {
    /// Attempts to convert the Vec<u8> to a [u8; 32]. Returns an error if the length is not exactly 32.
    fn to_array32(&self) -> Result<[u8; 32], String>;
}

impl VecToArray32 for Vec<u8> {
    fn to_array32(&self) -> Result<[u8; 32], String> {
        if self.len() != 32 {
            Err(format!("Expected vector length of 32, but found {}", self.len()))
        } else {
            // Since we've confirmed the length, try_into() will succeed.
            self.as_slice().try_into().map_err(|_| "Conversion failed".to_string())
        }
    }
}

/// Crypto functions for encrypting/decrypting the group key.
pub mod crypto {
    use super::*;
    use aes_gcm::{Aes256Gcm, aead::{Aead, KeyInit, Nonce}};
    use x25519_dalek::{PublicKey, StaticSecret};
    use rand::RngCore;
    use rand;

    /// Encrypt the plaintext (group key) using a shared secret derived from
    /// the admin’s private key and the client’s public key.
    pub fn encrypt_group_key(
        plaintext: &[u8],
        admin_private: StaticSecret,
        client_public: &PublicKey,
    ) -> Vec<u8> {
        // Compute the shared secret via X25519 Diffie–Hellman.
        let shared_secret = admin_private.diffie_hellman(client_public);
        let key_bytes = shared_secret.as_bytes();

        // Use the shared secret as the AES-256-GCM key.
        let cipher = Aes256Gcm::new_from_slice(key_bytes).expect("AES key error");
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::<Aes256Gcm>::from_slice(&nonce_bytes); // 96-bit nonce

        // Encrypt the plaintext (group key).
        let ciphertext = cipher.encrypt(nonce, plaintext)
            .expect("encryption failure!");

        // Prepend the nonce so that it can be used in decryption.
        [nonce_bytes.to_vec(), ciphertext].concat()
    }

    /// Decrypt the encrypted group key using the client’s private key and the admin’s public key.
    pub fn decrypt_group_key(
        ciphertext_with_nonce: &[u8],
        client_private: StaticSecret,
        admin_public: &PublicKey,
    ) -> Vec<u8> {
        // Split the nonce (first 12 bytes) and the ciphertext.
        let (nonce_bytes, ciphertext) = ciphertext_with_nonce.split_at(12);
        let nonce = Nonce::<Aes256Gcm>::from_slice(&nonce_bytes);

        // Compute the shared secret via X25519.
        let shared_secret = client_private.diffie_hellman(admin_public);
        let key_bytes = shared_secret.as_bytes();

        let cipher = Aes256Gcm::new_from_slice(key_bytes).expect("AES key error");
        cipher.decrypt(nonce, ciphertext)
            .expect("decryption failure!")
    }
}