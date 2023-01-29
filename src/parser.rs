use std::collections::HashMap;
use std::fmt::Error;
use std::vec::Vec;

enum ConfigStatement {
    Unknown(String),
}

type ParsedConfig = HashMap<String, Vec<ConfigStatement>>;

fn parse_line(line: &str) -> ConfigStatement {
    ConfigStatement::Unknown(line.to_string())
}

fn parse_config(config: &str) -> Result<ParsedConfig, Error> {
    let mut parsed_config: ParsedConfig = HashMap::new();
    let mut lines = config.lines().enumerate().peekable();

    while let Some((line_nr, line)) = lines.next() {
        let hostname = match line
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .as_slice()
        {
            [] => continue,
            ["Host", hostname] => hostname.to_string(),
            _ => panic!("Unknown config statement at line {}", line_nr), //FIXME
        };

        let mut statements: Vec<ConfigStatement> = Vec::new();
        while let Some((_, line)) = lines.peek() {
            if !line.starts_with(char::is_whitespace) {
                break;
            }

            statements.push(parse_line(line));
            lines.next();
        }

        parsed_config.insert(hostname.to_string(), statements);
    }

    Ok(parsed_config)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_CONFIG: &str = "
Host github.com
    HostName github.com
    User git
    IdentityFile ~/.ssh/id_rsa
    IdentitiesOnly yes
    LogLevel ERROR
    TCPKeepAlive yes
    ServerAliveInterval 60
    ServerAliveCountMax 3
    StrictHostKeyChecking no
    UserKnownHostsFile /dev/null
    AddKeysToAgent yes
    UseKeychain yes
    ForwardAgent yes
    ForwardX11 yes
    ForwardX11Trusted yes
    ProxyCommand ssh -q -W %h:%p gateway.example.com
    ControlMaster auto
    ControlPath ~/.ssh/sockets/%r@%h:%p
    ControlPersist 5m";

    #[test]
    fn test_parse_config() {
        let result = parse_config(SAMPLE_CONFIG).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result["github.com"].len(), 19);
    }
}
