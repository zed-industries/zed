use anyhow::{Context as _, Result};
pub use settings::SshPortForwardOption;
use std::net::IpAddr;
use util::shell::ShellKind;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SshConnectionHost {
    IpAddr(IpAddr),
    Hostname(String),
}

impl SshConnectionHost {
    pub fn to_bracketed_string(&self) -> String {
        match self {
            Self::IpAddr(IpAddr::V4(ip)) => ip.to_string(),
            Self::IpAddr(IpAddr::V6(ip)) => format!("[{}]", ip),
            Self::Hostname(hostname) => hostname.clone(),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::IpAddr(ip) => ip.to_string(),
            Self::Hostname(hostname) => hostname.clone(),
        }
    }
}

impl From<&str> for SshConnectionHost {
    fn from(value: &str) -> Self {
        if let Ok(address) = value.parse() {
            Self::IpAddr(address)
        } else {
            Self::Hostname(value.to_string())
        }
    }
}

impl From<String> for SshConnectionHost {
    fn from(value: String) -> Self {
        if let Ok(address) = value.parse() {
            Self::IpAddr(address)
        } else {
            Self::Hostname(value)
        }
    }
}

impl Default for SshConnectionHost {
    fn default() -> Self {
        Self::Hostname(Default::default())
    }
}

pub(crate) fn bracket_ipv6(host: &str) -> String {
    if host.contains(':') && !host.starts_with('[') {
        format!("[{}]", host)
    } else {
        host.to_string()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct SshConnectionOptions {
    pub host: SshConnectionHost,
    pub username: Option<String>,
    pub port: Option<u16>,
    pub password: Option<String>,
    pub args: Option<Vec<String>>,
    pub port_forwards: Option<Vec<SshPortForwardOption>>,
    pub connection_timeout: Option<u16>,

    pub nickname: Option<String>,
    pub upload_binary_over_ssh: bool,
}

impl From<settings::SshConnection> for SshConnectionOptions {
    fn from(val: settings::SshConnection) -> Self {
        SshConnectionOptions {
            host: val.host.to_string().into(),
            username: val.username,
            port: val.port,
            password: None,
            args: Some(val.args),
            nickname: val.nickname,
            upload_binary_over_ssh: val.upload_binary_over_ssh.unwrap_or_default(),
            port_forwards: val.port_forwards,
            connection_timeout: val.connection_timeout,
        }
    }
}

fn parse_port_number(port_str: &str) -> Result<u16> {
    port_str
        .parse()
        .with_context(|| format!("parsing port number: {port_str}"))
}

fn split_port_forward_tokens(spec: &str) -> Result<Vec<String>> {
    let mut tokens = Vec::new();
    let mut chars = spec.chars().peekable();

    while chars.peek().is_some() {
        if chars.peek() == Some(&'[') {
            chars.next();
            let mut bracket_content = String::new();
            loop {
                match chars.next() {
                    Some(']') => break,
                    Some(ch) => bracket_content.push(ch),
                    None => anyhow::bail!("Unmatched '[' in port forward spec: {spec}"),
                }
            }
            tokens.push(bracket_content);
            if chars.peek() == Some(&':') {
                chars.next();
            }
        } else {
            let mut token = String::new();
            for ch in chars.by_ref() {
                if ch == ':' {
                    break;
                }
                token.push(ch);
            }
            tokens.push(token);
        }
    }

    Ok(tokens)
}

fn parse_port_forward_spec(spec: &str) -> Result<SshPortForwardOption> {
    let tokens = if spec.contains('[') {
        split_port_forward_tokens(spec)?
    } else {
        spec.split(':').map(String::from).collect()
    };

    match tokens.len() {
        4 => {
            let local_port = parse_port_number(&tokens[1])?;
            let remote_port = parse_port_number(&tokens[3])?;

            Ok(SshPortForwardOption {
                local_host: Some(tokens[0].clone()),
                local_port,
                remote_host: Some(tokens[2].clone()),
                remote_port,
            })
        }
        3 => {
            let local_port = parse_port_number(&tokens[0])?;
            let remote_port = parse_port_number(&tokens[2])?;

            Ok(SshPortForwardOption {
                local_host: None,
                local_port,
                remote_host: Some(tokens[1].clone()),
                remote_port,
            })
        }
        _ => anyhow::bail!("Invalid port forward format: {spec}"),
    }
}

impl SshConnectionOptions {
    pub fn parse_command_line(input: &str) -> Result<Self> {
        let input = input.trim_start_matches("ssh ");
        let mut hostname: Option<String> = None;
        let mut username: Option<String> = None;
        let mut port: Option<u16> = None;
        let mut args = Vec::new();
        let mut port_forwards: Vec<SshPortForwardOption> = Vec::new();

        // disallowed: -E, -e, -F, -f, -G, -g, -M, -N, -n, -O, -q, -S, -s, -T, -t, -V, -v, -W
        const ALLOWED_OPTS: &[&str] = &[
            "-4", "-6", "-A", "-a", "-C", "-K", "-k", "-X", "-x", "-Y", "-y",
        ];
        const ALLOWED_ARGS: &[&str] = &[
            "-B", "-b", "-c", "-D", "-F", "-I", "-i", "-J", "-l", "-m", "-o", "-P", "-p", "-R",
            "-w",
        ];

        let mut tokens = ShellKind::Posix
            .split(input)
            .context("invalid input")?
            .into_iter();

        'outer: while let Some(arg) = tokens.next() {
            if ALLOWED_OPTS.contains(&(&arg as &str)) {
                args.push(arg.to_string());
                continue;
            }
            if arg == "-p" {
                port = tokens.next().and_then(|arg| arg.parse().ok());
                continue;
            } else if let Some(p) = arg.strip_prefix("-p") {
                port = p.parse().ok();
                continue;
            }
            if arg == "-l" {
                username = tokens.next();
                continue;
            } else if let Some(l) = arg.strip_prefix("-l") {
                username = Some(l.to_string());
                continue;
            }
            if arg == "-L" || arg.starts_with("-L") {
                let forward_spec = if arg == "-L" {
                    tokens.next()
                } else {
                    Some(arg.strip_prefix("-L").unwrap().to_string())
                };

                if let Some(spec) = forward_spec {
                    port_forwards.push(parse_port_forward_spec(&spec)?);
                } else {
                    anyhow::bail!("Missing port forward format");
                }
            }

            for a in ALLOWED_ARGS {
                if arg == *a {
                    args.push(arg);
                    if let Some(next) = tokens.next() {
                        args.push(next);
                    }
                    continue 'outer;
                } else if arg.starts_with(a) {
                    args.push(arg);
                    continue 'outer;
                }
            }
            if arg.starts_with("-") || hostname.is_some() {
                anyhow::bail!("unsupported argument: {:?}", arg);
            }
            let mut input = &arg as &str;
            // Destination might be: username1@username2@ip2@ip1
            if let Some((u, rest)) = input.rsplit_once('@') {
                input = rest;
                username = Some(u.to_string());
            }

            // Handle port parsing, accounting for IPv6 addresses
            // IPv6 addresses can be: 2001:db8::1 or [2001:db8::1]:22
            if input.starts_with('[') {
                if let Some((rest, p)) = input.rsplit_once("]:") {
                    input = rest.strip_prefix('[').unwrap_or(rest);
                    port = p.parse().ok();
                } else if input.ends_with(']') {
                    input = input.strip_prefix('[').unwrap_or(input);
                    input = input.strip_suffix(']').unwrap_or(input);
                }
            } else if let Some((rest, p)) = input.rsplit_once(':')
                && !rest.contains(":")
            {
                input = rest;
                port = p.parse().ok();
            }

            hostname = Some(input.to_string())
        }

        let Some(hostname) = hostname else {
            anyhow::bail!("missing hostname");
        };

        let port_forwards = match port_forwards.len() {
            0 => None,
            _ => Some(port_forwards),
        };

        Ok(Self {
            host: hostname.into(),
            username,
            port,
            port_forwards,
            args: Some(args),
            password: None,
            nickname: None,
            upload_binary_over_ssh: false,
            connection_timeout: None,
        })
    }

    pub fn ssh_destination(&self) -> String {
        let mut result = String::default();
        if let Some(username) = &self.username {
            // Username might be: username1@username2@ip2
            let username = urlencoding::encode(username);
            result.push_str(&username);
            result.push('@');
        }

        result.push_str(&self.host.to_string());
        result
    }

    pub fn additional_args_for_scp(&self) -> Vec<String> {
        self.args.iter().flatten().cloned().collect::<Vec<String>>()
    }

    pub fn additional_args(&self) -> Vec<String> {
        let mut args = self.additional_args_for_scp();

        if let Some(timeout) = self.connection_timeout {
            args.extend(["-o".to_string(), format!("ConnectTimeout={}", timeout)]);
        }

        if let Some(port) = self.port {
            args.push("-p".to_string());
            args.push(port.to_string());
        }

        if let Some(forwards) = &self.port_forwards {
            args.extend(forwards.iter().map(|pf| {
                let local_host = match &pf.local_host {
                    Some(host) => host,
                    None => "localhost",
                };
                let remote_host = match &pf.remote_host {
                    Some(host) => host,
                    None => "localhost",
                };

                format!(
                    "-L{}:{}:{}:{}",
                    bracket_ipv6(local_host),
                    pf.local_port,
                    bracket_ipv6(remote_host),
                    pf.remote_port
                )
            }));
        }

        args
    }

    pub fn connection_string(&self) -> String {
        let host = if let Some(port) = &self.port {
            format!("{}:{}", self.host.to_bracketed_string(), port)
        } else {
            self.host.to_string()
        };

        if let Some(username) = &self.username {
            format!("{}@{}", username, host)
        } else {
            host
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scp_args_exclude_port_forward_flags() {
        let options = SshConnectionOptions {
            host: "example.com".into(),
            args: Some(vec![
                "-p".to_string(),
                "2222".to_string(),
                "-o".to_string(),
                "StrictHostKeyChecking=no".to_string(),
            ]),
            port_forwards: Some(vec![SshPortForwardOption {
                local_host: Some("127.0.0.1".to_string()),
                local_port: 8080,
                remote_host: Some("127.0.0.1".to_string()),
                remote_port: 80,
            }]),
            ..Default::default()
        };

        let ssh_args = options.additional_args();
        assert!(
            ssh_args.iter().any(|arg| arg.starts_with("-L")),
            "expected ssh args to include port-forward: {ssh_args:?}"
        );

        let scp_args = options.additional_args_for_scp();
        assert_eq!(
            scp_args,
            vec![
                "-p".to_string(),
                "2222".to_string(),
                "-o".to_string(),
                "StrictHostKeyChecking=no".to_string(),
            ]
        );
    }

    #[test]
    fn test_host_parsing() -> Result<()> {
        let opts = SshConnectionOptions::parse_command_line("user@2001:db8::1")?;
        assert_eq!(opts.host, "2001:db8::1".into());
        assert_eq!(opts.username, Some("user".to_string()));
        assert_eq!(opts.port, None);

        let opts = SshConnectionOptions::parse_command_line("user@[2001:db8::1]:2222")?;
        assert_eq!(opts.host, "2001:db8::1".into());
        assert_eq!(opts.username, Some("user".to_string()));
        assert_eq!(opts.port, Some(2222));

        let opts = SshConnectionOptions::parse_command_line("user@[2001:db8::1]")?;
        assert_eq!(opts.host, "2001:db8::1".into());
        assert_eq!(opts.username, Some("user".to_string()));
        assert_eq!(opts.port, None);

        let opts = SshConnectionOptions::parse_command_line("2001:db8::1")?;
        assert_eq!(opts.host, "2001:db8::1".into());
        assert_eq!(opts.username, None);
        assert_eq!(opts.port, None);

        let opts = SshConnectionOptions::parse_command_line("[2001:db8::1]:2222")?;
        assert_eq!(opts.host, "2001:db8::1".into());
        assert_eq!(opts.username, None);
        assert_eq!(opts.port, Some(2222));

        let opts = SshConnectionOptions::parse_command_line("user@example.com:2222")?;
        assert_eq!(opts.host, "example.com".into());
        assert_eq!(opts.username, Some("user".to_string()));
        assert_eq!(opts.port, Some(2222));

        let opts = SshConnectionOptions::parse_command_line("user@192.168.1.1:2222")?;
        assert_eq!(opts.host, "192.168.1.1".into());
        assert_eq!(opts.username, Some("user".to_string()));
        assert_eq!(opts.port, Some(2222));

        Ok(())
    }

    #[test]
    fn test_parse_port_forward_spec_ipv6() -> Result<()> {
        let pf = parse_port_forward_spec("[::1]:8080:[::1]:80")?;
        assert_eq!(pf.local_host, Some("::1".to_string()));
        assert_eq!(pf.local_port, 8080);
        assert_eq!(pf.remote_host, Some("::1".to_string()));
        assert_eq!(pf.remote_port, 80);

        let pf = parse_port_forward_spec("8080:[::1]:80")?;
        assert_eq!(pf.local_host, None);
        assert_eq!(pf.local_port, 8080);
        assert_eq!(pf.remote_host, Some("::1".to_string()));
        assert_eq!(pf.remote_port, 80);

        let pf = parse_port_forward_spec("[2001:db8::1]:3000:[fe80::1]:4000")?;
        assert_eq!(pf.local_host, Some("2001:db8::1".to_string()));
        assert_eq!(pf.local_port, 3000);
        assert_eq!(pf.remote_host, Some("fe80::1".to_string()));
        assert_eq!(pf.remote_port, 4000);

        let pf = parse_port_forward_spec("127.0.0.1:8080:localhost:80")?;
        assert_eq!(pf.local_host, Some("127.0.0.1".to_string()));
        assert_eq!(pf.local_port, 8080);
        assert_eq!(pf.remote_host, Some("localhost".to_string()));
        assert_eq!(pf.remote_port, 80);

        Ok(())
    }

    #[test]
    fn test_port_forward_ipv6_formatting() {
        let options = SshConnectionOptions {
            host: "example.com".into(),
            port_forwards: Some(vec![SshPortForwardOption {
                local_host: Some("::1".to_string()),
                local_port: 8080,
                remote_host: Some("::1".to_string()),
                remote_port: 80,
            }]),
            ..Default::default()
        };

        let args = options.additional_args();
        assert!(
            args.iter().any(|arg| arg == "-L[::1]:8080:[::1]:80"),
            "expected bracketed IPv6 in -L flag: {args:?}"
        );
    }
}
