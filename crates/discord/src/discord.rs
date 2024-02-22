use discord_rpc_client::Client;
use gpui::{AppContext, Context, Global, Model};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

struct GlobalDiscord(Model<Discord>);

impl Global for GlobalDiscord {}

pub struct Discord {
    client: Arc<Mutex<Client>>,
    pub running: bool,
    initialized: bool,
    start_timestamp: Option<u64>,
}

pub fn init(cx: &mut AppContext) {
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
            client: Arc::new(Mutex::new(Client::new(1209561748273762375))),
            running: false,
            initialized: false,
            start_timestamp: None,
        }
    }

    pub fn start_discord_rpc(&mut self) {
        if !self.initialized {
            self.initialized = true;
            let client = Arc::clone(&self.client);
            thread::spawn(move || {
                let mut client = client
                    .lock()
                    .expect("Failed to lock the client for starting");
                client.start();
            });
        }

        self.start_timestamp = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
        );
        self.running = true;
    }

    pub fn stop_discord_rpc(&mut self) {
        let client = Arc::clone(&self.client);
        thread::spawn(move || {
            let mut client = client
                .lock()
                .expect("Failed to lock the client for stopping");
            let _ = client.clear_activity();
        });

        self.running = false;
    }

    pub fn set_activity(&self, filename: String, language: String, project: String) {
        let client = Arc::clone(&self.client);
        let start_timestamp = self.start_timestamp;
        let image_key = self.map_language_to_image_key(&language);

        thread::spawn(move || {
            let mut client = client
                .lock()
                .expect("Failed to lock the client for activity update");
            let language_text = language
                .is_empty()
                .then(|| "a file".to_string())
                .unwrap_or_else(|| format!("a {} file", language));
            let project_text = project
                .is_empty()
                .then(|| "No Project".to_string())
                .unwrap_or_else(|| format!("Project: {}", project));

            client
                .set_activity(|a| {
                    let activity = a
                        .details(&format!("Editing {}", filename))
                        .state(&project_text)
                        .assets(|ass| {
                            ass.large_image(&image_key)
                                .large_text(&format!("Editing {}", language_text))
                                .small_image("zed_small_blue")
                                .small_text("Zed")
                        });

                    if let Some(start) = start_timestamp {
                        activity.timestamps(|t| t.start(start))
                    } else {
                        activity
                    }
                })
                .expect("Failed to set activity");
        });
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
