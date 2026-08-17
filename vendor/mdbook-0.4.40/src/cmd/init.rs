use crate::get_book_dir;
use clap::{arg, ArgMatches, Command as ClapCommand};
use mdbook::config;
use mdbook::errors::Result;
use mdbook::MDBook;
use std::io;
use std::io::Write;
use std::process::Command;

// Create clap subcommand arguments
pub fn make_subcommand() -> ClapCommand {
    ClapCommand::new("init")
        .about("Creates the boilerplate structure and files for a new book")
        .arg(
            arg!([dir]
                "Directory to create the book in\n\
                (Defaults to the current directory when omitted)"
            )
            .value_parser(clap::value_parser!(std::path::PathBuf)),
        )
        .arg(arg!(--theme "Copies the default theme into your source folder"))
        .arg(arg!(--force "Skips confirmation prompts"))
        .arg(arg!(--title <title> "Sets the book title"))
        .arg(
            arg!(--ignore <ignore> "Creates a VCS ignore file (i.e. .gitignore)")
                .value_parser(["none", "git"]),
        )
}

// Init command implementation
pub fn execute(args: &ArgMatches) -> Result<()> {
    let book_dir = get_book_dir(args);
    let mut builder = MDBook::init(&book_dir);
    let mut config = config::Config::default();
    // If flag `--theme` is present, copy theme to src
    if args.get_flag("theme") {
        let theme_dir = book_dir.join("theme");
        println!();
        println!("Copying the default theme to {}", theme_dir.display());
        // Skip this if `--force` is present
        if !args.get_flag("force") && theme_dir.exists() {
            println!("This could potentially overwrite files already present in that directory.");
            print!("\nAre you sure you want to continue? (y/n) ");

            // Read answer from user and exit if it's not 'yes'
            if confirm() {
                builder.copy_theme(true);
            }
        } else {
            builder.copy_theme(true);
        }
    }

    if let Some(ignore) = args.get_one::<String>("ignore").map(|s| s.as_str()) {
        match ignore {
            "git" => builder.create_gitignore(true),
            _ => builder.create_gitignore(false),
        };
    } else if !args.get_flag("force") {
        println!("\nDo you want a .gitignore to be created? (y/n)");
        if confirm() {
            builder.create_gitignore(true);
        }
    }

    config.book.title = if args.contains_id("title") {
        args.get_one::<String>("title").map(String::from)
    } else if args.get_flag("force") {
        None
    } else {
        request_book_title()
    };

    if let Some(author) = get_author_name() {
        debug!("Obtained user name from gitconfig: {:?}", author);
        config.book.authors.push(author);
        builder.with_config(config);
    }

    builder.build()?;
    println!("\nAll done, no errors...");

    Ok(())
}

/// Obtains author name from git config file by running the `git config` command.
fn get_author_name() -> Option<String> {
    let output = Command::new("git")
        .args(["config", "--get", "user.name"])
        .output()
        .ok()?;

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_owned())
    } else {
        None
    }
}

/// Request book title from user and return if provided.
fn request_book_title() -> Option<String> {
    println!("What title would you like to give the book? ");
    io::stdout().flush().unwrap();
    let mut resp = String::new();
    io::stdin().read_line(&mut resp).unwrap();
    let resp = resp.trim();
    if resp.is_empty() {
        None
    } else {
        Some(resp.into())
    }
}

// Simple function for user confirmation
fn confirm() -> bool {
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    matches!(s.trim(), "Y" | "y" | "yes" | "Yes")
}
