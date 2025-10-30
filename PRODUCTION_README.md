# 🚀 NFT Marketplace - Production Deployment

## Prerequisites

- Docker & Docker Compose
- Domain name (optional but recommended)
- SSL certificates (for HTTPS)
- Solana wallet with SOL for gas fees
- API keys (Freepik, etc.)

## Quick Start

1. **Clone and setup**:
   ```bash
   git clone <your-repo>
   cd nft-marketplace
   ```

2. **Configure environment**:
   ```bash
   cp .env.production .env
   # Edit .env with your production values
   ```

3. **Deploy**:
   ```bash
   chmod +x deploy.sh
   ./deploy.sh
   ```

## Environment Configuration

### Required Variables

```bash
# Solana
SOLANA_RPC_URL=https://api.mainnet-beta.solana.com
SOLANA_PRIVATE_KEY=your_mainnet_private_key_base58

# APIs
FREEPIK_API_KEY=your_freepik_api_key

# Domain (for CORS)
CORS_ORIGINS=https://yourdomain.com
```

### Getting a Solana Private Key

1. Create a new Solana wallet or use existing
2. Export private key as base58 string
3. Fund with SOL for transaction fees
4. **NEVER commit private keys to git**

## Deployment Options

### 1. Docker Compose (Recommended)

```bash
# Local deployment
docker-compose up -d

# With custom domain
docker-compose -f docker-compose.prod.yml up -d
```

### 2. Cloud Platforms

#### Railway
```bash
# Install Railway CLI
npm install -g @railway/cli

# Login and deploy
railway login
railway init
railway up
```

#### Render
1. Connect GitHub repo
2. Create Web Service
3. Use Dockerfile
4. Set environment variables

#### DigitalOcean App Platform
1. Create app from source
2. Use Dockerfile
3. Configure environment variables

### 3. VPS Deployment

```bash
# On your VPS
git clone <your-repo>
cd nft-marketplace
docker-compose up -d

# Setup nginx reverse proxy
sudo apt install nginx
sudo cp nginx.conf /etc/nginx/sites-available/nft-marketplace
sudo ln -s /etc/nginx/sites-available/nft-marketplace /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

## SSL Configuration

### Let's Encrypt (Free)

```bash
# Install certbot
sudo apt install certbot python3-certbot-nginx

# Get certificate
sudo certbot --nginx -d yourdomain.com

# Auto-renewal is automatic
```

### Manual SSL

1. Place certificates in `ssl/` directory
2. Update `nginx.conf` with SSL configuration
3. Uncomment HTTPS server block

## Monitoring & Maintenance

### Health Checks

```bash
# Check backend health
curl http://localhost:3001/

# Check docker services
docker-compose ps

# View logs
docker-compose logs -f backend
```

### Updates

```bash
# Pull latest changes
git pull origin main

# Rebuild and deploy
docker-compose build --no-cache
docker-compose up -d

# Clean up old images
docker image prune -f
```

### Backup

```bash
# Backup environment file
cp .env .env.backup

# Backup database (if added later)
docker exec nft-marketplace-db pg_dump -U user dbname > backup.sql
```

## Security Checklist

- [ ] Private keys not in git history
- [ ] Environment variables configured
- [ ] Firewall configured (only 80, 443, 22 open)
- [ ] SSL certificates installed
- [ ] CORS properly configured
- [ ] Rate limiting enabled
- [ ] Monitoring alerts setup
- [ ] Regular security updates

## Troubleshooting

### Common Issues

1. **Port already in use**:
   ```bash
   sudo lsof -i :3001
   sudo kill -9 <PID>
   ```

2. **Build fails**:
   ```bash
   docker-compose build --no-cache --progress=plain
   ```

3. **Service unhealthy**:
   ```bash
   docker-compose logs backend
   docker-compose restart backend
   ```

4. **Out of memory**:
   ```bash
   docker system prune -a
   # Increase server RAM
   ```

## Performance Optimization

- Use Redis for caching (future)
- Implement rate limiting
- Database indexing
- CDN for static assets
- Horizontal scaling with load balancer

## Support

For issues:
1. Check logs: `docker-compose logs -f`
2. Verify environment variables
3. Test locally first
4. Check network connectivity

---

🎉 **Your NFT Marketplace is now production-ready!**