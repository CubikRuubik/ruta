use axum::{
    extract::State,
    http::StatusCode,
    response::sse::{Event, Sse},
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use database::entity::erc20_transfers::Erc20Transfers;
use futures::stream::{self, Stream};
use serde::{Deserialize, Serialize};
use sqlx::Pool;
use std::{convert::Infallible, time::Duration};
use tokio::sync::broadcast;
use tokio_stream::{wrappers::BroadcastStream, StreamExt as _};

use crate::erc20::Erc20Transfer;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool<sqlx::Postgres>,
    pub transfer_tx: broadcast::Sender<Erc20Transfer>,
}

#[derive(Serialize, Deserialize)]
pub struct TransferResponse {
    pub id: i64,
    pub block_number: i64,
    pub transaction_hash: String,
    pub log_index: i32,
    pub from_address: String,
    pub to_address: String,
    pub amount: String,
    pub contract_address: String,
    pub created_at: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TokenSummaryResponse {
    pub contract_address: String,
    pub total_transferred: String,
    pub symbol: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TokenSymbolResponse {
    pub contract_address: String,
    pub symbol: String,
}

impl From<Erc20Transfers> for TransferResponse {
    fn from(transfer: Erc20Transfers) -> Self {
        Self {
            id: transfer.id,
            block_number: transfer.block_number,
            transaction_hash: hex::encode(&transfer.transaction_hash),
            log_index: transfer.log_index,
            from_address: hex::encode(&transfer.from_address),
            to_address: hex::encode(&transfer.to_address),
            amount: transfer.amount.to_string(),
            contract_address: transfer.contract_address,
            created_at: transfer.created_at.map(|dt| dt.to_rfc3339()),
        }
    }
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/transfers", get(get_transfers))
        .route("/transfers/stream", get(stream_transfers))
        .route("/tokens/:address/summary", get(get_token_summary))
        .route("/tokens/:address/symbol", get(get_token_symbol_endpoint))
        .with_state(state)
}

async fn get_transfers(
    State(state): State<AppState>,
) -> Result<Json<Vec<TransferResponse>>, StatusCode> {
    let transfers = Erc20Transfers::find_all(100, &state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = transfers.into_iter().map(TransferResponse::from).collect();
    Ok(Json(response))
}

async fn get_token_symbol_endpoint(
    axum::extract::Path(address): axum::extract::Path<String>,
    State(state): State<AppState>,
) -> Result<Json<TokenSymbolResponse>, StatusCode> {
    let symbol = crate::service::get_token_symbol(1, &address, &state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = TokenSymbolResponse {
        contract_address: address,
        symbol,
    };
    Ok(Json(response))
}

async fn stream_transfers(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.transfer_tx.subscribe();
    let stream = BroadcastStream::new(rx);

    let event_stream = stream.map(|transfer| match transfer {
        Ok(transfer) => {
            let data = serde_json::to_string(&transfer).unwrap_or_default();
            Ok(Event::default().data(data))
        }
        Err(_) => Ok(Event::default().data("error")),
    });

    Sse::new(event_stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive"),
    )
}

async fn get_token_summary(
    axum::extract::Path(address): axum::extract::Path<String>,
    State(state): State<AppState>,
) -> Result<Json<TokenSummaryResponse>, StatusCode> {
    let total = Erc20Transfers::sum_amounts_by_contract_address(&address, &state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let symbol = crate::service::get_token_symbol(1, &address, &state.db_pool)
        .await
        .ok();

    let response = TokenSummaryResponse {
        contract_address: address,
        total_transferred: total.to_string(),
        symbol,
    };
    Ok(Json(response))
}
