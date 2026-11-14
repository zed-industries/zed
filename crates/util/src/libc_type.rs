use crate::command::new_command;
use futures::StreamExt as _;
use smol::lock::OnceCell;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LibcType {
    Gnu,
    Musl,
}

/// Detects the libc of the host system, which is not necessarily the libc this
/// binary was built against: the remote server is statically linked with musl
/// and runs on glibc hosts, so `cfg!(target_env = "musl")` can't be used to pick
/// binaries to download.
pub async fn host_libc_type() -> LibcType {
    static HOST_LIBC_TYPE: OnceCell<LibcType> = OnceCell::new();
    *HOST_LIBC_TYPE.get_or_init(detect_host_libc_type).await
}

async fn detect_host_libc_type() -> LibcType {
    if let Some(libc_type) = libc_type_from_ldd().await {
        return libc_type;
    }

    let Ok(mut dir_entries) = smol::fs::read_dir("/lib").await else {
        // defaulting to gnu because nix doesn't have /lib files due to not following FHS
        return LibcType::Gnu;
    };

    while let Some(entry) = dir_entries.next().await {
        let Ok(entry) = entry else {
            continue;
        };
        if entry.file_name().to_string_lossy().starts_with("ld-musl-") {
            return LibcType::Musl;
        }
    }

    LibcType::Gnu
}

async fn libc_type_from_ldd() -> Option<LibcType> {
    let output = new_command("ldd").arg("--version").output().await.ok()?;
    // musl's ldd prints its version to stderr and exits with a failure status.
    parse_ldd_version(&String::from_utf8_lossy(&output.stdout))
        .or_else(|| parse_ldd_version(&String::from_utf8_lossy(&output.stderr)))
}

fn parse_ldd_version(ldd_version: &str) -> Option<LibcType> {
    if ldd_version.contains("GNU libc") || ldd_version.contains("GLIBC") {
        Some(LibcType::Gnu)
    } else if ldd_version.contains("musl") {
        Some(LibcType::Musl)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ldd_version() {
        assert_eq!(
            parse_ldd_version("ldd (Ubuntu GLIBC 2.39-0ubuntu8.4) 2.39"),
            Some(LibcType::Gnu)
        );
        assert_eq!(
            parse_ldd_version("ldd (GNU libc) 2.40"),
            Some(LibcType::Gnu)
        );
        assert_eq!(
            parse_ldd_version("musl libc (x86_64)\nVersion 1.2.5\n"),
            Some(LibcType::Musl)
        );
        assert_eq!(parse_ldd_version(""), None);
    }
}
