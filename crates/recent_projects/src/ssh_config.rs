use collections::BTreeSet;

pub fn parse_ssh_config_hosts(config: &str) -> BTreeSet<String> {
    let mut hosts = BTreeSet::new();
    let mut needs_another_line = false;
    for line in config.lines() {
        let line = line.trim_start();
        if let Some(line) = line.strip_prefix("Host") {
            match line.chars().next() {
                Some(c) if c == '\\' => {
                    needs_another_line = true;
                }
                Some(c) if c.is_whitespace() => {
                    if c == '\n' || c == '\r' {
                        needs_another_line = false;
                    } else {
                        parse_hosts_from(line, &mut hosts);
                    }
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
            .filter(|field| !field.is_empty())
            .map(|field| field.to_owned()),
    );
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
}
