#!/bin/bash

set -e

echo "🔍 Testing SubDock Substreams Setup"
echo "================================="

# Check if .env file exists and has API key
if [ ! -f ".env" ]; then
    echo "❌ .env file not found. Please create it from .env.example"
    exit 1
fi

# Check if API key is set
if grep -q "placeholder_key_replace_with_real_key" .env; then
    echo "⚠️  Please update ALCHEMY_API_KEY in .env file with your actual key"
    echo "   You can get one from: https://dashboard.alchemy.com/"
    exit 1
fi

echo "✅ Environment configuration found"

# Check Docker
if ! command -v docker &> /dev/null; then
    echo "❌ Docker not found. Please install Docker first."
    exit 1
fi

if ! command -v docker-compose &> /dev/null; then
    echo "❌ Docker Compose not found. Please install Docker Compose first."
    exit 1
fi

echo "✅ Docker and Docker Compose found"

# Build and start services
echo "🚀 Building and starting services..."
docker-compose up -d --build

# Wait for services to be ready
echo "⏳ Waiting for services to start..."
sleep 30

# Check if services are running
if docker-compose ps | grep -q "Up"; then
    echo "✅ Services are running!"
    echo ""
    echo "📊 Access Points:"
    echo "  MongoDB UI: http://localhost:8081"
    echo "  MongoDB Direct: mongodb://admin:password@localhost:27017/substreams"
    echo ""
    echo "📝 Check logs with:"
    echo "  docker-compose logs -f substreams"
    echo ""
    echo "🛑 Stop services with:"
    echo "  docker-compose down"
else
    echo "❌ Services failed to start. Check logs with: docker-compose logs"
    exit 1
fi