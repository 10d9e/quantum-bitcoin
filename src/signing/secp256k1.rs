use crate::signing::SigningAlgorithm;
use secp256k1::ecdsa::Signature;
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Secp256k1Algorithm;

impl SigningAlgorithm for Secp256k1Algorithm {
    type PrivateKey = SecretKey;
    type PublicKey = PublicKey;
    type Signature = Signature;

    fn generate_keypair() -> (Self::PrivateKey, Self::PublicKey) {
        let secp = Secp256k1::new();
        secp.generate_keypair(&mut rand::thread_rng())
    }

    fn sign_message(private_key: &Self::PrivateKey, message: &str) -> Self::Signature {
        let secp = Secp256k1::new();
        let message_hash = Sha256::digest(message.as_bytes());
        let msg = Message::from_digest(message_hash.into());
        secp.sign_ecdsa(&msg, private_key)
    }

    fn verify_message(
        public_key: &Self::PublicKey,
        message: &str,
        signature: &Self::Signature,
    ) -> bool {
        let secp = Secp256k1::new();
        let message_hash = Sha256::digest(message.as_bytes());
        let msg = Message::from_digest(message_hash.into());
        secp.verify_ecdsa(&msg, signature, public_key).is_ok()
    }

    fn serialize_public_key(public_key: &Self::PublicKey) -> Vec<u8> {
        public_key.serialize().to_vec()
    }

    fn serialize_signature(signature: &Self::Signature) -> Vec<u8> {
        let sig = signature.serialize_der();
        sig.to_vec()
    }
}
