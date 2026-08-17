#![feature(test)]

extern crate test;

#[cfg(feature = "system")]
#[bench]
fn bench_new(b: &mut test::Bencher) {
    b.iter(|| {
        sysinfo::System::new();
    });
}

#[cfg(feature = "system")]
#[bench]
fn bench_new_all(b: &mut test::Bencher) {
    b.iter(|| {
        sysinfo::System::new_all();
    });
}

#[cfg(feature = "system")]
#[bench]
fn bench_refresh_all(b: &mut test::Bencher) {
    let mut s = sysinfo::System::new_all();

    b.iter(move || {
        s.refresh_all();
    });
}

#[cfg(feature = "system")]
#[bench]
fn bench_refresh_processes(b: &mut test::Bencher) {
    let mut s = sysinfo::System::new();

    s.refresh_processes(sysinfo::ProcessesToUpdate::All, true); // to load the whole processes list a first time.
    b.iter(move || {
        s.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
    });
}

#[cfg(feature = "system")]
#[bench]
fn bench_first_refresh_processes(b: &mut test::Bencher) {
    b.iter(move || {
        let mut s = sysinfo::System::new();
        s.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
    });
}

#[cfg(feature = "system")]
#[bench]
fn bench_refresh_process(b: &mut test::Bencher) {
    let mut s = sysinfo::System::new();

    s.refresh_all();
    // to be sure it'll exist for at least as long as we run
    let pid = sysinfo::get_current_pid().expect("failed to get current pid");
    b.iter(move || {
        s.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[pid]), true);
    });
}

#[bench]
fn bench_refresh_disk(b: &mut test::Bencher) {
    let mut disks = sysinfo::Disks::new_with_refreshed_list();

    let disks = disks.list_mut();
    let disk = &mut disks[0];
    b.iter(move || {
        disk.refresh();
    });
}

#[bench]
fn bench_refresh_disks(b: &mut test::Bencher) {
    let mut disks = sysinfo::Disks::new_with_refreshed_list();

    b.iter(move || {
        disks.refresh(true);
    });
}

#[cfg(feature = "network")]
#[bench]
fn bench_refresh_networks(b: &mut test::Bencher) {
    let mut n = sysinfo::Networks::new_with_refreshed_list();

    b.iter(move || {
        n.refresh(true);
    });
}

#[cfg(feature = "system")]
#[bench]
fn bench_refresh_memory(b: &mut test::Bencher) {
    let mut s = sysinfo::System::new();

    b.iter(move || {
        s.refresh_memory();
    });
}

#[cfg(feature = "system")]
#[bench]
fn bench_refresh_cpu_usage(b: &mut test::Bencher) {
    let mut s = sysinfo::System::new();

    s.refresh_cpu_usage();
    b.iter(move || {
        s.refresh_cpu_usage();
    });
}

#[cfg(feature = "component")]
#[bench]
fn bench_refresh_components(b: &mut test::Bencher) {
    let mut c = sysinfo::Components::new_with_refreshed_list();

    b.iter(move || {
        c.refresh(false);
    });
}

#[bench]
fn bench_refresh_users_list(b: &mut test::Bencher) {
    let mut users = sysinfo::Users::new_with_refreshed_list();

    b.iter(move || {
        users.refresh();
    });
}
