#[async_trait::async_trait]
pub trait RpcAggregatorCallback<TItem, TError> {
    async fn handle(
        &self,
        items: &[TItem],
        #[cfg(feature = "with-telemetry")] my_telemetry: &my_telemetry::MyTelemetryContext,
    ) -> Result<(), TError>;
}
