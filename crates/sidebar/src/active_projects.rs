use db::{kvp::KEY_VALUE_STORE, write_and_log};
use gpui::{App, Context, Entity, Global, prelude::*};
use util::ResultExt as _;
use workspace::PathList;

const ACTIVE_PROJECTS_KEY: &str = "active_projects";

struct GlobalActiveProjects(Entity<ActiveProjects>);

impl Global for GlobalActiveProjects {}

pub struct ActiveProjects {
    projects: Vec<PathList>,
}

impl ActiveProjects {
    pub fn init_global(cx: &mut App) {
        let active_projects = cx.new(|_cx| Self::load());
        cx.set_global(GlobalActiveProjects(active_projects));
    }

    pub fn global(cx: &App) -> Entity<Self> {
        cx.global::<GlobalActiveProjects>().0.clone()
    }

    pub fn try_global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<GlobalActiveProjects>().map(|g| g.0.clone())
    }

    fn load() -> Self {
        let projects = Self::load_from_db().unwrap_or_default();
        Self { projects }
    }

    fn load_from_db() -> anyhow::Result<Vec<PathList>> {
        let json = KEY_VALUE_STORE
            .read_kvp(ACTIVE_PROJECTS_KEY)?
            .unwrap_or_else(|| "[]".to_string());
        Ok(serde_json::from_str(&json)?)
    }

    pub fn projects(&self) -> &[PathList] {
        &self.projects
    }

    pub fn contains(&self, path_list: &PathList) -> bool {
        self.projects.iter().any(|p| p == path_list)
    }

    /// Add a project to the active list. No-op if already present.
    pub fn add(&mut self, path_list: PathList, cx: &mut Context<Self>) {
        if path_list.is_empty() || self.contains(&path_list) {
            return;
        }
        self.projects.push(path_list);
        self.persist(cx);
        cx.notify();
    }

    /// Remove a project from the active list. No-op if not present.
    pub fn remove(&mut self, path_list: &PathList, cx: &mut Context<Self>) {
        let before = self.projects.len();
        self.projects.retain(|p| p != path_list);
        if self.projects.len() != before {
            self.persist(cx);
            cx.notify();
        }
    }

    fn persist(&self, cx: &Context<Self>) {
        let projects = self.projects.clone();

        write_and_log(cx, move || async move {
            let json = serde_json::to_string(&projects)?;
            KEY_VALUE_STORE
                .write_kvp(ACTIVE_PROJECTS_KEY.into(), json)
                .await
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use std::path::PathBuf;

    fn make_path_list(paths: &[&str]) -> PathList {
        let paths: Vec<PathBuf> = paths.iter().map(PathBuf::from).collect();
        PathList::new(&paths)
    }

    #[gpui::test]
    fn test_add_and_contains(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let entity = cx.new(|_cx| ActiveProjects {
                projects: Vec::new(),
            });

            let pl = make_path_list(&["/home/user/project-a"]);
            entity.update(cx, |ap, cx| ap.add(pl.clone(), cx));

            entity.read_with(cx, |ap, _cx| {
                assert!(ap.contains(&pl));
                assert_eq!(ap.projects().len(), 1);
            });
        });
    }

    #[gpui::test]
    fn test_idempotent_add(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let entity = cx.new(|_cx| ActiveProjects {
                projects: Vec::new(),
            });

            let pl = make_path_list(&["/home/user/project-a"]);
            entity.update(cx, |ap, cx| {
                ap.add(pl.clone(), cx);
                ap.add(pl.clone(), cx);
            });

            entity.read_with(cx, |ap, _cx| {
                assert_eq!(ap.projects().len(), 1);
            });
        });
    }

    #[gpui::test]
    fn test_add_empty_is_noop(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let entity = cx.new(|_cx| ActiveProjects {
                projects: Vec::new(),
            });

            let empty = PathList::default();
            entity.update(cx, |ap, cx| ap.add(empty, cx));

            entity.read_with(cx, |ap, _cx| {
                assert_eq!(ap.projects().len(), 0);
            });
        });
    }

    #[gpui::test]
    fn test_remove(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let entity = cx.new(|_cx| ActiveProjects {
                projects: Vec::new(),
            });

            let pl_a = make_path_list(&["/home/user/project-a"]);
            let pl_b = make_path_list(&["/home/user/project-b"]);
            entity.update(cx, |ap, cx| {
                ap.add(pl_a.clone(), cx);
                ap.add(pl_b.clone(), cx);
            });

            entity.update(cx, |ap, cx| ap.remove(&pl_a, cx));

            entity.read_with(cx, |ap, _cx| {
                assert!(!ap.contains(&pl_a));
                assert!(ap.contains(&pl_b));
                assert_eq!(ap.projects().len(), 1);
            });
        });
    }

    #[gpui::test]
    fn test_remove_nonexistent_is_noop(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let entity = cx.new(|_cx| ActiveProjects {
                projects: Vec::new(),
            });

            let pl = make_path_list(&["/home/user/project-a"]);
            entity.update(cx, |ap, cx| ap.remove(&pl, cx));

            entity.read_with(cx, |ap, _cx| {
                assert_eq!(ap.projects().len(), 0);
            });
        });
    }

    #[gpui::test]
    fn test_serialization_roundtrip(_cx: &mut TestAppContext) {
        let pl = make_path_list(&["/home/user/project-b", "/home/user/project-a"]);
        let json = serde_json::to_string(&pl).expect("serialize");
        let roundtripped: PathList = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(pl, roundtripped);
        assert_eq!(pl.paths(), roundtripped.paths());
        assert_eq!(pl.order(), roundtripped.order());
    }

    #[gpui::test]
    fn test_multiple_paths_serialization_roundtrip(_cx: &mut TestAppContext) {
        let projects = vec![
            make_path_list(&["/z/third", "/a/first", "/m/second"]),
            make_path_list(&["/home/user/project-a"]),
        ];
        let json = serde_json::to_string(&projects).expect("serialize");
        let roundtripped: Vec<PathList> = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(projects, roundtripped);
    }
}
