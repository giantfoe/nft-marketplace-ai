# NFT Marketplace API Documentation

## Overview

This API provides endpoints for generating AI images, minting NFTs, and managing the NFT marketplace. All endpoints return JSON responses with a consistent structure.

## Base URL
```
http://localhost:3001/api/v1
```

## Response Format

All API responses follow this structure:

```json
{
  "success": boolean,
  "data": object | null,
  "error": {
    "code": string,
    "message": string,
    "details": object | null
  } | null,
  "message": string | null
}
```

## Authentication

Most endpoints require wallet signature authentication. You'll need to:

1. Connect to a Solana wallet (Phantom, Solflare, etc.)
2. Sign messages using the wallet
3. Include the signature and original message in API requests

## Endpoints

### 1. Generate AI Images

Generate AI images from text prompts.

**Endpoint:** `POST /api/v1/images/generate`

**Request Body:**
```json
{
  "prompt": "A majestic lion standing on a cliff at sunset",
  "style": "realistic", // optional
  "count": 1 // optional, 1-4 images
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "images": [
      {
        "id": "abc123",
        "url": "https://ai-statics.freepik.com/...",
        "prompt": "A majestic lion standing on a cliff at sunset",
        "style": "realistic",
        "created_at": "2025-10-30T13:45:00Z"
      }
    ],
    "request_id": "req_123456"
  }
}
```

**Frontend Usage:**
```javascript
const generateImages = async (prompt, style = null, count = 1) => {
  const response = await fetch('/api/v1/images/generate', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ prompt, style, count })
  });

  const result = await response.json();
  if (result.success) {
    return result.data.images;
  } else {
    throw new Error(result.error.message);
  }
};
```

### 2. Mint NFT

Mint an NFT using a pre-generated image.

**Endpoint:** `POST /api/v1/nfts/mint`

**Request Body:**
```json
{
  "name": "My Awesome NFT",
  "symbol": "AWESOME",
  "description": "An amazing AI-generated artwork", // optional
  "image_url": "https://ai-statics.freepik.com/...",
  "attributes": [ // optional
    {
      "trait_type": "Background",
      "value": "Mountain"
    }
  ],
  "creator_address": "YourWalletAddressHere",
  "signature": "base64_encoded_signature",
  "message": "Mint NFT: My Awesome NFT at timestamp"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "nft_address": "ABC123...",
    "transaction_signature": "TX123...",
    "image_short_url": "http://localhost:3001/image/abc123",
    "metadata_url": "http://localhost:3001/image/abc123",
    "fee_breakdown": {
      "mint_account_rent": 1461600,
      "metadata_account_rent": 5616720,
      "master_edition_rent": 2853600,
      "transaction_fee": 5000,
      "total_minting_cost": 9936920,
      "platform_fee": 993692,
      "total_fee": 10930612
    },
    "minted_at": "2025-10-30T13:45:00Z"
  }
}
```

**Frontend Usage:**
```javascript
const mintNFT = async (nftData, selectedImage) => {
  // Create signature message
  const message = `Mint NFT: ${nftData.name} at ${Date.now()}`;

  // Sign the message (using wallet adapter)
  const signature = await signMessage(new TextEncoder().encode(message));

  const requestData = {
    name: nftData.name,
    symbol: nftData.symbol,
    description: nftData.description,
    image_url: selectedImage.url,
    attributes: nftData.attributes,
    creator_address: publicKey.toBase58(),
    signature: Buffer.from(signature).toString('hex'),
    message
  };

  const response = await fetch('/api/v1/nfts/mint', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(requestData)
  });

  const result = await response.json();
  if (result.success) {
    return result.data;
  } else {
    throw new Error(result.error.message);
  }
};
```

### 3. Get Wallet NFTs

Retrieve NFTs owned by a wallet address.

**Endpoint:** `GET /api/v1/wallet/{address}/nfts`

**Query Parameters:**
- `limit` (optional): Number of NFTs to return (default: 20)
- `offset` (optional): Pagination offset (default: 0)

**Response:**
```json
{
  "success": true,
  "data": {
    "nfts": [
      {
        "address": "NFT123...",
        "name": "My NFT",
        "symbol": "MYNFT",
        "image_url": "http://localhost:3001/image/abc123",
        "metadata_url": "http://localhost:3001/image/abc123",
        "owner": "WalletAddress",
        "created_at": "2025-10-30T13:45:00Z"
      }
    ],
    "total_count": 1,
    "limit": 20,
    "offset": 0
  }
}
```

### 4. List NFT for Sale

List an NFT on the marketplace.

**Endpoint:** `POST /api/v1/marketplace/list`

**Request Body:**
```json
{
  "nft_address": "NFT123...",
  "price": 1000000, // Price in lamports
  "seller_address": "YourWalletAddress",
  "signature": "signature_hex",
  "message": "List NFT: NFT123... at timestamp"
}
```

### 5. Get Marketplace Listings

Get all NFTs listed for sale.

**Endpoint:** `GET /api/v1/marketplace/listings`

**Query Parameters:**
- `limit` (optional): Number of listings (default: 20)
- `offset` (optional): Pagination offset (default: 0)
- `sort_by` (optional): "price_asc", "price_desc", "recent"

### 6. Get Fee Estimates

Get estimated fees for various operations.

**Endpoint:** `GET /api/v1/fees/estimate`

**Response:**
```json
{
  "success": true,
  "data": {
    "mint_fee": {
      "mint_account_rent": 1461600,
      "metadata_account_rent": 5616720,
      "master_edition_rent": 2853600,
      "transaction_fee": 5000,
      "total_minting_cost": 9936920,
      "platform_fee": 993692,
      "total_fee": 10930612
    },
    "list_fee": 5000,
    "buy_fee": 5000
  }
}
```

### 7. Health Check

Check API health and version.

**Endpoint:** `GET /api/v1/health`

**Response:**
```json
{
  "success": true,
  "data": {
    "status": "healthy",
    "timestamp": "2025-10-30T13:45:00Z",
    "version": "0.1.0"
  }
}
```

## Error Codes

- `INVALID_PROMPT`: Prompt validation failed
- `PROMPT_TOO_LONG`: Prompt exceeds 1000 characters
- `SERVICE_UNAVAILABLE`: AI service not available
- `GENERATION_FAILED`: Image generation failed
- `INVALID_INPUT`: Invalid request parameters
- `MINT_FAILED`: NFT minting failed
- `LIST_FAILED`: NFT listing failed
- `FEE_ESTIMATE_FAILED`: Fee calculation failed

## Rate Limiting

- Image generation: 10 requests per minute per IP
- NFT operations: 5 requests per minute per wallet

## Frontend Integration Example

```javascript
// Complete NFT creation flow
const createNFT = async () => {
  try {
    // Step 1: Generate images
    const images = await generateImages("A beautiful dragon", "fantasy", 3);

    // Step 2: User selects an image (UI interaction)
    const selectedImage = images[0]; // User selection

    // Step 3: Mint NFT with selected image
    const nftData = {
      name: "Dragon NFT",
      symbol: "DRAGON",
      description: "A majestic fantasy dragon"
    };

    const mintedNFT = await mintNFT(nftData, selectedImage);

    console.log('NFT created:', mintedNFT);
    return mintedNFT;

  } catch (error) {
    console.error('NFT creation failed:', error);
    throw error;
  }
};
```

## Legacy Endpoints

The API also maintains backward compatibility with legacy endpoints:

- `POST /generate-image` - Single image generation
- `POST /mint-nft` - Direct NFT minting
- `GET /fee-estimate` - Fee estimates

## API Documentation

Interactive API documentation is available at:
```
http://localhost:3001/swagger-ui/
```

This provides a complete OpenAPI specification with request/response examples and the ability to test endpoints directly from the browser.