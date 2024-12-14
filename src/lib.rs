use crate::signing::SigningAlgorithm;
use sha2::Digest;
use sha2::Sha256;

pub mod signing;

#[derive(Debug)]
pub struct Wallet<A: SigningAlgorithm> {
    private_key: A::PrivateKey,
    public_key: A::PublicKey,
}

impl<A: SigningAlgorithm> Default for Wallet<A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A: SigningAlgorithm> Wallet<A> {
    pub fn new() -> Self {
        let (private_key, public_key) = A::generate_keypair();
        Self {
            private_key,
            public_key,
        }
    }

    pub fn get_address(&self) -> String {
        let public_key_bytes = A::serialize_public_key(&self.public_key);
        let mut hasher = Sha256::new();
        hasher.update(public_key_bytes);
        format!("{:x}", hasher.finalize())
    }
}

pub struct Transaction<A: SigningAlgorithm> {
    from: String,
    to: String,
    amount: u64,
    signature: Option<A::Signature>,
}

impl<A: SigningAlgorithm> std::fmt::Debug for Transaction<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let signature_debug = self
            .signature
            .as_ref()
            .map(|sig| hex::encode(A::serialize_signature(sig)));
        f.debug_struct("Transaction")
            .field("from", &self.from)
            .field("to", &self.to)
            .field("amount", &self.amount)
            .field("signature", &signature_debug)
            .finish()
    }
}

impl<A: SigningAlgorithm> Transaction<A> {
    pub fn new(from: String, to: String, amount: u64) -> Self {
        Self {
            from,
            to,
            amount,
            signature: None,
        }
    }

    pub fn sign_transaction(&mut self, wallet: &Wallet<A>) {
        let message = format!("{}{}{}", self.from, self.to, self.amount);
        self.signature = Some(A::sign_message(&wallet.private_key, &message));
    }

    pub fn is_valid(&self, wallet: &Wallet<A>) -> bool {
        if let Some(signature) = &self.signature {
            let message = format!("{}{}{}", self.from, self.to, self.amount);
            A::verify_message(&wallet.public_key, &message, signature)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct Block {
    index: u64,
    previous_hash: String,
    timestamp: u128,
    data: String,
    nonce: u64,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, previous_hash: String, data: String) -> Self {
        Self {
            index,
            previous_hash,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            data,
            nonce: 0,
            hash: String::new(),
        }
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}{}{}{}{}",
            self.index, self.previous_hash, self.timestamp, self.data, self.nonce
        ));
        format!("{:x}", hasher.finalize())
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        self.hash = self.calculate_hash(); // Ensure hash is initialized

        while self.hash[..difficulty.min(self.hash.len())] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        println!("Block mined: {}", self.hash);
    }
}
