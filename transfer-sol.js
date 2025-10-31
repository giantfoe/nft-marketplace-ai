const { Connection, PublicKey, Keypair, SystemProgram, Transaction, sendAndConfirmTransaction } = require('@solana/web3.js');
const bs58 = require('bs58');

async function transferSol() {
    // Connect to devnet
    const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
    
    // Test wallet private key (from backend/.env)
    const testWalletPrivateKey = 'your_private_key_here'; // We'll need to get this from .env
    
    // Backend wallet public key
    const backendWalletPubkey = new PublicKey('E9XL4ozWJTdkcoEVm19RLvb6tSCkZCzmofJxwSGgRMq4');
    
    // Decode the private key and create keypair
    const testWalletKeypair = Keypair.fromSecretKey(bs58.decode(testWalletPrivateKey));
    
    console.log('Test wallet:', testWalletKeypair.publicKey.toString());
    console.log('Backend wallet:', backendWalletPubkey.toString());
    
    // Check balances before transfer
    const testWalletBalance = await connection.getBalance(testWalletKeypair.publicKey);
    const backendWalletBalance = await connection.getBalance(backendWalletPubkey);
    
    console.log('Test wallet balance:', testWalletBalance / 1e9, 'SOL');
    console.log('Backend wallet balance:', backendWalletBalance / 1e9, 'SOL');
    
    // Transfer 1 SOL (1,000,000,000 lamports)
    const transferAmount = 1000000000;
    
    const transaction = new Transaction().add(
        SystemProgram.transfer({
            fromPubkey: testWalletKeypair.publicKey,
            toPubkey: backendWalletPubkey,
            lamports: transferAmount,
        })
    );
    
    console.log('Sending transaction...');
    const signature = await sendAndConfirmTransaction(connection, transaction, [testWalletKeypair]);
    console.log('Transaction confirmed:', signature);
    
    // Check balances after transfer
    const newTestWalletBalance = await connection.getBalance(testWalletKeypair.publicKey);
    const newBackendWalletBalance = await connection.getBalance(backendWalletPubkey);
    
    console.log('New test wallet balance:', newTestWalletBalance / 1e9, 'SOL');
    console.log('New backend wallet balance:', newBackendWalletBalance / 1e9, 'SOL');
}

transferSol().catch(console.error);