use std::sync::Arc;

use rust_extensions::TaskCompletion;

pub struct Request<
    TItem: Send + Sync + 'static,
    TResult: Send + Sync + 'static,
    TError: Send + Sync + 'static,
> {
    pub request_data: TItem,
    pub completion: TaskCompletion<TResult, Arc<TError>>,
    #[cfg(feature = "with-telemetry")]
    pub my_telemetry: my_telemetry::MyTelemetryContext,
}

pub struct RcpRequestData<
    TItem: Send + Sync + 'static,
    TResult: Send + Sync + 'static,
    TError: Send + Sync + 'static,
> {
    data: Option<Vec<TItem>>,
    completions: Vec<TaskCompletion<TResult, Arc<TError>>>,
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
        let mut data = Vec::with_capacity(requests.len());
        let mut completions = Vec::with_capacity(requests.len());

        for request in requests {
            data.push(request.request_data);
            completions.push(request.completion);
        }

        Self {
            data: Some(data),
            completions,
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
        if results.len() != self.completions.len() {
            return Err(format!(
                "amount of results {} != amount of requests {}",
                results.len(),
                self.completions.len()
            ));
        }

        for completion in &mut self.completions {
            let result = results.remove(0);
            if let Err(err) = completion.try_set_ok(result) {
                println!("can not set result: {:?}", err);
            }
        }

        Ok(())
    }

    pub fn set_result<TFn: Fn() -> TResult>(&mut self, result: TFn) -> Result<(), String> {
        for completion in &mut self.completions {
            if let Err(err) = completion.try_set_ok(result()) {
                println!("can not set result: {:?}", err);
            }
        }

        Ok(())
    }

    pub fn set_panic(mut self, message: &str) {
        for completion in &mut self.completions {
            if let Err(err) = completion.try_set_panic(message.to_string()) {
                println!("Can not set panic result to the task completion. {:?}", err);
            }
        }
    }

    pub fn set_error(mut self, err: TError) {
        let err = Arc::new(err);
        for completion in &mut self.completions {
            if let Err(err) = completion.try_set_error(err.clone()) {
                println!("set_error: {:?}", err);
            }
        }
    }
}
