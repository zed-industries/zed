use collections::HashMap;
use serde::{Deserialize, Serialize};

/// A recipe is a reusable template for configurations
/// It uses Go template syntax for variable substitution
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Recipe {
    /// Name of the recipe (e.g., "uv", "cargo", "npm")
    pub name: String,
    
    /// Command template for running (with Go template syntax)
    /// Example: "uv run {{ .file }}"
    #[serde(rename = "executeRun")]
    pub execute_run: String,
    
    /// Command template for debugging (with Go template syntax)
    /// Example: "uv run debugpy {{ .file }}"
    #[serde(rename = "executeDebug")]
    pub execute_debug: String,
    
    /// Optional description of what this recipe is for
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Optional default variables that can be overridden by configurations
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub defaults: HashMap<String, String>,
}

/// A collection of recipes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Recipes(pub Vec<Recipe>);

impl Recipe {
    /// Render the run command with the given variables
    pub fn render_run(&self, variables: &HashMap<String, String>) -> Result<String, String> {
        render_template(&self.execute_run, variables)
    }
    
    /// Render the debug command with the given variables
    pub fn render_debug(&self, variables: &HashMap<String, String>) -> Result<String, String> {
        render_template(&self.execute_debug, variables)
    }
}

/// Renders a Go-style template string with the given variables
/// Supports {{ .variable_name }} syntax
fn render_template(template: &str, variables: &HashMap<String, String>) -> Result<String, String> {
    let mut result = template.to_string();
    
    // Simple Go template parser for {{ .variable_name }}
    // This handles the basic case - a full Go template engine would be more complex
    let mut start = 0;
    while let Some(open_pos) = result[start..].find("{{") {
        let open_pos = start + open_pos;
        let close_pos = result[open_pos..]
            .find("}}")
            .ok_or_else(|| format!("Unclosed template variable at position {}", open_pos))?;
        let close_pos = open_pos + close_pos;
        
        // Extract the variable name (removing {{ }}, whitespace, and the leading .)
        let var_expr = result[open_pos + 2..close_pos].trim();
        let var_name = if let Some(stripped) = var_expr.strip_prefix('.') {
            stripped.trim()
        } else {
            var_expr
        };
        
        // Get the variable value
        let value = variables
            .get(var_name)
            .ok_or_else(|| format!("Variable '{}' not found in configuration", var_name))?;
        
        // Replace the template expression with the value
        result.replace_range(open_pos..close_pos + 2, value);
        start = open_pos + value.len();
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_render_template() {
        let mut vars = HashMap::new();
        vars.insert("file".to_string(), "src/main.py".to_string());
        vars.insert("port".to_string(), "8000".to_string());
        
        assert_eq!(
            render_template("uv run {{ .file }}", &vars).unwrap(),
            "uv run src/main.py"
        );
        
        assert_eq!(
            render_template("python {{ .file }} --port {{ .port }}", &vars).unwrap(),
            "python src/main.py --port 8000"
        );
        
        assert_eq!(
            render_template("no variables here", &vars).unwrap(),
            "no variables here"
        );
    }
    
    #[test]
    fn test_missing_variable() {
        let vars = HashMap::new();
        let result = render_template("run {{ .file }}", &vars);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Variable 'file' not found"));
    }
    
    #[test]
    fn test_unclosed_variable() {
        let vars = HashMap::new();
        let result = render_template("run {{ .file", &vars);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unclosed template variable"));
    }
}
