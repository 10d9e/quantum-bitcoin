use crate::signing::SigningAlgorithm;
use pqcrypto_dilithium::dilithium2::*;
use pqcrypto_traits::sign::PublicKey as PublicKeyTrait;
use pqcrypto_traits::sign::SignedMessage;

#[derive(Debug)]
pub struct DilithiumAlgorithm;

impl SigningAlgorithm for DilithiumAlgorithm {
    type PrivateKey = pqcrypto_dilithium::dilithium2::SecretKey;
    type PublicKey = pqcrypto_dilithium::dilithium2::PublicKey;
    type Signature = pqcrypto_dilithium::dilithium2::SignedMessage;

    fn generate_keypair() -> (Self::PrivateKey, Self::PublicKey) {
        let (public_key, secret_key) = keypair();
        (secret_key, public_key)
    }

    fn sign_message(private_key: &Self::PrivateKey, message: &str) -> Self::Signature {
        sign(message.as_bytes(), private_key)
    }

    fn verify_message(
        public_key: &Self::PublicKey,
        message: &str,
        signature: &Self::Signature,
    ) -> bool {
        let verifiedmsg = open(&signature, &public_key).unwrap();
        verifiedmsg == message.as_bytes()
    }

    fn serialize_public_key(public_key: &Self::PublicKey) -> Vec<u8> {
        public_key.as_bytes().to_vec()
    }

    fn serialize_signature(signature: &Self::Signature) -> Vec<u8> {
        signature.as_bytes().to_vec()
    }
}
