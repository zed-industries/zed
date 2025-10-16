use std::collections::HashSet;

/// Validates if a string is a complete, valid identifier.
/// This is a shared utility for both rainbow highlighting and semantic analysis.
/// Minimum length is 2 characters to avoid highlighting single-letter names.
#[inline]
pub fn is_valid_identifier(text: &str) -> bool {
    if text.len() < 2 {
        return false;
    }

    if text.contains(char::is_whitespace) || text.contains(|c: char| c.is_control()) {
        return false;
    }

    if text.chars().all(|c| c == '_') {
        return false;
    }

    let mut chars = text.chars();
    match chars.next() {
        Some(first) if first.is_alphabetic() || first == '_' => {
            chars.all(|c| c.is_alphanumeric() || c == '_')
        }
        _ => false,
    }
}

/// Language-specific configuration for rainbow highlighting.
/// Each language can provide its own set of keywords to exclude from rainbow highlighting.
pub trait RainbowConfig: Send + Sync {
    /// Returns the set of keywords/builtin identifiers to exclude from rainbow highlighting.
    /// This should include language keywords, common builtins, and reserved words.
    fn excluded_identifiers(&self) -> &HashSet<&'static str>;

    /// Checks if an identifier should be rainbow highlighted.
    /// Returns true if the identifier is valid and not a keyword.
    fn should_highlight(&self, identifier: &str) -> bool {
        is_valid_identifier(identifier) && !self.excluded_identifiers().contains(identifier)
    }
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

pub struct TypeScriptRainbowConfig {
    excluded: HashSet<&'static str>,
}

impl Default for TypeScriptRainbowConfig {
    fn default() -> Self {
        let mut excluded = HashSet::new();

        excluded.insert("abstract");
        excluded.insert("any");
        excluded.insert("as");
        excluded.insert("async");
        excluded.insert("await");
        excluded.insert("boolean");
        excluded.insert("break");
        excluded.insert("case");
        excluded.insert("catch");
        excluded.insert("class");
        excluded.insert("const");
        excluded.insert("constructor");
        excluded.insert("continue");
        excluded.insert("debugger");
        excluded.insert("declare");
        excluded.insert("default");
        excluded.insert("delete");
        excluded.insert("do");
        excluded.insert("else");
        excluded.insert("enum");
        excluded.insert("export");
        excluded.insert("extends");
        excluded.insert("false");
        excluded.insert("finally");
        excluded.insert("for");
        excluded.insert("from");
        excluded.insert("function");
        excluded.insert("get");
        excluded.insert("if");
        excluded.insert("implements");
        excluded.insert("import");
        excluded.insert("in");
        excluded.insert("instanceof");
        excluded.insert("interface");
        excluded.insert("is");
        excluded.insert("let");
        excluded.insert("module");
        excluded.insert("namespace");
        excluded.insert("never");
        excluded.insert("new");
        excluded.insert("null");
        excluded.insert("number");
        excluded.insert("of");
        excluded.insert("package");
        excluded.insert("private");
        excluded.insert("protected");
        excluded.insert("public");
        excluded.insert("readonly");
        excluded.insert("require");
        excluded.insert("return");
        excluded.insert("set");
        excluded.insert("static");
        excluded.insert("string");
        excluded.insert("super");
        excluded.insert("switch");
        excluded.insert("symbol");
        excluded.insert("this");
        excluded.insert("throw");
        excluded.insert("true");
        excluded.insert("try");
        excluded.insert("type");
        excluded.insert("typeof");
        excluded.insert("undefined");
        excluded.insert("var");
        excluded.insert("void");
        excluded.insert("while");
        excluded.insert("with");
        excluded.insert("yield");

        excluded.insert("console");
        excluded.insert("window");
        excluded.insert("document");
        excluded.insert("Array");
        excluded.insert("Object");
        excluded.insert("String");
        excluded.insert("Number");
        excluded.insert("Boolean");
        excluded.insert("Function");
        excluded.insert("Promise");

        Self { excluded }
    }
}

impl RainbowConfig for TypeScriptRainbowConfig {
    fn excluded_identifiers(&self) -> &HashSet<&'static str> {
        &self.excluded
    }
}

pub struct JavaScriptRainbowConfig {
    excluded: HashSet<&'static str>,
}

impl Default for JavaScriptRainbowConfig {
    fn default() -> Self {
        let mut excluded = HashSet::new();

        excluded.insert("arguments");
        excluded.insert("async");
        excluded.insert("await");
        excluded.insert("break");
        excluded.insert("case");
        excluded.insert("catch");
        excluded.insert("class");
        excluded.insert("const");
        excluded.insert("continue");
        excluded.insert("debugger");
        excluded.insert("default");
        excluded.insert("delete");
        excluded.insert("do");
        excluded.insert("else");
        excluded.insert("export");
        excluded.insert("extends");
        excluded.insert("false");
        excluded.insert("finally");
        excluded.insert("for");
        excluded.insert("from");
        excluded.insert("function");
        excluded.insert("if");
        excluded.insert("import");
        excluded.insert("in");
        excluded.insert("instanceof");
        excluded.insert("let");
        excluded.insert("new");
        excluded.insert("null");
        excluded.insert("of");
        excluded.insert("return");
        excluded.insert("static");
        excluded.insert("super");
        excluded.insert("switch");
        excluded.insert("this");
        excluded.insert("throw");
        excluded.insert("true");
        excluded.insert("try");
        excluded.insert("typeof");
        excluded.insert("undefined");
        excluded.insert("var");
        excluded.insert("void");
        excluded.insert("while");
        excluded.insert("with");
        excluded.insert("yield");

        excluded.insert("console");
        excluded.insert("window");
        excluded.insert("document");

        Self { excluded }
    }
}

impl RainbowConfig for JavaScriptRainbowConfig {
    fn excluded_identifiers(&self) -> &HashSet<&'static str> {
        &self.excluded
    }
}

pub struct GoRainbowConfig {
    excluded: HashSet<&'static str>,
}

impl Default for GoRainbowConfig {
    fn default() -> Self {
        let mut excluded = HashSet::new();

        excluded.insert("break");
        excluded.insert("case");
        excluded.insert("chan");
        excluded.insert("const");
        excluded.insert("continue");
        excluded.insert("default");
        excluded.insert("defer");
        excluded.insert("else");
        excluded.insert("fallthrough");
        excluded.insert("for");
        excluded.insert("func");
        excluded.insert("go");
        excluded.insert("goto");
        excluded.insert("if");
        excluded.insert("import");
        excluded.insert("interface");
        excluded.insert("map");
        excluded.insert("package");
        excluded.insert("range");
        excluded.insert("return");
        excluded.insert("select");
        excluded.insert("struct");
        excluded.insert("switch");
        excluded.insert("type");
        excluded.insert("var");

        excluded.insert("true");
        excluded.insert("false");
        excluded.insert("nil");
        excluded.insert("iota");

        excluded.insert("append");
        excluded.insert("cap");
        excluded.insert("close");
        excluded.insert("complex");
        excluded.insert("copy");
        excluded.insert("delete");
        excluded.insert("imag");
        excluded.insert("len");
        excluded.insert("make");
        excluded.insert("new");
        excluded.insert("panic");
        excluded.insert("print");
        excluded.insert("println");
        excluded.insert("real");
        excluded.insert("recover");

        Self { excluded }
    }
}

impl RainbowConfig for GoRainbowConfig {
    fn excluded_identifiers(&self) -> &HashSet<&'static str> {
        &self.excluded
    }
}

pub struct CppRainbowConfig {
    excluded: HashSet<&'static str>,
}

impl Default for CppRainbowConfig {
    fn default() -> Self {
        let mut excluded = HashSet::new();

        excluded.insert("alignas");
        excluded.insert("alignof");
        excluded.insert("and");
        excluded.insert("and_eq");
        excluded.insert("asm");
        excluded.insert("auto");
        excluded.insert("bitand");
        excluded.insert("bitor");
        excluded.insert("bool");
        excluded.insert("break");
        excluded.insert("case");
        excluded.insert("catch");
        excluded.insert("char");
        excluded.insert("char8_t");
        excluded.insert("char16_t");
        excluded.insert("char32_t");
        excluded.insert("class");
        excluded.insert("compl");
        excluded.insert("concept");
        excluded.insert("const");
        excluded.insert("consteval");
        excluded.insert("constexpr");
        excluded.insert("constinit");
        excluded.insert("const_cast");
        excluded.insert("continue");
        excluded.insert("co_await");
        excluded.insert("co_return");
        excluded.insert("co_yield");
        excluded.insert("decltype");
        excluded.insert("default");
        excluded.insert("delete");
        excluded.insert("do");
        excluded.insert("double");
        excluded.insert("dynamic_cast");
        excluded.insert("else");
        excluded.insert("enum");
        excluded.insert("explicit");
        excluded.insert("export");
        excluded.insert("extern");
        excluded.insert("false");
        excluded.insert("float");
        excluded.insert("for");
        excluded.insert("friend");
        excluded.insert("goto");
        excluded.insert("if");
        excluded.insert("inline");
        excluded.insert("int");
        excluded.insert("long");
        excluded.insert("mutable");
        excluded.insert("namespace");
        excluded.insert("new");
        excluded.insert("noexcept");
        excluded.insert("not");
        excluded.insert("not_eq");
        excluded.insert("nullptr");
        excluded.insert("operator");
        excluded.insert("or");
        excluded.insert("or_eq");
        excluded.insert("private");
        excluded.insert("protected");
        excluded.insert("public");
        excluded.insert("register");
        excluded.insert("reinterpret_cast");
        excluded.insert("requires");
        excluded.insert("return");
        excluded.insert("short");
        excluded.insert("signed");
        excluded.insert("sizeof");
        excluded.insert("static");
        excluded.insert("static_assert");
        excluded.insert("static_cast");
        excluded.insert("struct");
        excluded.insert("switch");
        excluded.insert("template");
        excluded.insert("this");
        excluded.insert("thread_local");
        excluded.insert("throw");
        excluded.insert("true");
        excluded.insert("try");
        excluded.insert("typedef");
        excluded.insert("typeid");
        excluded.insert("typename");
        excluded.insert("union");
        excluded.insert("unsigned");
        excluded.insert("using");
        excluded.insert("virtual");
        excluded.insert("void");
        excluded.insert("volatile");
        excluded.insert("wchar_t");
        excluded.insert("while");
        excluded.insert("xor");
        excluded.insert("xor_eq");

        excluded.insert("std");
        excluded.insert("cout");
        excluded.insert("cin");
        excluded.insert("endl");
        excluded.insert("NULL");

        Self { excluded }
    }
}

impl RainbowConfig for CppRainbowConfig {
    fn excluded_identifiers(&self) -> &HashSet<&'static str> {
        &self.excluded
    }
}

pub struct JavaRainbowConfig {
    excluded: HashSet<&'static str>,
}

impl Default for JavaRainbowConfig {
    fn default() -> Self {
        let mut excluded = HashSet::new();

        excluded.insert("abstract");
        excluded.insert("assert");
        excluded.insert("boolean");
        excluded.insert("break");
        excluded.insert("byte");
        excluded.insert("case");
        excluded.insert("catch");
        excluded.insert("char");
        excluded.insert("class");
        excluded.insert("const");
        excluded.insert("continue");
        excluded.insert("default");
        excluded.insert("do");
        excluded.insert("double");
        excluded.insert("else");
        excluded.insert("enum");
        excluded.insert("extends");
        excluded.insert("final");
        excluded.insert("finally");
        excluded.insert("float");
        excluded.insert("for");
        excluded.insert("goto");
        excluded.insert("if");
        excluded.insert("implements");
        excluded.insert("import");
        excluded.insert("instanceof");
        excluded.insert("int");
        excluded.insert("interface");
        excluded.insert("long");
        excluded.insert("native");
        excluded.insert("new");
        excluded.insert("package");
        excluded.insert("private");
        excluded.insert("protected");
        excluded.insert("public");
        excluded.insert("return");
        excluded.insert("short");
        excluded.insert("static");
        excluded.insert("strictfp");
        excluded.insert("super");
        excluded.insert("switch");
        excluded.insert("synchronized");
        excluded.insert("this");
        excluded.insert("throw");
        excluded.insert("throws");
        excluded.insert("transient");
        excluded.insert("try");
        excluded.insert("void");
        excluded.insert("volatile");
        excluded.insert("while");

        excluded.insert("true");
        excluded.insert("false");
        excluded.insert("null");

        excluded.insert("String");
        excluded.insert("System");
        excluded.insert("Object");
        excluded.insert("Integer");
        excluded.insert("Boolean");
        excluded.insert("Double");
        excluded.insert("Float");

        Self { excluded }
    }
}

impl RainbowConfig for JavaRainbowConfig {
    fn excluded_identifiers(&self) -> &HashSet<&'static str> {
        &self.excluded
    }
}

pub struct CSharpRainbowConfig {
    excluded: HashSet<&'static str>,
}

impl Default for CSharpRainbowConfig {
    fn default() -> Self {
        let mut excluded = HashSet::new();

        excluded.insert("abstract");
        excluded.insert("as");
        excluded.insert("base");
        excluded.insert("bool");
        excluded.insert("break");
        excluded.insert("byte");
        excluded.insert("case");
        excluded.insert("catch");
        excluded.insert("char");
        excluded.insert("checked");
        excluded.insert("class");
        excluded.insert("const");
        excluded.insert("continue");
        excluded.insert("decimal");
        excluded.insert("default");
        excluded.insert("delegate");
        excluded.insert("do");
        excluded.insert("double");
        excluded.insert("else");
        excluded.insert("enum");
        excluded.insert("event");
        excluded.insert("explicit");
        excluded.insert("extern");
        excluded.insert("false");
        excluded.insert("finally");
        excluded.insert("fixed");
        excluded.insert("float");
        excluded.insert("for");
        excluded.insert("foreach");
        excluded.insert("goto");
        excluded.insert("if");
        excluded.insert("implicit");
        excluded.insert("in");
        excluded.insert("int");
        excluded.insert("interface");
        excluded.insert("internal");
        excluded.insert("is");
        excluded.insert("lock");
        excluded.insert("long");
        excluded.insert("namespace");
        excluded.insert("new");
        excluded.insert("null");
        excluded.insert("object");
        excluded.insert("operator");
        excluded.insert("out");
        excluded.insert("override");
        excluded.insert("params");
        excluded.insert("private");
        excluded.insert("protected");
        excluded.insert("public");
        excluded.insert("readonly");
        excluded.insert("ref");
        excluded.insert("return");
        excluded.insert("sbyte");
        excluded.insert("sealed");
        excluded.insert("short");
        excluded.insert("sizeof");
        excluded.insert("stackalloc");
        excluded.insert("static");
        excluded.insert("string");
        excluded.insert("struct");
        excluded.insert("switch");
        excluded.insert("this");
        excluded.insert("throw");
        excluded.insert("true");
        excluded.insert("try");
        excluded.insert("typeof");
        excluded.insert("uint");
        excluded.insert("ulong");
        excluded.insert("unchecked");
        excluded.insert("unsafe");
        excluded.insert("ushort");
        excluded.insert("using");
        excluded.insert("virtual");
        excluded.insert("void");
        excluded.insert("volatile");
        excluded.insert("while");

        excluded.insert("var");
        excluded.insert("dynamic");
        excluded.insert("async");
        excluded.insert("await");
        excluded.insert("nameof");
        excluded.insert("when");

        excluded.insert("String");
        excluded.insert("Int32");
        excluded.insert("Console");
        excluded.insert("Task");
        excluded.insert("List");

        Self { excluded }
    }
}

impl RainbowConfig for CSharpRainbowConfig {
    fn excluded_identifiers(&self) -> &HashSet<&'static str> {
        &self.excluded
    }
}

pub struct PhpRainbowConfig {
    excluded: HashSet<&'static str>,
}

impl Default for PhpRainbowConfig {
    fn default() -> Self {
        let mut excluded = HashSet::new();

        excluded.insert("abstract");
        excluded.insert("and");
        excluded.insert("array");
        excluded.insert("as");
        excluded.insert("break");
        excluded.insert("callable");
        excluded.insert("case");
        excluded.insert("catch");
        excluded.insert("class");
        excluded.insert("clone");
        excluded.insert("const");
        excluded.insert("continue");
        excluded.insert("declare");
        excluded.insert("default");
        excluded.insert("die");
        excluded.insert("do");
        excluded.insert("echo");
        excluded.insert("else");
        excluded.insert("elseif");
        excluded.insert("empty");
        excluded.insert("enddeclare");
        excluded.insert("endfor");
        excluded.insert("endforeach");
        excluded.insert("endif");
        excluded.insert("endswitch");
        excluded.insert("endwhile");
        excluded.insert("eval");
        excluded.insert("exit");
        excluded.insert("extends");
        excluded.insert("final");
        excluded.insert("finally");
        excluded.insert("for");
        excluded.insert("foreach");
        excluded.insert("function");
        excluded.insert("global");
        excluded.insert("goto");
        excluded.insert("if");
        excluded.insert("implements");
        excluded.insert("include");
        excluded.insert("include_once");
        excluded.insert("instanceof");
        excluded.insert("insteadof");
        excluded.insert("interface");
        excluded.insert("isset");
        excluded.insert("list");
        excluded.insert("namespace");
        excluded.insert("new");
        excluded.insert("or");
        excluded.insert("print");
        excluded.insert("private");
        excluded.insert("protected");
        excluded.insert("public");
        excluded.insert("require");
        excluded.insert("require_once");
        excluded.insert("return");
        excluded.insert("static");
        excluded.insert("switch");
        excluded.insert("throw");
        excluded.insert("trait");
        excluded.insert("try");
        excluded.insert("unset");
        excluded.insert("use");
        excluded.insert("var");
        excluded.insert("while");
        excluded.insert("xor");
        excluded.insert("yield");

        excluded.insert("true");
        excluded.insert("false");
        excluded.insert("null");
        excluded.insert("self");
        excluded.insert("parent");

        Self { excluded }
    }
}

impl RainbowConfig for PhpRainbowConfig {
    fn excluded_identifiers(&self) -> &HashSet<&'static str> {
        &self.excluded
    }
}

pub struct RubyRainbowConfig {
    excluded: HashSet<&'static str>,
}

impl Default for RubyRainbowConfig {
    fn default() -> Self {
        let mut excluded = HashSet::new();

        excluded.insert("BEGIN");
        excluded.insert("END");
        excluded.insert("alias");
        excluded.insert("and");
        excluded.insert("begin");
        excluded.insert("break");
        excluded.insert("case");
        excluded.insert("class");
        excluded.insert("def");
        excluded.insert("defined?");
        excluded.insert("do");
        excluded.insert("else");
        excluded.insert("elsif");
        excluded.insert("end");
        excluded.insert("ensure");
        excluded.insert("false");
        excluded.insert("for");
        excluded.insert("if");
        excluded.insert("in");
        excluded.insert("module");
        excluded.insert("next");
        excluded.insert("nil");
        excluded.insert("not");
        excluded.insert("or");
        excluded.insert("redo");
        excluded.insert("rescue");
        excluded.insert("retry");
        excluded.insert("return");
        excluded.insert("self");
        excluded.insert("super");
        excluded.insert("then");
        excluded.insert("true");
        excluded.insert("undef");
        excluded.insert("unless");
        excluded.insert("until");
        excluded.insert("when");
        excluded.insert("while");
        excluded.insert("yield");

        excluded.insert("puts");
        excluded.insert("print");
        excluded.insert("gets");
        excluded.insert("require");
        excluded.insert("include");
        excluded.insert("extend");
        excluded.insert("attr_reader");
        excluded.insert("attr_writer");
        excluded.insert("attr_accessor");

        Self { excluded }
    }
}

impl RainbowConfig for RubyRainbowConfig {
    fn excluded_identifiers(&self) -> &HashSet<&'static str> {
        &self.excluded
    }
}

pub struct SwiftRainbowConfig {
    excluded: HashSet<&'static str>,
}

impl Default for SwiftRainbowConfig {
    fn default() -> Self {
        let mut excluded = HashSet::new();

        excluded.insert("associatedtype");
        excluded.insert("class");
        excluded.insert("deinit");
        excluded.insert("enum");
        excluded.insert("extension");
        excluded.insert("fileprivate");
        excluded.insert("func");
        excluded.insert("import");
        excluded.insert("init");
        excluded.insert("inout");
        excluded.insert("internal");
        excluded.insert("let");
        excluded.insert("open");
        excluded.insert("operator");
        excluded.insert("private");
        excluded.insert("protocol");
        excluded.insert("public");
        excluded.insert("rethrows");
        excluded.insert("static");
        excluded.insert("struct");
        excluded.insert("subscript");
        excluded.insert("typealias");
        excluded.insert("var");
        excluded.insert("break");
        excluded.insert("case");
        excluded.insert("continue");
        excluded.insert("default");
        excluded.insert("defer");
        excluded.insert("do");
        excluded.insert("else");
        excluded.insert("fallthrough");
        excluded.insert("for");
        excluded.insert("guard");
        excluded.insert("if");
        excluded.insert("in");
        excluded.insert("repeat");
        excluded.insert("return");
        excluded.insert("switch");
        excluded.insert("where");
        excluded.insert("while");
        excluded.insert("as");
        excluded.insert("catch");
        excluded.insert("false");
        excluded.insert("is");
        excluded.insert("nil");
        excluded.insert("super");
        excluded.insert("self");
        excluded.insert("Self");
        excluded.insert("throw");
        excluded.insert("throws");
        excluded.insert("true");
        excluded.insert("try");

        excluded.insert("Any");
        excluded.insert("String");
        excluded.insert("Int");
        excluded.insert("Bool");
        excluded.insert("Array");
        excluded.insert("Dictionary");
        excluded.insert("Optional");

        Self { excluded }
    }
}

impl RainbowConfig for SwiftRainbowConfig {
    fn excluded_identifiers(&self) -> &HashSet<&'static str> {
        &self.excluded
    }
}

pub struct KotlinRainbowConfig {
    excluded: HashSet<&'static str>,
}

impl Default for KotlinRainbowConfig {
    fn default() -> Self {
        let mut excluded = HashSet::new();

        excluded.insert("as");
        excluded.insert("break");
        excluded.insert("class");
        excluded.insert("continue");
        excluded.insert("do");
        excluded.insert("else");
        excluded.insert("false");
        excluded.insert("for");
        excluded.insert("fun");
        excluded.insert("if");
        excluded.insert("in");
        excluded.insert("interface");
        excluded.insert("is");
        excluded.insert("null");
        excluded.insert("object");
        excluded.insert("package");
        excluded.insert("return");
        excluded.insert("super");
        excluded.insert("this");
        excluded.insert("throw");
        excluded.insert("true");
        excluded.insert("try");
        excluded.insert("typealias");
        excluded.insert("typeof");
        excluded.insert("val");
        excluded.insert("var");
        excluded.insert("when");
        excluded.insert("while");

        excluded.insert("by");
        excluded.insert("catch");
        excluded.insert("constructor");
        excluded.insert("delegate");
        excluded.insert("dynamic");
        excluded.insert("field");
        excluded.insert("file");
        excluded.insert("finally");
        excluded.insert("get");
        excluded.insert("import");
        excluded.insert("init");
        excluded.insert("param");
        excluded.insert("property");
        excluded.insert("receiver");
        excluded.insert("set");
        excluded.insert("setparam");
        excluded.insert("where");

        excluded.insert("actual");
        excluded.insert("abstract");
        excluded.insert("annotation");
        excluded.insert("companion");
        excluded.insert("const");
        excluded.insert("crossinline");
        excluded.insert("data");
        excluded.insert("enum");
        excluded.insert("expect");
        excluded.insert("external");
        excluded.insert("final");
        excluded.insert("infix");
        excluded.insert("inline");
        excluded.insert("inner");
        excluded.insert("internal");
        excluded.insert("lateinit");
        excluded.insert("noinline");
        excluded.insert("open");
        excluded.insert("operator");
        excluded.insert("out");
        excluded.insert("override");
        excluded.insert("private");
        excluded.insert("protected");
        excluded.insert("public");
        excluded.insert("reified");
        excluded.insert("sealed");
        excluded.insert("suspend");
        excluded.insert("tailrec");
        excluded.insert("vararg");

        excluded.insert("String");
        excluded.insert("Int");
        excluded.insert("Boolean");
        excluded.insert("Any");
        excluded.insert("Unit");
        excluded.insert("Nothing");

        Self { excluded }
    }
}

impl RainbowConfig for KotlinRainbowConfig {
    fn excluded_identifiers(&self) -> &HashSet<&'static str> {
        &self.excluded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_identifier_basic() {
        assert!(is_valid_identifier("hello"));
        assert!(is_valid_identifier("my_variable"));
        assert!(is_valid_identifier("_private"));
        assert!(is_valid_identifier("snake_case_name"));
        assert!(is_valid_identifier("CamelCase"));
        assert!(is_valid_identifier("SCREAMING_SNAKE"));
        assert!(is_valid_identifier("var123"));
    }

    #[test]
    fn test_is_valid_identifier_invalid() {
        assert!(!is_valid_identifier(""));
        assert!(!is_valid_identifier(" "));
        assert!(!is_valid_identifier("a"));
        assert!(!is_valid_identifier("_"));
        assert!(!is_valid_identifier("__"));
        assert!(!is_valid_identifier("123abc"));
        assert!(!is_valid_identifier("hello world"));
        assert!(!is_valid_identifier("hello\nworld"));
        assert!(!is_valid_identifier("hello\tworld"));
    }

    #[test]
    fn test_rust_should_highlight() {
        let config = RustRainbowConfig::default();

        assert!(config.should_highlight("my_variable"));
        assert!(config.should_highlight("user_name"));
        assert!(!config.should_highlight("self"));
        assert!(!config.should_highlight("let"));
        assert!(!config.should_highlight("fn"));
        assert!(!config.should_highlight("_"));
        assert!(!config.should_highlight("a"));
    }

    #[test]
    fn test_python_should_highlight() {
        let config = PythonRainbowConfig::default();

        assert!(config.should_highlight("my_var"));
        assert!(config.should_highlight("user_data"));
        assert!(!config.should_highlight("def"));
        assert!(!config.should_highlight("class"));
        assert!(!config.should_highlight("print"));
        assert!(!config.should_highlight("len"));
    }

    #[test]
    fn test_all_configs_exclude_common_keywords() {
        let rust = RustRainbowConfig::default();
        let python = PythonRainbowConfig::default();
        let ts = TypeScriptRainbowConfig::default();

        assert!(rust.excluded_identifiers().contains("true"));
        assert!(python.excluded_identifiers().contains("true"));
        assert!(ts.excluded_identifiers().contains("true"));
    }
}
