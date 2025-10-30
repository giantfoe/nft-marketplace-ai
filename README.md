# NFT Marketplace Platform

A decentralized NFT marketplace built on Solana blockchain, featuring minting, trading, and collection management.

## Project Overview

This platform allows users to:
- Mint NFTs with custom metadata
- Create and manage collections
- List NFTs for sale
- Buy NFTs from listings
- View marketplace with available NFTs

## Tech Stack

- **Blockchain**: Solana (Devnet/Testnet)
- **Smart Contracts**: Rust + Anchor Framework
- **Backend**: Rust (Axum web framework)
- **Frontend**: Next.js + React + Tailwind CSS
- **Database**: PostgreSQL (optional, currently placeholder)
- **IPFS**: For decentralized metadata storage (placeholder)
- **Wallets**: Phantom, Solflare

## Project Structure


new-nft/ ├── backend/                 # Rust API server │   ├── src/ │   │   ├── main.rs         # Server setup, routes │   │   ├── nft.rs          # NFT operations (mint, list, buy) │   │   └── google_api.rs   # Image generation (optional) │
├── Cargo.toml │   └── Cargo.lock ├── contracts/               # Anchor smart contracts │   ├── nft_marketplace/ │   │   ├── src/lib.rs      # Contract logic │   │   ├── Cargo.toml │   │   └── Anchor.toml │   └── programs/ ├── nft-marketplace/
# Frontend application │   ├── pages/ │   │   ├── index.js        # Main page with tabs │   │   └── _app.js         # App wrapper with wallet provider │   ├── components/ │   │   ├── MintForm.js     # NFT minting form │   │   └── Marketplace.js
# NFT listings display │   ├── styles/globals.css  # Tailwind styles │   ├── package.json │   ├── tailwind.config.js │   └── index.html          # Simple HTML test interface ├── database/                # Database schema │   └── schema.sql
# PostgreSQL tables ├── scripts/                 # Legacy TypeScript scripts │   ├── create-collection.ts │   ├── create-nft.ts │   └── verify-ntf.ts ├── image-generation/        # AI image generation (optional) ├── smart-contracts-test/    #
Contract testing ├── docs/                    # Documentation │   ├── STRUCTURE_PLAN.md   # Development plan │   └── nft_platform_prd 2.md # Product requirements ├── .env                     # Environment variables (DO NOT COMMIT) ├─
.gitignore               # Git ignore rules ├── package.json             # Root dependencies └── README.md                # This file


## Development Roadmap

### Phase 1: Core Infrastructure (Current)
- [x] Project structure setup
- [x] Rust backend with Axum
- [x] Anchor smart contract skeleton
- [x] Next.js frontend setup
- [x] Wallet integration (Phantom, Solflare)
- [x] Basic HTML test interface

### Phase 2: NFT Minting
- [x] Backend mint NFT endpoint
- [x] Contract mint instruction
- [ ] IPFS metadata upload
- [ ] Frontend mint form integration
- [ ] Collection creation

### Phase 3: Marketplace Trading
- [ ] Contract list/buy instructions
- [ ] Backend list/buy endpoints
- [ ] Frontend marketplace display
- [ ] Buy NFT functionality

### Phase 4: Advanced Features
- [ ] Database integration
- [ ] User authentication
- [ ] Auction system
- [ ] Analytics dashboard
- [ ] Mobile responsive design

### Phase 5: Production
- [ ] Testing and audits
- [ ] Deployment (Vercel, Fly.io)
- [ ] Mainnet migration
- [ ] Security hardening

## Setup Instructions

### Prerequisites
- Rust (latest stable)
- Node.js 18+
- Anchor CLI
- Solana CLI
- PostgreSQL (optional)
- Phantom/Solflare wallet

### Backend Setup
```bash
cd backend
cargo build
cargo run

### Contract Setup

cd contracts/nft_marketplace
anchor build
anchor deploy

### Frontend Setup

cd nft-marketplace
npm install
npm run dev
# Or use HTML: python3 -m http.server 3000

### Environment Variables

Copy .env and fill:

• SOLANA_RPC_URL
• SOLANA_PRIVATE_KEY
• DATABASE_URL (optional)
• GOOGLE_API_KEY (optional)

## API Endpoints

• GET / - Health check
• POST /mint-nft - Mint new NFT
• POST /create-collection - Create collection
• POST /list-nft - List NFT for sale
• POST /buy-nft - Buy listed NFT
• GET /nfts - Get all NFTs

## Smart Contract Instructions

• mint_nft - Create NFT with metadata
• list_nft - List NFT for sale
• buy_nft - Purchase listed NFT

## Testing

### Backend

cd backend
cargo test

### Contracts

cd contracts/nft_marketplace
anchor test

### Frontend

cd nft-marketplace
npm test

## Deployment

### Backend

# Using Fly.io or similar
fly deploy

### Frontend

# Using Vercel
vercel --prod

### Contracts

anchor deploy --provider.cluster mainnet

## Contributing

1. Fork the repository
2. Create feature branch
3. Make changes
4. Test thoroughly
5. Submit PR

## Security

• Never commit private keys
• Use environment variables for secrets
• Audit smart contracts before mainnet
• Implement rate limiting
• Sanitize all inputs

## License

MIT License

## Contact

For questions or issues, please open a GitHub issue.