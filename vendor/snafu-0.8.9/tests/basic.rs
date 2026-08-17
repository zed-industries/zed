use snafu::prelude::*;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Could not open config file at {}", filename.display()))]
    OpenConfig {
        filename: PathBuf,
        source: io::Error,
    },
    #[snafu(display("Could not open config file at {source}"))]
    SaveConfig { source: io::Error },
    #[snafu(display("User ID {user_id} is invalid"))]
    InvalidUser { user_id: i32 },
    #[snafu(display("No user available"))]
    MissingUser,
}

type Result<T, E = Error> = std::result::Result<T, E>;

const CONFIG_FILENAME: &str = "/tmp/config";

fn example(root: impl AsRef<Path>, user_id: Option<i32>) -> Result<()> {
    let root = root.as_ref();
    let filename = &root.join(CONFIG_FILENAME);

    let config = fs::read(filename).context(OpenConfigSnafu { filename })?;

    let _user_id = match user_id {
        None => MissingUserSnafu.fail()?,
        Some(user_id) if user_id != 42 => InvalidUserSnafu { user_id }.fail()?,
        Some(user_id) => user_id,
    };

    fs::write(filename, config).context(SaveConfigSnafu)?;

    Ok(())
}

#[test]
fn implements_error() {
    fn check<T: std::error::Error>() {}
    check::<Error>();
    example("/some/directory/that/does/not/exist", None).unwrap_err();
}
