use anyhow::{Result, anyhow};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, sync::LazyLock};

/// Represents the safety assessment of a command
#[derive(Debug, Clone, PartialEq)]
pub enum CommandSafety {
    /// Command is safe to execute
    Safe,
    /// Command is dangerous and needs user approval
    Dangerous(DangerReason),
    /// Command is explicitly whitelisted by the user
    Whitelisted,
}

/// Reasons why a command might be considered dangerous
#[derive(Debug, Clone, PartialEq)]
pub enum DangerReason {
    /// Command can delete files or directories
    Destructive(String),
    /// Command can modify system settings
    SystemModification(String),
    /// Command can access sensitive information
    SensitiveAccess(String),
    /// Command can execute potentially harmful operations
    Execution(String),
    /// Command can modify network settings or access
    Network(String),
    /// Command is a general risky operation
    General(String),
}

impl DangerReason {
    pub fn description(&self) -> &str {
        match self {
            DangerReason::Destructive(desc) => desc,
            DangerReason::SystemModification(desc) => desc,
            DangerReason::SensitiveAccess(desc) => desc,
            DangerReason::Execution(desc) => desc,
            DangerReason::Network(desc) => desc,
            DangerReason::General(desc) => desc,
        }
    }

    pub fn category(&self) -> &'static str {
        match self {
            DangerReason::Destructive(_) => "Destructive",
            DangerReason::SystemModification(_) => "System Modification",
            DangerReason::SensitiveAccess(_) => "Sensitive Access",
            DangerReason::Execution(_) => "Code Execution",
            DangerReason::Network(_) => "Network Access",
            DangerReason::General(_) => "General Risk",
        }
    }
}

/// A dangerous command pattern with its associated metadata
#[derive(Debug, Clone)]
pub struct DangerousCommand {
    /// Regex pattern to match the command
    pub pattern: Regex,
    /// Human-readable description of why this command is dangerous
    pub reason: DangerReason,
    /// Operating systems this pattern applies to (empty = all)
    pub platforms: Vec<Platform>,
}

/// Supported operating system platforms
#[derive(Debug, Clone, PartialEq)]
pub enum Platform {
    Windows,
    MacOS,
    Linux,
    Unix, // macOS + Linux
}

/// User configuration for command safety
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommandSafetyConfig {
    /// Commands that are explicitly allowed to run without confirmation
    pub whitelist: HashSet<String>,
    /// Commands that are explicitly blocked
    pub blacklist: HashSet<String>,
    /// Whether to use the built-in dangerous command detection
    pub use_builtin_blacklist: bool,
}

impl Default for CommandSafetyConfig {
    fn default() -> Self {
        Self {
            whitelist: HashSet::new(),
            blacklist: HashSet::new(),
            use_builtin_blacklist: true,
        }
    }
}

/// Main command safety checker
pub struct CommandSafetyChecker {
    config: CommandSafetyConfig,
    current_platform: Platform,
}

impl CommandSafetyChecker {
    pub fn new(config: CommandSafetyConfig) -> Self {
        let current_platform = detect_platform();
        Self {
            config,
            current_platform,
        }
    }

    /// Check if a command is safe to execute
    pub fn check_command(&self, command: &str) -> CommandSafety {
        let normalized_command = normalize_command(command);

        // Check explicit blacklist first
        if self.is_blacklisted(&normalized_command) {
            return CommandSafety::Dangerous(DangerReason::General(
                "Command is in user blacklist".to_string(),
            ));
        }

        // Check explicit whitelist
        if self.is_whitelisted(&normalized_command) {
            return CommandSafety::Whitelisted;
        }

        // Check built-in dangerous patterns if enabled
        if self.config.use_builtin_blacklist {
            if let Some(danger) = self.check_builtin_patterns(&normalized_command) {
                return CommandSafety::Dangerous(danger);
            }
        }

        CommandSafety::Safe
    }

    fn is_whitelisted(&self, command: &str) -> bool {
        self.config.whitelist.iter().any(|pattern| {
            command.starts_with(pattern) || 
            pattern.contains('*') && wildcard_match(pattern, command)
        })
    }

    fn is_blacklisted(&self, command: &str) -> bool {
        self.config.blacklist.iter().any(|pattern| {
            command.starts_with(pattern) || 
            pattern.contains('*') && wildcard_match(pattern, command)
        })
    }

    fn check_builtin_patterns(&self, command: &str) -> Option<DangerReason> {
        for dangerous_cmd in DANGEROUS_COMMANDS.iter() {
            // Check if this pattern applies to current platform
            if !dangerous_cmd.platforms.is_empty() 
                && !dangerous_cmd.platforms.contains(&self.current_platform)
                && !dangerous_cmd.platforms.iter().any(|p| {
                    *p == Platform::Unix && (self.current_platform == Platform::MacOS || self.current_platform == Platform::Linux)
                }) {
                continue;
            }

            if dangerous_cmd.pattern.is_match(command) {
                return Some(dangerous_cmd.reason.clone());
            }
        }
        None
    }

    /// Add a command to the whitelist
    pub fn add_to_whitelist(&mut self, command: String) {
        self.config.whitelist.insert(command);
    }

    /// Remove a command from the whitelist
    pub fn remove_from_whitelist(&mut self, command: &str) {
        self.config.whitelist.remove(command);
    }

    /// Add a command to the blacklist
    pub fn add_to_blacklist(&mut self, command: String) {
        self.config.blacklist.insert(command);
    }

    /// Remove a command from the blacklist
    pub fn remove_from_blacklist(&mut self, command: &str) {
        self.config.blacklist.remove(command);
    }

    /// Get the current configuration
    pub fn config(&self) -> &CommandSafetyConfig {
        &self.config
    }

    /// Update the configuration
    pub fn update_config(&mut self, config: CommandSafetyConfig) {
        self.config = config;
    }
}

/// Detect the current platform
fn detect_platform() -> Platform {
    if cfg!(target_os = "windows") {
        Platform::Windows
    } else if cfg!(target_os = "macos") {
        Platform::MacOS
    } else if cfg!(target_os = "linux") {
        Platform::Linux
    } else {
        Platform::Unix // Default for Unix-like systems
    }
}

/// Normalize a command by trimming whitespace and converting to lowercase
fn normalize_command(command: &str) -> String {
    command.trim().to_lowercase()
}

/// Simple wildcard matching for patterns with *
fn wildcard_match(pattern: &str, text: &str) -> bool {
    let pattern_parts: Vec<&str> = pattern.split('*').collect();
    if pattern_parts.len() == 1 {
        return text == pattern;
    }

    let mut text_pos = 0;
    for (i, part) in pattern_parts.iter().enumerate() {
        if part.is_empty() {
            continue;
        }

        if i == 0 {
            // First part must match the beginning
            if !text.starts_with(part) {
                return false;
            }
            text_pos = part.len();
        } else if i == pattern_parts.len() - 1 {
            // Last part must match the end
            return text[text_pos..].ends_with(part);
        } else {
            // Middle parts must be found somewhere
            if let Some(pos) = text[text_pos..].find(part) {
                text_pos += pos + part.len();
            } else {
                return false;
            }
        }
    }
    true
}

/// Lazy static collection of dangerous command patterns
static DANGEROUS_COMMANDS: LazyLock<Vec<DangerousCommand>> = LazyLock::new(|| {
    vec![
        // ======= DESTRUCTIVE COMMANDS =======
        
        // rm commands (Unix/Linux/macOS)
        DangerousCommand {
            pattern: Regex::new(r"^rm\s+.*(-r|-rf|-fr|--recursive).*").unwrap(),
            reason: DangerReason::Destructive("Recursive file deletion can destroy important data".to_string()),
            platforms: vec![Platform::Unix],
        },
        DangerousCommand {
            pattern: Regex::new(r"^rm\s+.*(/|\$HOME|\~|/home|\*/|\.\.).*").unwrap(),
            reason: DangerReason::Destructive("Deleting system or home directories".to_string()),
            platforms: vec![Platform::Unix],
        },
        
        // Windows delete commands
        DangerousCommand {
            pattern: Regex::new(r"^(del|rmdir|rd)\s+.*(/s|/q|/f|\*|\.\.).*").unwrap(),
            reason: DangerReason::Destructive("Forced or recursive deletion on Windows".to_string()),
            platforms: vec![Platform::Windows],
        },
        DangerousCommand {
            pattern: Regex::new(r"^(del|rmdir|rd)\s+.*(c:\\|%.*%|\\windows|\\users).*").unwrap(),
            reason: DangerReason::Destructive("Deleting Windows system directories".to_string()),
            platforms: vec![Platform::Windows],
        },
        
        // Format commands
        DangerousCommand {
            pattern: Regex::new(r"^format\s+[a-z]:\s*(/.*)?$").unwrap(),
            reason: DangerReason::Destructive("Formatting disk drives destroys all data".to_string()),
            platforms: vec![Platform::Windows],
        },
        
        // dd command (disk destruction)
        DangerousCommand {
            pattern: Regex::new(r"^dd\s+.*if=/dev/(zero|random|urandom).*of=/dev/.*").unwrap(),
            reason: DangerReason::Destructive("Writing to block devices can destroy data".to_string()),
            platforms: vec![Platform::Unix],
        },
        DangerousCommand {
            pattern: Regex::new(r"^dd\s+.*of=/dev/(sd[a-z]|hd[a-z]|nvme\d+).*").unwrap(),
            reason: DangerReason::Destructive("Writing directly to disk devices".to_string()),
            platforms: vec![Platform::Unix],
        },
        
        // Shred and secure delete
        DangerousCommand {
            pattern: Regex::new(r"^shred\s+.*").unwrap(),
            reason: DangerReason::Destructive("Securely overwrites files making them unrecoverable".to_string()),
            platforms: vec![Platform::Unix],
        },
        
        // ======= SYSTEM MODIFICATION =======
        
        // Partition manipulation
        DangerousCommand {
            pattern: Regex::new(r"^(fdisk|parted|gparted|gdisk)\s+.*").unwrap(),
            reason: DangerReason::SystemModification("Partition table modification can make system unbootable".to_string()),
            platforms: vec![Platform::Unix],
        },
        DangerousCommand {
            pattern: Regex::new(r"^diskpart\s*").unwrap(),
            reason: DangerReason::SystemModification("Windows disk partitioning tool".to_string()),
            platforms: vec![Platform::Windows],
        },
        
        // System file modifications
        DangerousCommand {
            pattern: Regex::new(r"^(chmod|chown|chgrp)\s+.*-R.*(/|/etc|/usr|/var|/boot).*").unwrap(),
            reason: DangerReason::SystemModification("Recursive permission changes on system directories".to_string()),
            platforms: vec![Platform::Unix],
        },
        DangerousCommand {
            pattern: Regex::new(r"^(chmod|chown)\s+.*(777|666|755).*(/|/etc|/usr|/var|/boot).*").unwrap(),
            reason: DangerReason::SystemModification("Changing permissions on system directories".to_string()),
            platforms: vec![Platform::Unix],
        },
        
        // Registry modifications (Windows)
        DangerousCommand {
            pattern: Regex::new(r"^reg\s+(add|delete|import)\s+.*").unwrap(),
            reason: DangerReason::SystemModification("Modifying Windows registry can break system".to_string()),
            platforms: vec![Platform::Windows],
        },
        DangerousCommand {
            pattern: Regex::new(r"^regedit\s+.*").unwrap(),
            reason: DangerReason::SystemModification("Direct registry editing".to_string()),
            platforms: vec![Platform::Windows],
        },
        
        // System service modifications
        DangerousCommand {
            pattern: Regex::new(r"^(systemctl|service)\s+(stop|disable|mask)\s+.*").unwrap(),
            reason: DangerReason::SystemModification("Disabling system services can break functionality".to_string()),
            platforms: vec![Platform::Unix],
        },
        DangerousCommand {
            pattern: Regex::new(r"^sc\s+(stop|delete|config)\s+.*").unwrap(),
            reason: DangerReason::SystemModification("Modifying Windows services".to_string()),
            platforms: vec![Platform::Windows],
        },
        
        // ======= EXECUTION RISKS =======
        
        // Download and execute
        DangerousCommand {
            pattern: Regex::new(r".*(curl|wget|invoke-webrequest).*\|\s*(sh|bash|zsh|fish|powershell|cmd).*").unwrap(),
            reason: DangerReason::Execution("Downloading and executing scripts from the internet".to_string()),
            platforms: vec![],
        },
        DangerousCommand {
            pattern: Regex::new(r".*(curl|wget).*-s.*\|\s*(sudo\s+)?(sh|bash).*").unwrap(),
            reason: DangerReason::Execution("Silent download and execution with potential sudo".to_string()),
            platforms: vec![Platform::Unix],
        },
        
        // Sudo with dangerous operations
        DangerousCommand {
            pattern: Regex::new(r"^sudo\s+(rm|dd|fdisk|parted|chmod|chown|systemctl).*").unwrap(),
            reason: DangerReason::Execution("Running dangerous commands with elevated privileges".to_string()),
            platforms: vec![Platform::Unix],
        },
        DangerousCommand {
            pattern: Regex::new(r"^sudo\s+.*>.*(/etc/|/boot/|/usr/).*").unwrap(),
            reason: DangerReason::Execution("Writing to system directories with sudo".to_string()),
            platforms: vec![Platform::Unix],
        },
        
        // Running scripts with elevated privileges
        DangerousCommand {
            pattern: Regex::new(r"^(runas|start-process).*-verb.*runas.*").unwrap(),
            reason: DangerReason::Execution("Running commands with elevated privileges on Windows".to_string()),
            platforms: vec![Platform::Windows],
        },
        
        // ======= SENSITIVE ACCESS =======
        
        // Reading sensitive files
        DangerousCommand {
            pattern: Regex::new(r"^(cat|less|more|head|tail|grep)\s+.*(passwd|shadow|sudoers|ssh.*key|\.pem|\.p12|\.pfx).*").unwrap(),
            reason: DangerReason::SensitiveAccess("Accessing sensitive authentication files".to_string()),
            platforms: vec![Platform::Unix],
        },
        DangerousCommand {
            pattern: Regex::new(r"^(type|more|get-content)\s+.*\.(pem|p12|pfx|key).*").unwrap(),
            reason: DangerReason::SensitiveAccess("Accessing certificate or key files on Windows".to_string()),
            platforms: vec![Platform::Windows],
        },
        
        // Environment dumping
        DangerousCommand {
            pattern: Regex::new(r"^(env|printenv|set)\s*$").unwrap(),
            reason: DangerReason::SensitiveAccess("Dumping environment variables may expose secrets".to_string()),
            platforms: vec![],
        },
        
        // ======= NETWORK RISKS =======
        
        // Opening network connections
        DangerousCommand {
            pattern: Regex::new(r"^(nc|netcat|ncat)\s+.*-[a-z]*l.*").unwrap(),
            reason: DangerReason::Network("Opening network listeners can expose system".to_string()),
            platforms: vec![Platform::Unix],
        },
        DangerousCommand {
            pattern: Regex::new(r"^netsh\s+(interface|firewall|wlan).*").unwrap(),
            reason: DangerReason::Network("Modifying Windows network or firewall settings".to_string()),
            platforms: vec![Platform::Windows],
        },
        
        // ======= CRYPTO/BLOCKCHAIN RISKS =======
        
        // Cryptocurrency operations
        DangerousCommand {
            pattern: Regex::new(r".*(bitcoin|ethereum|crypto|wallet|mining|miner).*send.*").unwrap(),
            reason: DangerReason::General("Cryptocurrency transactions can result in financial loss".to_string()),
            platforms: vec![],
        },
        
        // ======= GENERAL RISKY OPERATIONS =======
        
        // Kernel modules
        DangerousCommand {
            pattern: Regex::new(r"^(insmod|rmmod|modprobe)\s+.*").unwrap(),
            reason: DangerReason::SystemModification("Loading/unloading kernel modules can crash system".to_string()),
            platforms: vec![Platform::Linux],
        },
        
        // Memory/process manipulation
        DangerousCommand {
            pattern: Regex::new(r"^(kill|killall|pkill)\s+.*-9.*").unwrap(),
            reason: DangerReason::General("Force killing processes can cause data loss".to_string()),
            platforms: vec![Platform::Unix],
        },
        DangerousCommand {
            pattern: Regex::new(r"^taskkill\s+.*(/f|/t).*").unwrap(),
            reason: DangerReason::General("Force terminating processes on Windows".to_string()),
            platforms: vec![Platform::Windows],
        },
        
        // System shutdown/restart
        DangerousCommand {
            pattern: Regex::new(r"^(shutdown|reboot|halt|poweroff)\s+.*").unwrap(),
            reason: DangerReason::General("System shutdown commands".to_string()),
            platforms: vec![Platform::Unix],
        },
        DangerousCommand {
            pattern: Regex::new(r"^shutdown\s+(/s|/r|/h).*").unwrap(),
            reason: DangerReason::General("Windows shutdown/restart commands".to_string()),
            platforms: vec![Platform::Windows],
        },
        
        // Disk filling
        DangerousCommand {
            pattern: Regex::new(r"^dd\s+.*of=.*bs=.*count=.*").unwrap(),
            reason: DangerReason::General("Large dd operations can fill disk space".to_string()),
            platforms: vec![Platform::Unix],
        },
        
        // Fork bombs and resource exhaustion
        DangerousCommand {
            pattern: Regex::new(r".*:\(\)\{.*\|\&\};\:.*").unwrap(),
            reason: DangerReason::General("Fork bomb can exhaust system resources".to_string()),
            platforms: vec![Platform::Unix],
        },
        
        // Wildcard dangers
        DangerousCommand {
            pattern: Regex::new(r"^(rm|del|mv|cp)\s+.*\*.*\*.*").unwrap(),
            reason: DangerReason::Destructive("Multiple wildcards can match unintended files".to_string()),
            platforms: vec![],
        },
    ]
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dangerous_commands() {
        let config = CommandSafetyConfig::default();
        let checker = CommandSafetyChecker::new(config);

        // Test destructive commands
        assert!(matches!(
            checker.check_command("rm -rf /"),
            CommandSafety::Dangerous(_)
        ));
        assert!(matches!(
            checker.check_command("rm -rf $HOME"),
            CommandSafety::Dangerous(_)
        ));

        // Test safe commands
        assert!(matches!(
            checker.check_command("ls -la"),
            CommandSafety::Safe
        ));
        assert!(matches!(
            checker.check_command("echo hello"),
            CommandSafety::Safe
        ));
    }

    #[test]
    fn test_whitelist() {
        let mut config = CommandSafetyConfig::default();
        config.whitelist.insert("rm -rf".to_string());
        
        let checker = CommandSafetyChecker::new(config);
        
        // Should be whitelisted even though it's normally dangerous
        assert!(matches!(
            checker.check_command("rm -rf /tmp/test"),
            CommandSafety::Whitelisted
        ));
    }

    #[test]
    fn test_wildcard_matching() {
        assert!(wildcard_match("rm*", "rm -rf"));
        assert!(wildcard_match("rm*rf", "rm -rf"));
        assert!(wildcard_match("*rm*", "sudo rm -rf"));
        assert!(!wildcard_match("rm*", "mv file"));
    }

    #[test]
    fn test_platform_specific() {
        let config = CommandSafetyConfig::default();
        let checker = CommandSafetyChecker::new(config);

        // Windows-specific command
        if cfg!(target_os = "windows") {
            assert!(matches!(
                checker.check_command("del /s /q c:\\*"),
                CommandSafety::Dangerous(_)
            ));
        }

        // Unix-specific command
        if cfg!(unix) {
            assert!(matches!(
                checker.check_command("sudo rm -rf /"),
                CommandSafety::Dangerous(_)
            ));
        }
    }
}