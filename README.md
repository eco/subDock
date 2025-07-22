# SubDock - IntentSource Substreams

A dockerized Substreams setup that indexes IntentSource contract events from Base L2 blockchain into MongoDB.

## Prerequisites

- Docker and Docker Compose
- Alchemy API key for Base L2 network

## Setup

1. **Configure environment variables:**
   ```bash
   cp .env.example .env
   ```
   
2. **Add your Alchemy API key to `.env`:**
   ```env
   ALCHEMY_API_KEY=your_actual_api_key_here
   ```

3. **Build and start services:**
   ```bash
   docker-compose up --build
   ```

## Services

- **MongoDB** (port 27017): Database for storing indexed events
- **Mongo Express** (port 8081): Web interface for MongoDB
- **Substreams**: Processor that indexes blockchain data

## Contract Details

- **Contract Address**: `0x2020ae689ED3e017450280CEA110d0ef6E640Da4`
- **Blockchain**: Base L2 (via Alchemy RPC)
- **Events Indexed**:
  - IntentCreated
  - IntentFunded 
  - IntentPartiallyFunded
  - Withdrawal
  - Refund
  - IntentProofChallenged

## Access Points

- **Mongo Express UI**: http://localhost:8081
- **MongoDB Direct**: mongodb://admin:password@localhost:27017/substreams

## Commands

- **Start services**: `docker-compose up -d`
- **View logs**: `docker-compose logs -f substreams`
- **Stop services**: `docker-compose down`
- **Reset data**: `docker-compose down -v`

## Development

The Substreams module is built in Rust and outputs data to MongoDB collections. Each event type gets its own collection with appropriate indexes for efficient querying.