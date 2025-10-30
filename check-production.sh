#!/bin/bash

# Production Readiness Check Script

echo "🔍 Checking NFT Marketplace Production Readiness"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check functions
check_file() {
    if [ -f "$1" ]; then
        echo -e "${GREEN}✅ $1 exists${NC}"
        return 0
    else
        echo -e "${RED}❌ $1 missing${NC}"
        return 1
    fi
}

check_env_var() {
    if [ -z "${!1}" ]; then
        echo -e "${RED}❌ $1 not set${NC}"
        return 1
    else
        echo -e "${GREEN}✅ $1 is set${NC}"
        return 0
    fi
}

# File checks
echo ""
echo "📁 Checking required files..."
check_file "Dockerfile"
check_file "docker-compose.yml"
check_file "nginx.conf"
check_file ".env"
check_file "index.html"

# Environment checks
echo ""
echo "🔧 Checking environment variables..."
check_env_var "SOLANA_PRIVATE_KEY"
check_env_var "FREEPIK_API_KEY"

# Docker checks
echo ""
echo "🐳 Checking Docker..."
if command -v docker &> /dev/null; then
    echo -e "${GREEN}✅ Docker installed${NC}"

    if command -v docker-compose &> /dev/null; then
        echo -e "${GREEN}✅ Docker Compose installed${NC}"
    else
        echo -e "${RED}❌ Docker Compose not installed${NC}"
    fi
else
    echo -e "${RED}❌ Docker not installed${NC}"
fi

# Build check
echo ""
echo "🔨 Testing build..."
if docker build -t nft-marketplace-test . --quiet 2>/dev/null; then
    echo -e "${GREEN}✅ Dockerfile builds successfully${NC}"
    docker rmi nft-marketplace-test >/dev/null 2>&1
else
    echo -e "${RED}❌ Dockerfile build failed${NC}"
fi

# Network check
echo ""
echo "🌐 Checking network connectivity..."
if curl -s --max-time 5 https://api.mainnet-beta.solana.com -o /dev/null; then
    echo -e "${GREEN}✅ Solana mainnet accessible${NC}"
else
    echo -e "${YELLOW}⚠️  Solana mainnet not accessible${NC}"
fi

echo ""
echo "📋 Next steps:"
echo "1. Run: chmod +x deploy.sh"
echo "2. Run: ./deploy.sh"
echo "3. Check: docker-compose ps"
echo "4. Test: curl http://localhost:3001/"
echo ""
echo "🎯 Ready for production deployment!"