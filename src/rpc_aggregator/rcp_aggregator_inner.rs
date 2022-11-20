use super::rpc_request_data::Request;

pub struct RpcAggregatorInner<TItem: Send + Sync + 'static, TError: Send + Sync + 'static> {
    pub receiver: Option<tokio::sync::mpsc::UnboundedReceiver<()>>,
    pub queue: Vec<Request<TItem, TError>>,
}

impl<TItem: Send + Sync + 'static, TError: Send + Sync + 'static>
    RpcAggregatorInner<TItem, TError>
{
    pub fn new(receiver: tokio::sync::mpsc::UnboundedReceiver<()>) -> Self {
        Self {
            receiver: Some(receiver),
            queue: Vec::new(),
        }
    }
}
