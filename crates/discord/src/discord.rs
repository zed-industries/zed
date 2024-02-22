use discord_rpc_client::{Client, Event};
use gpui::{AppContext, Context, Global, Model};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

struct GlobalDiscord(Model<Discord>);

impl Global for GlobalDiscord {}

pub struct Discord {
    client: Option<Arc<Mutex<Client>>>,
    running: Option<bool>,
    initialized: Option<bool>,
    start_timestamp: Option<u64>,
}

pub fn init(cx: &mut AppContext) {
    println!("Initializing Discord!");
    let discord = cx.new_model(move |_cx| Discord::new());
    Discord::set_global(discord.clone(), cx);
}

impl Discord {
    pub fn global(cx: &AppContext) -> Option<Model<Self>> {
        cx.try_global::<GlobalDiscord>()
            .map(|model| model.0.clone())
    }

    pub fn set_global(discord: Model<Self>, cx: &mut AppContext) {
        cx.set_global(GlobalDiscord(discord));
    }

    pub fn new() -> Self {
        Self {
            client: Some(Arc::new(Mutex::new(Client::new(1209561748273762375)))),
            running: Some(false),
            initialized: Some(false),
            start_timestamp: None,
        }
    }

    pub fn running(&self) -> Option<bool> {
        self.running
    }

    pub fn start_discord_rpc(&mut self) {
        if let Some(client) = self.client.clone() {
            if !self.initialized.unwrap_or_else(|| false) {
                self.initialized = Some(true);
                thread::spawn(move || {
                    let mut client = client.lock().unwrap();
                    client.on_event(Event::Ready, |_ctx| println!("Client is ready."));
                    client.start();
                });
            }

            self.start_timestamp = Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            );
            self.running = Some(true);
        } else {
            println!("Client is not available.");
        }
    }

    pub fn stop_discord_rpc(&mut self) {
        if let Some(client) = self.client.clone() {
            thread::spawn(move || {
                let mut client = client.lock().unwrap();
                let _ = client.clear_activity();
            });

            self.running = Some(false);
        } else {
            println!("Error stopping client.");
        }
    }

    pub fn set_activity(&self, filename: String, language: String, project: String) {
        if let Some(client) = self.client.clone() {
            let start_timestamp = self.start_timestamp;
            let image_key = self.map_language_to_image_key(&language);

            thread::spawn(move || {
                let mut client = client.lock().unwrap();
                let language_text = if language.is_empty() {
                    "a file".to_string()
                } else {
                    format!("a {} file", language)
                };
                let project = if project.is_empty() {
                    "No Project".to_string()
                } else {
                    format!("Project: {}", project)
                };
                println!("Updating status: {}, {}", filename, language);
                if let Err(why) = client.set_activity(|a| {
                    let activity = a
                        .details(&format!("Editing {}", filename))
                        .state(project)
                        .assets(|ass| {
                            ass.large_image(&image_key)
                                .large_text(&format!("Editing {}", language_text))
                                .small_image("zed_small_blue")
                                .small_text("Zed")
                        });
                    let activity = if let Some(start) = start_timestamp {
                        activity.timestamps(|t| t.start(start))
                    } else {
                        activity
                    };

                    activity
                }) {
                    println!("Failed to set presence: {}", why);
                }
            });
        } else {
            println!("Client is not available.");
        }
    }

    fn map_language_to_image_key(&self, language: &str) -> String {
        match language.to_lowercase().as_str() {
            "astro" | "bash" | "c" | "cplusplus" | "csharp" | "css" | "dart" | "docker"
            | "elixir" | "erlang" | "git" | "go" | "groovy" | "haskell" | "html" | "java"
            | "javascript" | "json" | "kotlin" | "latex" | "less" | "lua" | "markdown" | "perl"
            | "php" | "powershell" | "python" | "ruby" | "rust" | "sass" | "scala" | "swift"
            | "toml" | "typescript" | "xml" | "yaml" => language.to_lowercase(),
            "c++" => "cplusplus".to_string(),
            "c#" => "csharp".to_string(),
            _ => "code".to_string(),
        }
    }
}
