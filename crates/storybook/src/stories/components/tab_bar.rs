use ui::prelude::*;
use ui::{Tab, TabBar};

use crate::story::Story;

#[derive(Element, Default)]
pub struct TabBarStory {}

impl TabBarStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        Story::container(cx)
            .child(Story::title_for::<_, TabBar<V>>(cx))
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
