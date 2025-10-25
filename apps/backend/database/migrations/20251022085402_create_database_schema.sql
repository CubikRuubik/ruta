CREATE TABLE IF NOT EXISTS evm_chains
(
    id BIGINT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    last_synced_block_number BIGINT NULL,
    block_time INTEGER NOT NULL,

    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE UNLOGGED TABLE IF NOT EXISTS evm_logs
(
    id SERIAL PRIMARY KEY,
    chain_id BIGINT NOT NULL REFERENCES evm_chains(id),
    block_number NUMERIC NOT NULL,
    block_hash BYTEA NOT NULL,
    address BYTEA NOT NULL,
    transaction_hash BYTEA NOT NULL,
    transaction_index BIGINT NOT NULL,
    log_index BIGINT NOT NULL,
    removed BOOL DEFAULT FALSE,
    data BYTEA,
    event_signature BYTEA,
    topics BYTEA[],

    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS evm_sync_logs
(
    address BYTEA PRIMARY KEY,
    last_synced_block_number BIGINT NOT NULL DEFAULT 0,

    chain_id BIGINT NOT NULL REFERENCES evm_chains(id),

    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_evm_chains_updated_at
BEFORE UPDATE ON evm_chains
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_evm_sync_logs_updated_at
BEFORE UPDATE ON evm_sync_logs
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();


CREATE UNIQUE INDEX
  evm_logs_unique_on_chain_transaction_log_index
ON evm_logs (
  chain_id,
  transaction_hash,
  log_index
);