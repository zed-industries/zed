use std::collections::BTreeSet;

const FILTERED_GIT_PROVIDER_HOSTNAMES: &[&str] = &[
    "dev.azure.com",
    "bitbucket.org",
    "chromium.googlesource.com",
    "codeberg.org",
    "gitea.com",
    "gitee.com",
    "github.com",
    "gist.github.com",
    "gitlab.com",
    "sourcehut.org",
    "git.sr.ht",
];

pub fn parse_ssh_config_hosts(config: &str) -> BTreeSet<String> {
    parse_host_blocks(config)
        .into_iter()
        .flat_map(HostBlock::non_git_provider_hosts)
        .collect()
}

struct HostBlock {
    aliases: BTreeSet<String>,
    hostname: Option<String>,
}

impl HostBlock {
    fn non_git_provider_hosts(self) -> impl Iterator<Item = String> {
        let hostname = self.hostname;
        let hostname_ref = hostname.as_deref().map(is_git_provider_domain);
        self.aliases
            .into_iter()
            .filter(move |alias| !hostname_ref.unwrap_or_else(|| is_git_provider_domain(alias)))
    }
}

fn parse_host_blocks(config: &str) -> Vec<HostBlock> {
    let mut blocks = Vec::new();
    let mut aliases = BTreeSet::new();
    let mut hostname = None;
    let mut needs_continuation = false;

    for line in config.lines() {
        let line = line.trim_start();

        if needs_continuation {
            needs_continuation = line.trim_end().ends_with('\\');
            parse_hosts(line, &mut aliases);
            continue;
        }

        let Some((keyword, value)) = split_keyword_and_value(line) else {
            continue;
        };

        if keyword.eq_ignore_ascii_case("host") {
            if !aliases.is_empty() {
                blocks.push(HostBlock { aliases, hostname });
                aliases = BTreeSet::new();
                hostname = None;
            }
            parse_hosts(value, &mut aliases);
            needs_continuation = line.trim_end().ends_with('\\');
        } else if keyword.eq_ignore_ascii_case("hostname") {
            hostname = value.split_whitespace().next().map(ToOwned::to_owned);
        }
    }

    if !aliases.is_empty() {
        blocks.push(HostBlock { aliases, hostname });
    }

    blocks
}

fn parse_hosts(line: &str, hosts: &mut BTreeSet<String>) {
    hosts.extend(
        line.split_whitespace()
            .map(|field| field.trim_end_matches('\\'))
            .filter(|field| !field.starts_with("!"))
            .filter(|field| !field.contains("*"))
            .filter(|field| *field != "\\")
            .filter(|field| !field.is_empty())
            .map(|field| field.to_owned()),
    );
}

fn split_keyword_and_value(line: &str) -> Option<(&str, &str)> {
    let keyword_end = line.find(char::is_whitespace).unwrap_or(line.len());
    let keyword = &line[..keyword_end];
    if keyword.is_empty() {
        return None;
    }

    let value = line[keyword_end..].trim_start();
    Some((keyword, value))
}

fn is_git_provider_domain(host: &str) -> bool {
    let host = host.to_ascii_lowercase();
    FILTERED_GIT_PROVIDER_HOSTNAMES.contains(&host.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_thank_you_bjorn3() {
        let hosts = indoc! {"
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

            Host \\
                   somehost \\
                   anotherhost
              Hostname 192.168.3.3
        "};

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
    fn filters_git_provider_domains_from_hostname() {
        let hosts = indoc! {"
            Host github-personal
              HostName github.com

            Host gitlab-work
              HostName GITLAB.COM

            Host local
              HostName example.com
        "};

        assert_eq!(
            BTreeSet::from_iter(["local".to_owned()]),
            parse_ssh_config_hosts(hosts)
        );
    }

    #[test]
    fn falls_back_to_host_when_hostname_is_absent() {
        let hosts = indoc! {"
            Host github.com bitbucket.org keep-me
              User git
        "};

        assert_eq!(
            BTreeSet::from_iter(["keep-me".to_owned()]),
            parse_ssh_config_hosts(hosts)
        );
    }

    #[test]
    fn does_not_fuzzy_match_host_aliases() {
        let hosts = indoc! {"
            Host GitHub GitLab Bitbucket GITHUB github
              User git
        "};

        assert_eq!(
            BTreeSet::from_iter([
                "Bitbucket".to_owned(),
                "GITHUB".to_owned(),
                "GitHub".to_owned(),
                "GitLab".to_owned(),
                "github".to_owned(),
            ]),
            parse_ssh_config_hosts(hosts)
        );
    }

    #[test]
    fn uses_hostname_before_host_filtering() {
        let hosts = indoc! {"
            Host github.com keep-me
              HostName example.com
        "};

        assert_eq!(
            BTreeSet::from_iter(["github.com".to_owned(), "keep-me".to_owned()]),
            parse_ssh_config_hosts(hosts)
        );
    }
}
