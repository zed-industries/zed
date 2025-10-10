use std::collections::HashSet;

/// Language-specific configuration for rainbow highlighting.
/// Each language can provide its own set of keywords to exclude from rainbow highlighting.
pub trait RainbowConfig: Send + Sync {
    /// Returns the set of keywords/builtin identifiers to exclude from rainbow highlighting.
    /// This should include language keywords, common builtins, and reserved words.
    fn excluded_identifiers(&self) -> &HashSet<&'static str>;
}

/// Default rainbow config with common keywords across languages
pub struct DefaultRainbowConfig {
    excluded: HashSet<&'static str>,
}

impl Default for DefaultRainbowConfig {
    fn default() -> Self {
        let mut excluded = HashSet::new();
        
        // Common keywords that should never be rainbow highlighted
        excluded.insert("self");
        excluded.insert("super");
        excluded.insert("true");
        excluded.insert("false");
        excluded.insert("none");
        excluded.insert("null");
        excluded.insert("undefined");
        
        Self { excluded }
    }
}

impl RainbowConfig for DefaultRainbowConfig {
    fn excluded_identifiers(&self) -> &HashSet<&'static str> {
        &self.excluded
    }
}

/// Rust-specific rainbow configuration
pub struct RustRainbowConfig {
    excluded: HashSet<&'static str>,
}

impl Default for RustRainbowConfig {
    fn default() -> Self {
        let mut excluded = HashSet::new();
        
        // Rust keywords
        excluded.insert("self");
        excluded.insert("super");
        excluded.insert("crate");
        excluded.insert("true");
        excluded.insert("false");
        excluded.insert("none");
        excluded.insert("some");
        excluded.insert("ok");
        excluded.insert("err");
        excluded.insert("let");
        excluded.insert("mut");
        excluded.insert("fn");
        excluded.insert("pub");
        excluded.insert("impl");
        excluded.insert("struct");
        excluded.insert("enum");
        excluded.insert("trait");
        excluded.insert("type");
        excluded.insert("if");
        excluded.insert("else");
        excluded.insert("match");
        excluded.insert("for");
        excluded.insert("while");
        excluded.insert("loop");
        excluded.insert("break");
        excluded.insert("continue");
        excluded.insert("return");
        
        Self { excluded }
    }
}

impl RainbowConfig for RustRainbowConfig {
    fn excluded_identifiers(&self) -> &HashSet<&'static str> {
        &self.excluded
    }
}

/// Python-specific rainbow configuration  
pub struct PythonRainbowConfig {
    excluded: HashSet<&'static str>,
}

impl Default for PythonRainbowConfig {
    fn default() -> Self {
        let mut excluded = HashSet::new();
        
        // Python keywords
        excluded.insert("def");
        excluded.insert("class");
        excluded.insert("import");
        excluded.insert("from");
        excluded.insert("as");
        excluded.insert("pass");
        excluded.insert("with");
        excluded.insert("async");
        excluded.insert("await");
        excluded.insert("lambda");
        excluded.insert("yield");
        excluded.insert("raise");
        excluded.insert("try");
        excluded.insert("except");
        excluded.insert("finally");
        excluded.insert("assert");
        excluded.insert("del");
        excluded.insert("if");
        excluded.insert("else");
        excluded.insert("for");
        excluded.insert("while");
        excluded.insert("break");
        excluded.insert("continue");
        excluded.insert("return");
        excluded.insert("true");
        excluded.insert("false");
        excluded.insert("none");
        
        // Common builtins
        excluded.insert("print");
        excluded.insert("len");
        excluded.insert("str");
        excluded.insert("int");
        excluded.insert("float");
        excluded.insert("bool");
        excluded.insert("list");
        excluded.insert("dict");
        excluded.insert("set");
        excluded.insert("range");
        excluded.insert("map");
        excluded.insert("filter");
        excluded.insert("sum");
        excluded.insert("min");
        excluded.insert("max");
        excluded.insert("abs");
        excluded.insert("all");
        excluded.insert("any");
        
        Self { excluded }
    }
}

impl RainbowConfig for PythonRainbowConfig {
    fn excluded_identifiers(&self) -> &HashSet<&'static str> {
        &self.excluded
    }
}
