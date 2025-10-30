const axios = require('axios');
const { Connection, Keypair, PublicKey, Transaction, sendAndConfirmTransaction } = require('@solana/web3.js');
const bs58 = require('bs58');
const splToken = require('@solana/spl-token');
const nacl = require('tweetnacl');
require('dotenv').config();

async function main() {
  try {
    // Load environment variables
    const rpcUrl = process.env.SOLANA_RPC_URL || 'https://api.devnet.solana.com';
    const privateKey = process.env.SOLANA_PRIVATE_KEY;
    if (!privateKey) throw new Error('SOLANA_PRIVATE_KEY not set');

    const connection = new Connection(rpcUrl, 'confirmed');
    const payer = Keypair.fromSecretKey(bs58.default.decode(privateKey));
    const backendUrl = 'http://127.0.0.1:3001'; // Adjust if needed
    const targetWallet = new PublicKey('7P25ACncfhKXcurkfAwGxiVo9zRycM1U6obWYzV9WC1Z');

    // Step 1: Prepare mint request
    const name = 'Test NFT';
    const symbol = 'TNFT';
    const uri = 'https://example.com/nft.json';
    const creatorPubkey = payer.publicKey.toBase58();
    const message = 'mint test ' + Date.now(); // Unique message
    const messageBytes = Buffer.from(message);

    // Sign the message
    const signature = nacl.sign.detached(messageBytes, payer.secretKey);
    const signatureBase58 = bs58.default.encode(signature);

    const mintRequest = {
      name,
      symbol,
      uri,
      creator_pubkey: creatorPubkey,
      signature: signatureBase58,
      message
    };

    // Step 2: Call /mint-nft endpoint
    const mintResponse = await axios.post(`${backendUrl}/mint-nft`, mintRequest);
    const { nft_address, transaction_signature } = mintResponse.data;
    console.log(`NFT Minted: ${nft_address}, Tx: ${transaction_signature}`);

    // Step 3: Transfer the NFT
    const nftMint = new PublicKey(nft_address);

    // Get associated token accounts
    const sourceTokenAccount = await splToken.getAssociatedTokenAddress(
      nftMint,
      payer.publicKey
    );

    const destinationTokenAccount = await splToken.getAssociatedTokenAddress(
      nftMint,
      targetWallet
    );

    // Check if destination ATA exists, create if not
    const destAccountInfo = await connection.getAccountInfo(destinationTokenAccount);
    let transaction = new Transaction();

    if (!destAccountInfo) {
      transaction.add(
        splToken.createAssociatedTokenAccountInstruction(
          payer.publicKey,
          destinationTokenAccount,
          targetWallet,
          nftMint
        )
      );
    }

    // Transfer instruction
    transaction.add(
      splToken.createTransferInstruction(
        sourceTokenAccount,
        destinationTokenAccount,
        payer.publicKey,
        1 // NFT amount
      )
    );

    const transferSignature = await sendAndConfirmTransaction(connection, transaction, [payer]);
    console.log(`NFT Transferred: Tx ${transferSignature}`);

  } catch (error) {
    console.error('Error:', error.message);
    if (error.response) console.error(error.response.data);
  }
}

main();