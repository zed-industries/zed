use std::{path::PathBuf, sync::Arc};

use collections::HashMap;
use configuration::{Recipe, Recipes};
use gpui::{App, AppContext as _, Entity};
use settings::{InvalidSettingsError, parse_json_with_comments};
use util::rel_path::RelPath;
use worktree::WorktreeId;

/// Storage for recipes available in the project
pub struct RecipeStore {
    /// Recipes from global settings
    global_recipes: HashMap<PathBuf, Vec<Recipe>>,
    /// Recipes from worktree-specific settings
    worktree_recipes: HashMap<WorktreeId, HashMap<Arc<RelPath>, Vec<Recipe>>>,
}

impl RecipeStore {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_cx| {
            let mut store = Self {
                global_recipes: HashMap::default(),
                worktree_recipes: HashMap::default(),
            };
            
            // Add built-in recipes
            store.add_builtin_recipes();
            
            store
        })
    }
    
    /// Add built-in recipes that are always available
    fn add_builtin_recipes(&mut self) {
        let builtin_path = PathBuf::from("<builtin>");
        
        let builtin_recipes = vec![
            Recipe {
                name: "custom".to_string(),
                execute_run: "{{ .command }}".to_string(),
                execute_debug: "{{ .command }}".to_string(),
                description: Some("Custom command - pass command directly to shell".to_string()),
                defaults: HashMap::default(),
            },
        ];
        
        self.global_recipes.insert(builtin_path, builtin_recipes);
    }
    
    /// Update recipes from a settings file
    pub fn update_recipes(
        &mut self,
        location: RecipeSettingsLocation,
        recipes: Vec<Recipe>,
    ) {
        match location {
            RecipeSettingsLocation::Global(path) => {
                if recipes.is_empty() {
                    self.global_recipes.remove(&path);
                } else {
                    self.global_recipes.insert(path, recipes);
                }
            }
            RecipeSettingsLocation::Worktree {
                worktree_id,
                directory_in_worktree,
            } => {
                let worktree_recipes = self.worktree_recipes.entry(worktree_id).or_default();
                if recipes.is_empty() {
                    worktree_recipes.remove(&directory_in_worktree);
                } else {
                    worktree_recipes.insert(directory_in_worktree, recipes);
                }
            }
        }
    }
    
    /// Get a recipe by name, searching worktree-specific recipes first, then global
    pub fn get_recipe(&self, name: &str, worktree_id: Option<WorktreeId>) -> Option<Recipe> {
        // First check worktree-specific recipes if we have a worktree
        if let Some(worktree_id) = worktree_id {
            if let Some(worktree_recipes) = self.worktree_recipes.get(&worktree_id) {
                for recipes in worktree_recipes.values() {
                    if let Some(recipe) = recipes.iter().find(|r| r.name == name) {
                        return Some(recipe.clone());
                    }
                }
            }
        }
        
        // Then check global recipes
        for recipes in self.global_recipes.values() {
            if let Some(recipe) = recipes.iter().find(|r| r.name == name) {
                return Some(recipe.clone());
            }
        }
        
        None
    }
    
    /// List all available recipes
    pub fn list_recipes(&self, worktree_id: Option<WorktreeId>) -> Vec<Recipe> {
        let mut all_recipes = Vec::new();
        let mut seen_names = collections::HashSet::default();
        
        // Add worktree-specific recipes first (they have priority)
        if let Some(worktree_id) = worktree_id {
            if let Some(worktree_recipes) = self.worktree_recipes.get(&worktree_id) {
                for recipes in worktree_recipes.values() {
                    for recipe in recipes {
                        if seen_names.insert(recipe.name.clone()) {
                            all_recipes.push(recipe.clone());
                        }
                    }
                }
            }
        }
        
        // Add global recipes (skip duplicates)
        for recipes in self.global_recipes.values() {
            for recipe in recipes {
                if seen_names.insert(recipe.name.clone()) {
                    all_recipes.push(recipe.clone());
                }
            }
        }
        
        all_recipes
    }
}

#[derive(Debug, Clone)]
pub enum RecipeSettingsLocation {
    Global(PathBuf),
    Worktree {
        worktree_id: WorktreeId,
        directory_in_worktree: Arc<RelPath>,
    },
}

/// Parse a recipes JSON file
pub fn parse_recipe_file(content: String) -> Result<Vec<Recipe>, InvalidSettingsError> {
    let json_str = content.trim();
    if json_str.is_empty() {
        return Ok(Vec::new());
    }
    
    parse_json_with_comments::<Recipes>(json_str)
        .map(|recipes| recipes.0)
        .map_err(|err| InvalidSettingsError::InvalidConfigurationFile(format!("Failed to parse recipes: {}", err)))
}
