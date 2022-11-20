use std::sync::Arc;

use rust_extensions::TaskCompletion;

pub struct Request<
    TItem: Send + Sync + 'static,
    TResult: Send + Sync + 'static,
    TError: Send + Sync + 'static,
> {
    pub request_data: Vec<TItem>,
    pub completion: TaskCompletion<Vec<TResult>, Arc<TError>>,

    #[cfg(feature = "with-telemetry")]
    pub my_telemetry: my_telemetry::MyTelemetryContext,
}

pub struct RcpRequestData<
    TItem: Send + Sync + 'static,
    TResult: Send + Sync + 'static,
    TError: Send + Sync + 'static,
> {
    data: Option<Vec<TItem>>,
    completions: Vec<(usize, TaskCompletion<Vec<TResult>, Arc<TError>>)>,
    amount: usize,
    #[cfg(feature = "with-telemetry")]
    my_telemetry: Option<my_telemetry::MyTelemetryContext>,
}

impl<
        TItem: Send + Sync + 'static,
        TResult: Send + Sync + 'static,
        TError: Send + Sync + 'static,
    > RcpRequestData<TItem, TResult, TError>
{
    pub fn new(
        requests: Vec<Request<TItem, TResult, TError>>,
        #[cfg(feature = "with-telemetry")] my_telemetry: my_telemetry::MyTelemetryContext,
    ) -> Self {
        let mut data = Vec::new();
        let mut completions = Vec::with_capacity(requests.len());

        let mut amount = 0;

        for request in requests {
            let chunk_size = request.request_data.len();
            amount += request.request_data.len();
            data.extend(request.request_data);
            completions.push((chunk_size, request.completion));
        }

        Self {
            data: Some(data),
            completions,
            amount,
            #[cfg(feature = "with-telemetry")]
            my_telemetry: Some(my_telemetry),
        }
    }

    pub fn get_data_to_callback(&mut self) -> Arc<Vec<TItem>> {
        let mut new_result = None;
        std::mem::swap(&mut new_result, &mut self.data);
        Arc::new(new_result.unwrap())
    }

    #[cfg(feature = "with-telemetry")]
    pub fn get_telemetry(&mut self) -> Arc<my_telemetry::MyTelemetryContext> {
        let mut new_result = None;
        std::mem::swap(&mut new_result, &mut self.my_telemetry);
        Arc::new(new_result.unwrap())
    }

    pub fn set_results(&mut self, mut results: Vec<TResult>) -> Result<(), String> {
        if results.len() != self.amount {
            return Err(format!(
                "amount of results {} != amount of requests {}",
                results.len(),
                self.completions.len()
            ));
        }

        for (amount, completion) in &mut self.completions {
            let mut chunk = Vec::with_capacity(*amount);
            for _ in 0..*amount {
                let result = results.remove(0);
                chunk.push(result);
            }

            if let Err(err) = completion.try_set_ok(chunk) {
                panic!(
                    "Can not set ok result to request completion with result: {:?}",
                    err
                );
            }
        }

        Ok(())
    }

    pub fn set_panic(mut self, message: &str) {
        for (_, completion) in &mut self.completions {
            if let Err(err) = completion.try_set_panic(message.to_string()) {
                println!("Can not set panic result to the task completion. {:?}", err);
            }
        }
    }

    pub fn set_error(mut self, err: TError) {
        let err = Arc::new(err);
        for (_, completion) in &mut self.completions {
            if let Err(err) = completion.try_set_error(err.clone()) {
                println!("set_error: {:?}", err);
            }
        }
    }
}
