// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: CC0-1.0

use merge::Merge;
use serde::Deserialize;
use structopt::StructOpt;

#[derive(Debug, Default, Deserialize, Merge, StructOpt)]
#[serde(default)]
struct Args {
    #[structopt(short, long)]
    #[merge(strategy = merge::bool::overwrite_false)]
    debug: bool,

    input: Option<String>,

    output: Option<String>,
}

fn get_config() -> Option<Args> {
    let path: &std::path::Path = "args.toml".as_ref();
    if path.is_file() {
        let s = std::fs::read_to_string(path).expect("Could not read configuration file");
        Some(toml::from_str(&s).expect("Could not parse configuration"))
    } else {
        None
    }
}

fn get_env() -> Args {
    envy::prefixed("ARGS_")
        .from_env()
        .expect("Could not read environment variables")
}

fn main() {
    let mut args = Args::from_args();
    args.merge(get_env());
    if let Some(config) = get_config() {
        args.merge(config);
    }
    println!("{:?}", args);
}
