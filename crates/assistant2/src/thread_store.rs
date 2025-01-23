use std::sync::Arc;

use anyhow::Result;
use assistant_tool::{ToolId, ToolWorkingSet};
use collections::HashMap;
use context_server::manager::ContextServerManager;
use context_server::{ContextServerFactoryRegistry, ContextServerTool};
use gpui::{prelude::*, AppContext, Model, ModelContext, Task};
use project::Project;
use unindent::Unindent;
use util::ResultExt as _;

use crate::thread::{Thread, ThreadId};

pub struct ThreadStore {
    #[allow(unused)]
    project: Model<Project>,
    tools: Arc<ToolWorkingSet>,
    context_server_manager: Model<ContextServerManager>,
    context_server_tool_ids: HashMap<Arc<str>, Vec<ToolId>>,
    threads: Vec<Model<Thread>>,
}

impl ThreadStore {
    pub fn new(
        project: Model<Project>,
        tools: Arc<ToolWorkingSet>,
        cx: &mut AppContext,
    ) -> Task<Result<Model<Self>>> {
        cx.spawn(|mut cx| async move {
            let this = cx.new_model(|cx: &mut ModelContext<Self>| {
                let context_server_factory_registry =
                    ContextServerFactoryRegistry::default_global(cx);
                let context_server_manager = cx.new_model(|cx| {
                    ContextServerManager::new(context_server_factory_registry, project.clone(), cx)
                });

                let mut this = Self {
                    project,
                    tools,
                    context_server_manager,
                    context_server_tool_ids: HashMap::default(),
                    threads: Vec::new(),
                };
                this.mock_recent_threads(cx);
                this.register_context_server_handlers(cx);

                this
            })?;

            Ok(this)
        })
    }

    /// Returns the number of non-empty threads.
    pub fn non_empty_len(&self, cx: &AppContext) -> usize {
        self.threads
            .iter()
            .filter(|thread| !thread.read(cx).is_empty())
            .count()
    }

    pub fn threads(&self, cx: &ModelContext<Self>) -> Vec<Model<Thread>> {
        let mut threads = self
            .threads
            .iter()
            .filter(|thread| !thread.read(cx).is_empty())
            .cloned()
            .collect::<Vec<_>>();
        threads.sort_unstable_by_key(|thread| std::cmp::Reverse(thread.read(cx).updated_at()));
        threads
    }

    pub fn recent_threads(&self, limit: usize, cx: &ModelContext<Self>) -> Vec<Model<Thread>> {
        self.threads(cx).into_iter().take(limit).collect()
    }

    pub fn create_thread(&mut self, cx: &mut ModelContext<Self>) -> Model<Thread> {
        let thread = cx.new_model(|cx| Thread::new(self.tools.clone(), cx));
        self.threads.push(thread.clone());
        thread
    }

    pub fn open_thread(&self, id: &ThreadId, cx: &mut ModelContext<Self>) -> Option<Model<Thread>> {
        self.threads
            .iter()
            .find(|thread| thread.read(cx).id() == id)
            .cloned()
    }

    pub fn delete_thread(&mut self, id: &ThreadId, cx: &mut ModelContext<Self>) {
        self.threads.retain(|thread| thread.read(cx).id() != id);
    }

    fn register_context_server_handlers(&self, cx: &mut ModelContext<Self>) {
        cx.subscribe(
            &self.context_server_manager.clone(),
            Self::handle_context_server_event,
        )
        .detach();
    }

    fn handle_context_server_event(
        &mut self,
        context_server_manager: Model<ContextServerManager>,
        event: &context_server::manager::Event,
        cx: &mut ModelContext<Self>,
    ) {
        let tool_working_set = self.tools.clone();
        match event {
            context_server::manager::Event::ServerStarted { server_id } => {
                if let Some(server) = context_server_manager.read(cx).get_server(server_id) {
                    let context_server_manager = context_server_manager.clone();
                    cx.spawn({
                        let server = server.clone();
                        let server_id = server_id.clone();
                        |this, mut cx| async move {
                            let Some(protocol) = server.client() else {
                                return;
                            };

                            if protocol.capable(context_server::protocol::ServerCapability::Tools) {
                                if let Some(tools) = protocol.list_tools().await.log_err() {
                                    let tool_ids = tools
                                        .tools
                                        .into_iter()
                                        .map(|tool| {
                                            log::info!(
                                                "registering context server tool: {:?}",
                                                tool.name
                                            );
                                            tool_working_set.insert(Arc::new(
                                                ContextServerTool::new(
                                                    context_server_manager.clone(),
                                                    server.id(),
                                                    tool,
                                                ),
                                            ))
                                        })
                                        .collect::<Vec<_>>();

                                    this.update(&mut cx, |this, _cx| {
                                        this.context_server_tool_ids.insert(server_id, tool_ids);
                                    })
                                    .log_err();
                                }
                            }
                        }
                    })
                    .detach();
                }
            }
            context_server::manager::Event::ServerStopped { server_id } => {
                if let Some(tool_ids) = self.context_server_tool_ids.remove(server_id) {
                    tool_working_set.remove(&tool_ids);
                }
            }
        }
    }
}

impl ThreadStore {
    /// Creates some mocked recent threads for testing purposes.
    fn mock_recent_threads(&mut self, cx: &mut ModelContext<Self>) {
        use language_model::Role;

        self.threads.push(cx.new_model(|cx| {
            let mut thread = Thread::new(self.tools.clone(), cx);
            thread.set_summary("Introduction to quantum computing", cx);
            thread.insert_user_message("Hello! Can you help me understand quantum computing?", Vec::new(), cx);
            thread.insert_message(Role::Assistant, "Of course! I'd be happy to help you understand quantum computing. Quantum computing is a fascinating field that uses the principles of quantum mechanics to process information. Unlike classical computers that use bits (0s and 1s), quantum computers use quantum bits or 'qubits'. These qubits can exist in multiple states simultaneously, a property called superposition. This allows quantum computers to perform certain calculations much faster than classical computers. What specific aspect of quantum computing would you like to know more about?", cx);
            thread.insert_user_message("That's interesting! Can you explain how quantum entanglement is used in quantum computing?", Vec::new(), cx);
            thread.insert_message(Role::Assistant, "Certainly! Quantum entanglement is a key principle used in quantum computing. When two qubits become entangled, the state of one qubit is directly related to the state of the other, regardless of the distance between them. This property is used in quantum computing to create complex quantum states and to perform operations on multiple qubits simultaneously. Entanglement allows quantum computers to process information in ways that classical computers cannot, potentially solving certain problems much more efficiently. For example, it's crucial in quantum error correction and in algorithms like quantum teleportation, which is important for quantum communication.", cx);
            thread
        }));

        self.threads.push(cx.new_model(|cx| {
            let mut thread = Thread::new(self.tools.clone(), cx);
            thread.set_summary("Rust web development and async programming", cx);
            thread.insert_user_message("Can you show me an example of Rust code for a simple web server?", Vec::new(), cx);
            thread.insert_message(Role::Assistant, "Certainly! Here's an example of a simple web server in Rust using the `actix-web` framework:

        ```rust
        use actix_web::{web, App, HttpResponse, HttpServer, Responder};

        async fn hello() -> impl Responder {
            HttpResponse::Ok().body(\"Hello, World!\")
        }

        #[actix_web::main]
        async fn main() -> std::io::Result<()> {
            HttpServer::new(|| {
                App::new()
                    .route(\"/\", web::get().to(hello))
            })
            .bind(\"127.0.0.1:8080\")?
            .run()
            .await
        }
        ```

        This code creates a basic web server that responds with 'Hello, World!' when you access the root URL. Here's a breakdown of what's happening:

        1. We import necessary items from the `actix-web` crate.
        2. We define an async `hello` function that returns a simple HTTP response.
        3. In the `main` function, we set up the server to listen on `127.0.0.1:8080`.
        4. We configure the app to respond to GET requests on the root path with our `hello` function.

        To run this, you'd need to add `actix-web` to your `Cargo.toml` dependencies:

        ```toml
        [dependencies]
        actix-web = \"4.0\"
        ```

        Then you can run the server with `cargo run` and access it at `http://localhost:8080`.".unindent(), cx);
            thread.insert_user_message("That's great! Can you explain more about async functions in Rust?", Vec::new(), cx);
            thread.insert_message(Role::Assistant, "Certainly! Async functions are a key feature in Rust for writing efficient, non-blocking code, especially for I/O-bound operations. Here's an overview:

        1. **Syntax**: Async functions are declared using the `async` keyword:

           ```rust
           async fn my_async_function() -> Result<(), Error> {
               // Asynchronous code here
           }
           ```

        2. **Futures**: Async functions return a `Future`. A `Future` represents a value that may not be available yet but will be at some point.

        3. **Await**: Inside an async function, you can use the `.await` syntax to wait for other async operations to complete:

           ```rust
           async fn fetch_data() -> Result<String, Error> {
               let response = make_http_request().await?;
               let data = process_response(response).await?;
               Ok(data)
           }
           ```

        4. **Non-blocking**: Async functions allow the runtime to work on other tasks while waiting for I/O or other operations to complete, making efficient use of system resources.

        5. **Runtime**: To execute async code, you need a runtime like `tokio` or `async-std`. Actix-web, which we used in the previous example, includes its own runtime.

        6. **Error Handling**: Async functions work well with Rust's `?` operator for error handling.

        Async programming in Rust provides a powerful way to write concurrent code that's both safe and efficient. It's particularly useful for servers, network programming, and any application that deals with many concurrent operations.".unindent(), cx);
            thread
        }));

        self.threads.push(cx.new_model(|cx| {
            let mut thread = Thread::new(self.tools.clone(), cx);
            thread.set_summary("Rust code with long lines", cx);
            thread.insert_user_message("Could you write me some Rust code with long lines?", Vec::new(), cx);
            thread.insert_message(Role::Assistant, r#"Here's some Rust code with some intentionally long lines:
            ```rust
            use std::collections::{HashMap, HashSet};
            use std::sync::{Arc, Mutex};
            use std::thread;

            fn main() {
                let very_long_vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25];

                let complicated_hashmap: HashMap<String, Vec<(i32, f64, String)>> = [("key1".to_string(), vec![(1, 1.1, "value1".to_string()), (2, 2.2, "value2".to_string())]), ("key2".to_string(), vec![(3, 3.3, "value3".to_string()), (4, 4.4, "value4".to_string())])].iter().cloned().collect();

                let nested_structure = Arc::new(Mutex::new(HashMap::new()));

                let long_closure = |x: i32, y: i32, z: i32| -> i32 { let result = x * y + z; println!("The result of the long closure calculation is: {}", result); result };

                let thread_handles: Vec<_> = (0..10).map(|i| {
                    let nested_structure_clone = Arc::clone(&nested_structure);
                    thread::spawn(move || {
                        let mut lock = nested_structure_clone.lock().unwrap();
                        lock.entry(format!("thread_{}", i)).or_insert_with(|| HashSet::new()).insert(i * i);
                    })
                }).collect();

                for handle in thread_handles {
                    handle.join().unwrap();
                }

                println!("The final state of the nested structure is: {:?}", nested_structure.lock().unwrap());

                let complex_expression = very_long_vector.iter().filter(|&&x| x % 2 == 0).map(|&x| x * x).fold(0, |acc, x| acc + x) + long_closure(5, 10, 15);

                println!("The result of the complex expression is: {}", complex_expression);
            }
            ```"#.unindent(), cx);
            thread
        }));
    }
}
