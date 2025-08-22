use std::{path::Path, rc::Rc};

use crate::AgentServerCommand;
use acp_thread::AgentConnection;
use anyhow::Result;
use gpui::AsyncApp;
use thiserror::Error;

mod v0;
mod v1;

#[derive(Debug, Error)]
#[error("Unsupported version")]
pub struct UnsupportedVersion;

pub async fn connect(
    server_name: &'static str,
    command: AgentServerCommand,
    root_dir: &Path,
    cx: &mut AsyncApp,
) -> Result<Rc<dyn AgentConnection>> {
    let conn = v1::AcpConnection::stdio(server_name, command.clone(), root_dir, cx).await;

    match conn {
        Ok(conn) => Ok(Rc::new(conn) as _),
        Err(err) if err.is::<UnsupportedVersion>() => {
            // Consider re-using initialize response and subprocess when adding another version here
            let conn: Rc<dyn AgentConnection> =
                Rc::new(v0::AcpConnection::stdio(server_name, command, root_dir, cx).await?);
            Ok(conn)
        }
        Err(err) => Err(err),
    }
}
