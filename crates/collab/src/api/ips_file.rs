use collections::HashMap;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use util::SemanticVersion;

#[derive(Debug)]
pub struct IpsFile {
    pub header: Header,
    pub body: Body,
}

impl IpsFile {
    pub fn parse(bytes: &[u8]) -> anyhow::Result<IpsFile> {
        let mut split = bytes.splitn(2, |&b| b == b'\n');
        let header_bytes = split
            .next()
            .ok_or_else(|| anyhow::anyhow!("No header found"))?;
        let header: Header = serde_json::from_slice(header_bytes)
            .map_err(|e| anyhow::anyhow!("Failed to parse header: {}", e))?;

        let body_bytes = split
            .next()
            .ok_or_else(|| anyhow::anyhow!("No body found"))?;

        let body: Body = serde_json::from_slice(body_bytes)
            .map_err(|e| anyhow::anyhow!("Failed to parse body: {}", e))?;
        Ok(IpsFile { header, body })
    }

    pub fn faulting_thread(&self) -> Option<&Thread> {
        self.body.threads.get(self.body.faulting_thread? as usize)
    }

    pub fn app_version(&self) -> Option<SemanticVersion> {
        self.header.app_version.parse().ok()
    }

    pub fn timestamp(&self) -> anyhow::Result<chrono::DateTime<chrono::FixedOffset>> {
        chrono::DateTime::parse_from_str(&self.header.timestamp, "%Y-%m-%d %H:%M:%S%.f %#z")
            .map_err(|e| anyhow::anyhow!(e))
    }

    pub fn description(&self, panic: Option<&str>) -> String {
        let mut desc = if self.body.termination.indicator == "Abort trap: 6" {
            match panic {
                Some(panic_message) => format!("Panic `{}`", panic_message),
                None => "Crash `Abort trap: 6` (possible panic)".into(),
            }
        } else if let Some(msg) = &self.body.exception.message {
            format!("Exception `{}`", msg)
        } else {
            format!("Crash `{}`", self.body.termination.indicator)
        };
        if let Some(thread) = self.faulting_thread() {
            if let Some(queue) = thread.queue.as_ref() {
                desc += &format!(
                    " on thread {} ({})",
                    self.body.faulting_thread.unwrap_or_default(),
                    queue
                );
            } else {
                desc += &format!(
                    " on thread {} ({})",
                    self.body.faulting_thread.unwrap_or_default(),
                    thread.name.clone().unwrap_or_default()
                );
            }
        }
        desc
    }

    pub fn backtrace_summary(&self) -> String {
        if let Some(thread) = self.faulting_thread() {
            let mut frames = thread
                .frames
                .iter()
                .filter_map(|frame| {
                    if let Some(name) = &frame.symbol {
                        if self.is_ignorable_frame(name) {
                            return None;
                        }
                        Some(format!("{:#}", rustc_demangle::demangle(name)))
                    } else if let Some(image) = self.body.used_images.get(frame.image_index) {
                        Some(image.name.clone().unwrap_or("<unknown-image>".into()))
                    } else {
                        Some("<unknown>".into())
                    }
                })
                .collect::<Vec<_>>();

            let total = frames.len();
            if total > 21 {
                frames = frames.into_iter().take(20).collect();
                frames.push(format!("  and {} more...", total - 20))
            }
            frames.join("\n")
        } else {
            "<no backtrace available>".into()
        }
    }

    fn is_ignorable_frame(&self, symbol: &String) -> bool {
        [
            "pthread_kill",
            "panic",
            "backtrace",
            "rust_begin_unwind",
            "abort",
        ]
        .iter()
        .any(|s| symbol.contains(s))
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Header {
    pub app_name: String,
    pub timestamp: String,
    pub app_version: String,
    pub slice_uuid: String,
    pub build_version: String,
    pub platform: i64,
    #[serde(rename = "bundleID", default)]
    pub bundle_id: String,
    pub share_with_app_devs: i64,
    pub is_first_party: i64,
    pub bug_type: String,
    pub os_version: String,
    pub roots_installed: i64,
    pub name: String,
    pub incident_id: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Body {
    pub uptime: i64,
    pub proc_role: String,
    pub version: i64,
    #[serde(rename = "userID")]
    pub user_id: i64,
    pub deploy_version: i64,
    pub model_code: String,
    #[serde(rename = "coalitionID")]
    pub coalition_id: i64,
    pub os_version: OsVersion,
    pub capture_time: String,
    pub code_signing_monitor: i64,
    pub incident: String,
    pub pid: i64,
    pub translated: bool,
    pub cpu_type: String,
    #[serde(rename = "roots_installed")]
    pub roots_installed: i64,
    #[serde(rename = "bug_type")]
    pub bug_type: String,
    pub proc_launch: String,
    pub proc_start_abs_time: i64,
    pub proc_exit_abs_time: i64,
    pub proc_name: String,
    pub proc_path: String,
    pub bundle_info: BundleInfo,
    pub store_info: StoreInfo,
    pub parent_proc: String,
    pub parent_pid: i64,
    pub coalition_name: String,
    pub crash_reporter_key: String,
    #[serde(rename = "codeSigningID")]
    pub code_signing_id: String,
    #[serde(rename = "codeSigningTeamID")]
    pub code_signing_team_id: String,
    pub code_signing_flags: i64,
    pub code_signing_validation_category: i64,
    pub code_signing_trust_level: i64,
    pub instruction_byte_stream: InstructionByteStream,
    pub sip: String,
    pub exception: Exception,
    pub termination: Termination,
    pub asi: Asi,
    pub ext_mods: ExtMods,
    pub faulting_thread: Option<i64>,
    pub threads: Vec<Thread>,
    pub used_images: Vec<UsedImage>,
    pub shared_cache: SharedCache,
    pub vm_summary: String,
    pub legacy_info: LegacyInfo,
    pub log_writing_signature: String,
    pub trial_info: TrialInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct OsVersion {
    pub train: String,
    pub build: String,
    pub release_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct BundleInfo {
    #[serde(rename = "CFBundleShortVersionString")]
    pub cfbundle_short_version_string: String,
    #[serde(rename = "CFBundleVersion")]
    pub cfbundle_version: String,
    #[serde(rename = "CFBundleIdentifier")]
    pub cfbundle_identifier: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct StoreInfo {
    pub device_identifier_for_vendor: String,
    pub third_party: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct InstructionByteStream {
    #[serde(rename = "beforePC")]
    pub before_pc: String,
    #[serde(rename = "atPC")]
    pub at_pc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Exception {
    pub codes: String,
    pub raw_codes: Vec<i64>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub subtype: Option<String>,
    pub signal: String,
    pub port: Option<i64>,
    pub guard_id: Option<i64>,
    pub message: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Termination {
    pub flags: i64,
    pub code: i64,
    pub namespace: String,
    pub indicator: String,
    pub by_proc: String,
    pub by_pid: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Asi {
    #[serde(rename = "libsystem_c.dylib")]
    pub libsystem_c_dylib: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ExtMods {
    pub caller: ExtMod,
    pub system: ExtMod,
    pub targeted: ExtMod,
    pub warnings: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ExtMod {
    #[serde(rename = "thread_create")]
    pub thread_create: i64,
    #[serde(rename = "thread_set_state")]
    pub thread_set_state: i64,
    #[serde(rename = "task_for_pid")]
    pub task_for_pid: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Thread {
    pub thread_state: HashMap<String, Value>,
    pub id: i64,
    pub triggered: Option<bool>,
    pub name: Option<String>,
    pub queue: Option<String>,
    pub frames: Vec<Frame>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Frame {
    pub image_offset: i64,
    pub symbol: Option<String>,
    pub symbol_location: Option<i64>,
    pub image_index: usize,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct UsedImage {
    pub source: String,
    pub arch: Option<String>,
    pub base: i64,
    #[serde(rename = "CFBundleShortVersionString")]
    pub cfbundle_short_version_string: Option<String>,
    #[serde(rename = "CFBundleIdentifier")]
    pub cfbundle_identifier: Option<String>,
    pub size: i64,
    pub uuid: String,
    pub path: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "CFBundleVersion")]
    pub cfbundle_version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SharedCache {
    pub base: i64,
    pub size: i64,
    pub uuid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LegacyInfo {
    pub thread_triggered: ThreadTriggered,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ThreadTriggered {
    pub name: String,
    pub queue: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TrialInfo {
    pub rollouts: Vec<Rollout>,
    pub experiments: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Rollout {
    pub rollout_id: String,
    pub factor_pack_ids: HashMap<String, Value>,
    pub deployment_id: i64,
}
