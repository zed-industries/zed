use crate::Action;
use anyhow::{anyhow, Result};
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId},
    collections::{HashMap, HashSet},
    fmt::{Debug, Write},
};
use tree_sitter::{Language, Node, Parser};

extern "C" {
    fn tree_sitter_context_predicate() -> Language;
}

pub struct Matcher {
    pending_views: HashMap<usize, Context>,
    pending_keystrokes: Vec<Keystroke>,
    keymap: Keymap,
}

#[derive(Default)]
pub struct Keymap {
    bindings: Vec<Binding>,
    binding_indices_by_action_type: HashMap<TypeId, SmallVec<[usize; 3]>>,
}

pub struct Binding {
    keystrokes: SmallVec<[Keystroke; 2]>,
    action: Box<dyn Action>,
    context_predicate: Option<ContextPredicate>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Keystroke {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub cmd: bool,
    pub function: bool,
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
    Matches(Vec<(usize, Box<dyn Action>)>),
}

impl Debug for MatchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatchResult::None => f.debug_struct("MatchResult::None").finish(),
            MatchResult::Pending => f.debug_struct("MatchResult::Pending").finish(),
            MatchResult::Matches(matches) => f
                .debug_list()
                .entries(
                    matches
                        .iter()
                        .map(|(view_id, action)| format!("{view_id}, {}", action.name())),
                )
                .finish(),
        }
    }
}

impl PartialEq for MatchResult {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (MatchResult::None, MatchResult::None) => true,
            (MatchResult::Pending, MatchResult::Pending) => true,
            (MatchResult::Matches(matches), MatchResult::Matches(other_matches)) => {
                matches.len() == other_matches.len()
                    && matches.iter().zip(other_matches.iter()).all(
                        |((view_id, action), (other_view_id, other_action))| {
                            view_id == other_view_id && action.eq(other_action.as_ref())
                        },
                    )
            }
            _ => false,
        }
    }
}

impl Eq for MatchResult {}

impl Matcher {
    pub fn new(keymap: Keymap) -> Self {
        Self {
            pending_views: HashMap::new(),
            pending_keystrokes: Vec::new(),
            keymap,
        }
    }

    pub fn set_keymap(&mut self, keymap: Keymap) {
        self.clear_pending();
        self.keymap = keymap;
    }

    pub fn add_bindings<T: IntoIterator<Item = Binding>>(&mut self, bindings: T) {
        self.clear_pending();
        self.keymap.add_bindings(bindings);
    }

    pub fn clear_bindings(&mut self) {
        self.clear_pending();
        self.keymap.clear();
    }

    pub fn bindings_for_action_type(&self, action_type: TypeId) -> impl Iterator<Item = &Binding> {
        self.keymap.bindings_for_action_type(action_type)
    }

    pub fn clear_pending(&mut self) {
        self.pending_keystrokes.clear();
        self.pending_views.clear();
    }

    pub fn has_pending_keystrokes(&self) -> bool {
        !self.pending_keystrokes.is_empty()
    }

    pub fn push_keystroke(
        &mut self,
        keystroke: Keystroke,
        dispatch_path: Vec<(usize, Context)>,
    ) -> MatchResult {
        let mut any_pending = false;
        let mut matched_bindings = Vec::new();

        let first_keystroke = self.pending_keystrokes.is_empty();
        self.pending_keystrokes.push(keystroke);

        for (view_id, context) in dispatch_path {
            // Don't require pending view entry if there are no pending keystrokes
            if !first_keystroke && !self.pending_views.contains_key(&view_id) {
                continue;
            }

            // If there is a previous view context, invalidate that view if it
            // has changed
            if let Some(previous_view_context) = self.pending_views.remove(&view_id) {
                if previous_view_context != context {
                    continue;
                }
            }

            // Find the bindings which map the pending keystrokes and current context
            for binding in self.keymap.bindings.iter().rev() {
                if binding.keystrokes.starts_with(&self.pending_keystrokes)
                    && binding
                        .context_predicate
                        .as_ref()
                        .map(|c| c.eval(&context))
                        .unwrap_or(true)
                {
                    // If the binding is completed, push it onto the matches list
                    if binding.keystrokes.len() == self.pending_keystrokes.len() {
                        matched_bindings.push((view_id, binding.action.boxed_clone()));
                    } else {
                        // Otherwise, the binding is still pending
                        self.pending_views.insert(view_id, context.clone());
                        any_pending = true;
                    }
                }
            }
        }

        if !any_pending {
            self.clear_pending();
        }

        if !matched_bindings.is_empty() {
            MatchResult::Matches(matched_bindings)
        } else if any_pending {
            MatchResult::Pending
        } else {
            MatchResult::None
        }
    }

    pub fn keystrokes_for_action(
        &self,
        action: &dyn Action,
        cx: &Context,
    ) -> Option<SmallVec<[Keystroke; 2]>> {
        for binding in self.keymap.bindings.iter().rev() {
            if binding.action.eq(action)
                && binding
                    .context_predicate
                    .as_ref()
                    .map_or(true, |predicate| predicate.eval(cx))
            {
                return Some(binding.keystrokes.clone());
            }
        }
        None
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
                .or_insert_with(SmallVec::new)
                .push(ix);
        }
        Self {
            binding_indices_by_action_type,
            bindings,
        }
    }

    fn bindings_for_action_type(&self, action_type: TypeId) -> impl Iterator<Item = &'_ Binding> {
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
            .map(Keystroke::parse)
            .collect::<Result<_>>()?;

        Ok(Self {
            keystrokes,
            action,
            context_predicate: context,
        })
    }

    pub fn keystrokes(&self) -> &[Keystroke] {
        &self.keystrokes
    }

    pub fn action(&self) -> &dyn Action {
        self.action.as_ref()
    }
}

impl Keystroke {
    pub fn parse(source: &str) -> anyhow::Result<Self> {
        let mut ctrl = false;
        let mut alt = false;
        let mut shift = false;
        let mut cmd = false;
        let mut function = false;
        let mut key = None;

        let mut components = source.split('-').peekable();
        while let Some(component) = components.next() {
            match component {
                "ctrl" => ctrl = true,
                "alt" => alt = true,
                "shift" => shift = true,
                "cmd" => cmd = true,
                "fn" => function = true,
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

        let key = key.ok_or_else(|| anyhow!("Invalid keystroke `{}`", source))?;

        Ok(Keystroke {
            ctrl,
            alt,
            shift,
            cmd,
            function,
            key,
        })
    }

    pub fn modified(&self) -> bool {
        self.ctrl || self.alt || self.shift || self.cmd
    }
}

impl std::fmt::Display for Keystroke {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.ctrl {
            f.write_char('^')?;
        }
        if self.alt {
            f.write_char('⎇')?;
        }
        if self.cmd {
            f.write_char('⌘')?;
        }
        if self.shift {
            f.write_char('⇧')?;
        }
        let key = match self.key.as_str() {
            "backspace" => '⌫',
            "up" => '↑',
            "down" => '↓',
            "left" => '←',
            "right" => '→',
            "tab" => '⇥',
            "escape" => '⎋',
            key => {
                if key.len() == 1 {
                    key.chars().next().unwrap().to_ascii_uppercase()
                } else {
                    return f.write_str(key);
                }
            }
        };
        f.write_char(key)
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
            "source" => Self::from_node(node.child(0).ok_or_else(|| anyhow!(parse_error))?, source),
            "identifier" => Ok(Self::Identifier(node.utf8_text(source)?.into())),
            "not" => {
                let child = Self::from_node(
                    node.child_by_field_name("expression")
                        .ok_or_else(|| anyhow!(parse_error))?,
                    source,
                )?;
                Ok(Self::Not(Box::new(child)))
            }
            "and" | "or" => {
                let left = Box::new(Self::from_node(
                    node.child_by_field_name("left")
                        .ok_or_else(|| anyhow!(parse_error))?,
                    source,
                )?);
                let right = Box::new(Self::from_node(
                    node.child_by_field_name("right")
                        .ok_or_else(|| anyhow!(parse_error))?,
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
                    .ok_or_else(|| anyhow!(parse_error))?
                    .utf8_text(source)?
                    .into();
                let right = node
                    .child_by_field_name("right")
                    .ok_or_else(|| anyhow!(parse_error))?
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
                    .ok_or_else(|| anyhow!(parse_error))?,
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
    use anyhow::Result;
    use serde::Deserialize;

    use crate::{actions, impl_actions};

    use super::*;

    #[test]
    fn test_push_keystroke() -> Result<()> {
        actions!(test, [B, AB, C, D, DA]);

        let mut ctx1 = Context::default();
        ctx1.set.insert("1".into());

        let mut ctx2 = Context::default();
        ctx2.set.insert("2".into());

        let dispatch_path = vec![(2, ctx2), (1, ctx1)];

        let keymap = Keymap::new(vec![
            Binding::new("a b", AB, Some("1")),
            Binding::new("b", B, Some("2")),
            Binding::new("c", C, Some("2")),
            Binding::new("d", D, Some("1")),
            Binding::new("d", D, Some("2")),
            Binding::new("d a", DA, Some("2")),
        ]);

        let mut matcher = Matcher::new(keymap);

        // Binding with pending prefix always takes precedence
        assert_eq!(
            matcher.push_keystroke(Keystroke::parse("a")?, dispatch_path.clone()),
            MatchResult::Pending,
        );
        // B alone doesn't match because a was pending, so AB is returned instead
        assert_eq!(
            matcher.push_keystroke(Keystroke::parse("b")?, dispatch_path.clone()),
            MatchResult::Matches(vec![(1, Box::new(AB))]),
        );
        assert!(!matcher.has_pending_keystrokes());

        // Without an a prefix, B is dispatched like expected
        assert_eq!(
            matcher.push_keystroke(Keystroke::parse("b")?, dispatch_path.clone()),
            MatchResult::Matches(vec![(2, Box::new(B))]),
        );
        assert!(!matcher.has_pending_keystrokes());

        // If a is prefixed, C will not be dispatched because there
        // was a pending binding for it
        assert_eq!(
            matcher.push_keystroke(Keystroke::parse("a")?, dispatch_path.clone()),
            MatchResult::Pending,
        );
        assert_eq!(
            matcher.push_keystroke(Keystroke::parse("c")?, dispatch_path.clone()),
            MatchResult::None,
        );
        assert!(!matcher.has_pending_keystrokes());

        // If a single keystroke matches multiple bindings in the tree
        // all of them are returned so that we can fallback if the action
        // handler decides to propagate the action
        assert_eq!(
            matcher.push_keystroke(Keystroke::parse("d")?, dispatch_path.clone()),
            MatchResult::Matches(vec![(2, Box::new(D)), (1, Box::new(D))]),
        );
        // If none of the d action handlers consume the binding, a pending
        // binding may then be used
        assert_eq!(
            matcher.push_keystroke(Keystroke::parse("a")?, dispatch_path.clone()),
            MatchResult::Matches(vec![(2, Box::new(DA))]),
        );
        assert!(!matcher.has_pending_keystrokes());

        Ok(())
    }

    #[test]
    fn test_keystroke_parsing() -> Result<()> {
        assert_eq!(
            Keystroke::parse("ctrl-p")?,
            Keystroke {
                key: "p".into(),
                ctrl: true,
                alt: false,
                shift: false,
                cmd: false,
                function: false,
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
                function: false,
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
                function: false,
            }
        );

        Ok(())
    }

    #[test]
    fn test_context_predicate_parsing() -> Result<()> {
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
    fn test_context_predicate_eval() -> Result<()> {
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
    fn test_matcher() -> Result<()> {
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
            matcher.push_keystroke(Keystroke::parse("a")?, vec![(1, ctx_a.clone())]),
            MatchResult::Matches(vec![(1, Box::new(A("x".to_string())))])
        );
        matcher.clear_pending();

        // Multi-keystroke match
        assert_eq!(
            matcher.push_keystroke(Keystroke::parse("a")?, vec![(1, ctx_b.clone())]),
            MatchResult::Pending
        );
        assert_eq!(
            matcher.push_keystroke(Keystroke::parse("b")?, vec![(1, ctx_b.clone())]),
            MatchResult::Matches(vec![(1, Box::new(Ab))])
        );
        matcher.clear_pending();

        // Failed matches don't interfere with matching subsequent keys
        assert_eq!(
            matcher.push_keystroke(Keystroke::parse("x")?, vec![(1, ctx_a.clone())]),
            MatchResult::None
        );
        assert_eq!(
            matcher.push_keystroke(Keystroke::parse("a")?, vec![(1, ctx_a.clone())]),
            MatchResult::Matches(vec![(1, Box::new(A("x".to_string())))])
        );
        matcher.clear_pending();

        // Pending keystrokes are cleared when the context changes
        assert_eq!(
            matcher.push_keystroke(Keystroke::parse("a")?, vec![(1, ctx_b.clone())]),
            MatchResult::Pending
        );
        assert_eq!(
            matcher.push_keystroke(Keystroke::parse("b")?, vec![(1, ctx_a.clone())]),
            MatchResult::None
        );
        matcher.clear_pending();

        let mut ctx_c = Context::default();
        ctx_c.set.insert("c".into());

        // Pending keystrokes are maintained per-view
        assert_eq!(
            matcher.push_keystroke(
                Keystroke::parse("a")?,
                vec![(1, ctx_b.clone()), (2, ctx_c.clone())]
            ),
            MatchResult::Pending
        );
        assert_eq!(
            matcher.push_keystroke(Keystroke::parse("b")?, vec![(1, ctx_b.clone())]),
            MatchResult::Matches(vec![(1, Box::new(Ab))])
        );

        Ok(())
    }
}
