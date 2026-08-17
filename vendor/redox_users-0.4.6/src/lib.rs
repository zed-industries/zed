//! `redox-users` is designed to be a small, low-ish level interface
//! to system user and group information, as well as user password
//! authentication. It is OS-specific and will break horribly on platforms
//! that are not [Redox-OS](https://redox-os.org).
//!
//! # Permissions
//! Because this is a system level tool dealing with password
//! authentication, programs are often required to run with
//! escalated priveleges. The implementation of the crate is
//! privelege unaware. The only privelege requirements are those
//! laid down by the system administrator over these files:
//! - `/etc/group`
//!   - Read: Required to access group information
//!   - Write: Required to change group information
//! - `/etc/passwd`
//!   - Read: Required to access user information
//!   - Write: Required to change user information
//! - `/etc/shadow`
//!   - Read: Required to authenticate users
//!   - Write: Required to set user passwords
//!
//! # Reimplementation
//! This crate is designed to be as small as possible without
//! sacrificing critical functionality. The idea is that a small
//! enough redox-users will allow easy re-implementation based on
//! the same flexible API. This would allow more complicated authentication
//! schemes for redox in future without breakage of existing
//! software.

use std::fmt::Debug;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
#[cfg(target_os = "redox")]
use std::os::unix::fs::OpenOptionsExt;
#[cfg(not(target_os = "redox"))]
use std::os::unix::io::AsRawFd;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::slice::{Iter, IterMut};
#[cfg(not(test))]
#[cfg(feature = "auth")]
use std::thread;
use std::time::Duration;

use thiserror::Error;
#[cfg(feature = "auth")]
use zeroize::Zeroize;

//#[cfg(not(target_os = "redox"))]
//use nix::fcntl::{flock, FlockArg};

#[cfg(target_os = "redox")]
use libredox::flag::{O_EXLOCK, O_SHLOCK};

const PASSWD_FILE: &'static str = "/etc/passwd";
const GROUP_FILE: &'static str = "/etc/group";
#[cfg(feature = "auth")]
const SHADOW_FILE: &'static str = "/etc/shadow";

const MIN_ID: usize = 1000;
const MAX_ID: usize = 6000;
const DEFAULT_TIMEOUT: u64 = 3;

const USERNAME_LEN_MIN: usize = 3;
const USERNAME_LEN_MAX: usize = 32;

/// Errors that might happen while using this crate
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("os error: {reason}")]
    Os { reason: &'static str },

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("failed to generate seed: {0}")]
    Getrandom(#[from] getrandom::Error),

    #[cfg(feature = "auth")]
    #[error("")]
    Argon(#[from] argon2::Error),

    #[error("parse error line {line}: {reason}")]
    Parsing { reason: String, line: usize },

    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("user not found")]
    UserNotFound,

    #[error("group not found")]
    GroupNotFound,

    #[error("user already exists")]
    UserAlreadyExists,

    #[error("group already exists")]
    GroupAlreadyExists,

    #[error("invalid name '{name}'")]
    InvalidName { name: String },

    /// Used for invalid string field values of [`User`]
    #[error("invalid entry element '{data}'")]
    InvalidData { data: String },
}
pub type Result<T, E = Error> = core::result::Result<T, E>;

#[inline]
fn parse_error(line: usize, reason: &str) -> Error {
    Error::Parsing {
        reason: reason.into(),
        line,
    }
}

impl From<libredox::error::Error> for Error {
    fn from(syscall_error: libredox::error::Error) -> Error {
        Error::Io(std::io::Error::from(syscall_error))
    }
}

#[derive(Clone, Copy, Debug)]
enum Lock {
    Shared,
    Exclusive,
}

impl Lock {
    fn can_write(&self) -> bool {
        match self {
            Lock::Shared => false,
            Lock::Exclusive => true,
        }
    }

    #[cfg(target_os = "redox")]
    fn as_olock(self) -> i32 {
        (match self {
            Lock::Shared => O_SHLOCK,
            Lock::Exclusive => O_EXLOCK,
        }) as i32
    }

    /*#[cfg(not(target_os = "redox"))]
    fn as_flock(self) -> FlockArg {
        match self {
            Lock::Shared => FlockArg::LockShared,
            Lock::Exclusive => FlockArg::LockExclusive,
        }
    }*/
}

/// Naive semi-cross platform file locking (need to support linux for tests).
#[allow(dead_code)]
fn locked_file(file: impl AsRef<Path>, lock: Lock) -> Result<File, Error> {
    #[cfg(test)]
    println!("Open file: {}", file.as_ref().display());

    #[cfg(target_os = "redox")]
    {
        Ok(OpenOptions::new()
            .read(true)
            .write(lock.can_write())
            .custom_flags(lock.as_olock())
            .open(file)?)
    }
    #[cfg(not(target_os = "redox"))]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    {
        let file = OpenOptions::new()
            .read(true)
            .write(lock.can_write())
            .open(file)?;
        let fd = file.as_raw_fd();
        eprintln!("Fd: {}", fd);
        //flock(fd, lock.as_flock())?;
        Ok(file)
    }
}

/// Reset a file for rewriting (user/group dbs must be erased before write-out)
fn reset_file(fd: &mut File) -> Result<(), Error> {
    fd.set_len(0)?;
    fd.seek(SeekFrom::Start(0))?;
    Ok(())
}

/// Is a string safe to write to `/etc/group` or `/etc/passwd`?
fn is_safe_string(s: &str) -> bool {
    !s.contains(';')
}

const PORTABLE_FILE_NAME_CHARS: &str =
    "0123456789._-abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// This function is used by [`UserBuilder`] and [`GroupBuilder`] to determine
/// if a name for a user/group is valid. It is provided for convenience.
///
/// Usernames must match the [POSIX standard
/// for usernames](https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap03.html#tag_03_437)
/// . The "portable filename character set" is defined as `A-Z`, `a-z`, `0-9`,
/// and `._-` (see [here](https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap03.html#tag_03_282)).
///
/// Usernames may not be more than 32 or less than 3 characters in length.
pub fn is_valid_name(name: &str) -> bool {
    if name.len() < USERNAME_LEN_MIN || name.len() > USERNAME_LEN_MAX {
        false
    } else if let Some(first) = name.chars().next() {
        first != '-' &&
            name.chars().all(|c| {
                PORTABLE_FILE_NAME_CHARS.contains(c)
            })
    } else {
        false
    }
}

/// Marker types for [`User`] and [`AllUsers`].
pub mod auth {
    #[cfg(feature = "auth")]
    use std::fmt;

    #[cfg(feature = "auth")]
    use zeroize::Zeroize;

    #[cfg(feature = "auth")]
    use crate::Error;

    /// Marker type indicating that a `User` only has access to world-readable
    /// user information, and cannot authenticate.
    #[derive(Debug, Default)]
    pub struct Basic {}

    /// Marker type indicating that a `User` has access to all user
    /// information, including password hashes.
    #[cfg(feature = "auth")]
    #[derive(Default, Zeroize)]
    #[zeroize(drop)]
    pub struct Full {
        pub(crate) hash: String,
    }

    #[cfg(feature = "auth")]
    impl Full {
        pub(crate) fn empty() -> Full {
            Full { hash: "".into() }
        }

        pub(crate) fn is_empty(&self) -> bool {
            &self.hash == ""
        }

        pub(crate) fn unset() -> Full {
            Full { hash: "!".into() }
        }

        pub(crate) fn is_unset(&self) -> bool {
            &self.hash == "!"
        }

        pub(crate) fn passwd(pw: &str) -> Result<Full, Error> {
            Ok(if pw != "" {
                let mut buf = [0u8; 8];
                getrandom::getrandom(&mut buf)?;
                let mut salt = format!("{:X}", u64::from_ne_bytes(buf));

                let config = argon2::Config::default();
                let hash: String = argon2::hash_encoded(
                    pw.as_bytes(),
                    salt.as_bytes(),
                    &config
                )?;

                buf.zeroize();
                salt.zeroize();
                Full { hash } // note that move == shallow copy in Rust
            } else {
                Full::empty()
            })
        }

        pub(crate) fn verify(&self, pw: &str) -> bool {
            match self.hash.as_str() {
                "" => pw == "",
                "!" => false,
                //TODO: When does this panic? Should this function return
                // Result? Or does it need to simply fail to verify if
                // verify_encoded() fails?
                hash => argon2::verify_encoded(&hash, pw.as_bytes())
                    .expect("failed to verify hash"),
            }
        }
    }

    #[cfg(feature = "auth")]
    impl fmt::Debug for Full {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("Full")
                .finish()
        }
    }
}

/// A builder pattern for adding [`User`]s to [`AllUsers`]. Fields are verified
/// when the group is built via [`AllUsers::add_user`]. See the documentation
/// of that function for default values.
///
/// Note that this builder is not available when the `auth` feature of the
/// crate is disabled.
///
/// # Example
/// ```no_run
/// # use redox_users::{AllGroups, Config, GroupBuilder, UserBuilder};
/// let mut allgs = AllGroups::new(Config::default()).unwrap();
///
/// let g = GroupBuilder::new("foobar")
///     .user("foobar");
/// let foobar_g = allgs.add_group(g).unwrap();
///
/// let u = UserBuilder::new("foobar")
///     .gid(foobar_g.gid)
///     .name("Foo Bar")
///     // Note that this directory will not be created
///     .home("file:/home/foobar");
/// ```
#[cfg(feature = "auth")]
pub struct UserBuilder {
    user: String,
    uid: Option<usize>,
    gid: Option<usize>,
    name: Option<String>,
    home: Option<String>,
    shell: Option<String>,
}

#[cfg(feature = "auth")]
impl UserBuilder {
    /// Create a new `UserBuilder` with the login name for the new user.
    pub fn new(user: impl AsRef<str>) -> UserBuilder {
        UserBuilder {
            user: user.as_ref().to_string(),
            uid: None,
            gid: None,
            name: None,
            home: None,
            shell: None,
        }
    }

    /// Set the user id for this user.
    pub fn uid(mut self, uid: usize) -> UserBuilder {
        self.uid = Some(uid);
        self
    }

    /// Set the primary group id for this user.
    pub fn gid(mut self, gid: usize) -> UserBuilder {
        self.gid = Some(gid);
        self
    }

    /// Set the GECOS field for this user.
    pub fn name(mut self, name: impl AsRef<str>) -> UserBuilder {
        self.name = Some(name.as_ref().to_string());
        self
    }

    /// Set the home directory for this user.
    pub fn home(mut self, home: impl AsRef<str>) -> UserBuilder {
        self.home = Some(home.as_ref().to_string());
        self
    }

    /// Set the login shell for this user.
    pub fn shell(mut self, shell: impl AsRef<str>) -> UserBuilder {
        self.shell = Some(shell.as_ref().to_string());
        self
    }
}

/// A struct representing a Redox user.
/// Currently maps to an entry in the `/etc/passwd` file.
///
/// `A` should be a type from [`crate::auth`].
///
/// # Unset vs. Blank Passwords
/// A note on unset passwords vs. blank passwords. A blank password
/// is a hash field that is completely blank (aka, `""`). According
/// to this crate, successful login is only allowed if the input
/// password is blank as well.
///
/// An unset password is one whose hash is not empty (`""`), but
/// also not a valid serialized argon2rs hashing session. This
/// hash always returns `false` upon attempted verification. The
/// most commonly used hash for an unset password is `"!"`, but
/// this crate makes no distinction. The most common way to unset
/// the password is to use [`User::unset_passwd`].
#[derive(Debug)]
pub struct User<A> {
    /// Username (login name)
    pub user: String,
    /// User id
    pub uid: usize,
    /// Group id
    pub gid: usize,
    /// Real name (human readable, can contain spaces)
    pub name: String,
    /// Home directory path
    pub home: String,
    /// Shell path
    pub shell: String,

    // Failed login delay duration
    auth_delay: Duration,

    #[allow(dead_code)]
    auth: A,
}

impl<A: Default> User<A> {
    /// Get a Command to run the user's default shell (see [`User::login_cmd`]
    /// for more docs).
    pub fn shell_cmd(&self) -> Command { self.login_cmd(&self.shell) }

    /// Provide a login command for the user, which is any entry point for
    /// starting a user's session, whether a shell (use [`User::shell_cmd`]
    /// instead) or a graphical init.
    ///
    /// The `Command` will use the user's `uid` and `gid`, its `current_dir`
    /// will be set to the user's home directory, and the follwing enviroment
    /// variables will be populated:
    ///
    ///    - `USER` set to the user's `user` field.
    ///    - `UID` set to the user's `uid` field.
    ///    - `GROUPS` set the user's `gid` field.
    ///    - `HOME` set to the user's `home` field.
    ///    - `SHELL` set to the user's `shell` field.
    pub fn login_cmd<T>(&self, cmd: T) -> Command
        where T: std::convert::AsRef<std::ffi::OsStr> + AsRef<str>
    {
        let mut command = Command::new(cmd);
        command
            .uid(self.uid as u32)
            .gid(self.gid as u32)
            .current_dir(&self.home)
            .env("USER", &self.user)
            .env("UID", format!("{}", self.uid))
            .env("GROUPS", format!("{}", self.gid))
            .env("HOME", &self.home)
            .env("SHELL", &self.shell);
        command
    }

    fn from_passwd_entry(s: &str, line: usize) -> Result<User<A>, Error> {
        let mut parts = s.split(';');

        let user = parts
            .next()
            .ok_or(parse_error(line, "expected user"))?;
        let uid = parts
            .next()
            .ok_or(parse_error(line, "expected uid"))?
            .parse::<usize>()?;
        let gid = parts
            .next()
            .ok_or(parse_error(line, "expected uid"))?
            .parse::<usize>()?;
        let name = parts
            .next()
            .ok_or(parse_error(line, "expected real name"))?;
        let home = parts
            .next()
            .ok_or(parse_error(line, "expected home dir path"))?;
        let shell = parts
            .next()
            .ok_or(parse_error(line, "expected shell path"))?;

        Ok(User::<A> {
            user: user.into(),
            uid,
            gid,
            name: name.into(),
            home: home.into(),
            shell: shell.into(),
            auth: A::default(),
            auth_delay: Duration::default(),
        })
    }
}

#[cfg(feature = "auth")]
impl User<auth::Full> {
    /// Set the password for a user. Make **sure** that `password`
    /// is actually what the user wants as their password (this doesn't).
    ///
    /// To set the password blank, pass `""` as `password`.
    ///
    /// Note that `password` is taken as a reference, so it is up to the caller
    /// to properly zero sensitive memory (see `zeroize` on crates.io).
    pub fn set_passwd(&mut self, password: impl AsRef<str>) -> Result<(), Error> {
        self.auth = auth::Full::passwd(password.as_ref())?;
        Ok(())
    }

    /// Unset the password ([`User::verify_passwd`] always returns `false`).
    pub fn unset_passwd(&mut self) {
        self.auth = auth::Full::unset();
    }

    /// Verify the password. If the hash is empty, this only returns `true` if
    /// `password` is also empty.
    ///
    /// Note that this is a blocking operation if the password is incorrect.
    /// See [`Config::auth_delay`] to set the wait time. Default is 3 seconds.
    ///
    /// Note that `password` is taken as a reference, so it is up to the caller
    /// to properly zero sensitive memory (see `zeroize` on crates.io).
    pub fn verify_passwd(&self, password: impl AsRef<str>) -> bool {
        let verified = self.auth.verify(password.as_ref());
        if !verified {
            #[cfg(not(test))] // Make tests run faster
            thread::sleep(self.auth_delay);
        }
        verified
    }

    /// Determine if the hash for the password is blank ([`User::verify_passwd`]
    /// returns `true` *only* when the password is blank).
    pub fn is_passwd_blank(&self) -> bool {
        self.auth.is_empty()
    }

    /// Determine if the hash for the password is unset
    /// ([`User::verify_passwd`] returns `false` regardless of input).
    pub fn is_passwd_unset(&self) -> bool {
        self.auth.is_unset()
    }

    /// Format this user as an entry in `/etc/passwd`.
    fn passwd_entry(&self) -> Result<String, Error> {
        if !is_safe_string(&self.user) {
            Err(Error::InvalidName { name: self.user.to_string() })
        } else if !is_safe_string(&self.name) {
            Err(Error::InvalidData { data: self.name.to_string() })
        } else if !is_safe_string(&self.home) {
            Err(Error::InvalidData { data: self.home.to_string() })
        } else if !is_safe_string(&self.shell) {
            Err(Error::InvalidData { data: self.shell.to_string() })
        } else {
            #[cfg_attr(rustfmt, rustfmt_skip)]
            Ok(format!("{};{};{};{};{};{}\n",
                self.user, self.uid, self.gid, self.name, self.home, self.shell
            ))
        }
    }

    fn shadow_entry(&self) -> Result<String, Error> {
        if !is_safe_string(&self.user) {
            Err(Error::InvalidName { name: self.user.to_string() })
        } else {
            Ok(format!("{};{}\n", self.user, self.auth.hash))
        }
    }
}

impl<A> Name for User<A> {
    fn name(&self) -> &str {
        &self.user
    }
}

impl<A> Id for User<A> {
    fn id(&self) -> usize {
        self.uid
    }
}

/// A builder pattern for adding [`Group`]s to [`AllGroups`]. Fields are
/// verified when the `Group` is built, via [`AllGroups::add_group`].
///
/// # Example
/// ```
/// # use redox_users::GroupBuilder;
/// // When added, this group will use the first available group id
/// let mygroup = GroupBuilder::new("group_name");
///
/// // A little more stuff:
/// let other = GroupBuilder::new("special")
///     .gid(9055)
///     .user("some_username");
/// ```
pub struct GroupBuilder {
    // Group name
    group: String,

    gid: Option<usize>,

    users: Vec<String>,
}

impl GroupBuilder {
    /// Create a new `GroupBuilder` with the given group name.
    pub fn new(group: impl AsRef<str>) -> GroupBuilder {
        GroupBuilder {
            group: group.as_ref().to_string(),
            gid: None,
            users: vec![],
        }
    }

    /// Set the group id of this group.
    pub fn gid(mut self, gid: usize) -> GroupBuilder {
        self.gid = Some(gid);
        self
    }

    /// Add a user to this group. Call this function multiple times to add more
    /// users.
    pub fn user(mut self, user: impl AsRef<str>) -> GroupBuilder {
        self.users.push(user.as_ref().to_string());
        self
    }
}

/// A struct representing a Redox user group.
/// Currently maps to an `/etc/group` file entry.
#[derive(Debug)]
pub struct Group {
    /// Group name
    pub group: String,
    /// Password (unused, usually "x")
    pub password: String,
    /// Unique group id
    pub gid: usize,
    /// Group members' usernames
    pub users: Vec<String>,
}

impl Group {
    fn from_group_entry(s: &str, line: usize) -> Result<Group, Error> {
        let mut parts = s.trim()
            .split(';');

        let group = parts
            .next()
            .ok_or(parse_error(line, "expected group"))?;
        let password = parts
            .next()
            .ok_or(parse_error(line, "expected password"))?;
        let gid = parts
            .next()
            .ok_or(parse_error(line, "expected gid"))?
            .parse::<usize>()?;
        let users_str = parts.next()
            .unwrap_or("");
        let users = users_str.split(',')
            .filter_map(|u| if u == "" {
                None
            } else {
                Some(u.into())
            })
            .collect();

        Ok(Group {
            group: group.into(),
            password: password.into(),
            gid,
            users,
        })
    }

    fn group_entry(&self) -> Result<String, Error> {
        if !is_safe_string(&self.group) {
            Err(Error::InvalidName { name: self.group.to_string() })
        } else {
            for username in self.users.iter() {
                if !is_safe_string(&username) {
                    return Err(Error::InvalidData { data: username.to_string() });
                }
            }

            #[cfg_attr(rustfmt, rustfmt_skip)]
            Ok(format!("{};{};{};{}\n",
                self.group,
                self.password,
                self.gid,
                self.users.join(",").trim_matches(',')
            ))
        }
    }
}

impl Name for Group {
    fn name(&self) -> &str {
        &self.group
    }
}

impl Id for Group {
    fn id(&self) -> usize {
        self.gid
    }
}

/// Gets the current process effective user ID.
///
/// This function issues the `geteuid` system call returning the process effective
/// user id.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// # use redox_users::get_euid;
/// let euid = get_euid().unwrap();
/// ```
pub fn get_euid() -> Result<usize, Error> {
    libredox::call::geteuid()
        .map_err(From::from)
}

/// Gets the current process real user ID.
///
/// This function issues the `getuid` system call returning the process real
/// user id.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// # use redox_users::get_uid;
/// let uid = get_uid().unwrap();
/// ```
pub fn get_uid() -> Result<usize, Error> {
    libredox::call::getruid()
        .map_err(From::from)
}

/// Gets the current process effective group ID.
///
/// This function issues the `getegid` system call returning the process effective
/// group id.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// # use redox_users::get_egid;
/// let egid = get_egid().unwrap();
/// ```
pub fn get_egid() -> Result<usize, Error> {
    libredox::call::getegid()
        .map_err(From::from)
}

/// Gets the current process real group ID.
///
/// This function issues the `getegid` system call returning the process real
/// group id.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// # use redox_users::get_gid;
/// let gid = get_gid().unwrap();
/// ```
pub fn get_gid() -> Result<usize, Error> {
    libredox::call::getrgid()
        .map_err(From::from)
}

/// A generic configuration that allows fine control of an [`AllUsers`] or
/// [`AllGroups`].
///
/// `auth_delay` is not used by [`AllGroups`]
///
/// In most situations, [`Config::default`](struct.Config.html#impl-Default)
/// will work just fine. The other fields are for finer control if it is
/// required.
///
/// # Example
/// ```
/// # use redox_users::Config;
/// use std::time::Duration;
///
/// let cfg = Config::default()
///     .min_id(500)
///     .max_id(1000)
///     .auth_delay(Duration::from_secs(5));
/// ```
#[derive(Clone, Debug)]
pub struct Config {
    root_fs: PathBuf,
    auth_delay: Duration,
    min_id: usize,
    max_id: usize,
    lock: Lock,
}

impl Config {
    /// Set the delay for a failed authentication. Default is 3 seconds.
    pub fn auth_delay(mut self, delay: Duration) -> Config {
        self.auth_delay = delay;
        self
    }

    /// Set the smallest ID possible to use when finding an unused ID.
    pub fn min_id(mut self, id: usize) -> Config {
        self.min_id = id;
        self
    }

    /// Set the largest possible ID to use when finding an unused ID.
    pub fn max_id(mut self, id: usize) -> Config {
        self.max_id = id;
        self
    }

    /// Set the scheme relative to which the [`AllUsers`] or [`AllGroups`]
    /// should be looking for its data files. This is a compromise between
    /// exposing implementation details and providing fine enough
    /// control over the behavior of this API.
    // FIXME rename to root_fs the next time we release a breaking change
    pub fn scheme(mut self, scheme: String) -> Config {
        self.root_fs = PathBuf::from(scheme);
        self
    }

    /// Allow writes to group, passwd, and shadow files
    pub fn writeable(mut self, writeable: bool) -> Config {
        self.lock = if writeable {
            Lock::Exclusive
        } else {
            Lock::Shared
        };
        self
    }

    // Prepend a path with the scheme in this Config
    fn in_root_fs(&self, path: impl AsRef<Path>) -> PathBuf {
        let mut canonical_path = self.root_fs.clone();
        // Should be a little careful here, not sure I want this behavior
        if path.as_ref().is_absolute() {
            // This is nasty
            canonical_path.push(path.as_ref().to_string_lossy()[1..].to_string());
        } else {
            canonical_path.push(path);
        }
        canonical_path
    }
}

impl Default for Config {
    /// The default root filesystem is `/`.
    ///
    /// The default auth delay is 3 seconds.
    ///
    /// The default min and max ids are 1000 and 6000.
    fn default() -> Config {
        Config {
            root_fs: PathBuf::from("/"),
            auth_delay: Duration::new(DEFAULT_TIMEOUT, 0),
            min_id: MIN_ID,
            max_id: MAX_ID,
            lock: Lock::Shared,
        }
    }
}

// Nasty hack to prevent the compiler complaining about
// "leaking" `AllInner`
mod sealed {
    use crate::Config;

    pub trait Name {
        fn name(&self) -> &str;
    }

    pub trait Id {
        fn id(&self) -> usize;
    }

    pub trait AllInner {
        // Group+User, thanks Dad
        type Gruser: Name + Id;

        /// These functions grab internal elements so that the other
        /// methods of `All` can manipulate them.
        fn list(&self) -> &Vec<Self::Gruser>;
        fn list_mut(&mut self) -> &mut Vec<Self::Gruser>;
        fn config(&self) -> &Config;
    }
}

use sealed::{AllInner, Id, Name};

/// This trait is used to remove repetitive API items from
/// [`AllGroups`] and [`AllUsers`]. It uses a hidden trait
/// so that the implementations of functions can be implemented
/// at the trait level. Do not try to implement this trait.
pub trait All: AllInner {
    /// Get an iterator borrowing all [`User`]s or [`Group`]s on the system.
    fn iter(&self) -> Iter<<Self as AllInner>::Gruser> {
        self.list().iter()
    }

    /// Get an iterator mutably borrowing all [`User`]s or [`Group`]s on the
    /// system.
    fn iter_mut(&mut self) -> IterMut<<Self as AllInner>::Gruser> {
        self.list_mut().iter_mut()
    }

    /// Borrow the [`User`] or [`Group`] with a given name.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// # use redox_users::{All, AllUsers, Config};
    /// let users = AllUsers::basic(Config::default()).unwrap();
    /// let user = users.get_by_name("root").unwrap();
    /// ```
    fn get_by_name(&self, name: impl AsRef<str>) -> Option<&<Self as AllInner>::Gruser> {
        self.iter()
            .find(|gruser| gruser.name() == name.as_ref() )
    }

    /// Mutable version of [`All::get_by_name`].
    fn get_mut_by_name(&mut self, name: impl AsRef<str>) -> Option<&mut <Self as AllInner>::Gruser> {
        self.iter_mut()
            .find(|gruser| gruser.name() == name.as_ref() )
    }

    /// Borrow the [`User`] or [`Group`] with the given ID.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// # use redox_users::{All, AllUsers, Config};
    /// let users = AllUsers::basic(Config::default()).unwrap();
    /// let user = users.get_by_id(0).unwrap();
    /// ```
    fn get_by_id(&self, id: usize) -> Option<&<Self as AllInner>::Gruser> {
        self.iter()
            .find(|gruser| gruser.id() == id )
    }

    /// Mutable version of [`All::get_by_id`].
    fn get_mut_by_id(&mut self, id: usize) -> Option<&mut <Self as AllInner>::Gruser> {
        self.iter_mut()
            .find(|gruser| gruser.id() == id )
    }

    /// Provides an unused id based on the min and max values in the [`Config`]
    /// passed to the `All`'s constructor.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use redox_users::{All, AllUsers, Config};
    /// let users = AllUsers::basic(Config::default()).unwrap();
    /// let uid = users.get_unique_id().expect("no available uid");
    /// ```
    fn get_unique_id(&self) -> Option<usize> {
        for id in self.config().min_id..self.config().max_id {
            if !self.iter().any(|gruser| gruser.id() == id ) {
                return Some(id)
            }
        }
        None
    }

    /// Remove a [`User`] or [`Group`] from this `All` given it's name. If the
    /// Gruser was removed return `true`, else return `false`. This ensures
    /// that the Gruser no longer exists.
    fn remove_by_name(&mut self, name: impl AsRef<str>) -> bool {
        let list = self.list_mut();
        let indx = list.iter()
            .enumerate()
            .find_map(|(indx, gruser)| if gruser.name() == name.as_ref() {
                    Some(indx)
                } else {
                    None
                });
        if let Some(indx) = indx {
            list.remove(indx);
            true
        } else {
            false
        }
    }

    /// Id version of [`All::remove_by_name`].
    fn remove_by_id(&mut self, id: usize) -> bool {
        let list = self.list_mut();
        let indx = list.iter()
            .enumerate()
            .find_map(|(indx, gruser)| if gruser.id() == id {
                    Some(indx)
                } else {
                    None
                });
        if let Some(indx) = indx {
            list.remove(indx);
            true
        } else {
            false
        }
    }
}

/// `AllUsers` provides (borrowed) access to all the users on the system.
/// Note that this struct implements [`All`] for all of its access functions.
///
/// # Notes
/// Note that everything in this section also applies to [`AllGroups`].
///
/// * If you mutate anything owned by an `AllUsers`, you must call the
///   [`AllUsers::save`] in order for those changes to be applied to the system.
/// * The API here is kept small. Most mutating actions can be accomplished via
///   the [`All::get_mut_by_id`] and [`All::get_mut_by_name`]
///   functions.
#[derive(Debug)]
pub struct AllUsers<A> {
    users: Vec<User<A>>,
    config: Config,

    // Hold on to the locked fds to prevent race conditions
    #[allow(dead_code)]
    passwd_fd: File,
    #[allow(dead_code)]
    shadow_fd: Option<File>,
}

impl<A: Default> AllUsers<A> {
    pub fn new(config: Config) -> Result<AllUsers<A>, Error> {
        let mut passwd_fd = locked_file(config.in_root_fs(PASSWD_FILE), config.lock)?;
        let mut passwd_cntnt = String::new();
        passwd_fd.read_to_string(&mut passwd_cntnt)?;

        let mut passwd_entries = Vec::new();
        for (indx, line) in passwd_cntnt.lines().enumerate() {
            let mut user = User::from_passwd_entry(line, indx)?;
            user.auth_delay = config.auth_delay;
            passwd_entries.push(user);
        }

        Ok(AllUsers::<A> {
            users: passwd_entries,
            config,
            passwd_fd,
            shadow_fd: None,
        })
    }
}

impl AllUsers<auth::Basic> {
    /// Provide access to all user information on the system except
    /// authentication. This is adequate for almost all uses of `AllUsers`.
    pub fn basic(config: Config) -> Result<AllUsers<auth::Basic>, Error> {
        Self::new(config)
    }
}

#[cfg(feature = "auth")]
impl AllUsers<auth::Full> {
    /// If access to password related methods for the [`User`]s yielded by this
    /// `AllUsers` is required, use this constructor.
    pub fn authenticator(config: Config) -> Result<AllUsers<auth::Full>, Error> {
        let mut shadow_fd = locked_file(config.in_root_fs(SHADOW_FILE), config.lock)?;
        let mut shadow_cntnt = String::new();
        shadow_fd.read_to_string(&mut shadow_cntnt)?;
        let shadow_entries: Vec<&str> = shadow_cntnt.lines().collect();

        let mut new = Self::new(config)?;
        new.shadow_fd = Some(shadow_fd);

        for (indx, entry) in shadow_entries.iter().enumerate() {
            let mut entry = entry.split(';');
            let name = entry.next().ok_or(parse_error(indx,
                "error parsing shadowfile: expected username"
            ))?;
            let hash = entry.next().ok_or(parse_error(indx,
                "error parsing shadowfile: expected hash"
            ))?;
            new.users
                .iter_mut()
                .find(|user| user.user == name)
                .ok_or(parse_error(indx,
                    "error parsing shadowfile: unkown user"
                ))?.auth.hash = hash.to_string();
        }

        shadow_cntnt.zeroize();
        Ok(new)
    }

    /// Consumes a builder, adding a new user to this `AllUsers`. Returns a
    /// reference to the created user.
    ///
    /// Make sure to call [`AllUsers::save`] in order for the new user to be
    /// applied to the system.
    ///
    /// Note that the user's password is set unset (see
    /// [Unset vs Blank Passwords](struct.User.html#unset-vs-blank-passwords))
    /// during this call.
    ///
    /// Also note that the user is not added to any groups when this builder is
    /// consumed. In order to keep the system in a consistent state, it is
    /// reccomended to also use an instance of [`AllGroups`] to update group
    /// information when creating new users.
    ///
    /// # Defaults
    /// Fields not passed to the builder before calling this function are as
    /// follows:
    /// - `uid`: [`AllUsers::get_unique_id`] is called on self to get the next
    ///   available id.
    /// - `gid`: `99`. This is the default UID for the group `nobody`. Note
    ///   that the user is NOT added to this group in `/etc/groups`.
    /// - `name`: The login name passed to [`UserBuilder::new`].
    /// - `home`: `"/"`
    /// - `shell`: `file:/bin/ion`
    pub fn add_user(&mut self, builder: UserBuilder) -> Result<&User<auth::Full>, Error> {
        if !is_valid_name(&builder.user) {
            return Err(Error::InvalidName { name: builder.user });
        }

        let uid = builder.uid.unwrap_or_else(||
            self.get_unique_id()
                .expect("no remaining unused user ids")
        );

        if self.iter().any(|user| user.user == builder.user || user.uid == uid) {
            Err(Error::UserAlreadyExists)
        } else {
            self.users.push(User {
                user: builder.user.clone(),
                uid,
                gid: builder.gid.unwrap_or(99),
                name: builder.name.unwrap_or(builder.user),
                home: builder.home.unwrap_or("/".to_string()),
                shell: builder.shell.unwrap_or("file:/bin/ion".to_string()),
                auth: auth::Full::unset(),
                auth_delay: self.config.auth_delay
            });
            Ok(&self.users[self.users.len() - 1])
        }
    }

    /// Syncs the data stored in the `AllUsers` instance to the filesystem.
    /// To apply changes to the system from an `AllUsers`, you MUST call this
    /// function!
    pub fn save(&mut self) -> Result<(), Error> {
        let mut userstring = String::new();

        // Need to be careful to prevent allocations here so that
        // shadowstring can be zeroed when this process is complete.
        // 1 is suppossedly parallelism, not sure exactly what this means.
        // 16 is the max length of a u64, which is used as the salt.
        // 2 accounts for the semicolon separator and newline
        let acfg = argon2::Config::default();
        let argon_len = argon2::encoded_len(
            acfg.variant, acfg.mem_cost, acfg.time_cost,
            1, 16, acfg.hash_length) as usize;
        let mut shadowstring = String::with_capacity(
            self.users.len() * (USERNAME_LEN_MAX + argon_len + 2)
        );

        for user in &self.users {
            userstring.push_str(&user.passwd_entry()?);

            let mut shadow_entry = user.shadow_entry()?;
            shadowstring.push_str(&shadow_entry);

            shadow_entry.zeroize();
        }

        let mut shadow_fd = self.shadow_fd.as_mut()
            .expect("shadow_fd should exist for AllUsers<auth::Full>");

        reset_file(&mut self.passwd_fd)?;
        self.passwd_fd.write_all(userstring.as_bytes())?;

        reset_file(&mut shadow_fd)?;
        shadow_fd.write_all(shadowstring.as_bytes())?;

        shadowstring.zeroize();
        Ok(())
    }
}

impl<A> AllInner for AllUsers<A> {
    type Gruser = User<A>;

    fn list(&self) -> &Vec<Self::Gruser> {
        &self.users
    }

    fn list_mut(&mut self) -> &mut Vec<Self::Gruser> {
        &mut self.users
    }

    fn config(&self) -> &Config {
        &self.config
    }
}

impl<A> All for AllUsers<A> {}
/*
#[cfg(not(target_os = "redox"))]
impl<A> Drop for AllUsers<A> {
    fn drop(&mut self) {
        eprintln!("Dropping AllUsers");
        let _ = flock(self.passwd_fd.as_raw_fd(), FlockArg::Unlock);
        if let Some(fd) = self.shadow_fd.as_ref() {
            eprintln!("Shadow");
            let _ = flock(fd.as_raw_fd(), FlockArg::Unlock);
        }
    }
}
*/
/// `AllGroups` provides (borrowed) access to all groups on the system. Note
/// that this struct implements [`All`] for all of its access functions.
///
/// General notes that also apply to this struct may be found with
/// [`AllUsers`].
#[derive(Debug)]
pub struct AllGroups {
    groups: Vec<Group>,
    config: Config,

    group_fd: File,
}

impl AllGroups {
    /// Create a new `AllGroups`.
    pub fn new(config: Config) -> Result<AllGroups, Error> {
        let mut group_fd = locked_file(config.in_root_fs(GROUP_FILE), config.lock)?;
        let mut group_cntnt = String::new();
        group_fd.read_to_string(&mut group_cntnt)?;

        let mut entries: Vec<Group> = Vec::new();
        for (indx, line) in group_cntnt.lines().enumerate() {
            let group = Group::from_group_entry(line, indx)?;
            entries.push(group);
        }

        Ok(AllGroups {
            groups: entries,
            config,
            group_fd,
        })
    }

    /// Consumes a builder, adding a new group to this `AllGroups`. Returns a
    /// reference to the created `Group`.
    ///
    /// Make sure to call [`AllGroups::save`] in order for the new group to be
    /// applied to the system.
    ///
    /// # Defaults
    /// If a builder is not passed a group id ([`GroupBuilder::gid`]) before
    /// being passed to this function, [`AllGroups::get_unique_id`] is used.
    ///
    /// If the builder is not passed any users ([`GroupBuilder::user`]), the
    /// group will still be created.
    pub fn add_group(&mut self, builder: GroupBuilder) -> Result<&Group, Error> {
        let group_exists = self.iter()
            .any(|group| {
                let gid_taken = if let Some(gid) = builder.gid {
                    group.gid == gid
                } else {
                    false
                };
                group.group == builder.group || gid_taken
            });

        if group_exists {
            Err(Error::GroupAlreadyExists)
        } else if !is_valid_name(&builder.group) {
            Err(Error::InvalidName { name: builder.group })
        } else {
            for username in builder.users.iter() {
                if !is_valid_name(username) {
                    return Err(Error::InvalidName { name: username.to_string() });
                }
            }

            self.groups.push(Group {
                group: builder.group,
                password: "x".into(),
                gid: builder.gid.unwrap_or_else(||
                    self.get_unique_id()
                        .expect("no remaining unused group IDs")
                ),
                users: builder.users,
            });
            Ok(&self.groups[self.groups.len() - 1])
        }
    }

    /// Syncs the data stored in this `AllGroups` instance to the filesystem.
    /// To apply changes from an `AllGroups`, you MUST call this function!
    pub fn save(&mut self) -> Result<(), Error> {
        let mut groupstring = String::new();
        for group in &self.groups {
            groupstring.push_str(&group.group_entry()?);
        }

        reset_file(&mut self.group_fd)?;
        self.group_fd.write_all(groupstring.as_bytes())?;
        Ok(())
    }
}

impl AllInner for AllGroups {
    type Gruser = Group;

    fn list(&self) -> &Vec<Self::Gruser> {
        &self.groups
    }

    fn list_mut(&mut self) -> &mut Vec<Self::Gruser> {
        &mut self.groups
    }

    fn config(&self) -> &Config {
        &self.config
    }
}

impl All for AllGroups {}
/*
#[cfg(not(target_os = "redox"))]
impl Drop for AllGroups {
    fn drop(&mut self) {
        eprintln!("Dropping AllGroups");
        let _ = flock(self.group_fd.as_raw_fd(), FlockArg::Unlock);
    }
}*/

#[cfg(test)]
mod test {
    use super::*;

    const TEST_PREFIX: &'static str = "tests";

    /// Needed for the file checks, this is done by the library
    fn test_prefix(filename: &str) -> String {
        let mut complete = String::from(TEST_PREFIX);
        complete.push_str(filename);
        complete
    }

    #[test]
    fn test_safe_string() {
        assert!(is_safe_string("Hello\\$!"));
        assert!(!is_safe_string("semicolons are awesome; yeah!"));
    }

    #[test]
    fn test_portable_filename() {
        let valid = |s| {
            assert!(is_valid_name(s));
        };
        let invld = |s| {
            assert!(!is_valid_name(s));
        };
        valid("valid");
        valid("vld.io");
        valid("hyphen-ated");
        valid("under_scores");
        valid("1334");

        invld("-no_flgs");
        invld("invalid!");
        invld("also:invalid");
        invld("coolie-o?");
        invld("sh");
        invld("avery_very_very_very_loooooooonnggg-username");
    }

    fn test_cfg() -> Config {
        Config::default()
            // Since all this really does is prepend `sheme` to the consts
            .scheme(TEST_PREFIX.to_string())
            .writeable(true)
    }

    fn read_locked_file(file: impl AsRef<Path>) -> Result<String, Error> {
        let mut fd = locked_file(file, Lock::Shared)?;
        let mut cntnt = String::new();
        fd.read_to_string(&mut cntnt)?;
        Ok(cntnt)
    }

    // *** struct.User ***
    #[cfg(feature = "auth")]
    #[test]
    fn attempt_user_api() {
        let mut users = AllUsers::authenticator(test_cfg()).unwrap();
        let user = users.get_mut_by_id(1000).unwrap();

        assert_eq!(user.is_passwd_blank(), true);
        assert_eq!(user.is_passwd_unset(), false);
        assert_eq!(user.verify_passwd(""), true);
        assert_eq!(user.verify_passwd("Something"), false);

        user.set_passwd("hi,i_am_passwd").unwrap();

        assert_eq!(user.is_passwd_blank(), false);
        assert_eq!(user.is_passwd_unset(), false);
        assert_eq!(user.verify_passwd(""), false);
        assert_eq!(user.verify_passwd("Something"), false);
        assert_eq!(user.verify_passwd("hi,i_am_passwd"), true);

        user.unset_passwd();

        assert_eq!(user.is_passwd_blank(), false);
        assert_eq!(user.is_passwd_unset(), true);
        assert_eq!(user.verify_passwd(""), false);
        assert_eq!(user.verify_passwd("Something"), false);
        assert_eq!(user.verify_passwd("hi,i_am_passwd"), false);

        user.set_passwd("").unwrap();

        assert_eq!(user.is_passwd_blank(), true);
        assert_eq!(user.is_passwd_unset(), false);
        assert_eq!(user.verify_passwd(""), true);
        assert_eq!(user.verify_passwd("Something"), false);
    }

    // *** struct.AllUsers ***
    #[cfg(feature = "auth")]
    #[test]
    fn get_user() {
        let users = AllUsers::authenticator(test_cfg()).unwrap();

        let root = users.get_by_id(0).expect("'root' user missing");
        assert_eq!(root.user, "root".to_string());
        assert_eq!(root.auth.hash.as_str(),
            "$argon2i$m=4096,t=10,p=1$Tnc4UVV0N00$ML9LIOujd3nmAfkAwEcSTMPqakWUF0OUiLWrIy0nGLk");
        assert_eq!(root.uid, 0);
        assert_eq!(root.gid, 0);
        assert_eq!(root.name, "root".to_string());
        assert_eq!(root.home, "file:/root".to_string());
        assert_eq!(root.shell, "file:/bin/ion".to_string());

        let user = users.get_by_name("user").expect("'user' user missing");
        assert_eq!(user.user, "user".to_string());
        assert_eq!(user.auth.hash.as_str(), "");
        assert_eq!(user.uid, 1000);
        assert_eq!(user.gid, 1000);
        assert_eq!(user.name, "user".to_string());
        assert_eq!(user.home, "file:/home/user".to_string());
        assert_eq!(user.shell, "file:/bin/ion".to_string());
        println!("{:?}", users);

        let li = users.get_by_name("loip").expect("'loip' user missing");
        println!("got loip");
        assert_eq!(li.user, "loip");
        assert_eq!(li.auth.hash.as_str(), "!");
        assert_eq!(li.uid, 1007);
        assert_eq!(li.gid, 1007);
        assert_eq!(li.name, "Lorem".to_string());
        assert_eq!(li.home, "file:/home/lorem".to_string());
        assert_eq!(li.shell, "file:/bin/ion".to_string());
    }

    #[cfg(feature = "auth")]
    #[test]
    fn manip_user() {
        let mut users = AllUsers::authenticator(test_cfg()).unwrap();
        // NOT testing `get_unique_id`
        let id = 7099;

        let fb = UserBuilder::new("fbar")
            .uid(id)
            .gid(id)
            .name("Foo Bar")
            .home("/home/foob")
            .shell("/bin/zsh");

        users
            .add_user(fb)
            .expect("failed to add user 'fbar'");
        //                                            weirdo ^^^^^^^^ :P
        users.save().unwrap();
        let p_file_content = read_locked_file(test_prefix(PASSWD_FILE)).unwrap();
        assert_eq!(
            p_file_content,
            concat!(
                "root;0;0;root;file:/root;file:/bin/ion\n",
                "user;1000;1000;user;file:/home/user;file:/bin/ion\n",
                "loip;1007;1007;Lorem;file:/home/lorem;file:/bin/ion\n",
                "fbar;7099;7099;Foo Bar;/home/foob;/bin/zsh\n"
            )
        );
        let s_file_content = read_locked_file(test_prefix(SHADOW_FILE)).unwrap();
        assert_eq!(s_file_content, concat!(
            "root;$argon2i$m=4096,t=10,p=1$Tnc4UVV0N00$ML9LIOujd3nmAfkAwEcSTMPqakWUF0OUiLWrIy0nGLk\n",
            "user;\n",
            "loip;!\n",
            "fbar;!\n"
        ));

        {
            println!("{:?}", users);
            let fb = users.get_mut_by_name("fbar")
                .expect("'fbar' user missing");
            fb.shell = "/bin/fish".to_string(); // That's better
            fb.set_passwd("").unwrap();
        }
        users.save().unwrap();
        let p_file_content = read_locked_file(test_prefix(PASSWD_FILE)).unwrap();
        assert_eq!(
            p_file_content,
            concat!(
                "root;0;0;root;file:/root;file:/bin/ion\n",
                "user;1000;1000;user;file:/home/user;file:/bin/ion\n",
                "loip;1007;1007;Lorem;file:/home/lorem;file:/bin/ion\n",
                "fbar;7099;7099;Foo Bar;/home/foob;/bin/fish\n"
            )
        );
        let s_file_content = read_locked_file(test_prefix(SHADOW_FILE)).unwrap();
        assert_eq!(s_file_content, concat!(
            "root;$argon2i$m=4096,t=10,p=1$Tnc4UVV0N00$ML9LIOujd3nmAfkAwEcSTMPqakWUF0OUiLWrIy0nGLk\n",
            "user;\n",
            "loip;!\n",
            "fbar;\n"
        ));

        users.remove_by_id(id);
        users.save().unwrap();
        let file_content = read_locked_file(test_prefix(PASSWD_FILE)).unwrap();
        assert_eq!(
            file_content,
            concat!(
                "root;0;0;root;file:/root;file:/bin/ion\n",
                "user;1000;1000;user;file:/home/user;file:/bin/ion\n",
                "loip;1007;1007;Lorem;file:/home/lorem;file:/bin/ion\n"
            )
        );
    }

    /* struct.Group */
    #[test]
    fn empty_groups() {
        let group_trailing = Group::from_group_entry("nobody;x;2066; ", 0).unwrap();
        assert_eq!(group_trailing.users.len(), 0);

        let group_no_trailing = Group::from_group_entry("nobody;x;2066;", 0).unwrap();
        assert_eq!(group_no_trailing.users.len(), 0);

        assert_eq!(group_trailing.group, group_no_trailing.group);
        assert_eq!(group_trailing.gid, group_no_trailing.gid);
        assert_eq!(group_trailing.users, group_no_trailing.users);
    }

    /* struct.AllGroups */
    #[test]
    fn get_group() {
        let groups = AllGroups::new(test_cfg()).unwrap();
        let user = groups.get_by_name("user").unwrap();
        assert_eq!(user.group, "user");
        assert_eq!(user.gid, 1000);
        assert_eq!(user.users, vec!["user"]);

        let wheel = groups.get_by_id(1).unwrap();
        assert_eq!(wheel.group, "wheel");
        assert_eq!(wheel.gid, 1);
        assert_eq!(wheel.users, vec!["user", "root"]);
    }

    #[test]
    fn manip_group() {
        let id = 7099;
        let mut groups = AllGroups::new(test_cfg()).unwrap();

        let fb = GroupBuilder::new("fbar")
            // NOT testing `get_unique_id`
            .gid(id)
            .user("fbar");

        groups.add_group(fb).unwrap();
        groups.save().unwrap();
        let file_content = read_locked_file(test_prefix(GROUP_FILE)).unwrap();
        assert_eq!(
            file_content,
            concat!(
                "root;x;0;root\n",
                "user;x;1000;user\n",
                "wheel;x;1;user,root\n",
                "loip;x;1007;loip\n",
                "fbar;x;7099;fbar\n"
            )
        );

        {
            let fb = groups.get_mut_by_name("fbar").unwrap();
            fb.users.push("user".to_string());
        }
        groups.save().unwrap();
        let file_content = read_locked_file(test_prefix(GROUP_FILE)).unwrap();
        assert_eq!(
            file_content,
            concat!(
                "root;x;0;root\n",
                "user;x;1000;user\n",
                "wheel;x;1;user,root\n",
                "loip;x;1007;loip\n",
                "fbar;x;7099;fbar,user\n"
            )
        );

        groups.remove_by_id(id);
        groups.save().unwrap();
        let file_content = read_locked_file(test_prefix(GROUP_FILE)).unwrap();
        assert_eq!(
            file_content,
            concat!(
                "root;x;0;root\n",
                "user;x;1000;user\n",
                "wheel;x;1;user,root\n",
                "loip;x;1007;loip\n"
            )
        );
    }

    #[test]
    fn empty_group() {
        let mut groups = AllGroups::new(test_cfg()).unwrap();
        let nobody = GroupBuilder::new("nobody")
            .gid(2260);

        groups.add_group(nobody).unwrap();
        groups.save().unwrap();
        let file_content = read_locked_file(test_prefix(GROUP_FILE)).unwrap();
        assert_eq!(
            file_content,
            concat!(
                "root;x;0;root\n",
                "user;x;1000;user\n",
                "wheel;x;1;user,root\n",
                "loip;x;1007;loip\n",
                "nobody;x;2260;\n",
            )
        );

        drop(groups);
        let mut groups = AllGroups::new(test_cfg()).unwrap();

        groups.remove_by_name("nobody");
        groups.save().unwrap();

        let file_content = read_locked_file(test_prefix(GROUP_FILE)).unwrap();
        assert_eq!(
            file_content,
            concat!(
                "root;x;0;root\n",
                "user;x;1000;user\n",
                "wheel;x;1;user,root\n",
                "loip;x;1007;loip\n"
            )
        );
    }

    // *** Misc ***
    #[test]
    fn users_get_unused_ids() {
        let users = AllUsers::basic(test_cfg()).unwrap();
        let id = users.get_unique_id().unwrap();
        if id < users.config.min_id || id > users.config.max_id {
            panic!("User ID is not between allowed margins")
        } else if let Some(_) = users.get_by_id(id) {
            panic!("User ID is used!");
        }
    }

    #[test]
    fn groups_get_unused_ids() {
        let groups = AllGroups::new(test_cfg()).unwrap();
        let id = groups.get_unique_id().unwrap();
        if id < groups.config.min_id || id > groups.config.max_id {
            panic!("Group ID is not between allowed margins")
        } else if let Some(_) = groups.get_by_id(id) {
            panic!("Group ID is used!");
        }
    }
}
