use std::marker::PhantomData;

use ui::prelude::*;
use ui::{Tab, TabBar};

use crate::story::Story;

#[derive(Element)]
pub struct TabBarStory<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync + Clone> TabBarStory<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        Story::container(cx)
            .child(Story::title_for::<_, TabBar<S>>(cx))
            .child(Story::label(cx, "Default"))
            .child(TabBar::new(vec![
                Tab::new()
                    .title("Cargo.toml".to_string())
                    .current(false)
                    .git_status(GitStatus::Modified),
                Tab::new()
                    .title("Channels Panel".to_string())
                    .current(false),
                Tab::new()
                    .title("channels_panel.rs".to_string())
                    .current(true)
                    .git_status(GitStatus::Modified),
                Tab::new()
                    .title("workspace.rs".to_string())
                    .current(false)
                    .git_status(GitStatus::Modified),
                Tab::new()
                    .title("icon_button.rs".to_string())
                    .current(false),
                Tab::new()
                    .title("storybook.rs".to_string())
                    .current(false)
                    .git_status(GitStatus::Created),
                Tab::new().title("theme.rs".to_string()).current(false),
                Tab::new()
                    .title("theme_registry.rs".to_string())
                    .current(false),
                Tab::new()
                    .title("styleable_helpers.rs".to_string())
                    .current(false),
            ]))
    }
}
