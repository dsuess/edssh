mod parser;
use clap::Parser;

use shellexpand;
use std::{error::Error, fs, ops::Deref, path::Path};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Arguments {
    host: String,
    #[arg(short = 'n', long)]
    host_name: Option<String>,

    #[arg(short, long)]
    port: Option<String>,

    #[arg(short, long, default_value = "~/.ssh/config")]
    config: String,

    #[arg(short, long, action)]
    write: bool,

    #[arg(short, long, default_value_t = 4)]
    indent_spaces: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Arguments::parse();
    if args.indent_spaces <= 0 {
        return Err("Indent must be greater than 0".into());
    }
    let config_path = shellexpand::tilde(&args.config).deref().to_string();

    // Input
    let config_str = fs::read_to_string(&config_path)?;
    let mut config = parser::parse_config(&config_str)?;
    let mut entry = &mut config
        .iter_mut()
        .find(|(name, _)| name == &args.host)
        .ok_or(format!("Host '{}' not found", &args.host))?;

    // Mutate
    for statement in entry.1.iter_mut() {
        match statement {
            parser::ConfigStatement::HostName(host) => {
                if let Some(new_host) = &args.host_name {
                    *host = new_host.to_string();
                }
            }
            parser::ConfigStatement::Port(port) => {
                if let Some(new_port) = &args.port {
                    *port = new_port.to_string();
                }
            }
            _ => {}
        }
    }

    // Output
    let out = parser::format_config(&config, &" ".repeat(args.indent_spaces));
    if args.write {
        fs::write(&config_path, out)?;
    } else {
        print!("{}", out)
    }

    Ok(())
}
