use lazy_static::lazy_static;
use url::Url;

use crate::{hosting_provider::HostingProvider, permalink::ParsedGitRemote};

lazy_static! {
    static ref GITHUB_PULL_REQUEST_NUMBER: regex::Regex =
        regex::Regex::new(r"\(#(\d+)\)$").unwrap();
}

#[derive(Clone, Debug)]
pub struct PullRequest {
    pub number: u32,
    pub url: Url,
}

pub fn extract_pull_request(remote: &ParsedGitRemote, message: &str) -> Option<PullRequest> {
    match remote.provider {
        HostingProvider::Github => {
            let line = message.lines().next()?;
            let capture = GITHUB_PULL_REQUEST_NUMBER.captures(line)?;
            let number = capture.get(1)?.as_str().parse::<u32>().ok()?;

            let mut url = remote.provider.base_url();
            let path = format!("/{}/{}/pull/{}", remote.owner, remote.repo, number);
            url.set_path(&path);

            Some(PullRequest { number, url })
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use unindent::Unindent;

    use crate::{
        hosting_provider::HostingProvider, permalink::ParsedGitRemote,
        pull_request::extract_pull_request,
    };

    #[test]
    fn test_github_pull_requests() {
        let remote = ParsedGitRemote {
            provider: HostingProvider::Github,
            owner: "zed-industries",
            repo: "zed",
        };

        let message = "This does not contain a pull request";
        assert!(extract_pull_request(&remote, message).is_none());

        // Pull request number at end of first line
        let message = r#"
            project panel: do not expand collapsed worktrees on "collapse all entries" (#10687)

            Fixes #10597

            Release Notes:

            - Fixed "project panel: collapse all entries" expanding collapsed worktrees.
            "#
        .unindent();

        assert_eq!(
            extract_pull_request(&remote, &message)
                .unwrap()
                .url
                .as_str(),
            "https://github.com/zed-industries/zed/pull/10687"
        );

        // Pull request number in middle of line, which we want to ignore
        let message = r#"
            Follow-up to #10687 to fix problems

            See the original PR, this is a fix.
            "#
        .unindent();
        assert!(extract_pull_request(&remote, &message).is_none());
    }
}
