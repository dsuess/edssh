use std::collections::HashMap;
use std::fmt::Error;
use std::vec::Vec;

#[derive(Debug, PartialEq)]
enum ConfigStatement {
    HostName(String),
    Port(String),
    Unknown(String),
}

// FIXME This won't be able to deal with duplicates...
type ParsedConfig = HashMap<String, Vec<ConfigStatement>>;

// TODO Replace parsing by something more robust (e.g. regular expressions)
fn parse_line(line: &str) -> ConfigStatement {
    match line
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .as_slice()
    {
        ["HostName", host] => ConfigStatement::HostName(host.to_string()),
        ["Port", port] => ConfigStatement::Port(port.to_string()),
        _ => ConfigStatement::Unknown(line.trim().to_string()),
    }
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

fn format_line(statement: &ConfigStatement) -> String {
    match statement {
        ConfigStatement::HostName(host) => format!("HostName {}", host),
        ConfigStatement::Port(port) => format!("Port {}", port),
        ConfigStatement::Unknown(line) => line.to_string(),
    }
}

fn format_config(config: &ParsedConfig, indent: &str) -> String {
    let mut result = String::new();
    for (hostname, statements) in config.into_iter() {
        result.push_str(&format!("Host {}\n", hostname));
        for statement in statements {
            result.push_str(&format!("{}{}\n", indent, format_line(statement)));
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_CONFIG: &str = "
Host github
    HostName github.com
    IdentityFile ~/.ssh/id_rsa
    Port 22
";

    #[test]
    fn test_parse_config() {
        let result = parse_config(SAMPLE_CONFIG).unwrap();
        assert_eq!(result.len(), 1);
        let github = result.get("github").unwrap();
        assert_eq!(github.len(), 3);
        assert_eq!(
            github[0],
            ConfigStatement::HostName("github.com".to_string())
        );
        assert_eq!(
            github[1],
            ConfigStatement::Unknown("IdentityFile ~/.ssh/id_rsa".to_string())
        );
        assert_eq!(github[2], ConfigStatement::Port("22".to_string()));
    }

    #[test]
    fn test_roundtrip_parse_format() {
        let parsed = parse_config(SAMPLE_CONFIG).unwrap();
        let formatted = format_config(&parsed, "    ");
        assert_eq!(formatted.trim(), SAMPLE_CONFIG.trim());
    }

    const MULTI_HOST_SAMPLE_CONFIG: &str = "
Host github
  HostName github.com
  IdentityFile ~/.ssh/id_rsa
  Port 22
Host google
  HostName google.com
";
    #[test]
    fn test_roundtrip_parse_format_multi_host() {
        let parsed = parse_config(MULTI_HOST_SAMPLE_CONFIG).unwrap();
        let formatted = format_config(&parsed, "  ");
        assert_eq!(formatted.trim(), MULTI_HOST_SAMPLE_CONFIG.trim());
    }
}
