use crate::ProcResult;
#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};
use std::io::BufRead;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
/// Container group controller information.
pub struct CGroupController {
    /// The name of the controller.
    pub name: String,
    /// The  unique  ID  of  the  cgroup hierarchy on which this controller is mounted.
    ///
    /// If multiple cgroups v1 controllers are bound to the same  hierarchy, then each will show
    /// the same hierarchy ID in this field.  The value in this field will be 0 if:
    ///
    /// * the controller is not mounted on a cgroups v1 hierarchy;
    /// * the controller is bound to the cgroups v2 single unified hierarchy; or
    /// * the controller is disabled (see below).
    pub hierarchy: u32,
    /// The number of control groups in this hierarchy using this controller.
    pub num_cgroups: u32,
    /// This field contains the value `true` if this controller is enabled, or `false` if it has been disabled
    pub enabled: bool,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
/// Container group controller information.
// This contains a vector, but if each subsystem name is unique, maybe this can be a
// hashmap instead
pub struct CGroupControllers(pub Vec<CGroupController>);

impl crate::FromBufRead for CGroupControllers {
    fn from_buf_read<R: BufRead>(reader: R) -> ProcResult<Self> {
        let mut vec = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if line.starts_with('#') {
                continue;
            }

            let mut s = line.split_whitespace();
            let name = expect!(s.next(), "name").to_owned();
            let hierarchy = from_str!(u32, expect!(s.next(), "hierarchy"));
            let num_cgroups = from_str!(u32, expect!(s.next(), "num_cgroups"));
            let enabled = expect!(s.next(), "enabled") == "1";

            vec.push(CGroupController {
                name,
                hierarchy,
                num_cgroups,
                enabled,
            });
        }

        Ok(CGroupControllers(vec))
    }
}

/// Information about a process cgroup
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct ProcessCGroup {
    /// For cgroups version 1 hierarchies, this field contains a  unique  hierarchy  ID  number
    /// that  can  be  matched  to  a  hierarchy  ID  in /proc/cgroups.  For the cgroups version 2
    /// hierarchy, this field contains the value 0.
    pub hierarchy: u32,
    /// For cgroups version 1 hierarchies, this field contains a comma-separated list of the
    /// controllers bound to the hierarchy.
    ///
    /// For the cgroups version 2 hierarchy, this field is empty.
    pub controllers: Vec<String>,

    /// This field contains the pathname of the control group in the hierarchy to which the process
    /// belongs.
    ///
    /// This pathname is  relative  to  the mount point of the hierarchy.
    pub pathname: String,
}

/// Information about process cgroups.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct ProcessCGroups(pub Vec<ProcessCGroup>);

impl crate::FromBufRead for ProcessCGroups {
    fn from_buf_read<R: BufRead>(reader: R) -> ProcResult<Self> {
        let mut vec = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if line.starts_with('#') {
                continue;
            }

            let mut s = line.splitn(3, ':');
            let hierarchy = from_str!(u32, expect!(s.next(), "hierarchy"));
            let controllers = expect!(s.next(), "controllers")
                .split(',')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_owned())
                .collect();
            let pathname = expect!(s.next(), "path").to_owned();

            vec.push(ProcessCGroup {
                hierarchy,
                controllers,
                pathname,
            });
        }

        Ok(ProcessCGroups(vec))
    }
}

impl IntoIterator for ProcessCGroups {
    type IntoIter = std::vec::IntoIter<ProcessCGroup>;
    type Item = ProcessCGroup;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a ProcessCGroups {
    type IntoIter = std::slice::Iter<'a, ProcessCGroup>;
    type Item = &'a ProcessCGroup;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
