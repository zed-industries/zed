use octocrab::Octocrab;

struct ProgramArguments {
    gist_id: String,
    star: bool,
}

/// Rudimentary CLI interface. Tries to be nice to the caller without too
/// much bloat on the example.
fn parse_argv_or_exit() -> ProgramArguments {
    const USAGE: &str = r#"Usage:
    <program> (--star | --unstar) GIST_ID"#;
    let star = if let Some(param) = std::env::args().nth(1) {
        if param == "--star" {
            true
        } else if param == "--unstar" {
            false
        } else {
            eprintln!("error: Need (--star | --unstar) as first argument.");
            eprintln!("{USAGE}");
            std::process::exit(1);
        }
    } else {
        eprintln!("error: Need (--star | --unstar) as first argument.");
        eprintln!("{USAGE}");
        std::process::exit(1);
    };

    let gist_id = if let Some(gist_id) = std::env::args().nth(2) {
        gist_id
    } else {
        eprintln!("error: Need GIST_ID as second argument.");
        eprintln!("{USAGE}");
        std::process::exit(1);
    };
    ProgramArguments { gist_id, star }
}

/// This example tries to demonstrate interacting with a gists' 'stars'.
/// It does so by making a program that takes two CLI arguments:
///
/// 1) `--star` or `--unstar` to either star/unstar a gist
/// 2) A `GIST_ID` to identify which gist to operate on
///
/// The example will check if a gist is already starred / unstarred, before
/// performing the operation.
#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let args = parse_argv_or_exit();
    let octocrab = Octocrab::builder().personal_token(token).build()?;
    let gists_handler = octocrab.gists();
    let is_starred = gists_handler.is_starred(&args.gist_id).await?;

    if is_starred && args.star {
        println!("{gist_id} is already starred.", gist_id = &args.gist_id);
        return Ok(());
    }
    if !is_starred && !args.star {
        println!("{gist_id} is already un-starred.", gist_id = &args.gist_id);
        return Ok(());
    }

    if args.star {
        gists_handler.star(&args.gist_id).await?;
        println!("Starred {gist_id}.", gist_id = &args.gist_id)
    } else {
        gists_handler.unstar(&args.gist_id).await?;
        println!("Un-starred {gist_id}.", gist_id = &args.gist_id)
    }
    Ok(())
}
