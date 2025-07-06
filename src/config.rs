use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub templates: HashMap<String, LanguageTemplate>,
    pub global_rules: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageTemplate {
    pub name: String,
    pub rules: Vec<String>,
    pub best_practices: Vec<String>,
    pub common_patterns: Vec<String>,
}

impl Config {
    pub fn load_or_default() -> Result<Self> {
        let config_path = Self::config_path()?;
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            Ok(toml::from_str(&content)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn init() -> Result<()> {
        let config_path = Self::config_path()?;
        
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let default_config = Self::default();
        let content = toml::to_string_pretty(&default_config)?;
        fs::write(&config_path, content)?;
        
        Ok(())
    }

    fn config_path() -> Result<PathBuf> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        Ok(home_dir.join(".loku").join("config.toml"))
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut templates = HashMap::new();
        
        templates.insert("python".to_string(), LanguageTemplate {
            name: "Python".to_string(),
            rules: vec![
                "Follow PEP 8 style guide".to_string(),
                "Use type hints for function parameters and return types".to_string(),
                "Prefer f-strings for string formatting".to_string(),
                "Use list comprehensions when appropriate".to_string(),
            ],
            best_practices: vec![
                "Use virtual environments for dependency management".to_string(),
                "Write docstrings for all public functions and classes".to_string(),
                "Handle exceptions appropriately".to_string(),
                "Use meaningful variable and function names".to_string(),
            ],
            common_patterns: vec![
                "Context managers for resource management".to_string(),
                "Dataclasses for simple data structures".to_string(),
                "Enum for constants".to_string(),
            ],
        });

        templates.insert("rust".to_string(), LanguageTemplate {
            name: "Rust".to_string(),
            rules: vec![
                "Follow Rust naming conventions".to_string(),
                "Use Result<T, E> for error handling".to_string(),
                "Prefer borrowing over cloning".to_string(),
                "Use match expressions for pattern matching".to_string(),
            ],
            best_practices: vec![
                "Use cargo fmt and cargo clippy".to_string(),
                "Write comprehensive tests".to_string(),
                "Document public APIs with /// comments".to_string(),
                "Use iterators instead of loops when possible".to_string(),
            ],
            common_patterns: vec![
                "Builder pattern for complex constructors".to_string(),
                "Option<T> for nullable values".to_string(),
                "Traits for shared behavior".to_string(),
            ],
        });

        Self {
            templates,
            global_rules: vec![
                "Write clear, readable code".to_string(),
                "Use meaningful names for variables and functions".to_string(),
                "Keep functions small and focused".to_string(),
                "Write tests for your code".to_string(),
                "Handle errors gracefully".to_string(),
            ],
        }
    }
}