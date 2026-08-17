# Octocrab: A modern, extensible GitHub API client.

[![Rust](https://github.com/XAMPPRocky/octocrab/workflows/Rust/badge.svg)](https://github.com/XAMPPRocky/octocrab/actions?query=workflow%3ARust)
[![crates.io](https://img.shields.io/crates/d/octocrab.svg)](https://crates.io/crates/octocrab)
[![Help Wanted](https://img.shields.io/github/issues/XAMPPRocky/octocrab/help%20wanted?color=green)](https://github.com/XAMPPRocky/octocrab/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22)
[![Documentation](https://docs.rs/octocrab/badge.svg)](https://docs.rs/octocrab/)
[![Crates.io](https://img.shields.io/crates/v/octocrab?logo=rust)](https://crates.io/crates/octocrab/)

Octocrab is a third party GitHub API client, allowing you to easily build
your own GitHub integrations or bots in Rust. `Octocrab` comes with two primary
sets of APIs for communicating with GitHub, a high level strongly typed
semantic API, and a lower level HTTP API for extending behaviour.

## Adding Octocrab
Run this command in your terminal to add the latest version of `Octocrab`.

```bash
cargo add octocrab
```

## Semantic API
The semantic API provides strong typing around GitHub's API, a set of
[`models`] that maps to GitHub's types, and [`auth`] functions that are useful
for GitHub apps.
Currently, the following modules are available as of version `0.44`.

- [`actions`] GitHub Actions
- [`activity`] GitHub Activity
- [`apps`] GitHub Apps
- [`checks`] GitHub Checks
- [`codes_of_conduct`] GitHub Codes of Conduct
- [`code_scannings`] Code Scanning
- [`commits`] GitHub Commits
- [`current`] Information about the current user
- [`events`] GitHub Events
- [`gists`] Gists
- [`gitignore`] Gitignore templates
- [`graphql`] GraphQL
- [`issues`] Issues and related items, e.g. comments, labels, etc.
- [`licenses`] License Metadata
- [`markdown`] Rendering Markdown with GitHub
- [`orgs`] GitHub Organisations
- [`projects`] GitHub Projects
- [`pulls`] Pull Requests
- [`ratelimit`] Rate Limiting
- [`repos`] Repositories
- [`search`] Using GitHub's search
- [`teams`] Teams
- [`users`] Users

[`models`]: https://docs.rs/octocrab/latest/octocrab/models/index.html
[`auth`]: https://docs.rs/octocrab/latest/octocrab/auth/index.html
[`actions`]: https://docs.rs/octocrab/latest/octocrab/actions/struct.ActionsHandler.html
[`activity`]: https://docs.rs/octocrab/latest/octocrab/activity/struct.ActivityHandler.html
[`apps`]: https://docs.rs/octocrab/latest/octocrab/apps/struct.AppsHandler.html
[`checks`]: https://docs.rs/octocrab/latest/octocrab/checks/struct.ChecksHandler.html
[`codes_of_conduct`]: https://docs.rs/octocrab/latest/octocrab/codes_of_conduct/struct.CodesOfConductHandler.html
[`code_scannings`]: https://docs.rs/octocrab/latest/octocrab/code_scannings/struct.CodeScanningsHandler.html
[`commits`]: https://docs.rs/octocrab/latest/octocrab/commits/struct.CommitsHandler.html
[`current`]: https://docs.rs/octocrab/latest/octocrab/current/struct.CurrentAuthHandler.html
[`events`]: https://docs.rs/octocrab/latest/octocrab/events/struct.EventsHandler.html
[`gists`]: https://docs.rs/octocrab/latest/octocrab/gists/struct.GistsHandler.html
[`gitignore`]: https://docs.rs/octocrab/latest/octocrab/gitignore/struct.GitignoreHandler.html
[`graphql`]: https://docs.rs/octocrab/latest/octocrab/struct.Octocrab.html#graphql-api
[`issues`]: https://docs.rs/octocrab/latest/octocrab/issues/struct.IssueHandler.html
[`licenses`]: https://docs.rs/octocrab/latest/octocrab/licenses/struct.LicenseHandler.html
[`markdown`]: https://docs.rs/octocrab/latest/octocrab/markdown/struct.MarkdownHandler.html
[`orgs`]: https://docs.rs/octocrab/latest/octocrab/orgs/struct.OrgHandler.html
[`projects`]: https://docs.rs/octocrab/latest/octocrab/projects/struct.ProjectHandler.html
[`pulls`]: https://docs.rs/octocrab/latest/octocrab/pulls/struct.PullRequestHandler.html
[`ratelimit`]: https://docs.rs/octocrab/latest/octocrab/ratelimit/struct.RateLimitHandler.html
[`repos`]: https://docs.rs/octocrab/latest/octocrab/repos/struct.RepoHandler.html
[`search`]: https://docs.rs/octocrab/latest/octocrab/search/struct.SearchHandler.html
[`teams`]: https://docs.rs/octocrab/latest/octocrab/teams/struct.TeamHandler.html
[`users`]: https://docs.rs/octocrab/latest/octocrab/users/struct.UsersHandler.html

#### Getting a Pull Request
```rust
// Get pull request #5 from `XAMPPRocky/octocrab`.
let issue = octocrab::instance().pulls("XAMPPRocky", "octocrab").get(5).await?;
```

All methods with multiple optional parameters are built as `Builder`
structs, allowing you to easily specify parameters.

#### Listing issues
```rust
let octocrab = octocrab::instance();
// Returns the first page of all issues.
let mut page = octocrab
    .issues("XAMPPRocky", "octocrab")
    .list()
    // Optional Parameters
    .creator("XAMPPRocky")
    .state(params::State::All)
    .per_page(50)
    .send()
    .await?;

// Go through every page of issues. Warning: There's no rate limiting so
// be careful.
loop {
    for issue in &page {
        println!("{}", issue.title);
    }
    page = match octocrab
        .get_page::<models::issues::Issue>(&page.next)
        .await?
    {
        Some(next_page) => next_page,
        None => break,
    }
}
```

## HTTP API
The typed API currently doesn't cover all of GitHub's API at this time, and
even if it did GitHub is in active development and this library will
likely always be somewhat behind GitHub at some points in time. However that
shouldn't mean that in order to use those features, you have to fork
or replace `octocrab` with your own solution.

Instead `octocrab` exposes a suite of HTTP methods allowing you to easily
extend `Octocrab`'s existing behaviour. Using these HTTP methods allows you
to keep using the same authentication and configuration, while having
control over the request and response. There is a method for each HTTP
method, `get`, `post`, `patch`, `put`, `delete`, all of which accept a
relative route and a optional body.

```rust
let user: octocrab::models::User = octocrab::instance()
    .get("/user", None::<&()>)
    .await?;
```

Each of the HTTP methods expects a body, formats the URL with the base
URL, and errors if GitHub doesn't return a successful status, but this isn't
always desired when working with GitHub's API, sometimes you need to check
the response status or headers. As such there are companion methods `_get`,
`_post`, etc. that perform no additional pre or post-processing to
the request.

```rust
let octocrab = octocrab::instance();
let response = octocrab
    ._get("https://api.github.com/organizations")
    .await?;

// You can also use `Uri::builder().authority("<my custom base>").path_and_query("<my custom path>")` if you want to customize the base uri and path.
let response =  octocrab
    ._get(Uri::builder().path_and_query("/organizations").build().expect("valid uri"))
    .await?;
```

You can use the those HTTP methods to easily create your own extensions to
`Octocrab`'s typed API. (Requires `async_trait`).

```rust
use octocrab::{Octocrab, Page, Result, models};

#[async_trait::async_trait]
trait OrganisationExt {
  async fn list_every_organisation(&self) -> Result<Page<models::Organization>>;
}

#[async_trait::async_trait]
impl OrganisationExt for Octocrab {
  async fn list_every_organisation(&self) -> Result<Page<models::Organization>> {
    self.get("/organizations", None::<&()>).await
  }
}
```

You can also easily access new properties that aren't available in the
current models using `serde`.

```rust
#[derive(Deserialize)]
struct RepositoryWithVisibility {
    #[serde(flatten)]
    inner: octocrab::models::Repository,
    visibility: String,
}

let my_repo = octocrab::instance()
    .get::<RepositoryWithVisibility>("https://api.github.com/repos/XAMPPRocky/octocrab", None::<&()>)
    .await?;
```

## Static API
`Octocrab` also provides a statically reference counted version of its API,
allowing you to easily plug it into existing systems without worrying
about having to integrate and pass around the client.

```rust
// Initialises the static instance with your configuration and returns an
// instance of the client.
octocrab::initialise(octocrab::Octocrab::builder());
// Gets a instance of `Octocrab` from the static API. If you call this
// without first calling `octocrab::initialise` a default client will be
// initialised and returned instead.
let octocrab = octocrab::instance();
```

## GitHub Webhook Support

`octocrab` provides [deserializable datatypes](https://docs.rs/octocrab/latest/octocrab/models/webhook_events/index.html)
for the payloads received by a GitHub application [responding to
webhooks](https://docs.github.com/en/apps/creating-github-apps/writing-code-for-a-github-app/building-a-github-app-that-responds-to-webhook-events).
This allows you to write a typesafe application using Rust with
pattern-matching/enum-dispatch to respond to events.

**Note**: Webhook support in `octocrab` is still beta, not all known webhook events are
strongly typed.

```rust
use http::request::Request;
use tracing::{warn, info};
use octocrab::models::webhook_events::*;

let request_from_github = Request::post("https://my-webhook-url.com").body(vec![0_u8]).unwrap();
// request_from_github is the HTTP request your webhook handler received
let (parts, body) = request_from_github.into_parts();
let header = parts.headers.get("X-GitHub-Event").unwrap().to_str().unwrap();

let event = WebhookEvent::try_from_header_and_body(header, &body).unwrap();
// Now you can match on event type and call any specific handling logic
match event.kind {
    WebhookEventType::Ping => info!("Received a ping"),
    WebhookEventType::PullRequest => info!("Received a pull request event"),
    // ...
    _ => warn!("Ignored event"),
};
```
