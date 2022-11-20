#[async_trait::async_trait]
pub trait RpcAggregatorWithResultCallback<TItem, TResult, TError> {
    async fn handle(
        &self,
        items: &[TItem],
        #[cfg(feature = "with-telemetry")] items: &my_telemetry::MyTelemetryContext,
    ) -> Result<Vec<TResult>, TError>;
}
