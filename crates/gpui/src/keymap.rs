use crate::Action;
use anyhow::{anyhow, Result};
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId},
    collections::{HashMap, HashSet},
    fmt::Debug,
};
use tree_sitter::{Language, Node, Parser};

extern "C" {
    fn tree_sitter_context_predicate() -> Language;
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

#[derive(Default)]
pub struct Keymap {
    bindings: Vec<Binding>,
    binding_indices_by_action_type: HashMap<TypeId, SmallVec<[usize; 3]>>,
}

pub struct Binding {
    keystrokes: Vec<Keystroke>,
    action: Box<dyn Action>,
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
    Action(Box<dyn Action>),
}

impl Debug for MatchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatchResult::None => f.debug_struct("MatchResult::None").finish(),
            MatchResult::Pending => f.debug_struct("MatchResult::Pending").finish(),
            MatchResult::Action(action) => f
                .debug_tuple("MatchResult::Action")
                .field(&action.name())
                .finish(),
        }
    }
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

    pub fn clear_bindings(&mut self) {
        self.pending.clear();
        self.keymap.clear();
    }

    pub fn bindings_for_action_type(&self, action_type: TypeId) -> impl Iterator<Item = &Binding> {
        self.keymap.bindings_for_action_type(action_type)
    }

    pub fn clear_pending(&mut self) {
        self.pending.clear();
    }

    pub fn has_pending_keystrokes(&self) -> bool {
        !self.pending.is_empty()
    }

    pub fn push_keystroke(
        &mut self,
        keystroke: Keystroke,
        view_id: usize,
        cx: &Context,
    ) -> MatchResult {
        let pending = self.pending.entry(view_id).or_default();

        if let Some(pending_ctx) = pending.context.as_ref() {
            if pending_ctx != cx {
                pending.keystrokes.clear();
            }
        }

        pending.keystrokes.push(keystroke);

        let mut retain_pending = false;
        for binding in self.keymap.bindings.iter().rev() {
            if binding.keystrokes.starts_with(&pending.keystrokes)
                && binding.context.as_ref().map(|c| c.eval(cx)).unwrap_or(true)
            {
                if binding.keystrokes.len() == pending.keystrokes.len() {
                    self.pending.remove(&view_id);
                    return MatchResult::Action(binding.action.boxed_clone());
                } else {
                    retain_pending = true;
                    pending.context = Some(cx.clone());
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
        let mut binding_indices_by_action_type = HashMap::new();
        for (ix, binding) in bindings.iter().enumerate() {
            binding_indices_by_action_type
                .entry(binding.action.as_any().type_id())
                .or_insert_with(|| SmallVec::new())
                .push(ix);
        }
        Self {
            binding_indices_by_action_type,
            bindings,
        }
    }

    fn bindings_for_action_type<'a>(
        &'a self,
        action_type: TypeId,
    ) -> impl Iterator<Item = &'a Binding> {
        self.binding_indices_by_action_type
            .get(&action_type)
            .map(SmallVec::as_slice)
            .unwrap_or(&[])
            .iter()
            .map(|ix| &self.bindings[*ix])
    }

    fn add_bindings<T: IntoIterator<Item = Binding>>(&mut self, bindings: T) {
        for binding in bindings {
            self.binding_indices_by_action_type
                .entry(binding.action.as_any().type_id())
                .or_default()
                .push(self.bindings.len());
            self.bindings.push(binding);
        }
    }

    fn clear(&mut self) {
        self.bindings.clear();
        self.binding_indices_by_action_type.clear();
    }
}

impl Binding {
    pub fn new<A: Action>(keystrokes: &str, action: A, context: Option<&str>) -> Self {
        Self::load(keystrokes, Box::new(action), context).unwrap()
    }

    pub fn load(keystrokes: &str, action: Box<dyn Action>, context: Option<&str>) -> Result<Self> {
        let context = if let Some(context) = context {
            Some(ContextPredicate::parse(context)?)
        } else {
            None
        };

        let keystrokes = keystrokes
            .split_whitespace()
            .map(|key| Keystroke::parse(key))
            .collect::<Result<_>>()?;

        Ok(Self {
            keystrokes,
            action,
            context,
        })
    }

    pub fn keystrokes(&self) -> &[Keystroke] {
        &self.keystrokes
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

    pub fn modified(&self) -> bool {
        self.ctrl || self.alt || self.shift || self.cmd
    }
}

impl Context {
    pub fn extend(&mut self, other: &Context) {
        for v in &other.set {
            self.set.insert(v.clone());
        }
        for (k, v) in &other.map {
            self.map.insert(k.clone(), v.clone());
        }
    }
}

impl ContextPredicate {
    fn parse(source: &str) -> anyhow::Result<Self> {
        let mut parser = Parser::new();
        let language = unsafe { tree_sitter_context_predicate() };
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

    fn eval(&self, cx: &Context) -> bool {
        match self {
            Self::Identifier(name) => cx.set.contains(name.as_str()),
            Self::Equal(left, right) => cx
                .map
                .get(left)
                .map(|value| value == right)
                .unwrap_or(false),
            Self::NotEqual(left, right) => {
                cx.map.get(left).map(|value| value != right).unwrap_or(true)
            }
            Self::Not(pred) => !pred.eval(cx),
            Self::And(left, right) => left.eval(cx) && right.eval(cx),
            Self::Or(left, right) => left.eval(cx) || right.eval(cx),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use crate::{actions, impl_actions};

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
        #[derive(Clone, Deserialize, PartialEq, Eq, Debug)]
        pub struct A(pub String);
        impl_actions!(test, [A]);
        actions!(test, [B, Ab]);

        #[derive(Clone, Debug, Eq, PartialEq)]
        struct ActionArg {
            a: &'static str,
        }

        let keymap = Keymap::new(vec![
            Binding::new("a", A("x".to_string()), Some("a")),
            Binding::new("b", B, Some("a")),
            Binding::new("a b", Ab, Some("a || b")),
        ]);

        let mut ctx_a = Context::default();
        ctx_a.set.insert("a".into());

        let mut ctx_b = Context::default();
        ctx_b.set.insert("b".into());

        let mut matcher = Matcher::new(keymap);

        // Basic match
        assert_eq!(
            downcast(&matcher.test_keystroke("a", 1, &ctx_a)),
            Some(&A("x".to_string()))
        );

        // Multi-keystroke match
        assert!(matcher.test_keystroke("a", 1, &ctx_b).is_none());
        assert_eq!(downcast(&matcher.test_keystroke("b", 1, &ctx_b)), Some(&Ab));

        // Failed matches don't interfere with matching subsequent keys
        assert!(matcher.test_keystroke("x", 1, &ctx_a).is_none());
        assert_eq!(
            downcast(&matcher.test_keystroke("a", 1, &ctx_a)),
            Some(&A("x".to_string()))
        );

        // Pending keystrokes are cleared when the context changes
        assert!(&matcher.test_keystroke("a", 1, &ctx_b).is_none());
        assert_eq!(downcast(&matcher.test_keystroke("b", 1, &ctx_a)), Some(&B));

        let mut ctx_c = Context::default();
        ctx_c.set.insert("c".into());

        // Pending keystrokes are maintained per-view
        assert!(matcher.test_keystroke("a", 1, &ctx_b).is_none());
        assert!(matcher.test_keystroke("a", 2, &ctx_c).is_none());
        assert_eq!(downcast(&matcher.test_keystroke("b", 1, &ctx_b)), Some(&Ab));

        Ok(())
    }

    fn downcast<'a, A: Action>(action: &'a Option<Box<dyn Action>>) -> Option<&'a A> {
        action
            .as_ref()
            .and_then(|action| action.as_any().downcast_ref())
    }

    impl Matcher {
        fn test_keystroke(
            &mut self,
            keystroke: &str,
            view_id: usize,
            cx: &Context,
        ) -> Option<Box<dyn Action>> {
            if let MatchResult::Action(action) =
                self.push_keystroke(Keystroke::parse(keystroke).unwrap(), view_id, cx)
            {
                Some(action.boxed_clone())
            } else {
                None
            }
        }
    }
}
