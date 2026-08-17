//! Run this to update `github_schema.graphql`:
//!
//! ```sh
//! curl -L https://docs.github.com/public/fpt/schema.docs.graphql -o examples/github_schema.graphql
//! ```
use graphql_client::GraphQLQuery;

#[allow(clippy::upper_case_acronyms)]
type URI = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "examples/github_schema.graphql",
    query_path = "examples/issues_query.graphql",
    variables_derives = "Clone, Debug",
    response_derives = "Clone, Debug"
)]
pub struct IssuesQuery;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let octocrab = octocrab::Octocrab::builder()
        .personal_token(std::env::var("GITHUB_TOKEN").unwrap())
        .build()?;

    let mut variables = issues_query::Variables {
        owner: "XAMPPRocky".to_string(),
        name: "octocrab".to_string(),
        page_size: 5,
        before: None,
    };

    let pages_to_show = 3;
    let mut page = 1;
    loop {
        if page > pages_to_show {
            break;
        }

        let response: octocrab::Result<graphql_client::Response<issues_query::ResponseData>> =
            octocrab
                .graphql(&IssuesQuery::build_query(variables.clone()))
                .await;

        match response {
            Ok(response) => {
                println!("Page {page}:");
                let issues = &response
                    .data
                    .as_ref()
                    .unwrap()
                    .repository
                    .as_ref()
                    .unwrap()
                    .issues;
                print_issues(issues);
                if !update_page_info(&mut variables, issues) {
                    break;
                }
            }
            Err(error) => {
                println!("{error:#?}");
                break;
            }
        }

        page += 1;
    }

    Ok(())
}

fn print_issues(issues: &issues_query::IssuesQueryRepositoryIssues) {
    for issue in issues.nodes.as_ref().unwrap().iter().flatten() {
        println!("{} {}", issue.url, issue.title);
    }
}

fn update_page_info(
    variables: &mut issues_query::Variables,
    issues: &issues_query::IssuesQueryRepositoryIssues,
) -> bool {
    let page_info = &issues.page_info;
    if page_info.has_previous_page {
        variables.before = Some(page_info.start_cursor.as_ref().unwrap().clone());
        true
    } else {
        false
    }
}
