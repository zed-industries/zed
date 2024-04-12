use client::{RemoteProjectId, DevServerId};

pub struct Store {
    remote_projects: HashMap<RemoteProjectId, RemoteProject>,
    dev_servers: HashMap<DevServerId, DevServer>,
}

struct GlobalStore(Model<Store>);

impl Global for GlobalStore {}

pub fn init() {
    let store = cx.new_model(|cx| Store::new(client.clone(), cx))
}

impl Store {
    pub fn global(cx: &AppContext) -> Model<DevServer> {
        cx.global::<GlobalDevServer>().0.clone()
    }

pub fn new() -> Self {
    Self {
        _subscriptions: vec![
            client.add_message_handler(cx.weak_model(), Self::handle_dev_server_instructions)
        ],
        client,
    }
}

    fn handle_remote_projects_update(
    this: Model<Self>,
    envelope: TypedEnvelope<proto::DevServerInstructions>,
    _: Arc<Client>,
    mut cx: AsyncAppContext) -> Result<()> {

    }
}
