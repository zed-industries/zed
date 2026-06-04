//! Windows AppContainer sandbox integration.
//!
//! This module sandboxes agent-run terminal commands on Windows using
//! AppContainers: a process launched with an AppContainer package SID can
//! only access securable objects whose DACL grants access to that SID, to
//! one of its capability SIDs, or to the well-known `ALL APPLICATION
//! PACKAGES` SID. Windows pre-ACLs the OS itself (`C:\Windows`,
//! `C:\Program Files*`, ...) for `ALL APPLICATION PACKAGES`, so system
//! binaries and DLLs work out of the box, while everything else — including
//! the user profile — is denied by default.
//!
//! Unlike macOS Seatbelt there is no launcher prefix command: the sandbox
//! is applied at process-creation time. The integration therefore has two
//! halves:
//!
//! 1. **Parent side** ([`wrap_invocation`], called from Zed proper): create
//!    (or reopen) the per-thread AppContainer profile, grant its package SID
//!    inheritable ACEs on the writable and readable roots, write a small
//!    policy file, and rewrite the invocation to
//!    `zed.exe --sandbox-helper <policy> -- <program> <args...>`.
//! 2. **Helper side** ([`run_sandbox_helper`], invoked via the hidden
//!    `--sandbox-helper` flag on the main Zed binary): read the policy,
//!    derive the package SID, and spawn the real command via
//!    `CreateProcessW` with `PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES`,
//!    inheriting the helper's console, then relay the exit code.
//!
//! # Filesystem posture: explicit allowlist, no "read everywhere"
//!
//! Like Microsoft's MXC sandbox, access is granted **only to explicit
//! roots**: worktrees and the per-thread temp dir get read+write grants;
//! user-approved `fs_read_paths` get read-only grants. Reads anywhere else
//! (outside the `ALL APPLICATION PACKAGES`-readable OS directories) fail
//! with access-denied, and the model escalates via `fs_read_paths`,
//! `fs_write_paths`, or `unsandboxed`. There is deliberately no
//! profile-wide "read everywhere" grant (macOS-Seatbelt parity): in the
//! DACL tier that would walk every file the user owns.
//!
//! # Filesystem tiers: BFS when available, DACLs as fallback
//!
//! How the grants are enforced depends on the host (probed once per
//! process, see [`filesystem_tier`]), mirroring MXC's tiering:
//!
//! - **Brokered File System (Windows 25H2+).** Per-AppContainer path rules
//!   registered via `bfscfg.exe` (`--addpolicy
//!   --policybroker[readonly] --filename <path> --appid <container>
//!   --containerinherit`), which the OS evaluates **at access time**:
//!   O(1) per rule, no DACL mutation, no subtree walks, no admin. Rules
//!   are keyed by the per-thread AppContainer name and cleared wholesale
//!   (`--clearpolicy --appid`) when the thread is deleted. In this tier
//!   `allow_fs_write_all` is a single read-write rule on `%USERPROFILE%`.
//! - **DACL fallback (older Windows).** Inheritable allow-ACEs for the
//!   package SID on each granted root. Granting is expensive here:
//!   `SetNamedSecurityInfoW` eagerly rewrites the DACL of every existing
//!   descendant, so the walk cost is bounded by the size of the granted
//!   subtrees. `allow_fs_write_all` maps to a read+write grant over
//!   `%USERPROFILE%` for the shared [`PROFILE_WRITE_CAPABILITY`] SID —
//!   one whole-profile walk, at most once per user ever, and only if that
//!   escalation is ever approved. Granting metadata access to drive roots
//!   (next paragraph) needs `WRITE_DAC`, i.e. admin, in this tier; without
//!   it shells can't start, which is why the BFS tier is preferred.
//!
//! # Shell startup: drive-root metadata access
//!
//! Inside an AppContainer the well-known package SIDs have no rights on
//! the system-drive root, but shells (`cmd.exe`, `powershell.exe`,
//! `pwsh.exe`) stat `C:\` and enumerate drive roots during startup and
//! fail with access-denied — collapsing to an unusable `C:\` working
//! location. So every sandboxed run additionally grants **non-recursive,
//! metadata-only** read on the system drive root and the drive roots of
//! the granted paths (a BFS rule without `--containerinherit`, or a
//! non-inheriting [`ROOT_METADATA_MASK`] ACE). This grants no ability to
//! enumerate or read the root's contents — it just unblocks the startup
//! stat — and mirrors MXC's `prepare-system-drive`.
//!
//! All policy mutation happens in the parent so the cleanup story
//! (clearing BFS rules / revoking ACEs and deleting the profile when the
//! agent thread is deleted, see [`cleanup_profile`]) lives next to the
//! thread-lifecycle code. Network access is denied by default for
//! AppContainer processes; this milestone deliberately never grants the
//! network capabilities, so sandboxed commands have no outbound network
//! access regardless of [`SandboxPermissions::allow_network`].
//!
//! Set `ZED_SANDBOX_FORCE_DACL=1` to skip the BFS probe and exercise the
//! DACL tier on hosts that have the broker.

use std::collections::HashSet;
use std::ffi::{OsStr, OsString, c_void};
use std::io::Write as _;
use std::os::windows::ffi::OsStringExt as _;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use anyhow::{Context as _, Result, bail, ensure};
use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;
use windows::Win32::Foundation::{
    CloseHandle, ERROR_ALREADY_EXISTS, ERROR_SUCCESS, HANDLE, HLOCAL, LocalFree, WAIT_OBJECT_0,
};
use windows::Win32::Security::Authorization::{
    EXPLICIT_ACCESS_W, GRANT_ACCESS, GetNamedSecurityInfoW, SE_FILE_OBJECT, SetEntriesInAclW,
    SetNamedSecurityInfoW, TRUSTEE_IS_SID, TRUSTEE_IS_WELL_KNOWN_GROUP, TRUSTEE_W,
};
use windows::Win32::Security::Isolation::{
    CreateAppContainerProfile, DeleteAppContainerProfile, DeriveAppContainerSidFromAppContainerName,
};
use windows::Win32::Security::{
    ACCESS_ALLOWED_ACE, ACE_HEADER, ACL, CONTAINER_INHERIT_ACE, CopySid, DACL_SECURITY_INFORMATION,
    DeleteAce, DeriveCapabilitySidsFromName, EqualSid, FreeSid, GetAce, GetLengthSid,
    INHERITED_ACE, OBJECT_INHERIT_ACE, PSECURITY_DESCRIPTOR, PSID, SECURITY_CAPABILITIES,
    SID_AND_ATTRIBUTES, SUB_CONTAINERS_AND_OBJECTS_INHERIT,
};
use windows::Win32::Storage::FileSystem::{
    DELETE, FILE_GENERIC_EXECUTE, FILE_GENERIC_READ, FILE_GENERIC_WRITE,
};
use windows::Win32::System::Console::{
    GetStdHandle, STD_ERROR_HANDLE, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE,
};
use windows::Win32::System::SystemInformation::GetSystemDirectoryW;
use windows::Win32::System::SystemServices::{ACCESS_ALLOWED_ACE_TYPE, SE_GROUP_ENABLED};
use windows::Win32::System::Threading::{
    CreateProcessW, DeleteProcThreadAttributeList, EXTENDED_STARTUPINFO_PRESENT,
    GetExitCodeProcess, INFINITE, InitializeProcThreadAttributeList, LPPROC_THREAD_ATTRIBUTE_LIST,
    PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES, PROCESS_INFORMATION, STARTF_USESTDHANDLES,
    STARTUPINFOEXW, STARTUPINFOW, UpdateProcThreadAttribute, WaitForSingleObject,
};
use windows::core::{HRESULT, HSTRING, PCWSTR, PWSTR};

/// Per-command relaxations of the default AppContainer sandbox.
///
/// All-false is the default, fully-sandboxed run. Setting any field
/// requires user approval before the command is launched.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SandboxPermissions {
    /// Allow network access for the command.
    ///
    /// Currently recorded in the policy file but **not enforced as a
    /// grant**: the network capabilities are never added, so sandboxed
    /// commands run with the AppContainer's default network deny. Mapping
    /// this to the `internetClient` / `privateNetworkClientServer`
    /// capability SIDs is a follow-up milestone.
    pub allow_network: bool,
    /// Allow broad filesystem writes. There is no "write anywhere" grant in
    /// the AppContainer model, so this adds the [`PROFILE_WRITE_CAPABILITY`]
    /// to the process, making the user profile readable and writable;
    /// writes outside the user profile and the granted roots still fail
    /// (the escape hatch for those is running unsandboxed). The first time
    /// this is ever used on a machine, ensuring the capability ACE walks
    /// the whole profile subtree — see the module docs.
    pub allow_fs_write: bool,
}

/// Custom capability granting read+write over `%USERPROFILE%`. Backs
/// `allow_fs_write_all` (see the module docs): the ACE is ensured at most
/// once per user ever, is inert for any process that doesn't explicitly
/// list this capability at launch, and is only listed for commands the
/// user approved with `allow_fs_write_all`.
const PROFILE_WRITE_CAPABILITY: &str = "zedAgentSandboxProfileWrite";

/// Rights granted on writable roots: full read/write/execute plus delete,
/// inherited by the whole subtree.
const WRITE_ACCESS_MASK: u32 =
    FILE_GENERIC_READ.0 | FILE_GENERIC_WRITE.0 | FILE_GENERIC_EXECUTE.0 | DELETE.0;

/// Rights granted on read-only roots: read+execute so commands can load
/// user-level toolchains and config.
const READ_ACCESS_MASK: u32 = FILE_GENERIC_READ.0 | FILE_GENERIC_EXECUTE.0;

/// How filesystem grants are enforced on this host. See the module docs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FilesystemTier {
    /// Brokered File System: path rules registered with `bfscfg.exe` (at
    /// the contained absolute path), evaluated by the OS at access time.
    Bfs { bfscfg: PathBuf },
    /// Downlevel fallback: inheritable package-SID ACEs on each granted
    /// root.
    Dacl,
}

/// Probe (once per process) for the Brokered File System.
///
/// The broker's configuration tool ships as `System32\bfscfg.exe` on
/// Windows 25H2+; its presence is the availability signal (matching MXC's
/// tier detector). The System32 path comes from `GetSystemDirectoryW` —
/// boot configuration, not the scrubbable process environment — and the
/// resolved absolute path is the one we later execute, so a rogue
/// `bfscfg.exe` on `PATH` or next to zed.exe is never picked up.
pub fn filesystem_tier() -> &'static FilesystemTier {
    static TIER: OnceLock<FilesystemTier> = OnceLock::new();
    TIER.get_or_init(|| {
        if std::env::var_os("ZED_SANDBOX_FORCE_DACL").is_some_and(|value| value == "1") {
            return FilesystemTier::Dacl;
        }
        match system_directory() {
            Ok(system32) => {
                let bfscfg = system32.join("bfscfg.exe");
                if bfscfg.exists() {
                    FilesystemTier::Bfs { bfscfg }
                } else {
                    FilesystemTier::Dacl
                }
            }
            Err(_) => FilesystemTier::Dacl,
        }
    })
}

/// Resolve `System32` via `GetSystemDirectoryW`.
fn system_directory() -> Result<PathBuf> {
    let mut buffer = vec![0u16; 260];
    let length = unsafe { GetSystemDirectoryW(Some(&mut buffer)) } as usize;
    ensure!(length > 0, "GetSystemDirectoryW failed");
    if length > buffer.len() {
        buffer.resize(length, 0);
        let length = unsafe { GetSystemDirectoryW(Some(&mut buffer)) } as usize;
        ensure!(
            length > 0 && length <= buffer.len(),
            "GetSystemDirectoryW retry failed"
        );
        buffer.truncate(length);
    } else {
        buffer.truncate(length);
    }
    Ok(PathBuf::from(OsString::from_wide(&buffer)))
}

/// The on-disk policy the parent writes for the helper. Only contains what
/// the helper needs to *launch*: the ACL work has already happened in the
/// parent by the time the helper runs.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct SandboxPolicy {
    profile_name: String,
    allow_network: bool,
    /// Include [`PROFILE_WRITE_CAPABILITY`] in the process's capability
    /// list, making the user profile writable for this command.
    #[serde(default)]
    allow_fs_write: bool,
}

/// RAII handle returned by [`wrap_invocation`]; keeps the on-disk policy
/// file alive for the duration of the spawned command (the helper reads it
/// lazily at startup). The Windows analog of the macOS `SeatbeltConfigFile`.
pub struct AppContainerLaunchConfig {
    /// The temporary file containing the serialized [`SandboxPolicy`].
    _policy_file: NamedTempFile,
}

/// Compute the AppContainer profile name for an agent thread.
///
/// Profile names have length (64 chars) and charset restrictions, so the
/// thread id is hashed rather than embedded directly. The result is stable
/// for a given thread id, making profile creation idempotent across
/// commands and Zed restarts.
pub fn profile_name_for_thread(thread_id: &str) -> String {
    use sha2::{Digest as _, Sha256};
    let digest = Sha256::digest(thread_id.as_bytes());
    let mut name = String::from("Zed.AgentSandbox.");
    for byte in &digest[..8] {
        name.push_str(&format!("{byte:02x}"));
    }
    name
}

/// An AppContainer profile and its package SID.
pub struct AppContainerProfile {
    name: String,
    sid: OwnedSid,
}

impl AppContainerProfile {
    /// Create the profile if it doesn't exist yet, or derive the SID of the
    /// existing profile. Idempotent; no admin rights required.
    pub fn create_or_open(name: &str) -> Result<Self> {
        let name_wide = HSTRING::from(name);
        let description = HSTRING::from("Zed agent terminal sandbox");
        let sid = unsafe {
            match CreateAppContainerProfile(&name_wide, &name_wide, &description, None) {
                Ok(sid) => sid,
                Err(error) if error.code() == HRESULT::from_win32(ERROR_ALREADY_EXISTS.0) => {
                    DeriveAppContainerSidFromAppContainerName(&name_wide).with_context(|| {
                        format!("failed to derive AppContainer SID for existing profile {name}")
                    })?
                }
                Err(error) => {
                    return Err(error)
                        .with_context(|| format!("failed to create AppContainer profile {name}"));
                }
            }
        };
        Ok(Self {
            name: name.to_string(),
            sid: OwnedSid(sid),
        })
    }

    /// Delete the profile (its registry entry and package directories).
    pub fn delete(name: &str) -> Result<()> {
        unsafe { DeleteAppContainerProfile(&HSTRING::from(name)) }
            .with_context(|| format!("failed to delete AppContainer profile {name}"))
    }

    fn sid(&self) -> PSID {
        self.sid.0
    }
}

/// Wrap a process invocation so it runs inside the given per-thread
/// AppContainer profile.
///
/// Grants the profile's package SID inheritable read+write ACEs on each of
/// `writable_directories` and read-only ACEs on each of
/// `readonly_directories`, then returns the helper invocation `zed.exe
/// --sandbox-helper <policy> -- <program> <args...>` along with an
/// [`AppContainerLaunchConfig`] that **must** be kept alive for the
/// duration of the command.
///
/// This function does blocking work (process spawns in the BFS tier;
/// potentially long DACL-propagation walks in the fallback tier), so call
/// it from a background thread, never the UI thread.
///
/// # Arguments
/// * `program` - The program to invoke (typically a shell).
/// * `args` - The full argument list that would have been passed to
///   `program`.
/// * `writable_directories` - Directory subtrees where the command is
///   allowed to write. Pass the project's worktree paths here, not the
///   command's working directory (which is model-controlled).
/// * `readonly_directories` - Directory subtrees the user approved for
///   read-only access (e.g. `~\.cargo`), beyond the
///   `ALL APPLICATION PACKAGES`-readable OS directories.
/// * `permissions` - Sandbox relaxations requested for this command.
/// * `profile_name` - The per-thread AppContainer profile name (see
///   [`profile_name_for_thread`]).
pub fn wrap_invocation(
    program: &str,
    args: &[String],
    writable_directories: &[&Path],
    readonly_directories: &[&Path],
    permissions: SandboxPermissions,
    profile_name: &str,
) -> Result<(String, Vec<String>, AppContainerLaunchConfig)> {
    let profile = AppContainerProfile::create_or_open(profile_name)?;

    let tier = filesystem_tier();
    log::info!(
        "sandbox: configuring {} grants for {} writable + {} readable roots (profile {profile_name})",
        match tier {
            FilesystemTier::Bfs { .. } => "BFS",
            FilesystemTier::Dacl => "DACL",
        },
        writable_directories.len(),
        readonly_directories.len(),
    );

    // Shells stat `C:\` and enumerate drive roots during startup; without
    // metadata access to those roots they fail with access-denied and
    // collapse to an unusable `C:\` location. Grant minimal,
    // non-recursive metadata access to the system drive and the drive
    // roots of every granted path. See the module docs.
    let drive_roots = distinct_drive_roots(writable_directories, readonly_directories);

    match tier {
        FilesystemTier::Bfs { bfscfg } => {
            for root in &drive_roots {
                ensure_bfs_rule(bfscfg, profile_name, root, BfsAccess::ReadOnly)?;
            }
            configure_bfs_rules(
                bfscfg,
                profile_name,
                writable_directories,
                readonly_directories,
                permissions.allow_fs_write,
            )?;
        }
        FilesystemTier::Dacl => {
            for root in &drive_roots {
                // Modifying a drive root's DACL usually needs `WRITE_DAC`
                // (admin). Without it shells can't start in this tier — the
                // BFS tier or an admin one-time grant (MXC's
                // `prepare-system-drive`) is the fix — so log rather than
                // failing the whole command.
                if let Err(error) = grant_ace(profile.sid(), root, ROOT_METADATA_MASK, false) {
                    log::warn!(
                        "sandbox: could not grant AppContainer metadata access to drive root {} \
                         (shells may fail to start; needs admin or the BFS tier): {error:#}",
                        root.display()
                    );
                }
            }

            for directory in writable_directories {
                let directory = canonicalize_or_original(directory);
                grant_subtree(profile.sid(), &directory, WRITE_ACCESS_MASK).with_context(|| {
                    format!(
                        "failed to grant sandbox write access to {}",
                        directory.display()
                    )
                })?;
            }

            for directory in readonly_directories {
                let directory = canonicalize_or_original(directory);
                grant_subtree(profile.sid(), &directory, READ_ACCESS_MASK).with_context(|| {
                    format!(
                        "failed to grant sandbox read access to {}",
                        directory.display()
                    )
                })?;
            }

            if permissions.allow_fs_write {
                let user_profile = user_profile_dir()?;
                let write_capability = derive_capability_sid(PROFILE_WRITE_CAPABILITY)?;
                grant_subtree(write_capability.as_psid(), &user_profile, WRITE_ACCESS_MASK)
                    .with_context(|| {
                        format!(
                            "failed to grant sandbox write access to {}",
                            user_profile.display()
                        )
                    })?;
            }
        }
    }

    let mut policy_file = NamedTempFile::new().context("failed to create sandbox policy file")?;
    let policy = SandboxPolicy {
        profile_name: profile.name,
        allow_network: permissions.allow_network,
        allow_fs_write: permissions.allow_fs_write,
    };
    serde_json::to_writer(&mut policy_file, &policy)
        .context("failed to write sandbox policy file")?;
    policy_file
        .flush()
        .context("failed to flush sandbox policy file")?;
    let policy_path = policy_file
        .path()
        .to_str()
        .with_context(|| {
            format!(
                "sandbox policy file path contains invalid UTF-8: {}",
                policy_file.path().display()
            )
        })?
        .to_string();

    let helper_program = std::env::current_exe()
        .context("failed to locate the Zed executable for the sandbox helper")?;
    let helper_program = helper_program
        .to_str()
        .with_context(|| {
            format!(
                "Zed executable path contains invalid UTF-8: {}",
                helper_program.display()
            )
        })?
        .to_string();

    let mut wrapped_args = vec![
        "--sandbox-helper".to_string(),
        policy_path,
        "--".to_string(),
        program.to_string(),
    ];
    wrapped_args.extend(args.iter().cloned());

    Ok((
        helper_program,
        wrapped_args,
        AppContainerLaunchConfig {
            _policy_file: policy_file,
        },
    ))
}

/// Best-effort cleanup when an agent thread is deleted: clear the BFS
/// rules registered for the thread's AppContainer identity (when the
/// broker is available), revoke the ACEs granted to the thread's package
/// SID on every recorded root, then delete the profile. The DACL tier's
/// user-profile grant is left alone: it targets the shared capability SID,
/// which is deliberately persistent and grants nothing to processes that
/// don't list the capability.
///
/// This is a hygiene concern, not a security one — BFS rules and ACEs for
/// a deleted profile grant access to nobody — so failures are aggregated
/// rather than aborting at the first error.
pub fn cleanup_profile(profile_name: &str, granted_roots: &[PathBuf]) -> Result<()> {
    let sid = unsafe { DeriveAppContainerSidFromAppContainerName(&HSTRING::from(profile_name)) }
        .with_context(|| format!("failed to derive AppContainer SID for {profile_name}"))?;
    let sid = OwnedSid(sid);

    let mut errors = Vec::new();

    // Clear all BFS rules registered for this AppContainer identity. Done
    // unconditionally when the broker is available: rules may exist even
    // if this session never added any (e.g. they were added by a previous
    // Zed session for the same thread).
    if let FilesystemTier::Bfs { bfscfg } = filesystem_tier() {
        if let Err(error) = run_bfscfg(bfscfg, &bfs_clear_policy_args(profile_name)) {
            errors.push(format!("{error:#}"));
        }
    }

    // Revoke DACL grants regardless of the current tier: ACEs may predate
    // the BFS tier (older sessions, or runs with `ZED_SANDBOX_FORCE_DACL`),
    // and revoking is a no-op when none exist.
    let roots = granted_roots
        .iter()
        .map(|root| canonicalize_or_original(root));
    for root in roots {
        if let Err(error) = revoke_subtree(sid.0, &root) {
            errors.push(format!("{}: {error:#}", root.display()));
        }
    }

    if let Err(error) = AppContainerProfile::delete(profile_name) {
        errors.push(format!("{error:#}"));
    }

    if errors.is_empty() {
        Ok(())
    } else {
        bail!(
            "failed to fully clean up AppContainer profile {profile_name}: {}",
            errors.join("; ")
        );
    }
}

/// Access level of a single BFS path rule.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum BfsAccess {
    ReadWrite,
    ReadOnly,
}

/// Register the BFS rules for one command: read-write rules for the
/// writable roots, read-only rules for the readable roots, and — when
/// `allow_fs_write` was approved — a read-write rule over `%USERPROFILE%`
/// (the BFS encoding of "broad writes": O(1), unlike the DACL tier's
/// capability walk).
fn configure_bfs_rules(
    bfscfg: &Path,
    app_container_name: &str,
    writable_directories: &[&Path],
    readonly_directories: &[&Path],
    allow_fs_write: bool,
) -> Result<()> {
    for directory in writable_directories {
        ensure_bfs_rule(bfscfg, app_container_name, directory, BfsAccess::ReadWrite)?;
    }
    for directory in readonly_directories {
        ensure_bfs_rule(bfscfg, app_container_name, directory, BfsAccess::ReadOnly)?;
    }
    if allow_fs_write {
        let user_profile = user_profile_dir()?;
        ensure_bfs_rule(
            bfscfg,
            app_container_name,
            &user_profile,
            BfsAccess::ReadWrite,
        )?;
    }
    Ok(())
}

/// Register one BFS path rule, deduplicating within this Zed session so a
/// rule already registered for the thread isn't re-added on every command.
///
/// Rules outlive this process and are cleared wholesale by
/// [`cleanup_profile`] (`--clearpolicy --appid`), so a rule re-added after
/// a Zed restart at worst duplicates an existing entry — harmless, and
/// removed with everything else at thread deletion.
fn ensure_bfs_rule(
    bfscfg: &Path,
    app_container_name: &str,
    directory: &Path,
    access: BfsAccess,
) -> Result<()> {
    static REGISTERED_RULES: OnceLock<std::sync::Mutex<HashSet<(String, PathBuf, BfsAccess)>>> =
        OnceLock::new();

    let directory = canonicalize_or_original(directory);
    let key = (app_container_name.to_string(), directory.clone(), access);
    let registered = REGISTERED_RULES.get_or_init(Default::default);
    {
        let registered = registered
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if registered.contains(&key) {
            return Ok(());
        }
    }

    run_bfscfg(
        bfscfg,
        &bfs_add_policy_args(app_container_name, &directory, access),
    )
    .with_context(|| {
        format!(
            "failed to register a sandbox filesystem rule for {}",
            directory.display()
        )
    })?;

    registered
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .insert(key);
    Ok(())
}

fn bfs_add_policy_args(
    app_container_name: &str,
    directory: &Path,
    access: BfsAccess,
) -> Vec<OsString> {
    let mut args: Vec<OsString> = vec![
        "--addpolicy".into(),
        match access {
            BfsAccess::ReadWrite => "--policybroker",
            BfsAccess::ReadOnly => "--policybrokerreadonly",
        }
        .into(),
        "--filename".into(),
        directory.as_os_str().to_owned(),
        "--appid".into(),
        app_container_name.into(),
    ];
    // Cover the whole subtree. Skipped for bare drive roots, where the
    // broker rejects subtree rules (mirrors MXC's special case); the rule
    // then covers the root directory itself only, which fails closed —
    // deeper accesses get denied and the model can escalate.
    if directory.parent().is_some() {
        args.push("--containerinherit".into());
    }
    args
}

fn bfs_clear_policy_args(app_container_name: &str) -> Vec<OsString> {
    vec![
        "--clearpolicy".into(),
        "--appid".into(),
        app_container_name.into(),
    ]
}

/// Marker `bfscfg.exe` prints when an operation fails (it does not always
/// reflect failures in its exit code, so the output is checked too — same
/// convention MXC relies on).
const BFSCFG_FAILURE_MARKER: &str = "Unable to perform policy operation";

#[allow(
    clippy::disallowed_methods,
    reason = "runs on the background executor: wrap_invocation and cleanup_profile are documented as blocking"
)]
fn run_bfscfg(bfscfg: &Path, args: &[OsString]) -> Result<()> {
    use std::os::windows::process::CommandExt as _;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;

    let output = std::process::Command::new(bfscfg)
        .args(args.iter().map(OsStr::new))
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .with_context(|| format!("failed to run {}", bfscfg.display()))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    ensure!(
        output.status.success()
            && !stdout.contains(BFSCFG_FAILURE_MARKER)
            && !stderr.contains(BFSCFG_FAILURE_MARKER),
        "bfscfg.exe failed (status {:?}): {}{}",
        output.status.code(),
        stdout.trim(),
        stderr.trim(),
    );
    Ok(())
}

/// Entry point for the hidden `zed --sandbox-helper` subcommand.
///
/// `args` is everything after `--sandbox-helper` on the command line:
/// `<policy path> -- <program> <args...>`. Returns the process exit code to
/// relay (the wrapped command's exit code, or 1 if the helper itself
/// failed).
pub fn run_sandbox_helper(args: Vec<String>) -> i32 {
    match run_helper(args) {
        Ok(exit_code) => exit_code,
        Err(error) => {
            eprintln!("zed sandbox helper: {error:#}");
            1
        }
    }
}

fn run_helper(args: Vec<String>) -> Result<i32> {
    let mut args = args.into_iter();
    let policy_path = args.next().context("missing sandbox policy file path")?;
    ensure!(
        args.next().as_deref() == Some("--"),
        "expected `--` between the policy file path and the command"
    );
    let command: Vec<String> = args.collect();
    ensure!(!command.is_empty(), "missing command to run in the sandbox");

    let policy_json = std::fs::read_to_string(&policy_path)
        .with_context(|| format!("failed to read sandbox policy file {policy_path}"))?;
    let policy: SandboxPolicy = serde_json::from_str(&policy_json)
        .with_context(|| format!("failed to parse sandbox policy file {policy_path}"))?;

    // The parent created the profile before spawning us; `create_or_open`
    // is idempotent and recovers the package SID either way.
    let profile = AppContainerProfile::create_or_open(&policy.profile_name)?;

    // The capability list controls which of the persistent capability-SID
    // ACEs apply to this process. File-I/O-only milestone: the network
    // capabilities are never included, so network access stays
    // default-denied even when the policy requested it.
    let mut capabilities = Vec::new();
    if policy.allow_fs_write {
        capabilities.push(derive_capability_sid(PROFILE_WRITE_CAPABILITY)?);
    }

    spawn_in_container(&profile, &capabilities, &command)
}

/// Spawn `command` inside the AppContainer with the given capability SIDs
/// enabled, wait for it, and relay the exit code. The child inherits the
/// helper's console and standard handles (the helper is the ConPTY child
/// Zed spawned).
fn spawn_in_container(
    profile: &AppContainerProfile,
    capabilities: &[SidBuffer],
    command: &[String],
) -> Result<i32> {
    let command_line = build_command_line(command);
    let mut command_line_wide: Vec<u16> = command_line.encode_utf16().chain([0]).collect();
    let mut capability_attributes: Vec<SID_AND_ATTRIBUTES> = capabilities
        .iter()
        .map(|capability| SID_AND_ATTRIBUTES {
            Sid: capability.as_psid(),
            Attributes: SE_GROUP_ENABLED as u32,
        })
        .collect();

    unsafe {
        let mut attribute_list_size = 0usize;
        // This first call fails by design (with ERROR_INSUFFICIENT_BUFFER)
        // and reports the required buffer size.
        InitializeProcThreadAttributeList(None, 1, None, &mut attribute_list_size).ok();
        ensure!(
            attribute_list_size > 0,
            "failed to size the proc-thread attribute list"
        );
        let mut attribute_list_buffer = vec![0u8; attribute_list_size];
        let attribute_list =
            LPPROC_THREAD_ATTRIBUTE_LIST(attribute_list_buffer.as_mut_ptr().cast());
        InitializeProcThreadAttributeList(Some(attribute_list), 1, None, &mut attribute_list_size)
            .context("failed to initialize the proc-thread attribute list")?;

        let result = (|| {
            let security_capabilities = SECURITY_CAPABILITIES {
                AppContainerSid: profile.sid(),
                Capabilities: capability_attributes.as_mut_ptr(),
                CapabilityCount: capability_attributes.len() as u32,
                Reserved: 0,
            };
            UpdateProcThreadAttribute(
                attribute_list,
                0,
                PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES as usize,
                Some(&security_capabilities as *const SECURITY_CAPABILITIES as *const c_void),
                std::mem::size_of::<SECURITY_CAPABILITIES>(),
                None,
                None,
            )
            .context("failed to set the AppContainer security capabilities")?;

            let mut startup_info = STARTUPINFOEXW::default();
            startup_info.StartupInfo.cb = std::mem::size_of::<STARTUPINFOEXW>() as u32;
            startup_info.lpAttributeList = attribute_list;
            // Pass our standard handles through when we have them; when the
            // helper has no usable std handles (e.g. a GUI-subsystem build
            // without redirection) fall back to plain console inheritance.
            let std_handles = [
                GetStdHandle(STD_INPUT_HANDLE),
                GetStdHandle(STD_OUTPUT_HANDLE),
                GetStdHandle(STD_ERROR_HANDLE),
            ];
            if let [Ok(stdin), Ok(stdout), Ok(stderr)] = std_handles
                && !stdin.is_invalid()
                && !stdout.is_invalid()
                && !stderr.is_invalid()
            {
                startup_info.StartupInfo.dwFlags = STARTF_USESTDHANDLES;
                startup_info.StartupInfo.hStdInput = stdin;
                startup_info.StartupInfo.hStdOutput = stdout;
                startup_info.StartupInfo.hStdError = stderr;
            }

            let mut process_information = PROCESS_INFORMATION::default();
            CreateProcessW(
                PCWSTR::null(),
                Some(PWSTR(command_line_wide.as_mut_ptr())),
                None,
                None,
                true,
                EXTENDED_STARTUPINFO_PRESENT,
                None,
                PCWSTR::null(),
                &startup_info as *const STARTUPINFOEXW as *const STARTUPINFOW,
                &mut process_information,
            )
            .with_context(|| format!("failed to spawn `{command_line}` in the AppContainer"))?;

            if let Err(error) = CloseHandle(process_information.hThread) {
                eprintln!("zed sandbox helper: failed to close thread handle: {error}");
            }
            let process: HANDLE = process_information.hProcess;

            let wait_result = WaitForSingleObject(process, INFINITE);
            let exit_code = if wait_result == WAIT_OBJECT_0 {
                let mut exit_code = 0u32;
                GetExitCodeProcess(process, &mut exit_code)
                    .context("failed to read the sandboxed command's exit code")?;
                Ok(exit_code as i32)
            } else {
                Err(anyhow::anyhow!(
                    "failed to wait for the sandboxed command: {wait_result:?}"
                ))
            };
            if let Err(error) = CloseHandle(process) {
                eprintln!("zed sandbox helper: failed to close process handle: {error}");
            }
            exit_code
        })();

        DeleteProcThreadAttributeList(attribute_list);
        result
    }
}

/// Rights granted on drive roots so shells can start: just the metadata
/// reads tools issue against `C:\` during startup
/// (`FILE_READ_ATTRIBUTES | FILE_READ_EA | READ_CONTROL | SYNCHRONIZE`,
/// `0x0012_0088`). Deliberately excludes `FILE_LIST_DIRECTORY` /
/// `FILE_READ_DATA`, so the container still can't enumerate the drive root
/// — same minimal grant MXC's `prepare-system-drive` applies.
const ROOT_METADATA_MASK: u32 = 0x0012_0088;

/// Add an inheritable allow-ACE for `sid` on `root`, covering the whole
/// subtree via `OBJECT_INHERIT_ACE | CONTAINER_INHERIT_ACE`. Idempotent: if
/// an inheritable allow-ACE for the same SID already covers `access_mask`,
/// the DACL is left untouched.
fn grant_subtree(sid: PSID, root: &Path, access_mask: u32) -> Result<()> {
    grant_ace(sid, root, access_mask, true)
}

/// Add an allow-ACE for `sid` on `root`. When `inheritable`, the ACE
/// propagates to the whole subtree (`OBJECT_INHERIT_ACE |
/// CONTAINER_INHERIT_ACE`) — which eagerly rewrites every descendant's
/// DACL; when not, it applies to `root` itself only (O(1), used for
/// drive-root metadata grants). Idempotent against an existing allow-ACE
/// for the same SID with matching mask and inheritance.
fn grant_ace(sid: PSID, root: &Path, access_mask: u32, inheritable: bool) -> Result<()> {
    let root_wide = HSTRING::from(root.as_os_str());
    unsafe {
        let mut security_descriptor = PSECURITY_DESCRIPTOR::default();
        let mut existing_acl: *mut ACL = std::ptr::null_mut();
        let result = GetNamedSecurityInfoW(
            &root_wide,
            SE_FILE_OBJECT,
            DACL_SECURITY_INFORMATION,
            None,
            None,
            Some(&mut existing_acl),
            None,
            &mut security_descriptor,
        );
        ensure!(
            result == ERROR_SUCCESS,
            "failed to read the DACL of {}: {result:?}",
            root.display()
        );
        let _security_descriptor_guard = LocalFreeGuard(HLOCAL(security_descriptor.0));

        // A null DACL grants unrestricted access to everyone (including
        // AppContainer processes); replacing it with a one-ACE DACL would
        // *remove* everyone else's access, so leave it alone.
        if existing_acl.is_null() {
            return Ok(());
        }

        if acl_contains_allow_ace(existing_acl, sid, access_mask, inheritable) {
            return Ok(());
        }

        let inheritance = if inheritable {
            SUB_CONTAINERS_AND_OBJECTS_INHERIT
        } else {
            windows::Win32::Security::ACE_FLAGS(0)
        };
        let explicit_access = EXPLICIT_ACCESS_W {
            grfAccessPermissions: access_mask,
            grfAccessMode: GRANT_ACCESS,
            grfInheritance: inheritance,
            Trustee: TRUSTEE_W {
                TrusteeForm: TRUSTEE_IS_SID,
                TrusteeType: TRUSTEE_IS_WELL_KNOWN_GROUP,
                ptstrName: PWSTR(sid.0.cast()),
                ..Default::default()
            },
        };
        let mut new_acl: *mut ACL = std::ptr::null_mut();
        let result = SetEntriesInAclW(Some(&[explicit_access]), Some(existing_acl), &mut new_acl);
        ensure!(
            result == ERROR_SUCCESS,
            "failed to build a new DACL for {}: {result:?}",
            root.display()
        );
        let _new_acl_guard = LocalFreeGuard(HLOCAL(new_acl.cast()));

        let result = SetNamedSecurityInfoW(
            &root_wide,
            SE_FILE_OBJECT,
            DACL_SECURITY_INFORMATION,
            None,
            None,
            Some(new_acl),
            None,
        );
        ensure!(
            result == ERROR_SUCCESS,
            "failed to write the new DACL of {}: {result:?}",
            root.display()
        );
        Ok(())
    }
}

/// Remove every non-inherited allow-ACE for `sid` from the DACL of `root`.
fn revoke_subtree(sid: PSID, root: &Path) -> Result<()> {
    let root_wide = HSTRING::from(root.as_os_str());
    unsafe {
        let mut security_descriptor = PSECURITY_DESCRIPTOR::default();
        let mut acl: *mut ACL = std::ptr::null_mut();
        let result = GetNamedSecurityInfoW(
            &root_wide,
            SE_FILE_OBJECT,
            DACL_SECURITY_INFORMATION,
            None,
            None,
            Some(&mut acl),
            None,
            &mut security_descriptor,
        );
        ensure!(
            result == ERROR_SUCCESS,
            "failed to read the DACL of {}: {result:?}",
            root.display()
        );
        let _security_descriptor_guard = LocalFreeGuard(HLOCAL(security_descriptor.0));

        if acl.is_null() {
            return Ok(());
        }

        let mut removed_any = false;
        // Iterate backwards so removal doesn't shift the indices still to
        // be visited.
        for index in (0..(*acl).AceCount as u32).rev() {
            let mut ace_pointer: *mut c_void = std::ptr::null_mut();
            if GetAce(acl, index, &mut ace_pointer).is_err() {
                continue;
            }
            let header = &*(ace_pointer as *const ACE_HEADER);
            if header.AceType as u32 != ACCESS_ALLOWED_ACE_TYPE {
                continue;
            }
            // Inherited ACEs come from a parent directory's DACL; only
            // remove ACEs that were set directly on this object.
            if header.AceFlags as u32 & INHERITED_ACE.0 != 0 {
                continue;
            }
            let ace = &*(ace_pointer as *const ACCESS_ALLOWED_ACE);
            let ace_sid = PSID(&ace.SidStart as *const u32 as *mut c_void);
            if EqualSid(ace_sid, sid).is_ok() {
                DeleteAce(acl, index).with_context(|| {
                    format!(
                        "failed to delete an ACE from the DACL of {}",
                        root.display()
                    )
                })?;
                removed_any = true;
            }
        }

        if removed_any {
            let result = SetNamedSecurityInfoW(
                &root_wide,
                SE_FILE_OBJECT,
                DACL_SECURITY_INFORMATION,
                None,
                None,
                Some(acl),
                None,
            );
            ensure!(
                result == ERROR_SUCCESS,
                "failed to write the new DACL of {}: {result:?}",
                root.display()
            );
        }
        Ok(())
    }
}

/// Whether `acl` already contains an allow-ACE for `sid` covering at least
/// `access_mask` with the requested inheritance (inheritable ACEs carry
/// both `OBJECT_INHERIT_ACE` and `CONTAINER_INHERIT_ACE`; non-inheritable
/// ones carry neither).
unsafe fn acl_contains_allow_ace(
    acl: *const ACL,
    sid: PSID,
    access_mask: u32,
    inheritable: bool,
) -> bool {
    unsafe {
        let inherit_flags = (OBJECT_INHERIT_ACE.0 | CONTAINER_INHERIT_ACE.0) as u8;
        for index in 0..(*acl).AceCount as u32 {
            let mut ace_pointer: *mut c_void = std::ptr::null_mut();
            if GetAce(acl, index, &mut ace_pointer).is_err() {
                continue;
            }
            let header = &*(ace_pointer as *const ACE_HEADER);
            if header.AceType as u32 != ACCESS_ALLOWED_ACE_TYPE {
                continue;
            }
            let has_inherit = header.AceFlags & inherit_flags == inherit_flags;
            if has_inherit != inheritable {
                continue;
            }
            let ace = &*(ace_pointer as *const ACCESS_ALLOWED_ACE);
            if ace.Mask & access_mask != access_mask {
                continue;
            }
            let ace_sid = PSID(&ace.SidStart as *const u32 as *mut c_void);
            if EqualSid(ace_sid, sid).is_ok() {
                return true;
            }
        }
        false
    }
}

/// Derive the SID for a custom ("private") capability name. Capability
/// SIDs are deterministic hashes of the name — no registration is needed;
/// any process may list them at launch, and any DACL may reference them.
fn derive_capability_sid(name: &str) -> Result<SidBuffer> {
    unsafe {
        let mut group_sids: *mut PSID = std::ptr::null_mut();
        let mut group_count = 0u32;
        let mut capability_sids: *mut PSID = std::ptr::null_mut();
        let mut capability_count = 0u32;
        DeriveCapabilitySidsFromName(
            &HSTRING::from(name),
            &mut group_sids,
            &mut group_count,
            &mut capability_sids,
            &mut capability_count,
        )
        .with_context(|| format!("failed to derive the capability SID for {name}"))?;

        // Copy the capability SID into an owned buffer, then free
        // everything the API allocated (each SID and both arrays, all via
        // `LocalFree`).
        let result = if capability_count > 0 {
            let sid = *capability_sids;
            let length = GetLengthSid(sid) as usize;
            let mut buffer = vec![0u8; length];
            CopySid(length as u32, PSID(buffer.as_mut_ptr().cast()), sid)
                .map(|()| SidBuffer(buffer))
                .with_context(|| format!("failed to copy the capability SID for {name}"))
        } else {
            Err(anyhow::anyhow!("no capability SID derived for {name}"))
        };

        for index in 0..group_count as usize {
            LocalFree(Some(HLOCAL((*group_sids.add(index)).0)));
        }
        LocalFree(Some(HLOCAL(group_sids.cast())));
        for index in 0..capability_count as usize {
            LocalFree(Some(HLOCAL((*capability_sids.add(index)).0)));
        }
        LocalFree(Some(HLOCAL(capability_sids.cast())));

        result
    }
}

/// The distinct drive roots (e.g. `C:\`, `D:\`) that need metadata access
/// for shells to start: the system drive plus the root of every granted
/// directory. Deduplicated, case-insensitively.
fn distinct_drive_roots(writable: &[&Path], readonly: &[&Path]) -> Vec<PathBuf> {
    let mut roots: Vec<PathBuf> = Vec::new();
    let mut push = |path: PathBuf| {
        if !roots
            .iter()
            .any(|existing| existing.as_os_str().eq_ignore_ascii_case(path.as_os_str()))
        {
            roots.push(path);
        }
    };

    if let Some(system_drive) = system_directory().ok().and_then(|dir| drive_root(&dir)) {
        push(system_drive);
    }
    for directory in writable.iter().chain(readonly.iter()) {
        if let Some(root) = drive_root(&canonicalize_or_original(directory)) {
            push(root);
        }
    }
    roots
}

/// The drive-root prefix of `path` (e.g. `C:\Users\me` -> `C:\`). Returns
/// `None` for paths without a drive root (UNC, relative).
fn drive_root(path: &Path) -> Option<PathBuf> {
    use std::path::{Component, Prefix};
    let mut components = path.components();
    match components.next() {
        Some(Component::Prefix(prefix)) => match prefix.kind() {
            Prefix::Disk(_) | Prefix::VerbatimDisk(_) => {
                // Reattach the root separator: `Prefix` is just `C:`.
                let mut root = PathBuf::from(prefix.as_os_str());
                root.push(std::path::MAIN_SEPARATOR_STR);
                Some(root)
            }
            _ => None,
        },
        _ => None,
    }
}

fn user_profile_dir() -> Result<PathBuf> {
    let directory = std::env::var_os("USERPROFILE")
        .context("the USERPROFILE environment variable is not set")?;
    Ok(canonicalize_or_original(Path::new(&directory)))
}

/// Canonicalize so ACEs land on the real directory (resolving symlinks and
/// substituted drives), falling back to the original path if
/// canonicalization fails. `dunce` avoids `\\?\` extended-length paths,
/// which some of the security APIs handle inconsistently.
fn canonicalize_or_original(path: &Path) -> PathBuf {
    dunce::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

/// Join `command` into a single command line for `CreateProcessW`, quoting
/// each argument according to the standard MSVCRT rules (the inverse of
/// `CommandLineToArgvW`).
fn build_command_line(command: &[String]) -> String {
    let mut command_line = String::new();
    for (index, argument) in command.iter().enumerate() {
        if index > 0 {
            command_line.push(' ');
        }
        append_quoted_argument(&mut command_line, argument);
    }
    command_line
}

fn append_quoted_argument(command_line: &mut String, argument: &str) {
    let needs_quoting = argument.is_empty()
        || argument
            .chars()
            .any(|character| matches!(character, ' ' | '\t' | '\n' | '\x0B' | '"'));
    if !needs_quoting {
        command_line.push_str(argument);
        return;
    }

    command_line.push('"');
    let mut pending_backslashes = 0;
    for character in argument.chars() {
        match character {
            '\\' => pending_backslashes += 1,
            '"' => {
                // Backslashes directly preceding a quote must be doubled,
                // and the quote itself escaped.
                command_line.extend(std::iter::repeat_n('\\', pending_backslashes * 2 + 1));
                pending_backslashes = 0;
                command_line.push('"');
            }
            other => {
                command_line.extend(std::iter::repeat_n('\\', pending_backslashes));
                pending_backslashes = 0;
                command_line.push(other);
            }
        }
    }
    // Backslashes before the closing quote must be doubled so the quote
    // isn't treated as escaped.
    command_line.extend(std::iter::repeat_n('\\', pending_backslashes * 2));
    command_line.push('"');
}

/// A SID copied into an owned buffer, independent of how the original was
/// allocated.
struct SidBuffer(Vec<u8>);

impl SidBuffer {
    fn as_psid(&self) -> PSID {
        PSID(self.0.as_ptr() as *mut c_void)
    }
}

/// A SID allocated by the AppContainer profile APIs, freed with `FreeSid`.
struct OwnedSid(PSID);

impl Drop for OwnedSid {
    fn drop(&mut self) {
        unsafe {
            FreeSid(self.0);
        }
    }
}

/// A buffer allocated by the security APIs, freed with `LocalFree`.
struct LocalFreeGuard(HLOCAL);

impl Drop for LocalFreeGuard {
    fn drop(&mut self) {
        unsafe {
            LocalFree(Some(self.0));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_name_is_stable_and_valid() {
        let name = profile_name_for_thread("thread-id-1");
        assert_eq!(name, profile_name_for_thread("thread-id-1"));
        assert_ne!(name, profile_name_for_thread("thread-id-2"));
        // AppContainer profile names are limited to 64 characters drawn
        // from alphanumerics, '.', '_', and '-'.
        assert!(name.len() <= 64);
        assert!(
            name.chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'))
        );
        assert!(name.starts_with("Zed.AgentSandbox."));
    }

    #[test]
    fn test_policy_file_round_trip() {
        let policy = SandboxPolicy {
            profile_name: profile_name_for_thread("round-trip"),
            allow_network: true,
            allow_fs_write: true,
        };
        let serialized = serde_json::to_string(&policy).unwrap();
        let deserialized: SandboxPolicy = serde_json::from_str(&serialized).unwrap();
        assert_eq!(policy, deserialized);
    }

    #[test]
    fn test_bfs_add_policy_args_shape() {
        let args = bfs_add_policy_args(
            "Zed.AgentSandbox.0123",
            Path::new("D:\\src\\project"),
            BfsAccess::ReadWrite,
        );
        assert_eq!(
            args,
            vec![
                OsString::from("--addpolicy"),
                OsString::from("--policybroker"),
                OsString::from("--filename"),
                OsString::from("D:\\src\\project"),
                OsString::from("--appid"),
                OsString::from("Zed.AgentSandbox.0123"),
                OsString::from("--containerinherit"),
            ]
        );

        let args = bfs_add_policy_args(
            "Zed.AgentSandbox.0123",
            Path::new("C:\\Users\\me\\.cargo"),
            BfsAccess::ReadOnly,
        );
        assert!(args.contains(&OsString::from("--policybrokerreadonly")));
        assert!(args.contains(&OsString::from("--containerinherit")));

        // Drive roots don't get a subtree rule.
        let args = bfs_add_policy_args(
            "Zed.AgentSandbox.0123",
            Path::new("C:\\"),
            BfsAccess::ReadWrite,
        );
        assert!(!args.contains(&OsString::from("--containerinherit")));
    }

    #[test]
    fn test_drive_root_extraction() {
        assert_eq!(
            drive_root(Path::new("C:\\Users\\me\\project")),
            Some(PathBuf::from("C:\\"))
        );
        assert_eq!(drive_root(Path::new("D:\\")), Some(PathBuf::from("D:\\")));
        // UNC and relative paths have no drive root.
        assert_eq!(drive_root(Path::new("\\\\server\\share\\x")), None);
        assert_eq!(drive_root(Path::new("relative\\path")), None);
    }

    #[test]
    fn test_distinct_drive_roots_dedupes_and_includes_system_drive() {
        let writable = [Path::new("C:\\Users\\me\\a"), Path::new("D:\\src")];
        let readonly = [Path::new("c:\\users\\me\\b")];
        let roots = distinct_drive_roots(&writable, &readonly);
        // C:\ (system drive + the C: paths, case-insensitively deduped)
        // and D:\ — no duplicates.
        assert!(
            roots
                .iter()
                .filter(|r| r.as_os_str().eq_ignore_ascii_case("C:\\"))
                .count()
                == 1
        );
        assert!(roots.iter().any(|r| r == &PathBuf::from("D:\\")));
    }

    #[test]
    fn test_bfs_clear_policy_args_shape() {
        assert_eq!(
            bfs_clear_policy_args("Zed.AgentSandbox.0123"),
            vec![
                OsString::from("--clearpolicy"),
                OsString::from("--appid"),
                OsString::from("Zed.AgentSandbox.0123"),
            ]
        );
    }

    #[test]
    fn test_system_directory_resolves() {
        let system32 = system_directory().unwrap();
        assert!(system32.is_absolute());
        assert!(system32.join("kernel32.dll").exists());
    }

    #[test]
    fn test_capability_sid_derivation_is_stable() {
        let first = derive_capability_sid(PROFILE_WRITE_CAPABILITY).unwrap();
        let second = derive_capability_sid(PROFILE_WRITE_CAPABILITY).unwrap();
        assert!(unsafe { EqualSid(first.as_psid(), second.as_psid()) }.is_ok());

        let other = derive_capability_sid("zedAgentSandboxSomeOtherCapability").unwrap();
        assert!(unsafe { EqualSid(first.as_psid(), other.as_psid()) }.is_err());
    }

    #[test]
    fn test_command_line_quoting() {
        assert_eq!(build_command_line(&["simple".into()]), "simple");
        assert_eq!(
            build_command_line(&["C:\\Windows\\System32\\cmd.exe".into(), "/c".into()]),
            "C:\\Windows\\System32\\cmd.exe /c"
        );
        assert_eq!(build_command_line(&["with space".into()]), "\"with space\"");
        assert_eq!(build_command_line(&["".into()]), "\"\"");
        assert_eq!(
            build_command_line(&["say \"hi\"".into()]),
            "\"say \\\"hi\\\"\""
        );
        assert_eq!(
            build_command_line(&["trailing\\".into(), "next".into()]),
            "trailing\\ next"
        );
        assert_eq!(
            build_command_line(&["trailing backslash\\".into()]),
            "\"trailing backslash\\\\\""
        );
        assert_eq!(
            build_command_line(&["backslash \\\" quote".into()]),
            "\"backslash \\\\\\\" quote\""
        );
    }

    #[test]
    fn test_sid_derivation_is_stable() {
        let name = profile_name_for_thread("sid-derivation-test");
        let name_wide = HSTRING::from(name.as_str());
        let first =
            OwnedSid(unsafe { DeriveAppContainerSidFromAppContainerName(&name_wide) }.unwrap());
        let second =
            OwnedSid(unsafe { DeriveAppContainerSidFromAppContainerName(&name_wide) }.unwrap());
        assert!(unsafe { EqualSid(first.0, second.0) }.is_ok());
    }

    #[test]
    fn test_grant_subtree_is_idempotent_and_revocable() {
        let name = profile_name_for_thread("grant-idempotency-test");
        let sid = OwnedSid(
            unsafe { DeriveAppContainerSidFromAppContainerName(&HSTRING::from(name.as_str())) }
                .unwrap(),
        );
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path();

        grant_subtree(sid.0, root, WRITE_ACCESS_MASK).unwrap();
        assert_eq!(count_matching_aces(root, sid.0), 1);

        // Granting again must not add a second ACE.
        grant_subtree(sid.0, root, WRITE_ACCESS_MASK).unwrap();
        assert_eq!(count_matching_aces(root, sid.0), 1);

        // A narrower grant is already covered by the wider one.
        grant_subtree(sid.0, root, READ_ACCESS_MASK).unwrap();
        assert_eq!(count_matching_aces(root, sid.0), 1);

        revoke_subtree(sid.0, root).unwrap();
        assert_eq!(count_matching_aces(root, sid.0), 0);
    }

    fn count_matching_aces(root: &Path, sid: PSID) -> usize {
        let root_wide = HSTRING::from(root.as_os_str());
        unsafe {
            let mut security_descriptor = PSECURITY_DESCRIPTOR::default();
            let mut acl: *mut ACL = std::ptr::null_mut();
            let result = GetNamedSecurityInfoW(
                &root_wide,
                SE_FILE_OBJECT,
                DACL_SECURITY_INFORMATION,
                None,
                None,
                Some(&mut acl),
                None,
                &mut security_descriptor,
            );
            assert_eq!(result, ERROR_SUCCESS);
            let _guard = LocalFreeGuard(HLOCAL(security_descriptor.0));
            if acl.is_null() {
                return 0;
            }
            let mut count = 0;
            for index in 0..(*acl).AceCount as u32 {
                let mut ace_pointer: *mut c_void = std::ptr::null_mut();
                if GetAce(acl, index, &mut ace_pointer).is_err() {
                    continue;
                }
                let header = &*(ace_pointer as *const ACE_HEADER);
                if header.AceType as u32 != ACCESS_ALLOWED_ACE_TYPE {
                    continue;
                }
                let ace = &*(ace_pointer as *const ACCESS_ALLOWED_ACE);
                let ace_sid = PSID(&ace.SidStart as *const u32 as *mut c_void);
                if EqualSid(ace_sid, sid).is_ok() {
                    count += 1;
                }
            }
            count
        }
    }
}
