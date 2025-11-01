# RUTA (Rust Universal Tauri Analytics)

Real-time ERC-20 Transfer Analytics Desktop App

## Overview

RUTA is a comprehensive desktop application built with Tauri, React, and Rust that provides real-time monitoring and analytics for ERC-20 token transfers on Ethereum and other EVM-compatible chains. The application combines a high-performance Rust backend with a modern React frontend to deliver real-time insights into blockchain transactions.

## Features

- **Real-time ERC-20 Transfer Monitoring**: Live streaming of token transfers with Server-Sent Events (SSE)
- **Multi-Chain Support**: Configurable support for multiple EVM chains
- **Database Integration**: PostgreSQL with automatic migrations and data persistence
- **Modern Desktop UI**: Built with React, TypeScript, and Tailwind CSS
- **Cross-Platform**: Runs on Windows, macOS, and Linux
- **High Performance**: Rust backend ensures low latency and efficient resource usage

## Architecture

### Backend (Rust)

- **Indexer Service**: Real-time blockchain data indexing using Alloy
- **Database Layer**: PostgreSQL with SQLx for type-safe database operations
- **REST API**: Axum-based HTTP server with SSE support
- **Migration System**: Automatic database schema management

### Frontend (React/TypeScript)

- **Tauri Integration**: Native desktop capabilities
- **Real-time Updates**: SSE-powered live data streaming
- **Data Visualization**: Interactive charts and analytics
- **Modern UI**: Responsive design with Radix UI components

## Project Structure

```
ruta/
├── apps/
│   ├── backend/
│   │   ├── database/          # Database models and migrations
│   │   │   ├── migrations/    # SQL migration files
│   │   │   └── src/
│   │   │       ├── entity/    # Database entities
│   │   │       └── lib.rs     # Database initialization
│   │   └── indexer/           # Blockchain indexer service
│   │       ├── src/
│   │       │   ├── erc20.rs   # ERC-20 transfer parsing
│   │       │   ├── server.rs  # HTTP server and SSE
│   │       │   ├── service.rs # Indexing logic
│   │       │   └── main.rs    # Application entry point
│   │       └── Cargo.toml
│   └── desktop/               # Tauri desktop application
│       ├── src/
│       │   ├── components/    # React components
│       │   ├── hooks/         # Custom React hooks
│       │   ├── lib/           # Utilities
│       │   └── store/         # State management
│       ├── src-tauri/         # Tauri Rust backend
│       └── package.json
├── src/                       # Root Rust application
├── Cargo.toml                 # Workspace configuration
└── package.json               # NPM workspace config
```

## Database Schema

The application uses PostgreSQL with the following main tables:

- **evm_chains**: Supported blockchain networks
- **evm_sync_logs**: Indexing progress tracking per contract
- **token_transfers**: Individual ERC-20 transfer records

## Getting Started

### Prerequisites

- **Rust**: Latest stable version
- **Node.js**: v18 or higher
- **PostgreSQL**: v12 or higher
- **Tauri CLI**: `npm install @tauri-apps/cli --save-dev`

### Installation

1. **Clone the repository**

   ```bash
   git clone https://github.com/CubikRuubik/ruta.git
   cd ruta
   ```

2. **Install dependencies**

   ```bash
   npm run install:all
   ```

3. **Set up PostgreSQL database**

   ```sql
   CREATE DATABASE indexer_db;
   psql -U "YOUR_USER_NAME" -d indexer_db -f 20251022085402_create_database_schema.sql
   ```

4. **Configure environment**

   ```bash
   cp apps/backend/indexer/.env.example apps/backend/indexer/.env
   # Edit .env with your database URL and RPC endpoints
   ```

5. **Run database migrations**
   ```bash
   cd apps/backend/database
   cargo sqlx prepare --database-url "postgresql://user:pass@localhost/indexer_db"
   ```

### Development

**Start development servers:**

```bash
npm run dev
```

This will start both the backend indexer and the Tauri development environment concurrently.
in case you need a build:

**Build for production:**

```bash
npm run build
```

### API Endpoints

- `GET /transfers` - Get recent token transfers
- `GET /transfers/stream` - SSE stream of real-time transfers
- `GET /tokens/:address/summary` - Token summary statistics
- `GET /tokens/:address/symbol` - Token symbol information
- `GET /tokens/summaries` - All tracked token summaries

### Environment Variables

**Backend (.env):**

```env
DATABASE_URL=postgresql://user:pass@localhost/indexer_db
DATABASE_MAX_CONNECTIONS=5
CONTRACT_ADDRESSES=0xA0b86a33E6441e88C5F2712C3E9b74F5b8b4b4b4,0x123...
```

## Development

### Adding New Chains

1. Add chain configuration to database seed in `apps/backend/database/src/lib.rs`
2. Update RPC URLs in environment variables
3. Restart the indexer service

### Custom Token Tracking

Add contract addresses to the `CONTRACT_ADDRESSES` environment variable (comma-separated).

## Technologies Used

- **Backend**: Rust, Axum, SQLx, Alloy, Tokio
- **Frontend**: React, TypeScript, Tailwind CSS, Radix UI
- **Desktop**: Tauri
- **Database**: PostgreSQL
- **Build Tools**: Cargo, Vite, Rollup,Tauri CLI
