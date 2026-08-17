## Location of Profile Files
* The location of the config file will be loaded from the `AWS_CONFIG_FILE` environment variable
with a fallback to `~/.aws/config`
* The location of the credentials file will be loaded from the `AWS_SHARED_CREDENTIALS_FILE`
environment variable with a fallback to `~/.aws/credentials`

The location of these files can also be customized programmatically using [`ProfileFiles`](crate::profile::profile_file::ProfileFiles).

## Home directory resolution
Home directory resolution is implemented to match the behavior of the CLI & Python. `~` is only
used for home directory resolution when it:
- Starts the path
- Is followed immediately by `/` or a platform specific separator. (On windows, `~/` and `~\` both
  resolve to the home directory.

When determining the home directory, the following environment variables are checked:
- `HOME` on all platforms
- `USERPROFILE` on Windows
- The concatenation of `HOMEDRIVE` and `HOMEPATH` on Windows (`$HOMEDRIVE$HOMEPATH`)
