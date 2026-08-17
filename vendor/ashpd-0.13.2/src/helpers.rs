use std::io::Read;

pub(crate) fn is_flatpak() -> bool {
    std::path::PathBuf::from("/.flatpak-info").exists()
}

pub(crate) fn is_snap() -> bool {
    let pid = std::process::id();
    let path = format!("/proc/{pid}/cgroup");
    let mut file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(_) => return false,
    };

    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Ok(_) => cgroup_v2_is_snap(&buffer),
        Err(_) => false,
    }
}

fn cgroup_v2_is_snap(cgroups: &str) -> bool {
    cgroups
        .lines()
        .map(|line| {
            let (n, rest) = line.split_once(':')?;
            // Check that n is a number.
            n.parse::<u32>().ok()?;
            let unit = match rest.split_once(':') {
                Some(("", unit)) => Some(unit),
                Some(("freezer", unit)) => Some(unit),
                Some(("name=systemd", unit)) => Some(unit),
                _ => None,
            }?;
            let scope = std::path::Path::new(unit).file_name()?.to_str()?;

            Some(scope.starts_with("snap."))
        })
        .any(|x| x.unwrap_or(false))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cgroup_v2_is_snap() {
        let data =
            "0::/user.slice/user-1000.slice/user@1000.service/apps.slice/snap.something.scope\n";
        assert!(cgroup_v2_is_snap(data));

        let data = "0::/user.slice/user-1000.slice/user@1000.service/apps.slice\n";
        assert!(!cgroup_v2_is_snap(data));

        let data = "12:pids:/user.slice/user-1000.slice/user@1000.service
11:perf_event:/
10:net_cls,net_prio:/
9:cpuset:/
8:memory:/user.slice/user-1000.slice/user@1000.service/apps.slice/apps-org.gnome.Terminal.slice/vte-spawn-228ae109-a869-4533-8988-65ea4c10b492.scope
7:rdma:/
6:devices:/user.slice
5:blkio:/user.slice
4:hugetlb:/
3:freezer:/snap.portal-test
2:cpu,cpuacct:/user.slice
1:name=systemd:/user.slice/user-1000.slice/user@1000.service/apps.slice/apps-org.gnome.Terminal.slice/vte-spawn-228ae109-a869-4533-8988-65ea4c10b492.scope
0::/user.slice/user-1000.slice/user@1000.service/apps.slice/apps-org.gnome.Terminal.slice/vte-spawn-228ae109-a869-4533-8988-65ea4c10b492.scope\n";
        assert!(cgroup_v2_is_snap(data));
    }
}
