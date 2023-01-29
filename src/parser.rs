use std::vec::Vec;

#[derive(Debug, PartialEq)]
enum ConfigStatement {
    HostName(String),
    Port(String),
    Unknown(String),
}

type ParsedConfig = Vec<(String, Vec<ConfigStatement>)>;

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

fn parse_config(config: &str) -> Result<ParsedConfig, String> {
    let mut parsed_config: ParsedConfig = Vec::new();
    let mut lines = config.lines().enumerate().peekable();

    while let Some((line_nr, line)) = lines.next() {
        if line.starts_with(char::is_whitespace) {
            return Err(format!(
                "Malformed config at line {}: Unexpected whitespace",
                line_nr
            ));
        };

        let hostname = match line
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .as_slice()
        {
            [] => continue,
            ["Host", hostname] => hostname.to_string(),
            _ => {
                return Err(format!(
                    "Malformed config at line {}: Could not parse Host",
                    line_nr
                ))
            }
        };

        let mut statements: Vec<ConfigStatement> = Vec::new();
        while let Some((_, line)) = lines.peek() {
            if !line.starts_with(char::is_whitespace) {
                break;
            }

            statements.push(parse_line(line));
            lines.next();
        }

        parsed_config.push((hostname.to_string(), statements));
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
        let (name, statements) = &result[0];
        assert_eq!(name, "github");
        assert_eq!(statements.len(), 3);
        assert_eq!(
            statements[0],
            ConfigStatement::HostName("github.com".to_string())
        );
        assert_eq!(
            statements[1],
            ConfigStatement::Unknown("IdentityFile ~/.ssh/id_rsa".to_string())
        );
        assert_eq!(statements[2], ConfigStatement::Port("22".to_string()));
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

    const MALFORMED_SAMPLE_CONFIG: &str = "
  Host github
    HostName github.com
    IdentityFile ~/.ssh/id_rsa
    Port 22
";
    #[test]
    fn test_parse_malformed_config() {
        let res = parse_config(MALFORMED_SAMPLE_CONFIG);
        assert!(res.is_err());
    }
}
