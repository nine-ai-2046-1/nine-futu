use bytes::Bytes;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::sync::{mpsc, oneshot, Mutex};
use tokio::time::Duration;

use crate::connection::{ConnState, FutuConnection};
use crate::error::FutuError;
use crate::proto_layer::{ProtoRequest, ProtoResponse};

pub const INIT_CONNECT_PROTO_ID: u32 = 1001;
pub const KEEP_ALIVE_PROTO_ID: u32 = 1004;
pub const GET_GLOBAL_STATE_PROTO_ID: u32 = 1002;

pub struct PendingRequest {
    pub serial_no: u32,
    pub response_tx: oneshot::Sender<ProtoResponse>,
}

pub struct ConnectionContext {
    conn: FutuConnection,
    pending_requests: Arc<Mutex<HashMap<u32, oneshot::Sender<ProtoResponse>>>>,
    push_tx: Option<mpsc::UnboundedSender<ProtoResponse>>,
    opend_conn_id: u64,
    keep_alive_interval: Duration,
    last_keep_alive: std::time::Instant,
    last_recv_time: std::time::Instant,
}

impl ConnectionContext {
    pub fn new(host: &str, port: u16) -> Self {
        let (push_tx, _push_rx) = mpsc::unbounded_channel();

        Self {
            conn: FutuConnection::new(host, port),
            pending_requests: Arc::new(Mutex::new(HashMap::new())),
            push_tx: Some(push_tx),
            opend_conn_id: 0,
            keep_alive_interval: Duration::from_secs(8),
            last_keep_alive: std::time::Instant::now(),
            last_recv_time: std::time::Instant::now(),
        }
    }

    pub async fn connect(&mut self) -> Result<(), FutuError> {
        self.conn.connect().await?;
        self.conn.set_state(ConnState::Connected);
        Ok(())
    }

    pub async fn init_connect(
        &mut self,
        client_ver: i32,
        client_id: &str,
        is_encrypt: bool,
    ) -> Result<(), FutuError> {
        // Proto ID 1001 - InitConnect
        let serial_no = self.conn.serial_mgr.next();

        // For now, send empty body - actual protobuf encoding will be added later
        let request = ProtoRequest::new(INIT_CONNECT_PROTO_ID, serial_no, Bytes::new());
        let data = request.to_bytes();

        // Send and wait for response
        let (tx, rx) = oneshot::channel();

        {
            let mut pending = self.pending_requests.lock().await;
            pending.insert(serial_no, tx);
        }

        // Send request
        if let Some(stream) = &mut self.conn.stream {
            stream.write_all(&data).await?;
        }

        // Wait for response with timeout
        let response = tokio::time::timeout(Duration::from_secs(12), rx)
            .await
            .map_err(|_| FutuError::Timeout)?
            .map_err(|_| FutuError::ConnectionLost)?;

        // Parse response - for now assume success
        // TODO: Parse actual InitConnect response to get conn_id, keep_alive_interval
        self.opend_conn_id = 1; // Placeholder
        self.keep_alive_interval = Duration::from_secs(10);
        self.last_keep_alive = std::time::Instant::now();
        self.last_recv_time = std::time::Instant::now();

        self.conn.set_state(ConnState::Ready);

        Ok(())
    }

    pub async fn send_request(
        &mut self,
        proto_id: u32,
        body: Bytes,
    ) -> Result<oneshot::Receiver<ProtoResponse>, FutuError> {
        let serial_no = self.conn.serial_mgr.next();
        let request = ProtoRequest::new(proto_id, serial_no, body.clone());
        let data = request.to_bytes();

        let (tx, rx) = oneshot::channel();

        {
            let mut pending = self.pending_requests.lock().await;
            pending.insert(serial_no, tx);
        }

        // Send request
        if let Some(stream) = &mut self.conn.stream {
            stream.write_all(&data).await?;
        }

        Ok(rx)
    }

    pub async fn keep_alive(&mut self) -> Result<(), FutuError> {
        let now = std::time::Instant::now();
        if now.duration_since(self.last_keep_alive) >= self.keep_alive_interval {
            let body = Bytes::new();
            self.send_request(KEEP_ALIVE_PROTO_ID, body).await?;
            self.last_keep_alive = now;
        }
        Ok(())
    }

    pub async fn get_global_state(&mut self) -> Result<(), FutuError> {
        let body = Bytes::new();
        let rx = self.send_request(GET_GLOBAL_STATE_PROTO_ID, body).await?;
        let _response = tokio::time::timeout(Duration::from_secs(12), rx)
            .await
            .map_err(|_| FutuError::Timeout)?
            .map_err(|_| FutuError::ConnectionLost)?;
        Ok(())
    }

    pub async fn close(&mut self) {
        self.conn.close().await;
    }

    pub fn is_connected(&self) -> bool {
        self.conn.is_connected()
    }

    pub fn state(&self) -> ConnState {
        self.conn.state()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_context_creation() {
        let ctx = ConnectionContext::new("127.0.0.1", 11111);
        assert!(!ctx.is_connected());
    }
}
