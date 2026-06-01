use bytes::Bytes;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::context::ConnectionContext;
use crate::error::FutuError;
use crate::types::*;

pub const GET_SECURITY_SNAPSHOT_PROTO_ID: u32 = 3203;
pub const GET_BASIC_QOT_PROTO_ID: u32 = 3004;
pub const GET_KL_PROTO_ID: u32 = 3006;
pub const REQUEST_HISTORY_KL_PROTO_ID: u32 = 3103;
pub const GET_ORDER_BOOK_PROTO_ID: u32 = 3012;
pub const GET_TICKER_PROTO_ID: u32 = 3010;
pub const GET_MARKET_STATE_PROTO_ID: u32 = 3223;
pub const GET_CAPITAL_FLOW_PROTO_ID: u32 = 3211;
pub const GET_PLATE_SET_PROTO_ID: u32 = 3204;
pub const GET_PLATE_SECURITY_PROTO_ID: u32 = 3205;
pub const GET_STATIC_INFO_PROTO_ID: u32 = 3202;
pub const REQUEST_HISTORY_KL_QUOTA_PROTO_ID: u32 = 3104;

pub struct QuoteContext {
    ctx: Arc<Mutex<ConnectionContext>>,
}

impl QuoteContext {
    pub fn new(ctx: Arc<Mutex<ConnectionContext>>) -> Self {
        Self { ctx }
    }

    pub async fn get_market_snapshot(
        &self,
        codes: Vec<String>,
    ) -> Result<Vec<SnapshotData>, FutuError> {
        let mut ctx = self.ctx.lock().await;

        // Build request body
        let body = Bytes::new(); // TODO: Encode actual request

        let rx = ctx.send_request(GET_SECURITY_SNAPSHOT_PROTO_ID, body).await?;

        // Wait for response
        let response = tokio::time::timeout(
            std::time::Duration::from_secs(12),
            rx,
        )
        .await
        .map_err(|_| FutuError::Timeout)?
        .map_err(|_| FutuError::ConnectionLost)?;

        // Parse response - for now return empty
        // TODO: Parse actual response
        Ok(vec![])
    }

    pub async fn get_stock_quote(
        &self,
        codes: Vec<String>,
    ) -> Result<Vec<StockQuote>, FutuError> {
        let mut ctx = self.ctx.lock().await;

        let body = Bytes::new();
        let rx = ctx.send_request(GET_BASIC_QOT_PROTO_ID, body).await?;

        let _response = tokio::time::timeout(
            std::time::Duration::from_secs(12),
            rx,
        )
        .await
        .map_err(|_| FutuError::Timeout)?
        .map_err(|_| FutuError::ConnectionLost)?;

        Ok(vec![])
    }

    pub async fn get_cur_kline(
        &self,
        code: &str,
        num: u32,
        ktype: &str,
    ) -> Result<Vec<KlineBar>, FutuError> {
        let mut ctx = self.ctx.lock().await;

        let body = Bytes::new();
        let rx = ctx.send_request(GET_KL_PROTO_ID, body).await?;

        let _response = tokio::time::timeout(
            std::time::Duration::from_secs(12),
            rx,
        )
        .await
        .map_err(|_| FutuError::Timeout)?
        .map_err(|_| FutuError::ConnectionLost)?;

        Ok(vec![])
    }

    pub async fn request_history_kline(
        &self,
        code: &str,
        start: Option<&str>,
        end: Option<&str>,
        ktype: &str,
        max_count: u32,
    ) -> Result<Vec<KlineBar>, FutuError> {
        let mut ctx = self.ctx.lock().await;

        let body = Bytes::new();
        let rx = ctx.send_request(REQUEST_HISTORY_KL_PROTO_ID, body).await?;

        let _response = tokio::time::timeout(
            std::time::Duration::from_secs(12),
            rx,
        )
        .await
        .map_err(|_| FutuError::Timeout)?
        .map_err(|_| FutuError::ConnectionLost)?;

        Ok(vec![])
    }

    pub async fn get_order_book(
        &self,
        code: &str,
        num: u32,
    ) -> Result<OrderBook, FutuError> {
        let mut ctx = self.ctx.lock().await;

        let body = Bytes::new();
        let rx = ctx.send_request(GET_ORDER_BOOK_PROTO_ID, body).await?;

        let _response = tokio::time::timeout(
            std::time::Duration::from_secs(12),
            rx,
        )
        .await
        .map_err(|_| FutuError::Timeout)?
        .map_err(|_| FutuError::ConnectionLost)?;

        Ok(OrderBook {
            code: code.to_string(),
            bid: vec![],
            ask: vec![],
        })
    }

    pub async fn get_rt_ticker(
        &self,
        code: &str,
        num: u32,
    ) -> Result<Vec<Ticker>, FutuError> {
        let mut ctx = self.ctx.lock().await;

        let body = Bytes::new();
        let rx = ctx.send_request(GET_TICKER_PROTO_ID, body).await?;

        let _response = tokio::time::timeout(
            std::time::Duration::from_secs(12),
            rx,
        )
        .await
        .map_err(|_| FutuError::Timeout)?
        .map_err(|_| FutuError::ConnectionLost)?;

        Ok(vec![])
    }

    pub async fn get_market_state(
        &self,
        codes: Vec<String>,
    ) -> Result<Vec<MarketStateInfo>, FutuError> {
        let mut ctx = self.ctx.lock().await;

        let body = Bytes::new();
        let rx = ctx.send_request(GET_MARKET_STATE_PROTO_ID, body).await?;

        let _response = tokio::time::timeout(
            std::time::Duration::from_secs(12),
            rx,
        )
        .await
        .map_err(|_| FutuError::Timeout)?
        .map_err(|_| FutuError::ConnectionLost)?;

        Ok(vec![])
    }

    pub async fn get_capital_flow(
        &self,
        code: &str,
    ) -> Result<Vec<CapitalFlowData>, FutuError> {
        let mut ctx = self.ctx.lock().await;

        let body = Bytes::new();
        let rx = ctx.send_request(GET_CAPITAL_FLOW_PROTO_ID, body).await?;

        let _response = tokio::time::timeout(
            std::time::Duration::from_secs(12),
            rx,
        )
        .await
        .map_err(|_| FutuError::Timeout)?
        .map_err(|_| FutuError::ConnectionLost)?;

        Ok(vec![])
    }

    pub async fn get_plate_list(
        &self,
        market: &str,
    ) -> Result<Vec<PlateInfo>, FutuError> {
        let mut ctx = self.ctx.lock().await;

        let body = Bytes::new();
        let rx = ctx.send_request(GET_PLATE_SET_PROTO_ID, body).await?;

        let _response = tokio::time::timeout(
            std::time::Duration::from_secs(12),
            rx,
        )
        .await
        .map_err(|_| FutuError::Timeout)?
        .map_err(|_| FutuError::ConnectionLost)?;

        Ok(vec![])
    }

    pub async fn get_plate_stock(
        &self,
        plate_code: &str,
    ) -> Result<Vec<PlateStock>, FutuError> {
        let mut ctx = self.ctx.lock().await;

        let body = Bytes::new();
        let rx = ctx.send_request(GET_PLATE_SECURITY_PROTO_ID, body).await?;

        let _response = tokio::time::timeout(
            std::time::Duration::from_secs(12),
            rx,
        )
        .await
        .map_err(|_| FutuError::Timeout)?
        .map_err(|_| FutuError::ConnectionLost)?;

        Ok(vec![])
    }

    pub async fn get_stock_basicinfo(
        &self,
        market: &str,
        codes: Vec<String>,
    ) -> Result<Vec<SnapshotData>, FutuError> {
        let mut ctx = self.ctx.lock().await;

        let body = Bytes::new();
        let rx = ctx.send_request(GET_STATIC_INFO_PROTO_ID, body).await?;

        let _response = tokio::time::timeout(
            std::time::Duration::from_secs(12),
            rx,
        )
        .await
        .map_err(|_| FutuError::Timeout)?
        .map_err(|_| FutuError::ConnectionLost)?;

        Ok(vec![])
    }

    pub async fn get_history_kl_quota(&self) -> Result<HistoryKlQuota, FutuError> {
        let mut ctx = self.ctx.lock().await;

        let body = Bytes::new();
        let rx = ctx.send_request(REQUEST_HISTORY_KL_QUOTA_PROTO_ID, body).await?;

        let _response = tokio::time::timeout(
            std::time::Duration::from_secs(12),
            rx,
        )
        .await
        .map_err(|_| FutuError::Timeout)?
        .map_err(|_| FutuError::ConnectionLost)?;

        Ok(HistoryKlQuota {
            used_count: 0,
            total_count: 0,
            remain_count: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quote_context_creation() {
        // Test that quote context can be created
        // (actual connection test requires FutuOpenD running)
    }
}
