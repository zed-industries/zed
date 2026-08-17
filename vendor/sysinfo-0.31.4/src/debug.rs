// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(feature = "system")]
impl std::fmt::Debug for crate::Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cpu")
            .field("name", &self.name())
            .field("CPU usage", &self.cpu_usage())
            .field("frequency", &self.frequency())
            .field("vendor ID", &self.vendor_id())
            .field("brand", &self.brand())
            .finish()
    }
}

#[cfg(feature = "system")]
impl std::fmt::Debug for crate::System {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("System")
            .field("global CPU usage", &self.global_cpu_usage())
            .field("load average", &Self::load_average())
            .field("total memory", &self.total_memory())
            .field("free memory", &self.free_memory())
            .field("total swap", &self.total_swap())
            .field("free swap", &self.free_swap())
            .field("nb CPUs", &self.cpus().len())
            .field("nb processes", &self.processes().len())
            .finish()
    }
}

#[cfg(feature = "system")]
impl std::fmt::Debug for crate::Process {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Process")
            .field("pid", &self.pid())
            .field("parent", &self.parent())
            .field("name", &self.name())
            .field("environ", &self.environ())
            .field("command", &self.cmd())
            .field("executable path", &self.exe())
            .field("current working directory", &self.cwd())
            .field("memory usage", &self.memory())
            .field("virtual memory usage", &self.virtual_memory())
            .field("CPU usage", &self.cpu_usage())
            .field("status", &self.status())
            .field("root", &self.root())
            .field("disk_usage", &self.disk_usage())
            .field("user_id", &self.user_id())
            .field("effective_user_id", &self.effective_user_id())
            .finish()
    }
}

#[cfg(feature = "disk")]
impl std::fmt::Debug for crate::Disk {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            fmt,
            "Disk({:?})[FS: {:?}][Type: {:?}][removable: {}] mounted on {:?}: {}/{} B",
            self.name(),
            self.file_system(),
            self.kind(),
            if self.is_removable() { "yes" } else { "no" },
            self.mount_point(),
            self.available_space(),
            self.total_space(),
        )
    }
}

#[cfg(feature = "disk")]
impl std::fmt::Debug for crate::Disks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

#[cfg(feature = "component")]
impl std::fmt::Debug for crate::Components {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

#[cfg(feature = "component")]
impl std::fmt::Debug for crate::Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(critical) = self.critical() {
            write!(
                f,
                "{}: {}°C (max: {}°C / critical: {}°C)",
                self.label(),
                self.temperature(),
                self.max(),
                critical
            )
        } else {
            write!(
                f,
                "{}: {}°C (max: {}°C)",
                self.label(),
                self.temperature(),
                self.max()
            )
        }
    }
}

#[cfg(feature = "network")]
impl std::fmt::Debug for crate::Networks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

#[cfg(feature = "network")]
impl std::fmt::Debug for crate::NetworkData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NetworkData")
            .field("income", &self.received())
            .field("total income", &self.total_received())
            .field("outcome", &self.transmitted())
            .field("total outcome", &self.total_transmitted())
            .field("packets income", &self.packets_received())
            .field("total packets income", &self.total_packets_received())
            .field("packets outcome", &self.packets_transmitted())
            .field("total packets outcome", &self.total_packets_transmitted())
            .field("errors income", &self.errors_on_received())
            .field("total errors income", &self.total_errors_on_received())
            .field("errors outcome", &self.errors_on_transmitted())
            .field("total errors outcome", &self.total_errors_on_transmitted())
            .finish()
    }
}

#[cfg(feature = "user")]
impl std::fmt::Debug for crate::Users {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

#[cfg(feature = "user")]
impl std::fmt::Debug for crate::User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("uid", &self.id())
            .field("gid", &self.group_id())
            .field("name", &self.name())
            .finish()
    }
}
