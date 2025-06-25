# Remote Crate - Dependency Context and Architecture

## Overview

The `remote` crate is a critical component of Zed's remote development functionality, providing SSH-based remote connection capabilities. It is currently tightly coupled to SSH as the transport protocol, with the main abstraction being the `RemoteConnection` trait that could potentially support multiple transport mechanisms.

## Current Architecture

### Core Components

1. **SshRemoteClient** (`ssh_session.rs`)
   - Main public interface for remote connections
   - Manages connection state machine (Connecting, Connected, HeartbeatMissed, Reconnecting, Disconnected)
   - Handles reconnection logic with heartbeat monitoring
   - Provides RPC client for communication with remote server

2. **RemoteConnection Trait** (`ssh_session.rs`)
   - Currently internal trait (not public)
   - Defines interface for transport implementations
   - Methods: `start_proxy()`, `upload_directory()`, `kill()`, `has_been_killed()`, `ssh_args()`, `connection_options()`
   - Only implementation: `SshRemoteConnection`

3. **Protocol Module** (`protocol.rs`)
   - Wire protocol for message framing
   - Uses protobuf (via `rpc` crate) for message serialization
   - Handles reading/writing messages with length prefixes

4. **Connection Options** (`ssh_session.rs`)
   - `SshConnectionOptions`: Configuration for SSH connections
   - Includes host, username, port, password, args, port forwards, nickname
   - Can parse SSH command line format

## Dependencies from Other Crates

### Direct Cargo Dependencies
- **remote_server**: Uses remote for the server-side daemon
- **collab**: Uses remote in tests for remote collaboration scenarios

### Source Code Dependencies

1. **workspace** (Primary consumer)
   - Creates SSH connections via `open_ssh_project_with_new_connection()`
   - Manages SSH project lifecycle
   - Uses: `SshClientDelegate`, `SshConnectionOptions`, `ConnectionIdentifier`, `SshRemoteClient`

2. **project**
   - Stores optional `ssh_client: Option<Entity<SshRemoteClient>>`
   - Creates projects with SSH support via `Project::ssh()`
   - Manages remote project state and debugging

3. **recent_projects**
   - UI for SSH connection management
   - Connection history and persistence
   - SSH connection dialog and status display

4. **title_bar**
   - Displays connection status using `ConnectionState` enum

5. **extension_host**
   - Provides SSH client access to extensions

6. **zed** (main app)
   - Opens remote files/projects via open listener

## Key Public APIs

### Exported Types (from `remote.rs`)
```rust
pub use ssh_session::{
    ConnectionState,
    SshClientDelegate,
    SshConnectionOptions,
    SshPlatform,
    SshRemoteClient,
    SshRemoteEvent,
};
```

### SshClientDelegate Trait
Required by consumers to handle SSH events:
- `ask_password()`: UI prompt for passwords
- `get_download_params()`: Server binary download parameters
- `download_server_binary_locally()`: Download server binary
- `set_status()`: Update connection status in UI

### Connection States
- **Connecting**: Initial connection attempt
- **Connected**: Active connection with running heartbeat
- **HeartbeatMissed**: Temporary network issue detected
- **Reconnecting**: Attempting to restore connection
- **Disconnected**: Connection terminated

## Current SSH Implementation Details

1. **Master Socket**: Uses SSH ControlMaster for connection multiplexing
2. **Heartbeat**: 5-second interval with 5 missed heartbeats before reconnect
3. **Reconnection**: Up to 3 automatic reconnection attempts
4. **Binary Upload**: Can upload server binary over SSH or download from URL
5. **Port Forwarding**: Supports SSH -L style port forwards

## Integration Pattern

The typical flow for creating a remote connection:

1. **Workspace** creates `SshConnectionOptions` from user input
2. **Workspace** provides an `SshClientDelegate` implementation
3. **Workspace** calls into **Project** to create SSH-enabled project
4. **Project** creates `SshRemoteClient` internally
5. **SshRemoteClient** manages the actual SSH connection via `SshRemoteConnection`
6. RPC messages flow through the SSH tunnel to the remote server

## Constraints for Transport Adapter Pattern

1. **Async Trait Methods**: RemoteConnection uses `async_trait` without Send
2. **RPC Integration**: Must provide byte stream for RPC protocol
3. **Binary Upload**: Need mechanism to transfer server binary
4. **Reconnection**: Must support connection monitoring and recovery
5. **Platform Support**: Currently supports Linux and macOS remotes
6. **Authentication**: Must handle interactive authentication flows

## Areas Requiring Refactoring for Multi-Transport

1. **SSH-Specific Naming**: Many types and methods have "ssh" in the name
2. **Connection Options**: Currently SSH-specific, needs generalization
3. **RemoteConnection Trait**: Currently private, needs to be public
4. **Binary Transfer**: Assumes SSH/SCP for file transfer
5. **Connection String Format**: Assumes SSH URL format
6. **Shell Commands**: Assumes SSH command execution model

---

## Transport Adapter Pattern Implementation Plan

### Executive Summary

The plan introduces a transport-agnostic architecture by:
1. Creating a public `Transport` trait that generalizes the current `RemoteConnection` trait
2. Renaming SSH-specific types to be transport-agnostic  
3. Introducing a `TransportConfig` enum to support multiple connection types
4. Maintaining backward compatibility with existing SSH functionality

### Core Architecture

#### 1. **Core Transport Abstraction**

```rust
// New public trait in transport.rs
#[async_trait(?Send)]
pub trait Transport: Send + Sync {
    // Establish connection and start remote server
    async fn connect(
        &self,
        config: &TransportConfig,
        delegate: Arc<dyn TransportDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<Box<dyn TransportConnection>>;
    
    // Get human-readable name for UI
    fn name(&self) -> &'static str;
    
    // Check if this transport can handle the given config
    fn supports_config(&self, config: &TransportConfig) -> bool;
}

#[async_trait(?Send)]
pub trait TransportConnection: Send + Sync {
    // Start the proxy process that bridges RPC messages
    fn start_proxy(
        &self,
        unique_identifier: String,
        reconnect: bool,
        incoming_tx: UnboundedSender<Envelope>,
        outgoing_rx: UnboundedReceiver<Envelope>,
        connection_activity_tx: Sender<()>,
        cx: &mut AsyncApp,
    ) -> Task<Result<i32>>;
    
    // Upload files/directories to remote
    fn upload_directory(&self, src: PathBuf, dest: PathBuf, cx: &App) -> Task<Result<()>>;
    
    // Terminate connection
    async fn kill(&self) -> Result<()>;
    
    // Check if connection is terminated
    fn has_been_killed(&self) -> bool;
    
    // Get connection-specific arguments (for backwards compat)
    fn connection_args(&self) -> Vec<String>;
    
    // Get original connection config
    fn connection_config(&self) -> TransportConfig;
}
```

#### 2. **Configuration System**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransportConfig {
    Ssh(SshConnectionOptions),
    Mosh(MoshConnectionOptions),      // Future
    Docker(DockerConnectionOptions),   // Future
    Kubernetes(K8sConnectionOptions),  // Future
    Custom(CustomTransportConfig),     // Extensibility
}

// Rename SshConnectionOptions -> keep as-is for compatibility
// but make it clear it's SSH-specific

pub trait TransportDelegate: Send + Sync {
    // Generalized from SshClientDelegate
    fn ask_password(&self, prompt: String, tx: oneshot::Sender<String>, cx: &mut AsyncApp);
    fn get_download_params(&self, platform: RemotePlatform, ...) -> Task<Result<Option<(String, String)>>>;
    fn download_server_binary_locally(&self, platform: RemotePlatform, ...) -> Task<Result<PathBuf>>;
    fn set_status(&self, status: Option<&str>, cx: &mut AsyncApp);
}
```

#### 3. **Transport Registry**

```rust
pub struct TransportRegistry {
    transports: HashMap<&'static str, Arc<dyn Transport>>,
}

impl TransportRegistry {
    pub fn register(&mut self, transport: Arc<dyn Transport>) {
        self.transports.insert(transport.name(), transport);
    }
    
    pub fn create_connection(&self, config: TransportConfig, ...) -> Result<...> {
        // Find appropriate transport and create connection
    }
}
```

### Implementation Strategy

#### Phase 1: Internal Refactoring (Non-breaking)
- Extract `RemoteConnection` trait to public `transport` module as `TransportConnection`
- Create `SshTransport` implementing the new `Transport` trait
- Keep `SshRemoteClient` as the public API but internally use the transport system
- Add transport registry for runtime registration

#### Phase 2: API Migration (Deprecation warnings)
- Rename `SshRemoteClient` → `RemoteClient` 
- Rename `SshClientDelegate` → `RemoteClientDelegate`
- Keep old names as deprecated type aliases
- Update connection state to be transport-agnostic
- Use feature flags for new transports (e.g., `MoshTransportFeatureFlag`)

#### Phase 3: Full Transport Support
- Add transport selection UI
- Implement additional transports
- Remove deprecated APIs

### External Binary Architecture

**Critical Context**: Zed relies on external binaries (ssh, mosh, etc.) rather than implementing protocols internally. This design choice affects how transports are implemented.

#### SSH Transport (Current)
- Uses external `ssh` binary with ControlMaster for multiplexing
- Passes commands via shell execution
- Handles authentication via askpass helper

#### Mosh Transport Implementation Considerations

```rust
struct MoshTransport;

impl Transport for MoshTransport {
    async fn connect(&self, config: &TransportConfig, ...) -> Result<Box<dyn TransportConnection>> {
        let TransportConfig::Mosh(mosh_config) = config else {
            return Err(anyhow!("Invalid config"));
        };
        
        // Key differences from SSH:
        // 1. Mosh requires initial SSH connection to exchange keys
        // 2. Uses UDP for the actual transport
        // 3. Requires port range negotiation
        // 4. Connection state is more complex (prediction/synchronization)
        
        // Initial SSH connection to set up mosh
        let ssh_output = Command::new("ssh")
            .args(["-o", "ControlPath=none"])  // Don't use ControlMaster
            .arg(format!("{}@{}", mosh_config.user, mosh_config.host))
            .arg("mosh-server new -p 60000:61000")  // Port range
            .output()
            .await?;
            
        // Parse mosh connection info from output
        // MOSH CONNECT <port> <key>
        let connection_info = parse_mosh_server_output(&ssh_output.stdout)?;
        
        // Start mosh-client with connection info
        let mut mosh_client = Command::new("mosh-client");
        mosh_client
            .arg(connection_info.key)
            .arg(format!("{}:{}", mosh_config.host, connection_info.port));
            
        Ok(Box::new(MoshConnection { 
            client_process: mosh_client.spawn()?,
            connection_info,
            ...
        }))
    }
}
```

**Mosh-Specific Challenges:**
1. **Two-Phase Connection**: Initial SSH to start mosh-server, then UDP connection
2. **Key Exchange**: Must parse and handle mosh connection keys
3. **UDP NAT Traversal**: May require firewall configuration
4. **State Synchronization**: Mosh's predictive typing requires special handling
5. **Binary Transfer**: Still needs SSH for initial server binary upload

### Testing Strategy Updates

The existing test-support infrastructure should be updated to support the adapter pattern:

```rust
// Update existing test support in ssh_session.rs
#[cfg(any(test, feature = "test-support"))]
pub mod test_support {
    use super::*;
    
    pub struct FakeTransport {
        pub name: &'static str,
    }
    
    impl Transport for FakeTransport {
        // Mock implementation
    }
    
    // Update FakeRemoteConnection to implement TransportConnection
    impl TransportConnection for FakeRemoteConnection {
        // Existing implementation adapted
    }
}
```

Integration with existing test patterns:
- Maintain `simulate_disconnect()` functionality
- Support fake server creation for all transport types
- Enable transport-specific test scenarios

---

## Future Enhancements

### 1. Extension API for Transports

While initially implementing transports as built-in components, a future extension API could allow third-party transports:

```rust
// Potential extension manifest addition
{
    "id": "custom-transport",
    "name": "Custom Transport Provider",
    "version": "0.1.0",
    "transport": {
        "protocol": "custom",
        "url_scheme": "custom://",
        "binary_requirements": ["custom-client"]
    }
}
```

**Considerations:**
- Security implications of allowing arbitrary process execution
- Need for sandboxing and permission system
- API stability guarantees
- Authentication flow integration

**Recommendation**: Implement core transports (SSH, Mosh, Docker) as built-in first, then evaluate extension API based on community needs and security model.

### 2. Configuration Format & URL Schemes

In addition to the extension API, the configuration system could support multiple URL scheme formats:

**Protocol-specific URL schemes** (Recommended approach):
```
ssh://user@host:22/path
mosh://user@host:60001/path  
docker://container-name/path
k8s://context/namespace/pod/path
```

**Benefits:**
- Each transport owns its URL parsing logic
- Familiar patterns for developers
- Protocol-specific parameters in URL (e.g., mosh prediction modes)

**Implementation:**
- Transport registry includes URL scheme registration
- Transports implement `parse_url(&str) -> Result<TransportConfig>`
- UI can show protocol-specific connection dialogs

### 3. Enhanced Testing Framework

Future testing enhancements could include:

```rust
// Transport-agnostic test harness
pub struct TransportTestHarness {
    client_cx: TestAppContext,
    server_cx: TestAppContext,
    transport: Box<dyn Transport>,
}

impl TransportTestHarness {
    pub fn new_with_transport(transport: Box<dyn Transport>) -> Self { ... }
    
    pub async fn test_connection_lifecycle(&mut self) { ... }
    pub async fn test_reconnection(&mut self) { ... }
    pub async fn test_binary_transfer(&mut self) { ... }
}

// Property-based testing for transports
#[quickcheck]
fn transport_message_ordering(messages: Vec<TestMessage>) -> bool {
    // Verify message ordering is preserved across all transports
}
```

**Benefits:**
- Ensure consistent behavior across transports
- Catch transport-specific edge cases
- Performance benchmarking framework