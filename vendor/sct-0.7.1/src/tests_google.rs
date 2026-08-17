use super::{verify_sct, Log};

static GOOGLE_PILOT: Log = Log {
    description: "Google 'Pilot' log",
    url: "ct.googleapis.com/pilot/",
    operated_by: "Google",
    key: include_bytes!("testdata/google-pilot-pubkey.bin"),
    id: [
        164, 185, 9, 144, 180, 24, 88, 20, 135, 187, 19, 162, 204, 103, 112, 10, 60, 53, 152, 4,
        249, 27, 223, 184, 227, 119, 205, 14, 200, 13, 220, 16,
    ],
    max_merge_delay: 86400,
};

static SYMANTEC_LOG: Log = Log {
    description: "Symantec log",
    url: "ct.ws.symantec.com/",
    operated_by: "Symantec",
    key: include_bytes!("testdata/symantec-log-pubkey.bin"),
    id: [
        221, 235, 29, 43, 122, 13, 79, 166, 32, 139, 129, 173, 129, 104, 112, 126, 46, 142, 157, 1,
        213, 92, 136, 141, 61, 17, 196, 205, 182, 236, 190, 204,
    ],
    max_merge_delay: 86400,
};

#[test]
fn test_google_sct0() {
    let sct = include_bytes!("testdata/google-sct0.bin");
    let cert = include_bytes!("testdata/google-cert.bin");
    let logs = [&GOOGLE_PILOT, &SYMANTEC_LOG];
    let now = 1499619463644;

    assert_eq!(0, verify_sct(cert, sct, now, &logs).unwrap());
}

#[test]
fn test_google_sct1() {
    let sct = include_bytes!("testdata/google-sct1.bin");
    let cert = include_bytes!("testdata/google-cert.bin");
    let logs = [&GOOGLE_PILOT, &SYMANTEC_LOG];
    let now = 1499619463644;

    assert_eq!(1, verify_sct(cert, sct, now, &logs).unwrap());
}

#[test]
fn test_log_is_debug() {
    use alloc::format;
    format!("{:?}", GOOGLE_PILOT);
}
