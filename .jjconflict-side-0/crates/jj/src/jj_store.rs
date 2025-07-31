use std::path::Path;
use std::sync::Arc;

use gpui::{App, Entity, Global, prelude::*};

use crate::{JujutsuRepository, RealJujutsuRepository};

/// Note: We won't ultimately be storing the jj store in a global, we're just doing this for exploration purposes.
struct GlobalJujutsuStore(Entity<JujutsuStore>);

impl Global for GlobalJujutsuStore {}

pub struct JujutsuStore {
    repository: Arc<dyn JujutsuRepository>,
}

impl JujutsuStore {
    pub fn init_global(cx: &mut App) {
        let Some(repository) = RealJujutsuRepository::new(&Path::new(".")).ok() else {
            return;
        };

        let repository = Arc::new(repository);
        let jj_store = cx.new(|cx| JujutsuStore::new(repository, cx));

        cx.set_global(GlobalJujutsuStore(jj_store));
    }

    pub fn try_global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<GlobalJujutsuStore>()
            .map(|global| global.0.clone())
    }

    pub fn new(repository: Arc<dyn JujutsuRepository>, _cx: &mut Context<Self>) -> Self {
        Self { repository }
    }

    pub fn repository(&self) -> &Arc<dyn JujutsuRepository> {
        &self.repository
    }
}
