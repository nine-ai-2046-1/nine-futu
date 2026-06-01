use bytes::Bytes;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::context::ConnectionContext;
use crate::error::FutuError;
use crate::types::*;

pub const SUBSCRIBE_PROTO_ID: u32 = 3001;
pub const GET_SUB_INFO_PROTO_ID: u32 = 3003;

pub struct SubscriptionContext {
    ctx: Arc<Mutex<ConnectionContext>>,
    subscriptions: Vec<SubscriptionInfo>,
}

impl SubscriptionContext {
    pub fn new(ctx: Arc<Mutex<ConnectionContext>>) -> Self {
        Self {
            ctx,
            subscriptions: Vec::new(),
        }
    }

    pub async fn subscribe(
        &mut self,
        codes: Vec<String>,
        sub_types: Vec<SubType>,
    ) -> Result<(), FutuError> {
        let mut ctx = self.ctx.lock().await;

        // Build request body
        let body = Bytes::new(); // TODO: Encode actual request

        let rx = ctx.send_request(SUBSCRIBE_PROTO_ID, body).await?;

        // Wait for response
        let _response = tokio::time::timeout(
            std::time::Duration::from_secs(12),
            rx,
        )
        .await
        .map_err(|_| FutuError::Timeout)?
        .map_err(|_| FutuError::ConnectionLost)?;

        // Update local subscription state
        for code in codes {
            for sub_type in &sub_types {
                self.subscriptions.push(SubscriptionInfo {
                    code: code.clone(),
                    sub_type: *sub_type,
                    is_subscribed: true,
                });
            }
        }

        Ok(())
    }

    pub async fn unsubscribe(
        &mut self,
        codes: Vec<String>,
        sub_types: Vec<SubType>,
    ) -> Result<(), FutuError> {
        let mut ctx = self.ctx.lock().await;

        let body = Bytes::new();
        let rx = ctx.send_request(SUBSCRIBE_PROTO_ID, body).await?;

        let _response = tokio::time::timeout(
            std::time::Duration::from_secs(12),
            rx,
        )
        .await
        .map_err(|_| FutuError::Timeout)?
        .map_err(|_| FutuError::ConnectionLost)?;

        // Update local subscription state
        for code in codes {
            for sub_type in &sub_types {
                self.subscriptions.retain(|s| {
                    !(s.code == code && s.sub_type == *sub_type)
                });
            }
        }

        Ok(())
    }

    pub async fn unsubscribe_all(&mut self) -> Result<(), FutuError> {
        let mut ctx = self.ctx.lock().await;

        let body = Bytes::new();
        let rx = ctx.send_request(SUBSCRIBE_PROTO_ID, body).await?;

        let _response = tokio::time::timeout(
            std::time::Duration::from_secs(12),
            rx,
        )
        .await
        .map_err(|_| FutuError::Timeout)?
        .map_err(|_| FutuError::ConnectionLost)?;

        self.subscriptions.clear();

        Ok(())
    }

    pub async fn query_subscription(&self) -> Result<Vec<SubscriptionInfo>, FutuError> {
        let mut ctx = self.ctx.lock().await;

        let body = Bytes::new();
        let rx = ctx.send_request(GET_SUB_INFO_PROTO_ID, body).await?;

        let _response = tokio::time::timeout(
            std::time::Duration::from_secs(12),
            rx,
        )
        .await
        .map_err(|_| FutuError::Timeout)?
        .map_err(|_| FutuError::ConnectionLost)?;

        Ok(self.subscriptions.clone())
    }

    pub fn is_subscribed(&self, code: &str, sub_type: &SubType) -> bool {
        self.subscriptions.iter().any(|s| {
            s.code == code && s.sub_type == *sub_type && s.is_subscribed
        })
    }

    pub fn subscription_count(&self) -> usize {
        self.subscriptions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscription_tracking() {
        // Test local subscription tracking
        let mut subs = Vec::new();
        subs.push(SubscriptionInfo {
            code: "HK.00700".to_string(),
            sub_type: SubType::Quote,
            is_subscribed: true,
        });

        assert_eq!(subs.len(), 1);
        assert!(subs[0].is_subscribed);
    }
}
