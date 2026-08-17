// Take a look at the license at the top of the repository in the LICENSE file.

// This test is used to ensure that the users are not loaded by default.
//
// users.groupps()  multiple times doesn't return the same output https://github.com/GuillaumeGomez/sysinfo/issues/1233
//
// Example:
// ---- test_users 1 stdout ----
// user: Administrator group:[Group { inner: GroupInner { id: Gid(0), name: "Administrators" } }]
// ---- test_users 2 stdout ----
// user: Administrator group:[]
#[cfg(feature = "user")]
#[test]
fn test_users() {
    use sysinfo::Users;

    if !sysinfo::IS_SUPPORTED_SYSTEM {
        return;
    }
    let mut users = Users::new();
    assert_eq!(users.iter().count(), 0);
    users.refresh();
    assert!(users.iter().count() > 0);
    let count = users.first().unwrap().groups().iter().len();
    for _ in 1..10 {
        assert!(users.first().unwrap().groups().iter().len() == count)
    }
}

// This test ensures that there are actually groups listed, in particular for Windows.
#[cfg(feature = "user")]
#[test]
fn test_groups() {
    use sysinfo::Groups;

    if !sysinfo::IS_SUPPORTED_SYSTEM {
        return;
    }
    let mut groups = Groups::new();
    groups.refresh();
    assert!(groups.list().len() > 1);
}
