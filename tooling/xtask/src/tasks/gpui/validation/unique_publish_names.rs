use std::collections::BTreeMap;

use itertools::Itertools as _;
use thiserror::Error;

use super::{Rule, Violation};
use crate::tasks::gpui::publish_plan::PublishPlanEntry;

pub struct UniquePublishNames;

impl Rule for UniquePublishNames {
    fn name(&self) -> &'static str {
        "published crate names are unique"
    }

    fn check(&self, plan: &[PublishPlanEntry<'_>]) -> Vec<Violation> {
        duplicate_publish_names(plan.iter().map(|entry| {
            (
                entry.repository.target_name(entry.package.name()),
                entry.package.name(),
            )
        }))
        .into_iter()
        .map(Violation::from)
        .collect()
    }
}

#[derive(Debug, Error)]
#[error("`{crate_name}` and `{other}` would both be published as `{published_name}`")]
struct DuplicatePublishName {
    crate_name: String,
    other: String,
    published_name: String,
}

fn duplicate_publish_names<'a>(
    published: impl IntoIterator<Item = (String, &'a str)>,
) -> Vec<DuplicatePublishName> {
    let mut crates_by_published_name = BTreeMap::<String, Vec<&str>>::new();
    for (published_name, crate_name) in published {
        crates_by_published_name
            .entry(published_name)
            .or_default()
            .push(crate_name);
    }

    let mut duplicates = Vec::new();
    for (published_name, mut crates) in crates_by_published_name {
        crates.sort_unstable();
        for (crate_name, other) in crates.into_iter().tuple_windows() {
            duplicates.push(DuplicatePublishName {
                crate_name: crate_name.to_owned(),
                other: other.to_owned(),
                published_name: published_name.clone(),
            });
        }
    }
    duplicates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_crates_that_would_be_published_under_the_same_name() {
        let duplicates = duplicate_publish_names([
            ("gpui_util".to_owned(), "util"),
            ("gpui_collections".to_owned(), "collections"),
            ("gpui_util".to_owned(), "gpui_util"),
            ("zed-fork".to_owned(), "fork"),
        ]);
        assert_eq!(
            duplicates
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
            ["`gpui_util` and `util` would both be published as `gpui_util`"]
        );

        assert!(
            duplicate_publish_names([
                ("gpui_util".to_owned(), "util"),
                ("gpui_collections".to_owned(), "collections"),
            ])
            .is_empty()
        );
    }
}
