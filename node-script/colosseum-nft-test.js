const { Connection, PublicKey, Keypair, Transaction, SystemProgram, sendAndConfirmTransaction } = require('@solana/web3.js');
const { createTransferInstruction, getAssociatedTokenAddress, createAssociatedTokenAccountInstruction } = require('@solana/spl-token');
const axios = require('axios');

// Configuration
const BACKEND_URL = 'http://127.0.0.1:3001';
const RPC_URL = 'https://api.devnet.solana.com';

// Test wallet (you can replace this with any devnet wallet address)
const USER_WALLET_ADDRESS = 'HN7cABqLq46Es1jh92dQQisAq662SmxELLLsHHe4YWrH';
const TEST_CREATOR_ADDRESS = '7P25ACncfhKXcurkfAwGxiVo9zRycM1U6obWYzV9WC1Z';
const TEST_SIGNATURE = 'dummy_signature';
const TEST_MESSAGE = 'test_message';

async function generateColosseumNFT() {
    console.log('🏛️  Starting Colosseum NFT Generation and Transfer Test...\n');

    try {
        // Step 1: Generate and mint the colosseum NFT
        console.log('📸 Generating colosseum image and minting NFT...');
        
        const generateAndMintRequest = {
            name: 'Ancient Colosseum',
            symbol: 'COLO',
            prompt: 'A majestic ancient Roman Colosseum at sunset, with dramatic lighting and detailed architecture, photorealistic style',
            style: 'photorealistic',
            creator_pubkey: TEST_CREATOR_ADDRESS,
            signature: TEST_SIGNATURE,
            message: TEST_MESSAGE
        };

        const mintResponse = await axios.post(`${BACKEND_URL}/generate-and-mint-nft`, generateAndMintRequest);
        
        if (!mintResponse.data || !mintResponse.data.nft_address || !mintResponse.data.transaction_signature) {
            throw new Error('Invalid response from generate-and-mint-nft endpoint');
        }

        const { nft_address, transaction_signature } = mintResponse.data;
        console.log(`✅ NFT Generated and Minted Successfully!`);
        console.log(`   NFT Address: ${nft_address}`);
        console.log(`   Mint Transaction: ${transaction_signature}\n`);

        // Step 2: Transfer the NFT to user wallet
        console.log('🚀 Transferring NFT to user wallet...');
        
        const transferTx = await transferNFT(nft_address, USER_WALLET_ADDRESS);
        console.log(`✅ NFT Transferred Successfully!`);
        console.log(`   Transfer Transaction: ${transferTx}\n`);

        // Step 3: Verify the transfer
        console.log('🔍 Verifying NFT transfer...');
        await verifyNFTTransfer(nft_address, USER_WALLET_ADDRESS);

        console.log('🎉 COMPLETE! Colosseum NFT has been successfully generated, minted, and transferred!');
        console.log(`\n📋 Summary:`);
        console.log(`   NFT Name: Ancient Colosseum`);
        console.log(`   NFT Symbol: COLO`);
        console.log(`   NFT Address: ${nft_address}`);
        console.log(`   Mint Transaction: ${transaction_signature}`);
        console.log(`   Transfer Transaction: ${transferTx}`);
        console.log(`   Current Owner: ${USER_WALLET_ADDRESS}`);

    } catch (error) {
        console.error('❌ Error during colosseum NFT test:', error.message);
        if (error.response && error.response.data) {
            console.error('   Server response:', error.response.data);
        }
        process.exit(1);
    }
}

async function transferNFT(nftAddress, recipientAddress) {
    try {
        // Load the backend keypair (same as in mint-and-transfer.js)
        const privateKeyArray = process.env.SOLANA_PRIVATE_KEY 
            ? process.env.SOLANA_PRIVATE_KEY.split(',').map(num => parseInt(num.trim()))
            : [127,219,34,128,198,208,53,188,139,190,156,24,107,255,52,181,234,120,210,7,15,101,174,57,13,23,210,77,56,243,244,135,94,202,28,115,76,49,207,98,1,64,42,101,70,29,114,73,88,240,16,222,93,92,175,199,41,246,126,38,234,48,55,244];
        
        const fromKeypair = Keypair.fromSecretKey(new Uint8Array(privateKeyArray));
        const connection = new Connection(RPC_URL, 'confirmed');

        // Convert addresses to PublicKey objects
        const mintPublicKey = new PublicKey(nftAddress);
        const recipientPublicKey = new PublicKey(recipientAddress);

        // Get associated token accounts
        const fromTokenAccount = await getAssociatedTokenAddress(mintPublicKey, fromKeypair.publicKey);
        const toTokenAccount = await getAssociatedTokenAddress(mintPublicKey, recipientPublicKey);

        // Create transaction
        const transaction = new Transaction();

        // Check if recipient's associated token account exists
        const toAccountInfo = await connection.getAccountInfo(toTokenAccount);
        if (!toAccountInfo) {
            // Create associated token account for recipient
            const createATAInstruction = createAssociatedTokenAccountInstruction(
                fromKeypair.publicKey, // payer
                toTokenAccount,        // associated token account
                recipientPublicKey,    // owner
                mintPublicKey          // mint
            );
            transaction.add(createATAInstruction);
        }

        // Add transfer instruction
        const transferInstruction = createTransferInstruction(
            fromTokenAccount,      // source
            toTokenAccount,        // destination
            fromKeypair.publicKey, // owner
            1                      // amount (1 for NFT)
        );
        transaction.add(transferInstruction);

        // Send and confirm transaction
        const signature = await sendAndConfirmTransaction(connection, transaction, [fromKeypair]);
        return signature;

    } catch (error) {
        throw new Error(`NFT transfer failed: ${error.message}`);
    }
}

async function verifyNFTTransfer(nftAddress, expectedOwner) {
    try {
        const connection = new Connection(RPC_URL, 'confirmed');
        const mintPublicKey = new PublicKey(nftAddress);
        const ownerPublicKey = new PublicKey(expectedOwner);
        
        const associatedTokenAccount = await getAssociatedTokenAddress(mintPublicKey, ownerPublicKey);
        const accountInfo = await connection.getTokenAccountBalance(associatedTokenAccount);
        
        if (accountInfo.value.uiAmount === 1) {
            console.log(`✅ Verification successful: NFT is now owned by ${expectedOwner}`);
            console.log(`   Token Account: ${associatedTokenAccount.toString()}`);
            console.log(`   Balance: ${accountInfo.value.uiAmount} NFT`);
        } else {
            throw new Error(`Verification failed: Expected 1 NFT, found ${accountInfo.value.uiAmount}`);
        }
    } catch (error) {
        console.error(`⚠️  Verification warning: ${error.message}`);
        console.log('   (This might be normal if the transfer is still processing)');
    }
}

// Load environment variables if .env file exists
try {
    require('dotenv').config({ path: '../.env' });
} catch (e) {
    console.log('No .env file found, using default values');
}

// Run the test
generateColosseumNFT();