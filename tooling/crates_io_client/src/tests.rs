use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use futures::AsyncReadExt as _;
use http_client::{
    AsyncBody, FakeHttpClient, HttpClient, Method, Response, StatusCode,
    http::{HeaderMap, header},
};
use parking_lot::Mutex;
use serde_json::{Value, json};

use crate::{
    real_crates_io_client::{MIN_REQUEST_INTERVAL, RequestThrottle},
    simulated_crates_io_client::SimulatedCratesIoClient,
    *,
};

struct RecordedRequest {
    method: Method,
    uri: String,
    headers: HeaderMap,
    body: String,
}

/// Returns a fake HTTP client that records each request and answers it with
/// the status and JSON body `respond` returns for the request's URI.
fn recording_fake_http_client(
    respond: impl Fn(&str) -> (StatusCode, Value) + Send + Sync + 'static,
) -> (Arc<dyn HttpClient>, Arc<Mutex<Vec<RecordedRequest>>>) {
    let requests = Arc::new(Mutex::new(Vec::new()));
    let respond = Arc::new(respond);
    let fake_http_client = FakeHttpClient::create({
        let requests = requests.clone();
        move |mut request| {
            let requests = requests.clone();
            let respond = respond.clone();
            async move {
                let mut body = String::new();
                request.body_mut().read_to_string(&mut body).await?;
                let uri = request.uri().to_string();
                let (status, response_body) = respond(&uri);
                requests.lock().push(RecordedRequest {
                    method: request.method().clone(),
                    uri,
                    headers: request.headers().clone(),
                    body,
                });
                Ok(Response::builder()
                    .status(status)
                    .body(AsyncBody::from(response_body.to_string()))?)
            }
        }
    });
    (fake_http_client, requests)
}

fn crate_json(name: &str) -> Value {
    json!({
        "name": name,
        "description": null,
        "homepage": null,
        "documentation": null,
        "repository": null,
        "created_at": "2026-01-01T00:00:00Z",
        "updated_at": "2026-01-01T00:00:00Z",
        "downloads": 1,
        "recent_downloads": 1,
        "default_version": "0.1.0",
        "num_versions": 1,
        "yanked": false,
        "trustpub_only": true,
        "keywords": null,
        "categories": null,
    })
}

#[test]
fn test_requests_identify_zed_and_authenticate_with_api_token() {
    let (fake_http_client, requests) = recording_fake_http_client(|_| {
        let site_metadata = json!({
            "deployed_sha": "abc",
            "commit": "abc",
            "cdn_base": "https://static.crates.io",
            "read_only": false,
        });
        (StatusCode::OK, site_metadata)
    });
    let client = RealCratesIoClient::new(fake_http_client, ApiToken::new("secret"));

    smol::block_on(client.get_site_metadata()).unwrap();

    let requests = requests.lock();
    assert_eq!(
        requests[0].headers[header::USER_AGENT],
        "Zed GPUI releases (https://github.com/zed-industries/zed)"
    );
    assert_eq!(requests[0].headers[header::AUTHORIZATION], "secret");
}

#[test]
fn test_error_responses_distinguish_permission_and_upstream_failures() {
    let error_for = |status: StatusCode| {
        let (fake_http_client, _requests) = recording_fake_http_client(move |_| {
            let errors = json!({ "errors": [{ "detail": "first" }, { "detail": "second" }] });
            (status, errors)
        });
        let client = RealCratesIoClient::new(fake_http_client, ApiToken::new("secret"));
        smol::block_on(client.get_site_metadata()).unwrap_err()
    };

    assert!(matches!(
        error_for(StatusCode::FORBIDDEN),
        CratesIoError::PermissionDenied { status: StatusCode::FORBIDDEN, message } if message == "first; second"
    ));
    assert!(matches!(
        error_for(StatusCode::UNAUTHORIZED),
        CratesIoError::PermissionDenied { .. }
    ));
    assert!(matches!(
        error_for(StatusCode::NOT_FOUND),
        CratesIoError::NotFound { .. }
    ));
    assert!(matches!(
        error_for(StatusCode::SERVICE_UNAVAILABLE),
        CratesIoError::Upstream {
            status: StatusCode::SERVICE_UNAVAILABLE,
            ..
        }
    ));
}

#[test]
fn test_list_crates_requests_next_page_only_after_full_page() {
    let (fake_http_client, requests) = recording_fake_http_client(|uri| {
        let crates = if uri.contains("seek=") {
            vec![crate_json("last")]
        } else {
            (0..100)
                .map(|index| crate_json(&format!("crate_{index}")))
                .collect()
        };
        // crates.io can link a next page even when no more crates follow.
        let page = json!({
            "crates": crates,
            "meta": {
                "total": 101,
                "next_page": "?user_id=7&per_page=100&seek=abc",
                "prev_page": null,
            },
        });
        (StatusCode::OK, page)
    });
    let client = RealCratesIoClient::new(fake_http_client, ApiToken::new("secret"));

    let crates = smol::block_on(client.list_crates(&CrateListQuery {
        user_id: Some(UserId(7)),
        exclude_yanked: true,
        ..Default::default()
    }))
    .unwrap();

    assert_eq!(crates.len(), 101);
    assert_eq!(crates[100].name.as_str(), "last");
    let requests = requests.lock();
    assert_eq!(requests.len(), 2);
    assert_eq!(
        requests[0].uri,
        "https://crates.io/api/v1/crates?per_page=100&user_id=7&include_yanked=no"
    );
    assert_eq!(
        requests[1].uri,
        "https://crates.io/api/v1/crates?user_id=7&per_page=100&seek=abc"
    );
}

#[test]
fn test_team_identity_is_shared_across_ownership_assignments() -> anyhow::Result<()> {
    let client = SimulatedCratesIoClient::new();
    let owner = client.token_user();
    let crates: Vec<_> = ["alpha", "beta", "delta", "gamma"]
        .into_iter()
        .map(|name| client.test_crate(name, owner))
        .collect();
    let login = OwnerLogin::GitHubTeam {
        organization: "zed-industries".into(),
        team: "crates-io".into(),
    };
    let mut team_id = None;
    for (index, name) in crates.iter().enumerate() {
        if index % 2 == 0 {
            let id = client.test_team_owner(name, &login.to_string())?;
            if let Some(previous_id) = team_id {
                assert_eq!(id, previous_id);
            }
            team_id = Some(id);
        } else {
            smol::block_on(client.add_owners(name, std::slice::from_ref(&login)))?;
        }
    }
    let listed = smol::block_on(client.list_crates(&CrateListQuery {
        team_id,
        ..Default::default()
    }))?;
    assert_eq!(
        listed
            .into_iter()
            .map(|krate| krate.name)
            .collect::<Vec<_>>(),
        crates
    );
    Ok(())
}

#[test]
fn test_list_crates_filter_precedence() -> anyhow::Result<()> {
    let client = SimulatedCratesIoClient::new();
    let user_owned = client.test_crate("user-owned", client.token_user());
    let other_user = client.test_user();
    let keyword_match = client.test_crate("keyword-match", other_user);
    let team_owned = client.test_crate("team-owned", other_user);
    client.update_test_crate(&keyword_match, |krate| {
        krate.keywords = Some(vec!["keyword".into()]);
    })?;
    let team_id = client.test_team_owner(&team_owned, "github:zed-industries:crates-io")?;
    for (keyword, user_id, expected) in [
        (None, Some(client.token_user()), &user_owned),
        (Some(""), Some(client.token_user()), &user_owned),
        (Some(""), None, &team_owned),
        (Some("keyword"), Some(client.token_user()), &keyword_match),
        (Some("keyword"), None, &keyword_match),
    ] {
        let listed = smol::block_on(client.list_crates(&CrateListQuery {
            keyword: keyword.map(str::to_owned),
            user_id,
            team_id: Some(team_id),
            ..Default::default()
        }))?;
        assert_eq!(
            listed.iter().map(|krate| &krate.name).collect::<Vec<_>>(),
            [expected]
        );
    }
    let listed = smol::block_on(client.list_crates(&CrateListQuery {
        keyword: Some(" ".into()),
        user_id: Some(client.token_user()),
        team_id: Some(team_id),
        ..Default::default()
    }))?;
    assert!(listed.is_empty());
    Ok(())
}

#[test]
fn test_list_crates_filters_by_team_id() -> anyhow::Result<()> {
    let simulated_client = SimulatedCratesIoClient::new();
    let owner = simulated_client.token_user();
    let matching = simulated_client.test_crate("matching", owner);
    let other = simulated_client.test_crate("other", owner);
    simulated_client.test_crate("unowned-by-team", owner);
    let team_id = simulated_client.test_team_owner(&matching, "github:zed-industries:crates-io")?;
    let other_team_id = simulated_client.test_team_owner(&other, "github:zed-industries:other")?;
    let query = CrateListQuery {
        team_id: Some(team_id),
        ..Default::default()
    };

    let crates = smol::block_on(simulated_client.list_crates(&query))?;
    assert_eq!(
        crates.iter().map(|krate| &krate.name).collect::<Vec<_>>(),
        [&matching]
    );
    let crates = smol::block_on(simulated_client.list_crates(&CrateListQuery {
        team_id: Some(other_team_id),
        ..Default::default()
    }))?;
    assert_eq!(
        crates.iter().map(|krate| &krate.name).collect::<Vec<_>>(),
        [&other]
    );
    assert_eq!(
        smol::block_on(simulated_client.list_crates(&CrateListQuery::default()))?.len(),
        3
    );

    let (fake_http_client, requests) = recording_fake_http_client(|_| {
        (
            StatusCode::OK,
            json!({
                "crates": [crate_json("matching")],
                "meta": { "next_page": null },
            }),
        )
    });
    let real_client = RealCratesIoClient::new(fake_http_client, ApiToken::new("secret"));
    let crates = smol::block_on(real_client.list_crates(&query))?;
    assert_eq!(
        crates.iter().map(|krate| &krate.name).collect::<Vec<_>>(),
        [&matching]
    );
    let requests = requests.lock();
    assert_eq!(requests.len(), 1);
    assert_eq!(
        requests.first().map(|request| request.uri.as_str()),
        Some(
            format!(
                "https://crates.io/api/v1/crates?per_page=100&team_id={}",
                team_id.0
            )
            .as_str()
        )
    );
    Ok(())
}

#[test]
fn test_update_crate_sends_settings_wrapped_in_crate_object() {
    let (fake_http_client, requests) =
        recording_fake_http_client(|_| (StatusCode::OK, json!({ "crate": crate_json("gpui") })));
    let client = RealCratesIoClient::new(fake_http_client, ApiToken::new("secret"));

    smol::block_on(client.update_crate(
        &CrateName::new("gpui"),
        &CrateUpdate {
            trustpub_only: Some(true),
        },
    ))
    .unwrap();

    let requests = requests.lock();
    assert_eq!(requests[0].method, Method::PATCH);
    assert_eq!(requests[0].uri, "https://crates.io/api/v1/crates/gpui");
    assert_eq!(
        serde_json::from_str::<Value>(&requests[0].body).unwrap(),
        json!({ "crate": { "trustpub_only": true } })
    );
}

fn github_config_json(id: i32) -> Value {
    json!({
        "id": id,
        "crate": "gpui",
        "repository_owner": "zed-industries",
        "repository_owner_id": 79345384,
        "repository_name": "zed",
        "workflow_filename": "release.yml",
        "environment": null,
        "created_at": "2026-01-01T00:00:00Z",
    })
}

#[test]
fn test_github_config_requests_match_crates_io_api() {
    let (fake_http_client, requests) = recording_fake_http_client(|uri| {
        let body = if uri.contains("crate=") {
            json!({
                "github_configs": [github_config_json(3)],
                "meta": { "total": 1, "next_page": null },
            })
        } else {
            json!({ "github_config": github_config_json(3) })
        };
        (StatusCode::OK, body)
    });
    // Separate clients keep the test from waiting on the rate limit.
    let client = || RealCratesIoClient::new(fake_http_client.clone(), ApiToken::new("secret"));
    let gpui = CrateName::new("gpui");

    let configs = smol::block_on(client().list_github_configs(&gpui)).unwrap();
    smol::block_on(client().create_github_config(&NewGitHubConfig {
        crate_name: gpui,
        repository_owner: "zed-industries".to_owned(),
        repository_name: "zed".to_owned(),
        workflow_filename: "release.yml".to_owned(),
        environment: None,
    }))
    .unwrap();
    smol::block_on(client().delete_github_config(configs[0].id)).unwrap();

    let requests = requests.lock();
    assert_eq!(
        requests[0].uri,
        "https://crates.io/api/v1/trusted_publishing/github_configs?per_page=100&crate=gpui"
    );
    assert_eq!(requests[1].method, Method::POST);
    assert_eq!(
        serde_json::from_str::<Value>(&requests[1].body).unwrap(),
        json!({
            "github_config": {
                "crate": "gpui",
                "repository_owner": "zed-industries",
                "repository_name": "zed",
                "workflow_filename": "release.yml",
                "environment": null,
            }
        })
    );
    assert_eq!(requests[2].method, Method::DELETE);
    assert_eq!(
        requests[2].uri,
        "https://crates.io/api/v1/trusted_publishing/github_configs/3"
    );
}

#[test]
fn test_list_github_configs_rejects_full_page() {
    let (fake_http_client, _requests) = recording_fake_http_client(|_| {
        let configs = (0..100).map(github_config_json).collect::<Vec<_>>();
        let page = json!({
            "github_configs": configs,
            "meta": { "total": 101, "next_page": "?crate=gpui&per_page=100&seek=abc" },
        });
        (StatusCode::OK, page)
    });
    let client = RealCratesIoClient::new(fake_http_client, ApiToken::new("secret"));

    let error = smol::block_on(client.list_github_configs(&CrateName::new("gpui"))).unwrap_err();

    assert!(matches!(error, CratesIoError::Request(_)));
}

#[test]
fn test_owner_endpoints_match_crates_io_api() {
    // Listing owners and changing them share a URL, so one response serves both.
    let (fake_http_client, requests) = recording_fake_http_client(|_| {
        let body = json!({
            "users": [
                {
                    "kind": "user",
                    "id": 436643,
                    "login": "MrSubidubi",
                    "url": "https://github.com/MrSubidubi",
                    "name": "Finn Evers",
                    "avatar": null,
                    "github_username_matches": true,
                },
                {
                    "kind": "team",
                    "id": 12586,
                    "login": "github:zed-industries:crates-io",
                    "url": "https://github.com/zed-industries",
                    "name": "crates-io",
                    "avatar": null,
                },
            ],
            "msg": "Crates.io user maxbrunsfeld has been invited to be an owner of crate gpui,\
                    Crates.io user mikayla-maki already has a pending invitation to be an owner of crate gpui,\
                    team github:zed-industries:crates-io has been added as an owner of crate gpui,\
                    owners changed",
            "ok": true,
        });
        (StatusCode::OK, body)
    });
    // Separate clients keep the test from waiting on the rate limit.
    let client = || RealCratesIoClient::new(fake_http_client.clone(), ApiToken::new("secret"));
    let gpui = CrateName::new("gpui");
    let owner_logins = [
        OwnerLogin::Username("maxbrunsfeld".into()),
        OwnerLogin::CratesIoUser("maxbrunsfeld".to_owned().into()),
        OwnerLogin::GitHubUser("maxbrunsfeld".into()),
        OwnerLogin::GitHubTeam {
            organization: "zed-industries".into(),
            team: "crates-io".into(),
        },
    ];

    let owners = smol::block_on(client().list_owners(&gpui)).unwrap();
    let additions = smol::block_on(client().add_owners(&gpui, &owner_logins)).unwrap();
    smol::block_on(client().remove_owners(&gpui, &owner_logins)).unwrap();

    assert_eq!(
        owners,
        [
            Owner::User(User {
                id: UserId(436643),
                login: "MrSubidubi".to_owned(),
                name: Some("Finn Evers".to_owned()),
            }),
            Owner::Team(Team {
                id: TeamId(12586),
                login: "github:zed-industries:crates-io".to_owned(),
                name: Some("crates-io".to_owned()),
            }),
        ]
    );
    assert_eq!(
        additions,
        [
            OwnerAddition::UserInvited {
                username: "maxbrunsfeld".to_owned(),
            },
            OwnerAddition::UserAlreadyInvited {
                username: "mikayla-maki".to_owned(),
            },
            OwnerAddition::TeamAdded {
                login: "github:zed-industries:crates-io".to_owned(),
            },
            OwnerAddition::Other("owners changed".to_owned()),
        ]
    );
    let requests = requests.lock();
    let expected_body = json!({
        "owners": [
            "maxbrunsfeld",
            "crates.io:maxbrunsfeld",
            "github:maxbrunsfeld",
            "github:zed-industries:crates-io",
        ]
    });
    for (request, method) in requests
        .iter()
        .zip([Method::GET, Method::PUT, Method::DELETE])
    {
        assert_eq!(request.method, method);
        assert_eq!(request.uri, "https://crates.io/api/v1/crates/gpui/owners");
    }
    assert_eq!(
        serde_json::from_str::<Value>(&requests[1].body).unwrap(),
        expected_body
    );
    assert_eq!(
        serde_json::from_str::<Value>(&requests[2].body).unwrap(),
        expected_body
    );
}

#[test]
fn test_throttle_spaces_requests_by_minimum_interval() {
    let throttle = RequestThrottle::new();
    let start = Instant::now();

    assert_eq!(throttle.reserve(start), start);
    assert_eq!(throttle.reserve(start), start + MIN_REQUEST_INTERVAL);
    assert_eq!(throttle.reserve(start), start + 2 * MIN_REQUEST_INTERVAL);
    let later = start + Duration::from_secs(10);
    assert_eq!(throttle.reserve(later), later);
}
