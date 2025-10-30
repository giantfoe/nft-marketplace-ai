#!/bin/bash

# NFT Marketplace Production Deployment Script

set -e

echo "🚀 Starting NFT Marketplace Production Deployment"

# Check if .env exists
if [ ! -f ".env" ]; then
    echo "❌ .env file not found. Copy .env.production to .env and configure your values."
    exit 1
fi

# Build the application
echo "🔨 Building application..."
docker-compose build --no-cache

# Run database migrations if any
# echo "🗄️ Running database migrations..."
# docker-compose run --rm backend migrate

# Start the services
echo "🏃 Starting services..."
docker-compose up -d

# Wait for services to be healthy
echo "⏳ Waiting for services to be healthy..."
sleep 30

# Check health
echo "🔍 Checking service health..."
if curl -f http://localhost:3001/ > /dev/null 2>&1; then
    echo "✅ Backend is healthy!"
else
    echo "❌ Backend health check failed"
    exit 1
fi

echo "🎉 Deployment completed successfully!"
echo "🌐 Frontend: http://localhost"
echo "🔗 API: http://localhost/api"
echo ""
echo "To view logs: docker-compose logs -f"
echo "To stop: docker-compose down"