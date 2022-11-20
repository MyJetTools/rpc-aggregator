use crate::rpc_request_data::Request;

pub struct RpcAggregatorInner<
    TItem: Send + Sync + 'static,
    TResult: Send + Sync + 'static,
    TError: Send + Sync + 'static,
> {
    pub receiver: Option<tokio::sync::mpsc::UnboundedReceiver<()>>,
    pub queue: Vec<Request<TItem, TResult, TError>>,
}

impl<
        TItem: Send + Sync + 'static,
        TResult: Send + Sync + 'static,
        TError: Send + Sync + 'static,
    > RpcAggregatorInner<TItem, TResult, TError>
{
    pub fn new(receiver: tokio::sync::mpsc::UnboundedReceiver<()>) -> Self {
        Self {
            receiver: Some(receiver),
            queue: Vec::new(),
        }
    }
}
