use clap::{Arg, Command};
use std::path::Path;

mod template;
mod config;

use template::TemplateGenerator;
use config::Config;

fn main() -> anyhow::Result<()> {
    let matches = Command::new("loku")
        .version("0.1.0")
        .about("A context and prompt template generator")
        .subcommand(
            Command::new("claude")
                .about("Generate CLAUDE.md templates")
                .arg(
                    Arg::new("language")
                        .long("language")
                        .short('l')
                        .value_name("LANGUAGE")
                        .help("Programming language (e.g., python, rust, javascript)")
                        .required(false)
                )
                .arg(
                    Arg::new("output")
                        .long("output")
                        .short('o')
                        .value_name("PATH")
                        .help("Output file path")
                        .default_value("CLAUDE.md")
                )
        )
        .subcommand(
            Command::new("init")
                .about("Initialize loku configuration")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("claude", sub_matches)) => {
            let language = sub_matches.get_one::<String>("language");
            let output = sub_matches.get_one::<String>("output").unwrap();
            
            let config = Config::load_or_default()?;
            let generator = TemplateGenerator::new(config);
            
            generator.generate_claude_template(language, Path::new(output))?;
            println!("Generated CLAUDE.md template at {}", output);
        }
        Some(("init", _)) => {
            Config::init()?;
            println!("Initialized loku configuration");
        }
        _ => {
            println!("Use 'loku --help' for usage information");
        }
    }

    Ok(())
}