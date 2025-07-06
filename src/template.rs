use crate::config::{Config, LanguageTemplate};
use anyhow::Result;
use std::fs;
use std::path::Path;

pub struct TemplateGenerator {
    config: Config,
}

impl TemplateGenerator {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn generate_claude_template(&self, language: Option<&String>, output_path: &Path) -> Result<()> {
        let mut content = String::new();
        
        content.push_str("# Claude AI Assistant Instructions\n\n");
        content.push_str("You are Claude, an AI assistant created by Anthropic. ");
        content.push_str("You are here to help with programming tasks and questions.\n\n");
        
        content.push_str("## General Programming Guidelines\n\n");
        for rule in &self.config.global_rules {
            content.push_str(&format!("- {}\n", rule));
        }
        content.push('\n');

        if let Some(lang) = language {
            if let Some(template) = self.config.templates.get(lang) {
                content.push_str(&self.generate_language_section(template));
            } else {
                content.push_str(&format!("## {} Guidelines\n\n", lang));
                content.push_str(&format!("No specific guidelines found for {}. ", lang));
                content.push_str("Please follow general programming best practices.\n\n");
            }
        } else {
            content.push_str("## Available Languages\n\n");
            for (lang, template) in &self.config.templates {
                content.push_str(&format!("### {}\n\n", template.name));
                content.push_str(&format!("Use `loku claude --language {}` to generate specific guidelines.\n\n", lang));
            }
        }

        content.push_str("## Communication Style\n\n");
        content.push_str("- Be concise and direct\n");
        content.push_str("- Provide working code examples\n");
        content.push_str("- Explain complex concepts clearly\n");
        content.push_str("- Ask clarifying questions when needed\n");

        fs::write(output_path, content)?;
        Ok(())
    }

    fn generate_language_section(&self, template: &LanguageTemplate) -> String {
        let mut content = String::new();
        
        content.push_str(&format!("## {} Guidelines\n\n", template.name));
        
        content.push_str("### Rules\n\n");
        for rule in &template.rules {
            content.push_str(&format!("- {}\n", rule));
        }
        content.push('\n');
        
        content.push_str("### Best Practices\n\n");
        for practice in &template.best_practices {
            content.push_str(&format!("- {}\n", practice));
        }
        content.push('\n');
        
        content.push_str("### Common Patterns\n\n");
        for pattern in &template.common_patterns {
            content.push_str(&format!("- {}\n", pattern));
        }
        content.push('\n');
        
        content
    }
}