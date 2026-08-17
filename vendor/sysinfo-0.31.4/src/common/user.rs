// Take a look at the license at the top of the repository in the LICENSE file.

use std::cmp::Ordering;

use crate::{Gid, Uid, UserInner};

/// Type containing user information.
///
/// It is returned by [`Users`][crate::Users].
///
/// ```no_run
/// use sysinfo::Users;
///
/// let users = Users::new_with_refreshed_list();
/// for user in users.list() {
///     println!("{:?}", user);
/// }
/// ```
pub struct User {
    pub(crate) inner: UserInner,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
            && self.group_id() == other.group_id()
            && self.name() == other.name()
    }
}

impl Eq for User {}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for User {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name().cmp(other.name())
    }
}

impl User {
    /// Returns the ID of the user.
    ///
    /// ```no_run
    /// use sysinfo::Users;
    ///
    /// let users = Users::new_with_refreshed_list();
    /// for user in users.list() {
    ///     println!("{:?}", *user.id());
    /// }
    /// ```
    pub fn id(&self) -> &Uid {
        self.inner.id()
    }

    /// Returns the group ID of the user.
    ///
    /// ⚠️ This information is not set on Windows.  Windows doesn't have a `username` specific
    /// group assigned to the user. They do however have unique
    /// [Security Identifiers](https://docs.microsoft.com/en-us/windows/win32/secauthz/security-identifiers)
    /// made up of various [Components](https://docs.microsoft.com/en-us/windows/win32/secauthz/sid-components).
    /// Pieces of the SID may be a candidate for this field, but it doesn't map well to a single
    /// group ID.
    ///
    /// ```no_run
    /// use sysinfo::Users;
    ///
    /// let users = Users::new_with_refreshed_list();
    /// for user in users.list() {
    ///     println!("{}", *user.group_id());
    /// }
    /// ```
    pub fn group_id(&self) -> Gid {
        self.inner.group_id()
    }

    /// Returns the name of the user.
    ///
    /// ```no_run
    /// use sysinfo::Users;
    ///
    /// let users = Users::new_with_refreshed_list();
    /// for user in users.list() {
    ///     println!("{}", user.name());
    /// }
    /// ```
    pub fn name(&self) -> &str {
        self.inner.name()
    }

    /// Returns the groups of the user.
    ///
    /// ⚠️ This is computed every time this method is called.
    ///
    /// ```no_run
    /// use sysinfo::Users;
    ///
    /// let users = Users::new_with_refreshed_list();
    /// for user in users.list() {
    ///     println!("{} is in {:?}", user.name(), user.groups());
    /// }
    /// ```
    pub fn groups(&self) -> Vec<Group> {
        self.inner.groups()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub(crate) struct GroupInner {
    pub(crate) id: Gid,
    pub(crate) name: String,
}

/// Type containing group information.
///
/// It is returned by [`User::groups`] or [`Groups::list`].
///
/// ```no_run
/// use sysinfo::Users;
///
/// let mut users = Users::new_with_refreshed_list();
///
/// for user in users.list() {
///     println!(
///         "user: (ID: {:?}, group ID: {:?}, name: {:?})",
///         user.id(),
///         user.group_id(),
///         user.name(),
///     );
///     for group in user.groups() {
///         println!("group: (ID: {:?}, name: {:?})", group.id(), group.name());
///     }
/// }
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Group {
    pub(crate) inner: GroupInner,
}

impl Group {
    /// Returns the ID of the group.
    ///
    /// ⚠️ This information is not set on Windows.
    ///
    /// ```no_run
    /// use sysinfo::Users;
    ///
    /// let mut users = Users::new_with_refreshed_list();
    ///
    /// for user in users.list() {
    ///     for group in user.groups() {
    ///         println!("{:?}", group.id());
    ///     }
    /// }
    /// ```
    pub fn id(&self) -> &Gid {
        self.inner.id()
    }

    /// Returns the name of the group.
    ///
    /// ```no_run
    /// use sysinfo::Users;
    ///
    /// let mut users = Users::new_with_refreshed_list();
    ///
    /// for user in users.list() {
    ///     for group in user.groups() {
    ///         println!("{}", group.name());
    ///     }
    /// }
    /// ```
    pub fn name(&self) -> &str {
        self.inner.name()
    }
}

/// Interacting with users.
///
/// ```no_run
/// use sysinfo::Users;
///
/// let mut users = Users::new();
/// for user in users.list() {
///     println!("{} is in {} groups", user.name(), user.groups().len());
/// }
/// ```
pub struct Users {
    users: Vec<User>,
}

impl Default for Users {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Users> for Vec<User> {
    fn from(users: Users) -> Self {
        users.users
    }
}

impl From<Vec<User>> for Users {
    fn from(users: Vec<User>) -> Self {
        Self { users }
    }
}

impl std::ops::Deref for Users {
    type Target = [User];

    fn deref(&self) -> &Self::Target {
        self.list()
    }
}

impl std::ops::DerefMut for Users {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.list_mut()
    }
}

impl<'a> IntoIterator for &'a Users {
    type Item = &'a User;
    type IntoIter = std::slice::Iter<'a, User>;

    fn into_iter(self) -> Self::IntoIter {
        self.list().iter()
    }
}

impl<'a> IntoIterator for &'a mut Users {
    type Item = &'a mut User;
    type IntoIter = std::slice::IterMut<'a, User>;

    fn into_iter(self) -> Self::IntoIter {
        self.list_mut().iter_mut()
    }
}

impl Users {
    /// Creates a new empty [`Users`][crate::Users] type.
    ///
    /// If you want it to be filled directly, take a look at [`Users::new_with_refreshed_list`].
    ///
    /// ```no_run
    /// use sysinfo::Users;
    ///
    /// let mut users = Users::new();
    /// users.refresh_list();
    /// for user in users.list() {
    ///     println!("{user:?}");
    /// }
    /// ```
    pub fn new() -> Self {
        Self { users: Vec::new() }
    }

    /// Creates a new [`Users`][crate::Users] type with the user list loaded.
    /// It is a combination of [`Users::new`] and [`Users::refresh_list`].
    ///
    /// ```no_run
    /// use sysinfo::Users;
    ///
    /// let mut users = Users::new_with_refreshed_list();
    /// for user in users.list() {
    ///     println!("{user:?}");
    /// }
    /// ```
    pub fn new_with_refreshed_list() -> Self {
        let mut users = Self::new();
        users.refresh_list();
        users
    }

    /// Returns the users list.
    ///
    /// ```no_run
    /// use sysinfo::Users;
    ///
    /// let users = Users::new_with_refreshed_list();
    /// for user in users.list() {
    ///     println!("{user:?}");
    /// }
    /// ```
    pub fn list(&self) -> &[User] {
        &self.users
    }

    /// Returns the users list.
    ///
    /// ```no_run
    /// use sysinfo::Users;
    ///
    /// let mut users = Users::new_with_refreshed_list();
    /// users.list_mut().sort_by(|user1, user2| {
    ///     user1.name().partial_cmp(user2.name()).unwrap()
    /// });
    /// ```
    pub fn list_mut(&mut self) -> &mut [User] {
        &mut self.users
    }

    /// The user list will be emptied then completely recomputed.
    ///
    /// ```no_run
    /// use sysinfo::Users;
    ///
    /// let mut users = Users::new();
    /// users.refresh_list();
    /// ```
    pub fn refresh_list(&mut self) {
        crate::sys::get_users(&mut self.users);
    }

    /// Returns the [`User`] matching the given `user_id`.
    ///
    /// **Important**: The user list must be filled before using this method, otherwise it will
    /// always return `None` (through the `refresh_*` methods).
    ///
    /// It is a shorthand for:
    ///
    /// ```ignore
    /// # use sysinfo::Users;
    /// let users = Users::new_with_refreshed_list();
    /// users.list().find(|user| user.id() == user_id);
    /// ```
    ///
    /// Full example:
    ///
    #[cfg_attr(feature = "system", doc = "```no_run")]
    #[cfg_attr(not(feature = "system"), doc = "```ignore")]
    /// use sysinfo::{Pid, System, Users};
    ///
    /// let mut s = System::new_all();
    /// let users = Users::new_with_refreshed_list();
    ///
    /// if let Some(process) = s.process(Pid::from(1337)) {
    ///     if let Some(user_id) = process.user_id() {
    ///         println!("User for process 1337: {:?}", users.get_user_by_id(user_id));
    ///     }
    /// }
    /// ```
    pub fn get_user_by_id(&self, user_id: &Uid) -> Option<&User> {
        self.users.iter().find(|user| user.id() == user_id)
    }
}

/// Interacting with groups.
///
/// ```no_run
/// use sysinfo::Groups;
///
/// let mut groups = Groups::new();
/// for group in groups.list() {
///     println!("{}", group.name());
/// }
/// ```
pub struct Groups {
    groups: Vec<Group>,
}

impl Default for Groups {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Groups> for Vec<Group> {
    fn from(groups: Groups) -> Self {
        groups.groups
    }
}

impl From<Vec<Group>> for Groups {
    fn from(groups: Vec<Group>) -> Self {
        Self { groups }
    }
}

impl std::ops::Deref for Groups {
    type Target = [Group];

    fn deref(&self) -> &Self::Target {
        self.list()
    }
}

impl std::ops::DerefMut for Groups {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.list_mut()
    }
}

impl<'a> IntoIterator for &'a Groups {
    type Item = &'a Group;
    type IntoIter = std::slice::Iter<'a, Group>;

    fn into_iter(self) -> Self::IntoIter {
        self.list().iter()
    }
}

impl<'a> IntoIterator for &'a mut Groups {
    type Item = &'a mut Group;
    type IntoIter = std::slice::IterMut<'a, Group>;

    fn into_iter(self) -> Self::IntoIter {
        self.list_mut().iter_mut()
    }
}

impl Groups {
    /// Creates a new empty [`Groups`][crate::Groups] type.
    ///
    /// If you want it to be filled directly, take a look at [`Groups::new_with_refreshed_list`].
    ///
    /// ```no_run
    /// use sysinfo::Groups;
    ///
    /// let mut groups = Groups::new();
    /// groups.refresh_list();
    /// for group in groups.list() {
    ///     println!("{group:?}");
    /// }
    /// ```
    pub fn new() -> Self {
        Self { groups: Vec::new() }
    }

    /// Creates a new [`Groups`][crate::Groups] type with the user list loaded.
    /// It is a combination of [`Groups::new`] and [`Groups::refresh_list`].
    ///
    /// ```no_run
    /// use sysinfo::Groups;
    ///
    /// let mut groups = Groups::new_with_refreshed_list();
    /// for group in groups.list() {
    ///     println!("{group:?}");
    /// }
    /// ```
    pub fn new_with_refreshed_list() -> Self {
        let mut groups = Self::new();
        groups.refresh_list();
        groups
    }

    /// Returns the users list.
    ///
    /// ```no_run
    /// use sysinfo::Groups;
    ///
    /// let groups = Groups::new_with_refreshed_list();
    /// for group in groups.list() {
    ///     println!("{group:?}");
    /// }
    /// ```
    pub fn list(&self) -> &[Group] {
        &self.groups
    }

    /// Returns the groups list.
    ///
    /// ```no_run
    /// use sysinfo::Groups;
    ///
    /// let mut groups = Groups::new_with_refreshed_list();
    /// groups.list_mut().sort_by(|user1, user2| {
    ///     user1.name().partial_cmp(user2.name()).unwrap()
    /// });
    /// ```
    pub fn list_mut(&mut self) -> &mut [Group] {
        &mut self.groups
    }

    /// The group list will be emptied then completely recomputed.
    ///
    /// ```no_run
    /// use sysinfo::Users;
    ///
    /// let mut users = Users::new();
    /// users.refresh_list();
    /// ```
    pub fn refresh_list(&mut self) {
        crate::sys::get_groups(&mut self.groups);
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn check_list() {
        let mut users = Users::new();
        assert!(users.list().is_empty());
        users.refresh_list();
        assert!(users.list().len() >= MIN_USERS);
    }

    // This test exists to ensure that the `TryFrom<usize>` and `FromStr` traits are implemented
    // on `Uid`, `Gid` and `Pid`.
    #[allow(clippy::unnecessary_fallible_conversions)]
    #[test]
    fn check_uid_gid_from_impls() {
        use std::convert::TryFrom;
        use std::str::FromStr;

        #[cfg(not(windows))]
        {
            assert!(crate::Uid::try_from(0usize).is_ok());
            assert!(crate::Uid::from_str("0").is_ok());
        }
        #[cfg(windows)]
        {
            assert!(crate::Uid::from_str("S-1-5-18").is_ok()); // SECURITY_LOCAL_SYSTEM_RID
            assert!(crate::Uid::from_str("0").is_err());
        }

        assert!(crate::Gid::try_from(0usize).is_ok());
        assert!(crate::Gid::from_str("0").is_ok());
    }

    #[test]
    fn check_groups() {
        if !crate::IS_SUPPORTED_SYSTEM {
            return;
        }
        assert!(!Groups::new_with_refreshed_list().is_empty());
    }
}
