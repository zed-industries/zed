use std::collections::BTreeSet;

const FILTERED_GIT_PROVIDER_HOSTS: &[&str] = &[
    "dev.azure.com",
    "bitbucket",
    "bitbucket.org",
    "chromium.googlesource.com",
    "codeberg",
    "codeberg.org",
    "gitea",
    "gitea.com",
    "gitee",
    "gitee.com",
    "github",
    "github.com",
    "gitlab",
    "gitlab.com",
    "sourcehut",
    "sourcehut.org",
    "git.sr.ht",
];

pub fn parse_ssh_config_hosts(config: &str) -> BTreeSet<String> {
    let mut hosts = BTreeSet::new();
    let mut needs_another_line = false;
    for line in config.lines() {
        let line = line.trim_start();
        if let Some(line) = line.strip_prefix("Host") {
            match line.chars().next() {
                Some('\\') => {
                    needs_another_line = true;
                }
                Some('\n' | '\r') => {
                    needs_another_line = false;
                }
                Some(c) if c.is_whitespace() => {
                    parse_hosts_from(line, &mut hosts);
                }
                Some(_) | None => {
                    needs_another_line = false;
                }
            };

            if needs_another_line {
                parse_hosts_from(line, &mut hosts);
                needs_another_line = line.trim_end().ends_with('\\');
            } else {
                needs_another_line = false;
            }
        } else if needs_another_line {
            needs_another_line = line.trim_end().ends_with('\\');
            parse_hosts_from(line, &mut hosts);
        } else {
            needs_another_line = false;
        }
    }

    hosts
}

fn parse_hosts_from(line: &str, hosts: &mut BTreeSet<String>) {
    hosts.extend(
        line.split_whitespace()
            .filter(|field| !field.starts_with("!"))
            .filter(|field| !field.contains("*"))
            .filter(|field| !is_filtered_git_provider_host(field))
            .filter(|field| !field.is_empty())
            .map(|field| field.to_owned()),
    );
}

fn is_filtered_git_provider_host(host: &str) -> bool {
    let normalized_host = host.trim_end_matches('.').to_ascii_lowercase();
    log::info!("normalized_host: {}", normalized_host);
    FILTERED_GIT_PROVIDER_HOSTS.contains(&normalized_host.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thank_you_bjorn3() {
        let hosts = "
            Host *
              AddKeysToAgent yes
              UseKeychain yes
              IdentityFile ~/.ssh/id_ed25519

            Host whatever.*
            User another

            Host !not_this
            User not_me

            Host something
        HostName whatever.tld

        Host linux bsd host3
          User bjorn

        Host rpi
          user rpi
          hostname rpi.local

        Host \
               somehost \
        anotherhost
        Hostname 192.168.3.3";

        let expected_hosts = BTreeSet::from_iter([
            "something".to_owned(),
            "linux".to_owned(),
            "host3".to_owned(),
            "bsd".to_owned(),
            "rpi".to_owned(),
            "somehost".to_owned(),
            "anotherhost".to_owned(),
        ]);

        assert_eq!(expected_hosts, parse_ssh_config_hosts(hosts));
    }

    #[test]
    fn test_filters_git_hosting_providers() {
        let hosts = "
            Host dev.azure.com
            Host bitbucket.org
            Host codeberg.org
            Host gitea.com
            Host gitee.com
            Host github.com
            Host gitlab.com
            Host sourcehut.org
            Host git.sr.ht
            Host engineering-box
            Host custom-provider.internal
            Host GITHUB
        ";

        let expected_hosts = BTreeSet::from_iter([
            "custom-provider.internal".to_owned(),
            "engineering-box".to_owned(),
        ]);

        assert_eq!(expected_hosts, parse_ssh_config_hosts(hosts));
    }
}
