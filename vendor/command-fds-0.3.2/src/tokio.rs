use crate::{
    CommandFdExt, FdMapping, FdMappingCollision, map_fds, preserve_fds, validate_child_fds,
};
use std::os::fd::OwnedFd;
use tokio::process::Command;

impl CommandFdExt for Command {
    fn fd_mappings(
        &mut self,
        mut mappings: Vec<FdMapping>,
    ) -> Result<&mut Self, FdMappingCollision> {
        let child_fds = validate_child_fds(&mappings)?;

        unsafe {
            self.pre_exec(move || map_fds(&mut mappings, &child_fds));
        }

        Ok(self)
    }

    fn preserved_fds(&mut self, fds: Vec<OwnedFd>) -> &mut Self {
        unsafe {
            self.pre_exec(move || preserve_fds(&fds));
        }

        self
    }
}
