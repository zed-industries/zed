use crate::{environment, fmt_option_str, write_variable};
use std::{fs, io, path};

pub fn write_git_version(
    manifest_location: &path::Path,
    envmap: &environment::EnvironmentMap,
    mut w: &fs::File,
) -> io::Result<()> {
    use io::Write;

    // CIs will do shallow clones of repositories, causing libgit2 to error
    // out. We try to detect if we are running on a CI and ignore the
    // error.
    let (mut tag, mut dirty) = (
        envmap.get_override_var("GIT_VERSION"),
        envmap.get_override_var("GIT_DIRTY"),
    );
    if tag.is_none() || dirty.is_none() {
        if let Some((git_tag, git_dirty)) = get_repo_description(manifest_location).ok().flatten() {
            if tag.is_none() {
                tag = Some(git_tag);
            }
            if dirty.is_none() {
                dirty = Some(git_dirty);
            }
        };
    }
    write_variable!(
        w,
        "GIT_VERSION",
        "Option<&str>",
        fmt_option_str(tag),
        "If the crate was compiled from within a git-repository, \
        `GIT_VERSION` contains HEAD's tag. The short commit id is used if HEAD is not tagged."
    );
    write_variable!(
        w,
        "GIT_DIRTY",
        "Option<bool>",
        match dirty {
            Some(true) => "Some(true)",
            Some(false) => "Some(false)",
            None => "None",
        },
        "If the repository had dirty/staged files."
    );

    let (mut branch, mut commit, mut commit_short) = (
        envmap.get_override_var("GIT_HEAD_REF"),
        envmap.get_override_var::<String>("GIT_COMMIT_HASH"),
        envmap.get_override_var("GIT_COMMIT_HASH_SHORT"),
    );
    if branch.is_none() || commit.is_none() || commit_short.is_none() {
        if let Some((git_branch, git_commit, git_commit_short)) =
            get_repo_head(manifest_location).ok().flatten()
        {
            if branch.is_none() {
                branch = git_branch;
            }
            if commit.is_none() {
                commit = Some(git_commit);
            }
            if commit_short.is_none() {
                commit_short = Some(git_commit_short);
            }
        }
    }
    if let (Some(h), None) = (&commit, &commit_short) {
        commit_short = Some(h.chars().take(8).collect())
    }

    let doc = "If the crate was compiled from within a git-repository, `GIT_HEAD_REF` \
        contains full name to the reference pointed to by HEAD \
        (e.g.: `refs/heads/master`). If HEAD is detached or the branch name is not \
        valid UTF-8 `None` will be stored.\n";
    write_variable!(
        w,
        "GIT_HEAD_REF",
        "Option<&str>",
        fmt_option_str(branch),
        doc
    );

    write_variable!(
        w,
        "GIT_COMMIT_HASH",
        "Option<&str>",
        fmt_option_str(commit),
        "If the crate was compiled from within a git-repository, `GIT_COMMIT_HASH` \
    contains HEAD's full commit SHA-1 hash."
    );

    write_variable!(
        w,
        "GIT_COMMIT_HASH_SHORT",
        "Option<&str>",
        fmt_option_str(commit_short),
        "If the crate was compiled from within a git-repository, `GIT_COMMIT_HASH_SHORT` \
    contains HEAD's short commit SHA-1 hash."
    );

    Ok(())
}

/// Retrieves the git-tag or hash describing the exact version and a boolean
/// that indicates if the repository currently has dirty/staged files.
///
/// If a valid git-repo can't be discovered at or above the given path,
/// `Ok(None)` is returned instead of an `Err`-value.
///
/// # Errors
/// Errors from `git2` are returned if the repository does exists at all.
#[cfg(feature = "git2")]
pub fn get_repo_description(root: &std::path::Path) -> Result<Option<(String, bool)>, git2::Error> {
    match git2::Repository::discover(root) {
        Ok(repo) => {
            let mut desc_opt = git2::DescribeOptions::new();
            desc_opt.describe_tags().show_commit_oid_as_fallback(true);
            let tag = repo
                .describe(&desc_opt)
                .and_then(|desc| desc.format(None))?;
            let mut st_opt = git2::StatusOptions::new();
            st_opt.include_ignored(false);
            st_opt.include_untracked(false);
            let dirty = repo
                .statuses(Some(&mut st_opt))?
                .iter()
                .any(|status| !matches!(status.status(), git2::Status::CURRENT));
            Ok(Some((tag, dirty)))
        }
        Err(ref e)
            if e.class() == git2::ErrorClass::Repository
                && e.code() == git2::ErrorCode::NotFound =>
        {
            Ok(None)
        }
        Err(e) => Err(e),
    }
}

/// Retrieves the branch name and hash of HEAD.
///
/// The returned value is a tuple of head's reference-name, long-hash and short-hash. The
/// branch name will be `None` if the head is detached, or it's not valid UTF-8.
///
/// If a valid git-repo can't be discovered at or above the given path,
/// `Ok(None)` is returned instead of an `Err`-value.
///
/// # Errors
/// Errors from `git2` are returned if the repository does exists at all.
#[cfg(feature = "git2")]
pub fn get_repo_head(
    root: &std::path::Path,
) -> Result<Option<(Option<String>, String, String)>, git2::Error> {
    match git2::Repository::discover(root) {
        Ok(repo) => {
            // Supposed to be the reference pointed to by HEAD, but it's HEAD
            // itself, if detached
            let head_ref = repo.head()?;
            let branch = {
                // Check whether `head` is really the pointed to reference and
                // not HEAD itself.
                if repo.head_detached()? {
                    None
                } else {
                    head_ref.name()
                }
            };
            let head = head_ref.peel_to_commit()?;
            let commit = head.id();
            let commit_short = head.into_object().short_id()?;
            Ok(Some((
                branch.map(ToString::to_string),
                format!("{commit}"),
                commit_short.as_str().unwrap_or_default().to_string(),
            )))
        }
        Err(ref e)
            if e.class() == git2::ErrorClass::Repository
                && e.code() == git2::ErrorCode::NotFound =>
        {
            Ok(None)
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_git_repo() {
        use std::fs;
        use std::path;

        let repo_root = tempfile::tempdir().unwrap();
        assert_eq!(super::get_repo_description(repo_root.as_ref()), Ok(None));

        let repo = git2::Repository::init_opts(
            &repo_root,
            git2::RepositoryInitOptions::new()
                .external_template(false)
                .mkdir(false)
                .no_reinit(true)
                .mkpath(false),
        )
        .unwrap();

        let cruft_file = repo_root.path().join("cruftfile");
        std::fs::write(&cruft_file, "Who? Me?").unwrap();

        let project_root = repo_root.path().join("project_root");
        fs::create_dir(&project_root).unwrap();

        let sig = git2::Signature::now("foo", "bar").unwrap();
        let mut idx = repo.index().unwrap();
        idx.add_path(path::Path::new("cruftfile")).unwrap();
        idx.write().unwrap();
        let commit_oid = repo
            .commit(
                Some("HEAD"),
                &sig,
                &sig,
                "Testing testing 1 2 3",
                &repo.find_tree(idx.write_tree().unwrap()).unwrap(),
                &[],
            )
            .unwrap();

        let binding = repo
            .find_commit(commit_oid)
            .unwrap()
            .into_object()
            .short_id()
            .unwrap();

        let commit_oid_short = binding.as_str().unwrap();

        let commit_hash = commit_oid.to_string();
        let commit_hash_short = commit_oid_short.to_string();

        assert!(commit_hash.starts_with(&commit_hash_short));

        // The commit, the commit-id is something and the repo is not dirty
        let (tag, dirty) = super::get_repo_description(&project_root).unwrap().unwrap();
        assert!(!tag.is_empty());
        assert!(!dirty);

        // Tag the commit, it should be retrieved
        repo.tag(
            "foobar",
            &repo
                .find_object(commit_oid, Some(git2::ObjectType::Commit))
                .unwrap(),
            &sig,
            "Tagged foobar",
            false,
        )
        .unwrap();

        let (tag, dirty) = super::get_repo_description(&project_root).unwrap().unwrap();
        assert_eq!(tag, "foobar");
        assert!(!dirty);

        // Make some dirt
        std::fs::write(cruft_file, "now dirty").unwrap();
        let (tag, dirty) = super::get_repo_description(&project_root).unwrap().unwrap();
        assert_eq!(tag, "foobar");
        assert!(dirty);

        let branch_short_name = "baz";
        let branch_name = "refs/heads/baz";
        let commit = repo.find_commit(commit_oid).unwrap();
        repo.branch(branch_short_name, &commit, true).unwrap();
        repo.set_head(branch_name).unwrap();

        assert_eq!(
            super::get_repo_head(&project_root),
            Ok(Some((
                Some(branch_name.to_owned()),
                commit_hash,
                commit_hash_short
            )))
        );
    }

    #[test]
    fn detached_head_repo() {
        let repo_root = tempfile::tempdir().unwrap();
        let repo = git2::Repository::init_opts(
            &repo_root,
            git2::RepositoryInitOptions::new()
                .external_template(false)
                .mkdir(false)
                .no_reinit(true)
                .mkpath(false),
        )
        .unwrap();
        let sig = git2::Signature::now("foo", "bar").unwrap();
        let commit_oid = repo
            .commit(
                Some("HEAD"),
                &sig,
                &sig,
                "Testing",
                &repo
                    .find_tree(repo.index().unwrap().write_tree().unwrap())
                    .unwrap(),
                &[],
            )
            .unwrap();

        let binding = repo
            .find_commit(commit_oid)
            .unwrap()
            .into_object()
            .short_id()
            .unwrap();

        let commit_oid_short = binding.as_str().unwrap();

        let commit_hash = commit_oid.to_string();
        let commit_hash_short = commit_oid_short.to_string();

        assert!(commit_hash.starts_with(&commit_hash_short));

        repo.set_head_detached(commit_oid).unwrap();
        assert_eq!(
            super::get_repo_head(repo_root.as_ref()),
            Ok(Some((None, commit_hash, commit_hash_short)))
        );
    }
}
