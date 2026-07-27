//! Hierarchical navigation for pickers: a stack of pages whose rows either do
//! something or open another page.
//!
//! This module is deliberately data-only. Delegates keep ownership of matching
//! and rendering so each one can rank its rows appropriately — git refs want a
//! recency tiebreak, a list of commands wants plain fuzzy score — while this
//! module only tracks where the user is and what is on the page they're looking
//! at.

use std::rc::Rc;

use anyhow::Result;
use gpui::{Action, App, SharedString, Task, Window};
use ui::{Color, IconName};

/// What activating a row does.
pub enum Activate {
    /// A group heading. Never selectable, and hidden while the user is
    /// searching, since a heading can't answer a query.
    Section,
    /// Dispatch an action against the picker's focus handle, then dismiss.
    Action(Box<dyn Action>),
    /// Run a closure, then dismiss. Receives the query that was in the search
    /// field, so a row can act on what the user typed — which is how the
    /// palette offers "create a branch called <this>" without a modal.
    Run(Rc<dyn Fn(&str, &mut Window, &mut App)>),
    /// Descend into another page, leaving the picker open.
    Page(Children),
}

impl Clone for Activate {
    fn clone(&self) -> Self {
        match self {
            Self::Section => Self::Section,
            Self::Action(action) => Self::Action(action.boxed_clone()),
            Self::Run(run) => Self::Run(run.clone()),
            Self::Page(children) => Self::Page(children.clone()),
        }
    }
}

/// How a page's rows are produced.
#[derive(Clone)]
pub enum Children {
    /// Already in memory.
    Ready(Rc<[Node]>),
    /// Resolved the first time the page is opened. The work happens in the
    /// returned task, so building a page over a repository with 100k refs never
    /// runs on the frame that opened it.
    Deferred(Rc<dyn Fn(&mut App) -> Task<Result<Vec<Node>>>>),
}

impl Children {
    pub fn ready(nodes: impl Into<Rc<[Node]>>) -> Self {
        Self::Ready(nodes.into())
    }

    pub fn deferred(resolve: impl Fn(&mut App) -> Task<Result<Vec<Node>>> + 'static) -> Self {
        Self::Deferred(Rc::new(resolve))
    }
}

#[derive(Clone)]
pub struct Node {
    /// Stable across refreshes, so the selection can be restored by identity
    /// rather than by index after the underlying data changes.
    pub id: SharedString,
    pub label: SharedString,
    /// Matched by search but never rendered. Lets "sync" find "Pull", or a
    /// branch's full ref name match when only the short name is displayed.
    pub keywords: Option<SharedString>,
    /// Muted second line.
    pub detail: Option<SharedString>,
    pub icon: Option<IconName>,
    pub icon_color: Option<Color>,
    /// Muted text at the trailing edge, e.g. an ahead/behind count.
    pub trailing: Option<SharedString>,
    pub activate: Activate,
    /// The page this row's verbs live on, reached with Right or Tab. Distinct
    /// from `Activate::Page`: a row can both do something on Enter and offer a
    /// verb page, which is how a branch checks out on Enter but still exposes
    /// rename/delete/compare.
    pub submenu: Option<Children>,
    /// Unavailable rows stay visible with a reason attached. Hiding them makes
    /// the palette look like it lost a feature; explaining them teaches why the
    /// verb doesn't apply right now.
    pub disabled_reason: Option<SharedString>,
}

impl Node {
    fn new(
        id: impl Into<SharedString>,
        label: impl Into<SharedString>,
        activate: Activate,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            keywords: None,
            detail: None,
            icon: None,
            icon_color: None,
            trailing: None,
            activate,
            submenu: None,
            disabled_reason: None,
        }
    }

    pub fn section(label: impl Into<SharedString>) -> Self {
        let label = label.into();
        Self::new(format!("section:{label}"), label, Activate::Section)
    }

    pub fn action(
        id: impl Into<SharedString>,
        label: impl Into<SharedString>,
        action: Box<dyn Action>,
    ) -> Self {
        Self::new(id, label, Activate::Action(action))
    }

    pub fn run(
        id: impl Into<SharedString>,
        label: impl Into<SharedString>,
        run: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self::new(id, label, Activate::Run(Rc::new(run)))
    }

    pub fn page(
        id: impl Into<SharedString>,
        label: impl Into<SharedString>,
        children: Children,
    ) -> Self {
        Self::new(id, label, Activate::Page(children))
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn icon_color(mut self, color: Color) -> Self {
        self.icon_color = Some(color);
        self
    }

    pub fn detail(mut self, detail: impl Into<SharedString>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    pub fn keywords(mut self, keywords: impl Into<SharedString>) -> Self {
        self.keywords = Some(keywords.into());
        self
    }

    pub fn trailing(mut self, trailing: impl Into<SharedString>) -> Self {
        self.trailing = Some(trailing.into());
        self
    }

    pub fn submenu(mut self, children: Children) -> Self {
        self.submenu = Some(children);
        self
    }

    pub fn disabled(mut self, reason: impl Into<SharedString>) -> Self {
        self.disabled_reason = Some(reason.into());
        self
    }

    pub fn is_section(&self) -> bool {
        matches!(self.activate, Activate::Section)
    }

    pub fn is_selectable(&self) -> bool {
        !self.is_section() && self.disabled_reason.is_none()
    }

    /// The page reached by pressing Right on this row: its verb page if it has
    /// one, otherwise the page it descends into.
    pub fn child_page(&self) -> Option<&Children> {
        match (&self.submenu, &self.activate) {
            (Some(submenu), _) => Some(submenu),
            (None, Activate::Page(children)) => Some(children),
            _ => None,
        }
    }

    /// Whether the row shows a chevron, i.e. whether Right does anything.
    pub fn has_child_page(&self) -> bool {
        self.child_page().is_some()
    }
}

/// One level of navigation.
pub struct Page {
    pub title: SharedString,
    pub nodes: Rc<[Node]>,
    /// The query typed on this page. Stashed on descent and restored on the way
    /// back, so returning to a page doesn't discard the user's filter.
    pub query: String,
    /// The row that was selected on this page, restored the same way.
    pub selected: usize,
}

/// The navigation stack. Always holds at least the root page.
pub struct Stack {
    pages: Vec<Page>,
}

impl Stack {
    pub fn new(title: impl Into<SharedString>, nodes: impl Into<Rc<[Node]>>) -> Self {
        Self {
            pages: vec![Page {
                title: title.into(),
                nodes: nodes.into(),
                query: String::new(),
                selected: 0,
            }],
        }
    }

    pub fn current(&self) -> &Page {
        // The root page is never popped, so this cannot fail.
        self.pages.last().expect("stack always holds a root page")
    }

    pub fn current_mut(&mut self) -> &mut Page {
        self.pages
            .last_mut()
            .expect("stack always holds a root page")
    }

    pub fn nodes(&self) -> &[Node] {
        &self.current().nodes
    }

    pub fn depth(&self) -> usize {
        self.pages.len() - 1
    }

    pub fn at_root(&self) -> bool {
        self.depth() == 0
    }

    pub fn titles(&self) -> impl Iterator<Item = &SharedString> {
        self.pages.iter().map(|page| &page.title)
    }

    /// Replaces the root page's rows, e.g. after the repository changed. Only
    /// meaningful at the root; deeper pages hold data captured on descent.
    pub fn set_root_nodes(&mut self, nodes: impl Into<Rc<[Node]>>) {
        if let Some(root) = self.pages.first_mut() {
            root.nodes = nodes.into();
        }
    }

    /// Descends, remembering the query and selection of the page being left.
    pub fn push(
        &mut self,
        title: impl Into<SharedString>,
        nodes: impl Into<Rc<[Node]>>,
        leaving_query: String,
        leaving_selection: usize,
    ) {
        {
            let current = self.current_mut();
            current.query = leaving_query;
            current.selected = leaving_selection;
        }
        self.pages.push(Page {
            title: title.into(),
            nodes: nodes.into(),
            query: String::new(),
            selected: 0,
        });
    }

    /// Ascends one level, returning the query to restore. `None` at the root,
    /// which the caller should treat as "dismiss" or "do nothing".
    pub fn pop(&mut self) -> Option<String> {
        if self.at_root() {
            return None;
        }
        self.pages.pop();
        Some(self.current().query.clone())
    }

    /// Truncates to `depth` levels below the root, for breadcrumb clicks.
    /// Returns the query to restore, or `None` if nothing changed.
    pub fn truncate(&mut self, depth: usize) -> Option<String> {
        if depth >= self.depth() {
            return None;
        }
        self.pages.truncate(depth + 1);
        Some(self.current().query.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn leaf(id: &str) -> Node {
        Node::run(id, id, |_, _, _| {})
    }

    #[test]
    fn stack_restores_query_and_selection_on_the_way_back() {
        let mut stack = Stack::new("Git", vec![leaf("a"), leaf("b")]);
        assert!(stack.at_root());
        assert_eq!(stack.pop(), None, "root must never be popped");

        stack.push("Branches", vec![leaf("main")], "fea".into(), 3);
        assert_eq!(stack.depth(), 1);
        assert_eq!(stack.current().query, "");

        stack.push("Verbs", vec![leaf("checkout")], "ma".into(), 1);
        assert_eq!(stack.depth(), 2);
        assert_eq!(
            stack.titles().map(|t| t.to_string()).collect::<Vec<_>>(),
            vec!["Git", "Branches", "Verbs"]
        );

        assert_eq!(stack.pop().as_deref(), Some("ma"));
        assert_eq!(stack.current().selected, 1);
        assert_eq!(stack.pop().as_deref(), Some("fea"));
        assert_eq!(stack.current().selected, 3);
        assert!(stack.at_root());
    }

    #[test]
    fn truncate_jumps_to_a_breadcrumb_and_is_a_no_op_at_or_below_the_target() {
        let mut stack = Stack::new("Git", vec![leaf("a")]);
        stack.push("Branches", vec![leaf("b")], "fea".into(), 0);
        stack.push("Verbs", vec![leaf("c")], "ma".into(), 0);

        assert_eq!(stack.truncate(2), None, "already at depth 2");
        assert_eq!(stack.truncate(5), None, "deeper than the stack");
        assert_eq!(stack.truncate(0).as_deref(), Some("fea"));
        assert!(stack.at_root());
    }

    #[test]
    fn sections_are_headings_and_never_selectable() {
        let section = Node::section("Branches");
        assert!(section.is_section());
        assert!(!section.is_selectable());
        assert!(!section.has_child_page());

        let disabled = leaf("push").disabled("No upstream");
        assert!(
            !disabled.is_selectable(),
            "disabled rows stay visible but inert"
        );
    }

    #[test]
    fn submenu_wins_over_activate_for_the_child_page() {
        let with_both = Node::page("main", "main", Children::ready(vec![leaf("from-activate")]))
            .submenu(Children::ready(vec![leaf("from-submenu")]));
        match with_both.child_page() {
            Some(Children::Ready(nodes)) => assert_eq!(
                nodes.first().map(|node| node.id.as_ref()),
                Some("from-submenu")
            ),
            _ => panic!("expected the submenu's children"),
        }

        let category = Node::page("recent", "Recent", Children::ready(vec![leaf("x")]));
        assert!(category.has_child_page());

        assert!(!leaf("commit").has_child_page());
    }
}
