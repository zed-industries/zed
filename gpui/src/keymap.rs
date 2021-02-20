use anyhow::anyhow;
use std::{
    any::Any,
    collections::{HashMap, HashSet},
};
use tree_sitter::{Language, Node, Parser};

extern "C" {
    fn tree_sitter_zed_context_predicate() -> Language;
}

pub struct Matcher {
    pending: HashMap<usize, Pending>,
    keymap: Keymap,
}

#[derive(Default)]
struct Pending {
    keystrokes: Vec<Keystroke>,
    context: Option<Context>,
}

pub struct Keymap(Vec<Binding>);

pub struct Binding {
    keystrokes: Vec<Keystroke>,
    action: String,
    action_arg: Option<Box<dyn ActionArg>>,
    context: Option<ContextPredicate>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Keystroke {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub cmd: bool,
    pub key: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Context {
    pub set: HashSet<String>,
    pub map: HashMap<String, String>,
}

#[derive(Debug, Eq, PartialEq)]
enum ContextPredicate {
    Identifier(String),
    Equal(String, String),
    NotEqual(String, String),
    Not(Box<ContextPredicate>),
    And(Box<ContextPredicate>, Box<ContextPredicate>),
    Or(Box<ContextPredicate>, Box<ContextPredicate>),
}

trait ActionArg {
    fn boxed_clone(&self) -> Box<dyn Any>;
}

impl<T> ActionArg for T
where
    T: 'static + Any + Clone,
{
    fn boxed_clone(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }
}

pub enum MatchResult {
    None,
    Pending,
    Action {
        name: String,
        arg: Option<Box<dyn Any>>,
    },
}

impl Matcher {
    pub fn new(keymap: Keymap) -> Self {
        Self {
            pending: HashMap::new(),
            keymap,
        }
    }

    pub fn set_keymap(&mut self, keymap: Keymap) {
        self.pending.clear();
        self.keymap = keymap;
    }

    pub fn add_bindings<T: IntoIterator<Item = Binding>>(&mut self, bindings: T) {
        self.pending.clear();
        self.keymap.add_bindings(bindings);
    }

    pub fn push_keystroke(
        &mut self,
        keystroke: Keystroke,
        view_id: usize,
        ctx: &Context,
    ) -> MatchResult {
        let pending = self.pending.entry(view_id).or_default();

        if let Some(pending_ctx) = pending.context.as_ref() {
            if pending_ctx != ctx {
                pending.keystrokes.clear();
            }
        }

        pending.keystrokes.push(keystroke);

        let mut retain_pending = false;
        for binding in self.keymap.0.iter().rev() {
            if binding.keystrokes.starts_with(&pending.keystrokes)
                && binding
                    .context
                    .as_ref()
                    .map(|c| c.eval(ctx))
                    .unwrap_or(true)
            {
                if binding.keystrokes.len() == pending.keystrokes.len() {
                    self.pending.remove(&view_id);
                    return MatchResult::Action {
                        name: binding.action.clone(),
                        arg: binding.action_arg.as_ref().map(|arg| (*arg).boxed_clone()),
                    };
                } else {
                    retain_pending = true;
                    pending.context = Some(ctx.clone());
                }
            }
        }

        if retain_pending {
            MatchResult::Pending
        } else {
            self.pending.remove(&view_id);
            MatchResult::None
        }
    }
}

impl Default for Matcher {
    fn default() -> Self {
        Self::new(Keymap::default())
    }
}

impl Keymap {
    pub fn new(bindings: Vec<Binding>) -> Self {
        Self(bindings)
    }

    fn add_bindings<T: IntoIterator<Item = Binding>>(&mut self, bindings: T) {
        self.0.extend(bindings.into_iter());
    }
}

impl Default for Keymap {
    fn default() -> Self {
        Self(vec![
            Binding::new("up", "menu:select_prev", Some("menu")),
            Binding::new("ctrl-p", "menu:select_prev", Some("menu")),
            Binding::new("down", "menu:select_next", Some("menu")),
            Binding::new("ctrl-n", "menu:select_next", Some("menu")),
        ])
    }
}

impl Binding {
    pub fn new<S: Into<String>>(keystrokes: &str, action: S, context: Option<&str>) -> Self {
        let context = if let Some(context) = context {
            Some(ContextPredicate::parse(context).unwrap())
        } else {
            None
        };

        Self {
            keystrokes: keystrokes
                .split_whitespace()
                .map(|key| Keystroke::parse(key).unwrap())
                .collect(),
            action: action.into(),
            action_arg: None,
            context,
        }
    }

    pub fn with_arg<T: 'static + Any + Clone>(mut self, arg: T) -> Self {
        self.action_arg = Some(Box::new(arg));
        self
    }
}

impl Keystroke {
    pub fn parse(source: &str) -> anyhow::Result<Self> {
        let mut ctrl = false;
        let mut alt = false;
        let mut shift = false;
        let mut cmd = false;
        let mut key = None;

        let mut components = source.split("-").peekable();
        while let Some(component) = components.next() {
            match component {
                "ctrl" => ctrl = true,
                "alt" => alt = true,
                "shift" => shift = true,
                "cmd" => cmd = true,
                _ => {
                    if let Some(component) = components.peek() {
                        if component.is_empty() && source.ends_with('-') {
                            key = Some(String::from("-"));
                            break;
                        } else {
                            return Err(anyhow!("Invalid keystroke `{}`", source));
                        }
                    } else {
                        key = Some(String::from(component));
                    }
                }
            }
        }

        Ok(Keystroke {
            ctrl,
            alt,
            shift,
            cmd,
            key: key.unwrap(),
        })
    }
}

impl Context {
    pub fn extend(&mut self, other: Context) {
        for v in other.set {
            self.set.insert(v);
        }
        for (k, v) in other.map {
            self.map.insert(k, v);
        }
    }
}

impl ContextPredicate {
    fn parse(source: &str) -> anyhow::Result<Self> {
        let mut parser = Parser::new();
        let language = unsafe { tree_sitter_zed_context_predicate() };
        parser.set_language(language).unwrap();
        let source = source.as_bytes();
        let tree = parser.parse(source, None).unwrap();
        Self::from_node(tree.root_node(), source)
    }

    fn from_node(node: Node, source: &[u8]) -> anyhow::Result<Self> {
        let parse_error = "error parsing context predicate";
        let kind = node.kind();

        match kind {
            "source" => Self::from_node(node.child(0).ok_or(anyhow!(parse_error))?, source),
            "identifier" => Ok(Self::Identifier(node.utf8_text(source)?.into())),
            "not" => {
                let child = Self::from_node(
                    node.child_by_field_name("expression")
                        .ok_or(anyhow!(parse_error))?,
                    source,
                )?;
                Ok(Self::Not(Box::new(child)))
            }
            "and" | "or" => {
                let left = Box::new(Self::from_node(
                    node.child_by_field_name("left")
                        .ok_or(anyhow!(parse_error))?,
                    source,
                )?);
                let right = Box::new(Self::from_node(
                    node.child_by_field_name("right")
                        .ok_or(anyhow!(parse_error))?,
                    source,
                )?);
                if kind == "and" {
                    Ok(Self::And(left, right))
                } else {
                    Ok(Self::Or(left, right))
                }
            }
            "equal" | "not_equal" => {
                let left = node
                    .child_by_field_name("left")
                    .ok_or(anyhow!(parse_error))?
                    .utf8_text(source)?
                    .into();
                let right = node
                    .child_by_field_name("right")
                    .ok_or(anyhow!(parse_error))?
                    .utf8_text(source)?
                    .into();
                if kind == "equal" {
                    Ok(Self::Equal(left, right))
                } else {
                    Ok(Self::NotEqual(left, right))
                }
            }
            "parenthesized" => Self::from_node(
                node.child_by_field_name("expression")
                    .ok_or(anyhow!(parse_error))?,
                source,
            ),
            _ => Err(anyhow!(parse_error)),
        }
    }

    fn eval(&self, ctx: &Context) -> bool {
        match self {
            Self::Identifier(name) => ctx.set.contains(name.as_str()),
            Self::Equal(left, right) => ctx
                .map
                .get(left)
                .map(|value| value == right)
                .unwrap_or(false),
            Self::NotEqual(left, right) => ctx
                .map
                .get(left)
                .map(|value| value != right)
                .unwrap_or(true),
            Self::Not(pred) => !pred.eval(ctx),
            Self::And(left, right) => left.eval(ctx) && right.eval(ctx),
            Self::Or(left, right) => left.eval(ctx) || right.eval(ctx),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keystroke_parsing() -> anyhow::Result<()> {
        assert_eq!(
            Keystroke::parse("ctrl-p")?,
            Keystroke {
                key: "p".into(),
                ctrl: true,
                alt: false,
                shift: false,
                cmd: false,
            }
        );

        assert_eq!(
            Keystroke::parse("alt-shift-down")?,
            Keystroke {
                key: "down".into(),
                ctrl: false,
                alt: true,
                shift: true,
                cmd: false,
            }
        );

        assert_eq!(
            Keystroke::parse("shift-cmd--")?,
            Keystroke {
                key: "-".into(),
                ctrl: false,
                alt: false,
                shift: true,
                cmd: true,
            }
        );

        Ok(())
    }

    #[test]
    fn test_context_predicate_parsing() -> anyhow::Result<()> {
        use ContextPredicate::*;

        assert_eq!(
            ContextPredicate::parse("a && (b == c || d != e)")?,
            And(
                Box::new(Identifier("a".into())),
                Box::new(Or(
                    Box::new(Equal("b".into(), "c".into())),
                    Box::new(NotEqual("d".into(), "e".into())),
                ))
            )
        );

        assert_eq!(
            ContextPredicate::parse("!a")?,
            Not(Box::new(Identifier("a".into())),)
        );

        Ok(())
    }

    #[test]
    fn test_context_predicate_eval() -> anyhow::Result<()> {
        let predicate = ContextPredicate::parse("a && b || c == d")?;

        let mut context = Context::default();
        context.set.insert("a".into());
        assert!(!predicate.eval(&context));

        context.set.insert("b".into());
        assert!(predicate.eval(&context));

        context.set.remove("b");
        context.map.insert("c".into(), "x".into());
        assert!(!predicate.eval(&context));

        context.map.insert("c".into(), "d".into());
        assert!(predicate.eval(&context));

        let predicate = ContextPredicate::parse("!a")?;
        assert!(predicate.eval(&Context::default()));

        Ok(())
    }

    #[test]
    fn test_matcher() -> anyhow::Result<()> {
        #[derive(Clone, Debug, Eq, PartialEq)]
        struct ActionArg {
            a: &'static str,
        }

        let keymap = Keymap(vec![
            Binding::new("a", "a", Some("a")).with_arg(ActionArg { a: "b" }),
            Binding::new("b", "b", Some("a")),
            Binding::new("a b", "a_b", Some("a || b")),
        ]);

        let mut ctx_a = Context::default();
        ctx_a.set.insert("a".into());

        let mut ctx_b = Context::default();
        ctx_b.set.insert("b".into());

        let mut matcher = Matcher::new(keymap);

        // Basic match
        assert_eq!(
            matcher.test_keystroke("a", 1, &ctx_a),
            Some(("a".to_string(), Some(ActionArg { a: "b" })))
        );

        // Multi-keystroke match
        assert_eq!(matcher.test_keystroke::<()>("a", 1, &ctx_b), None);
        assert_eq!(
            matcher.test_keystroke::<()>("b", 1, &ctx_b),
            Some(("a_b".to_string(), None))
        );

        // Failed matches don't interfere with matching subsequent keys
        assert_eq!(matcher.test_keystroke::<()>("x", 1, &ctx_a), None);
        assert_eq!(
            matcher.test_keystroke("a", 1, &ctx_a),
            Some(("a".to_string(), Some(ActionArg { a: "b" })))
        );

        // Pending keystrokes are cleared when the context changes
        assert_eq!(matcher.test_keystroke::<()>("a", 1, &ctx_b), None);
        assert_eq!(
            matcher.test_keystroke::<()>("b", 1, &ctx_a),
            Some(("b".to_string(), None))
        );

        let mut ctx_c = Context::default();
        ctx_c.set.insert("c".into());

        // Pending keystrokes are maintained per-view
        assert_eq!(matcher.test_keystroke::<()>("a", 1, &ctx_b), None);
        assert_eq!(matcher.test_keystroke::<()>("a", 2, &ctx_c), None);
        assert_eq!(
            matcher.test_keystroke::<()>("b", 1, &ctx_b),
            Some(("a_b".to_string(), None))
        );

        Ok(())
    }

    impl Matcher {
        fn test_keystroke<A: Any + Clone>(
            &mut self,
            keystroke: &str,
            view_id: usize,
            ctx: &Context,
        ) -> Option<(String, Option<A>)> {
            if let MatchResult::Action { name, arg } =
                self.push_keystroke(Keystroke::parse(keystroke).unwrap(), view_id, ctx)
            {
                Some((name, arg.and_then(|arg| arg.downcast_ref::<A>().cloned())))
            } else {
                None
            }
        }
    }
}
