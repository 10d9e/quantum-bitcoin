pub trait SigningAlgorithm {
    type PrivateKey;
    type PublicKey;
    type Signature;

    fn generate_keypair() -> (Self::PrivateKey, Self::PublicKey);
    fn sign_message(private_key: &Self::PrivateKey, message: &str) -> Self::Signature;
    fn verify_message(
        public_key: &Self::PublicKey,
        message: &str,
        signature: &Self::Signature,
    ) -> bool;

    fn serialize_public_key(public_key: &Self::PublicKey) -> Vec<u8>;
    fn serialize_signature(signature: &Self::Signature) -> Vec<u8>; // New method
}

pub mod dilithium;
pub mod secp256k1;
