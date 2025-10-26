CREATE TABLE IF NOT EXISTS evm_chains (
    id BIGINT PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    rpc_url TEXT,
    block_time INTEGER
);

CREATE TABLE IF NOT EXISTS evm_sync_logs (
    contract_address VARCHAR(42) NOT NULL PRIMARY KEY,
    last_synced_block_number BIGINT NOT NULL DEFAULT 0,
    chain_id BIGINT NOT NULL REFERENCES evm_chains(id)
);

CREATE TABLE IF NOT EXISTS token_transfers (
    id BIGSERIAL PRIMARY KEY,
    block_number BIGINT NOT NULL,
    transaction_hash BYTEA NOT NULL,
    log_index INTEGER NOT NULL,
    from_address BYTEA NOT NULL,
    to_address BYTEA NOT NULL,
    amount DECIMAL(78,0) NOT NULL,
    contract_address VARCHAR(42) NOT NULL REFERENCES evm_sync_logs(contract_address),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(transaction_hash, log_index)
);

CREATE INDEX idx_token_transfers_contract ON token_transfers(contract_address);
CREATE INDEX idx_token_transfers_block ON token_transfers(block_number);
CREATE INDEX idx_token_transfers_from ON token_transfers(from_address);
CREATE INDEX idx_token_transfers_to ON token_transfers(to_address);
CREATE INDEX idx_token_transfers_tx_hash ON token_transfers(transaction_hash);

CREATE INDEX idx_token_transfers_contract_block ON token_transfers(contract_address, block_number DESC);