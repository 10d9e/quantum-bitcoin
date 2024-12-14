use quantum_bitcoin::signing::secp256k1::Secp256k1Algorithm;
use quantum_bitcoin::Block;
use quantum_bitcoin::Transaction;
use quantum_bitcoin::Wallet;

fn main() {
    // Create wallets using Secp256k1Algorithm
    let wallet1: Wallet<Secp256k1Algorithm> = Wallet::default();
    let wallet2: Wallet<Secp256k1Algorithm> = Wallet::default();

    println!("Wallet 1 Address: {}", wallet1.get_address());
    println!("Wallet 2 Address: {}", wallet2.get_address());

    // Create a transaction and sign it
    let mut transaction: Transaction<Secp256k1Algorithm> =
        Transaction::new(wallet1.get_address(), wallet2.get_address(), 100);
    transaction.sign_transaction(&wallet1);

    println!("Transaction: {:?}", transaction);

    // Validate the transaction
    if transaction.is_valid(&wallet1) {
        println!("Transaction is valid.");
    } else {
        println!("Transaction is invalid.");
    }

    // Mining blocks
    let difficulty = 2;
    let mut genesis_block = Block::new(0, "0".to_string(), "Genesis Block".to_string());
    genesis_block.mine_block(difficulty);
    println!("Genesis Block: {:?}", genesis_block);

    let mut block2 = Block::new(1, genesis_block.hash.clone(), format!("{:?}", transaction));
    block2.mine_block(difficulty);
    println!("Block 2: {:?}", block2);
}
