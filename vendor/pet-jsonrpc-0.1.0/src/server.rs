// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use crate::send_error;
use serde_json::{self, Value};
use std::{
    collections::HashMap,
    io::{self, Read},
    sync::Arc,
};

type RequestHandler<C> = Arc<dyn Fn(Arc<C>, u32, Value)>;
type NotificationHandler<C> = Arc<dyn Fn(Arc<C>, Value)>;
type ErrorHandler = Arc<dyn Fn(Option<u32>, i32, String)>;

pub struct HandlersKeyedByMethodName<C> {
    context: Arc<C>,
    requests: HashMap<&'static str, RequestHandler<C>>,
    notifications: HashMap<&'static str, NotificationHandler<C>>,
    send_error: ErrorHandler,
}

impl<C> HandlersKeyedByMethodName<C> {
    pub fn new(context: Arc<C>) -> Self {
        HandlersKeyedByMethodName {
            context,
            requests: HashMap::new(),
            notifications: HashMap::new(),
            send_error: Arc::new(send_error),
        }
    }

    #[cfg(test)]
    fn new_with_error_handler(
        context: Arc<C>,
        send_error: impl Fn(Option<u32>, i32, String) + 'static,
    ) -> Self {
        HandlersKeyedByMethodName {
            context,
            requests: HashMap::new(),
            notifications: HashMap::new(),
            send_error: Arc::new(send_error),
        }
    }

    pub fn add_request_handler<F>(&mut self, method: &'static str, handler: F)
    where
        F: Fn(Arc<C>, u32, Value) + Send + Sync + 'static,
    {
        self.requests.insert(
            method,
            Arc::new(move |context, id, params| {
                handler(context, id, params);
            }),
        );
    }

    pub fn add_notification_handler<F>(&mut self, method: &'static str, handler: F)
    where
        F: Fn(Arc<C>, Value) + Send + Sync + 'static,
    {
        self.notifications.insert(
            method,
            Arc::new(move |context, params| {
                handler(context, params);
            }),
        );
    }

    fn handle_request(&self, message: Value) {
        match message["method"].as_str() {
            Some(method) => {
                if let Some(id) = message["id"].as_u64() {
                    if let Some(handler) = self.requests.get(method) {
                        handler(self.context.clone(), id as u32, message["params"].clone());
                    } else {
                        eprint!("Failed to find handler for method: {method}");
                        (self.send_error)(
                            Some(id as u32),
                            -1,
                            format!("Failed to find handler for request {method}"),
                        );
                    }
                } else {
                    // No id, so this is a notification
                    if let Some(handler) = self.notifications.get(method) {
                        handler(self.context.clone(), message["params"].clone());
                    } else {
                        eprint!("Failed to find handler for method: {method}");
                    }
                }
            }
            None => {
                eprint!("Failed to get method from message: {message}");
                (self.send_error)(
                    message["id"].as_u64().map(|id| id as u32),
                    -3,
                    format!("Failed to extract method from JSONRPC payload {message:?}"),
                );
            }
        };
    }
}

/// Starts the jsonrpc server that listens for requests on stdin.
/// This function will block forever.
pub fn start_server<C>(handlers: &HandlersKeyedByMethodName<C>) -> ! {
    let mut stdin = io::stdin();
    loop {
        let mut input = String::new();
        match stdin.read_line(&mut input) {
            Ok(_) => {
                let mut empty_line = String::new();
                match get_content_length(&input) {
                    Ok(content_length) => {
                        let _ = stdin.read_line(&mut empty_line);
                        let mut buffer = vec![0; content_length];

                        match stdin.read_exact(&mut buffer) {
                            Ok(_) => {
                                let request =
                                    String::from_utf8_lossy(&buffer[..content_length]).to_string();
                                if let Err(err) = handle_payload(handlers, &request) {
                                    eprint!("Failed to parse LINE: {request}, {err:?}")
                                }
                                continue;
                            }
                            Err(err) => {
                                eprint!("Failed to read exactly {content_length} bytes, {err:?}")
                            }
                        }
                    }
                    Err(err) => eprint!("Failed to get content length from {input}, {err:?}"),
                };
            }
            Err(error) => eprint!("Error in reading a line from stdin: {error}"),
        }
    }
}

fn handle_payload<C>(
    handlers: &HandlersKeyedByMethodName<C>,
    payload: &str,
) -> Result<(), serde_json::Error> {
    let request = serde_json::from_str(payload)?;
    handlers.handle_request(request);
    Ok(())
}

/// Parses the content length from the given line.
fn get_content_length(line: &str) -> Result<usize, String> {
    let line = line.trim();
    if let Some(content_length) = line.find("Content-Length: ") {
        let start = content_length + "Content-Length: ".len();
        if let Ok(length) = line[start..].parse::<usize>() {
            Ok(length)
        } else {
            Err(format!(
                "Failed to parse content length from {} for {}",
                &line[start..],
                line
            ))
        }
    } else {
        Err(format!(
            "String 'Content-Length' not found in input => {line}"
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::sync::Mutex;

    #[derive(Default)]
    struct TestContext {
        request: Mutex<Option<(u32, Value)>>,
        notification: Mutex<Option<Value>>,
        errors: Mutex<Vec<(Option<u32>, i32, String)>>,
    }

    fn create_handlers_with_recorded_errors(
        context: Arc<TestContext>,
    ) -> HandlersKeyedByMethodName<TestContext> {
        let error_context = context.clone();
        HandlersKeyedByMethodName::new_with_error_handler(context, move |id, code, message| {
            error_context
                .errors
                .lock()
                .unwrap()
                .push((id, code, message));
        })
    }

    #[test]
    fn get_content_length_parses_valid_header() {
        assert_eq!(get_content_length("Content-Length: 42\r\n").unwrap(), 42);
    }

    #[test]
    fn get_content_length_rejects_missing_header() {
        let error = get_content_length("Content-Type: application/json\r\n").unwrap_err();

        assert!(error.contains("String 'Content-Length' not found"));
    }

    #[test]
    fn get_content_length_rejects_non_numeric_length() {
        let error = get_content_length("Content-Length: nope\r\n").unwrap_err();

        assert!(error.contains("Failed to parse content length"));
    }

    #[test]
    fn handle_request_routes_request_and_notification_messages() {
        let context = Arc::new(TestContext::default());
        let mut handlers = HandlersKeyedByMethodName::new(context.clone());
        handlers.add_request_handler("request/method", |context, id, params| {
            *context.request.lock().unwrap() = Some((id, params));
        });
        handlers.add_notification_handler("notification/method", |context, params| {
            *context.notification.lock().unwrap() = Some(params);
        });

        handlers.handle_request(json!({
            "jsonrpc": "2.0",
            "id": 7,
            "method": "request/method",
            "params": { "value": 42 }
        }));
        handlers.handle_request(json!({
            "jsonrpc": "2.0",
            "method": "notification/method",
            "params": ["item"]
        }));

        assert_eq!(
            *context.request.lock().unwrap(),
            Some((7, json!({ "value": 42 })))
        );
        assert_eq!(*context.notification.lock().unwrap(), Some(json!(["item"])));
    }

    #[test]
    fn handle_payload_routes_valid_jsonrpc_payload() {
        let context = Arc::new(TestContext::default());
        let mut handlers = HandlersKeyedByMethodName::new(context.clone());
        handlers.add_request_handler("request/method", |context, id, params| {
            *context.request.lock().unwrap() = Some((id, params));
        });

        handle_payload(
            &handlers,
            r#"{"jsonrpc":"2.0","id":9,"method":"request/method","params":{"ok":true}}"#,
        )
        .unwrap();

        assert_eq!(
            *context.request.lock().unwrap(),
            Some((9, json!({ "ok": true })))
        );
    }

    #[test]
    fn handle_payload_rejects_malformed_json_without_dispatching() {
        let context = Arc::new(TestContext::default());
        let mut handlers = HandlersKeyedByMethodName::new(context.clone());
        handlers.add_request_handler("request/method", |context, id, params| {
            *context.request.lock().unwrap() = Some((id, params));
        });

        let error = handle_payload(
            &handlers,
            r#"{"jsonrpc":"2.0","id":9,"method":"request/method","params": "#,
        )
        .unwrap_err();

        assert!(error.is_eof());
        assert!(context.request.lock().unwrap().is_none());
    }

    #[test]
    fn handle_request_reports_unknown_methods_without_invoking_known_handlers() {
        let context = Arc::new(TestContext::default());
        let mut handlers = create_handlers_with_recorded_errors(context.clone());
        handlers.add_request_handler("known/request", |context, id, params| {
            *context.request.lock().unwrap() = Some((id, params));
        });
        handlers.add_notification_handler("known/notification", |context, params| {
            *context.notification.lock().unwrap() = Some(params);
        });

        handlers.handle_request(json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "unknown/request",
            "params": null
        }));
        handlers.handle_request(json!({
            "jsonrpc": "2.0",
            "method": "unknown/notification",
            "params": null
        }));

        assert!(context.request.lock().unwrap().is_none());
        assert!(context.notification.lock().unwrap().is_none());
        assert_eq!(
            context.errors.lock().unwrap().as_slice(),
            &[(
                Some(1),
                -1,
                "Failed to find handler for request unknown/request".to_string()
            )]
        );
    }

    #[test]
    fn handle_request_reports_missing_method_with_request_id() {
        let context = Arc::new(TestContext::default());
        let handlers = create_handlers_with_recorded_errors(context.clone());

        let message = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "params": { "value": 42 }
        });

        handlers.handle_request(message.clone());

        assert_eq!(
            context.errors.lock().unwrap().as_slice(),
            &[(
                Some(1),
                -3,
                format!("Failed to extract method from JSONRPC payload {message:?}")
            )]
        );
    }

    #[test]
    fn handle_request_reports_missing_method_with_null_id() {
        let context = Arc::new(TestContext::default());
        let handlers = create_handlers_with_recorded_errors(context.clone());

        let message = json!({
            "jsonrpc": "2.0",
            "params": { "value": 42 }
        });

        handlers.handle_request(message.clone());

        assert_eq!(
            context.errors.lock().unwrap().as_slice(),
            &[(
                None,
                -3,
                format!("Failed to extract method from JSONRPC payload {message:?}")
            )]
        );
    }
}
