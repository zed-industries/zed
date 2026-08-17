// Copyright 2016 The Fancy Regex Authors.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//! Analysis of regex expressions.

use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::cmp::min;

use bit_set::BitSet;

use crate::alloc::string::ToString;
use crate::parse::ExprTree;
use crate::vm::CaptureGroupRange;
use crate::{AstNode, CaptureGroupTarget, CompileError, Error, Expr, Result};

#[cfg(not(feature = "std"))]
use alloc::collections::BTreeMap as Map;
#[cfg(feature = "std")]
use std::collections::HashMap as Map;

/// Analysis information for a regex expression.
///
/// This structure contains the results of analyzing a regex expression,
/// including size information, whether it requires backtracking features,
/// and references to child expressions in the tree.
#[derive(Debug)]
pub struct Info<'a> {
    pub(crate) capture_groups: CaptureGroupRange,
    /// The minimum number of characters this expression will match
    pub min_size: usize,
    /// Whether this expression always matches the same number of characters
    pub const_size: bool,
    /// Tracks the minimum number of characters that would be consumed in the innermost capture group
    /// before this expression is matched.
    pub(crate) min_pos_in_group: usize,
    /// Whether this expression requires backtracking features (lookaround, backrefs, etc.)
    pub hard: bool,
    /// The expression being analyzed
    pub expr: &'a Expr,
    /// Analysis information for child expressions
    pub children: Vec<Info<'a>>,
}

impl<'a> Info<'a> {
    /// Returns the start (first) group number for this expression.
    pub fn start_group(&self) -> usize {
        self.capture_groups.start()
    }

    /// Returns the end (last) group number for this expression.
    pub fn end_group(&self) -> usize {
        self.capture_groups.end()
    }

    pub(crate) fn is_literal(&self) -> bool {
        match *self.expr {
            Expr::Literal { casei, .. } => !casei,
            Expr::Concat(_) => self.children.iter().all(|child| child.is_literal()),
            _ => false,
        }
    }

    pub(crate) fn push_literal(&self, buf: &mut String) {
        match *self.expr {
            // could be more paranoid about checking casei
            Expr::Literal { ref val, .. } => buf.push_str(val),
            Expr::Concat(_) => {
                for child in &self.children {
                    child.push_literal(buf);
                }
            }
            _ => panic!("push_literal called on non-literal"),
        }
    }
}

struct SizeInfo {
    min_size: usize,
    const_size: bool,
}

/// Represents a subroutine call and its minimum position within a group
#[derive(Debug, Clone)]
struct SubroutineCallInfo {
    /// The group being called
    target_group: usize,
    /// The minimum number of characters consumed in the haystack by the capture group in which this call occurs
    min_pos: usize,
}

struct Analyzer<'a> {
    backrefs: &'a BitSet,
    /// The next group number to assign when a new capture group is encountered.
    /// Starts at 0 or 1 depending on whether explicit capture group 0 is enabled,
    /// and increments for each capture group found during traversal.
    next_group_number: usize,
    /// Stores the analysis info for each group by group number
    // NOTE: uses a Map instead of a Vec because sometimes we start from capture group 1, other times 0
    group_info: Map<usize, SizeInfo>,
    /// Tracks subroutine calls: maps from a group to the subroutines it calls
    subroutine_calls: Map<usize, Vec<SubroutineCallInfo>>,
    /// Groups that are directly executed from root (not inside {0})
    root_groups: BitSet,
    /// Pre-populated map of capture group index to the inner expression of that group.
    /// Only pre-populated/used when there are subroutine calls, to enable correct resolution
    /// of forward references, to detect whether they are const-size etc.
    group_exprs: Map<usize, &'a Expr>,
    /// Track groups currently being analyzed to prevent infinite recursion
    analyzing_groups: BitSet,
    /// When true, nodes that could produce empty matches (min size 0 and not const size)
    /// are promoted to hard so the VM can backtrack past them to find a non-empty match.
    find_not_empty: bool,
}

impl<'a> Analyzer<'a> {
    // enclosing_group: The group that currently contains the expression being analyzed.
    // This is used to track which group "owns" any subroutine calls found,
    // and to determine if we're at the root level (group 0).
    fn visit(
        &mut self,
        expr: &'a Expr,
        min_pos_in_group: usize,
        inside_zero_rep: bool,
        enclosing_group: usize,
    ) -> Result<Info<'a>> {
        let start_group = self.next_group_number;
        let mut children = Vec::new();
        let mut min_size = 0;
        let mut const_size = false;
        let mut hard = false;
        match *expr {
            Expr::Assertion(assertion) if assertion.is_hard() => {
                const_size = true;
                hard = true;
            }
            Expr::Empty | Expr::Assertion(_) => {
                const_size = true;
            }
            Expr::Any { .. } => {
                min_size = 1;
                const_size = true;
            }
            Expr::GeneralNewline { .. } => {
                // \R matches either \r\n (2 chars) or single newline chars (1 char)
                // So it has min_size = 1, but is not const_size
                min_size = 1;
                const_size = false;
                hard = true; // requires backtracking to handle \r\n without backtracking to \r
            }
            Expr::Literal { ref val, casei } => {
                // right now each character in a literal gets its own node, that might change
                min_size = 1;
                const_size = literal_const_size(val, casei);
            }
            Expr::Concat(ref v) => {
                const_size = true;
                let mut pos_in_group = min_pos_in_group;
                for child in v {
                    let child_info =
                        self.visit(child, pos_in_group, inside_zero_rep, enclosing_group)?;
                    min_size += child_info.min_size;
                    const_size &= child_info.const_size;
                    hard |= child_info.hard;
                    pos_in_group += child_info.min_size;
                    children.push(child_info);
                }
            }
            Expr::Alt(ref v) => {
                let child_info =
                    self.visit(&v[0], min_pos_in_group, inside_zero_rep, enclosing_group)?;
                min_size = child_info.min_size;
                const_size = child_info.const_size;
                hard = child_info.hard;
                children.push(child_info);
                for child in &v[1..] {
                    let child_info =
                        self.visit(child, min_pos_in_group, inside_zero_rep, enclosing_group)?;
                    const_size &= child_info.const_size && min_size == child_info.min_size;
                    min_size = min(min_size, child_info.min_size);
                    hard |= child_info.hard;
                    children.push(child_info);
                }
            }
            Expr::Group(ref child) => {
                let group = self.next_group_number;
                self.next_group_number += 1;
                self.analyzing_groups.insert(group);

                // Track if this group is executed from root (not inside {0})
                if enclosing_group == 0 && !inside_zero_rep {
                    self.root_groups.insert(group);
                }

                let child_info = self.visit(child, 0, inside_zero_rep, group)?;
                self.analyzing_groups.remove(group);
                min_size = child_info.min_size;
                const_size = child_info.const_size;
                // Store the group info for use by backrefs
                self.group_info.insert(
                    group,
                    SizeInfo {
                        min_size,
                        const_size,
                    },
                );
                // If there's a backref to this group, we potentially have to backtrack within the
                // group. E.g. with `(x|xy)\1` and input `xyxy`, `x` matches but then the backref
                // doesn't, so we have to backtrack and try `xy`.
                hard = child_info.hard | self.backrefs.contains(group);
                children.push(child_info);
            }
            Expr::LookAround(ref child, _) => {
                // NOTE: min_pos_in_group might seem weird for lookbehinds
                let child_info =
                    self.visit(child, min_pos_in_group, inside_zero_rep, enclosing_group)?;
                // min_size = 0
                const_size = true;
                hard = true;
                children.push(child_info);
            }
            Expr::Repeat {
                ref child, lo, hi, ..
            } => {
                // If lo and hi are both 0, we're in a zero-repetition (unreachable)
                let child_inside_zero_rep = if lo == 0 && hi == 0 {
                    true
                } else {
                    inside_zero_rep
                };
                let child_info = self.visit(
                    child,
                    min_pos_in_group,
                    child_inside_zero_rep,
                    enclosing_group,
                )?;
                min_size = child_info.min_size * lo;
                const_size = child_info.const_size && lo == hi;
                hard = child_info.hard;
                children.push(child_info);
            }
            Expr::Delegate { .. } => {
                // Delegate expressions always match exactly 1 character.
                // This constraint ensures consistency in the AST representation.
                min_size = 1;
                const_size = true;
            }
            Expr::Backref { group, .. } => {
                if group == 0 {
                    return Err(Error::CompileError(Box::new(CompileError::InvalidBackref(
                        group,
                    ))));
                }

                // Check if this backref is inside a recursed group that it refers to
                // This happens when:
                // 1. The backref refers to a group we're currently analyzing
                // 2. That group has a subroutine call to itself
                if self.analyzing_groups.contains(group) {
                    // Check if the group has a recursive subroutine call to itself
                    if let Some(calls) = self.subroutine_calls.get(&group) {
                        if calls.iter().any(|call| call.target_group == group) {
                            return Err(Error::CompileError(Box::new(
                                CompileError::FeatureNotYetSupported(
                                    "Backreference to a capture group from within the same group when it's being recursed".to_string()
                                )
                            )));
                        }
                    }
                }

                // Look up the referenced group's size information
                if let Some(&SizeInfo {
                    min_size: group_min_size,
                    const_size: group_const_size,
                }) = self.group_info.get(&group)
                {
                    min_size = group_min_size;
                    const_size = group_const_size;
                }
                hard = true;
            }
            Expr::AtomicGroup(ref child) => {
                let child_info =
                    self.visit(child, min_pos_in_group, inside_zero_rep, enclosing_group)?;
                min_size = child_info.min_size;
                const_size = child_info.const_size;
                hard = true; // TODO: possibly could weaken
                children.push(child_info);
            }
            Expr::KeepOut => {
                hard = true;
                const_size = true;
            }
            Expr::ContinueFromPreviousMatchEnd => {
                hard = true;
                const_size = true;
            }
            Expr::BackrefExistsCondition { .. } => {
                hard = true;
                const_size = true;
            }
            Expr::BacktrackingControlVerb(_) => {
                hard = true;
                const_size = true;
            }
            Expr::Conditional {
                ref condition,
                ref true_branch,
                ref false_branch,
            } => {
                hard = true;

                let child_info_condition = self.visit(
                    condition,
                    min_pos_in_group,
                    inside_zero_rep,
                    enclosing_group,
                )?;
                let child_info_truth = self.visit(
                    true_branch,
                    min_pos_in_group + child_info_condition.min_size,
                    inside_zero_rep,
                    enclosing_group,
                )?;
                let child_info_false = self.visit(
                    false_branch,
                    min_pos_in_group,
                    inside_zero_rep,
                    enclosing_group,
                )?;

                min_size = child_info_condition.min_size
                    + min(child_info_truth.min_size, child_info_false.min_size);
                const_size = child_info_condition.const_size
                    && child_info_truth.const_size
                    && child_info_false.const_size
                    // if the condition's size plus the truth branch's size is equal to the false branch's size then it's const size
                    && child_info_condition.min_size + child_info_truth.min_size == child_info_false.min_size;

                children.push(child_info_condition);
                children.push(child_info_truth);
                children.push(child_info_false);
            }
            Expr::SubroutineCall(target_group) => {
                // Track this subroutine call
                // Only skip tracking if we're in unreachable code at the root level
                // Calls inside groups should always be tracked, even if the group is inside {0} at root,
                // because the group can be called as a subroutine from elsewhere
                if !inside_zero_rep || enclosing_group != 0 {
                    self.subroutine_calls
                        .entry(enclosing_group)
                        .or_default()
                        .push(SubroutineCallInfo {
                            target_group,
                            min_pos: min_pos_in_group,
                        });
                }

                // Look up the target group's min_size if available (similar to backrefs)
                // This is important for accurate left recursion detection
                if let Some(&SizeInfo {
                    min_size: group_min_size,
                    const_size: group_const_size,
                }) = self.group_info.get(&target_group)
                {
                    min_size = group_min_size;
                    const_size = group_const_size;
                } else if self.analyzing_groups.contains(target_group) {
                    // Currently analyzing this group - circular reference
                    // Use conservative defaults to avoid infinite recursion
                    min_size = 0;
                    const_size = false;
                } else if let Some(&group_expr) = self.group_exprs.get(&target_group) {
                    // If the group hasn't been seen yet (forward reference),
                    // directly analyze it now
                    self.analyzing_groups.insert(target_group);

                    // Save and restore next_group_number around the recursive call
                    let prev_next_group_number = self.next_group_number;
                    self.next_group_number = target_group + 1;
                    let group_info = self.visit(group_expr, 0, inside_zero_rep, target_group)?;
                    self.next_group_number = prev_next_group_number;

                    self.analyzing_groups.remove(target_group);

                    min_size = group_info.min_size;
                    const_size = group_info.const_size;
                    // Store the analysis result for future lookups
                    self.group_info.insert(
                        target_group,
                        SizeInfo {
                            min_size,
                            const_size,
                        },
                    );
                } else {
                    // Group doesn't exist - this shouldn't happen as the parser would have caught it
                    min_size = 0;
                    const_size = false;
                }
                hard = true;
            }
            Expr::AstNode(ref astnode, ix) => {
                // An unresolved AstNode means the resolver couldn't find the target (e.g. a
                // subroutine call to an unknown name). Match each SubroutineCall variant to
                // produce an accurate error message; any other AstNode variant is an internal
                // error and gets a distinct UnresolvedAstNode error.
                match astnode {
                    AstNode::SubroutineCall(CaptureGroupTarget::ByName(name)) => {
                        return Err(Error::CompileError(Box::new(
                            CompileError::SubroutineCallTargetNotFound(
                                format!("named group '{}'", name),
                                ix,
                            ),
                        )));
                    }
                    AstNode::SubroutineCall(CaptureGroupTarget::ByNumber(n)) => {
                        return Err(Error::CompileError(Box::new(
                            CompileError::SubroutineCallTargetNotFound(
                                format!("group number {}", n),
                                ix,
                            ),
                        )));
                    }
                    AstNode::SubroutineCall(CaptureGroupTarget::Relative(n)) => {
                        return Err(Error::CompileError(Box::new(
                            CompileError::SubroutineCallTargetNotFound(
                                format!("relative group {}{}", if *n >= 0 { "+" } else { "" }, n),
                                ix,
                            ),
                        )));
                    }
                    AstNode::AstGroup { .. }
                    | AstNode::Backref { .. }
                    | AstNode::BackrefExistsCondition { .. } => {
                        return Err(Error::CompileError(Box::new(
                            CompileError::UnresolvedAstNode(ix, format!("{:?}", astnode)),
                        )));
                    }
                }
            }
            Expr::BackrefWithRelativeRecursionLevel { .. } => {
                return Err(Error::CompileError(Box::new(
                    CompileError::FeatureNotYetSupported("Backref at recursion level".to_string()),
                )));
            }
            Expr::Absent(ref absent) => {
                use crate::Absent::*;
                match absent {
                    Repeater(ref child) => {
                        let child_info =
                            self.visit(child, min_pos_in_group, inside_zero_rep, enclosing_group)?;
                        min_size = 0;
                        const_size = false;
                        hard = true;
                        children.push(child_info);
                    }
                    Expression {
                        ref absent,
                        ref exp,
                    } => {
                        let absent_info =
                            self.visit(absent, min_pos_in_group, inside_zero_rep, enclosing_group)?;
                        let exp_info =
                            self.visit(exp, min_pos_in_group, inside_zero_rep, enclosing_group)?;
                        min_size = exp_info.min_size;
                        const_size = false;
                        hard = true;
                        children.push(absent_info);
                        children.push(exp_info);
                    }
                    Stopper(ref child) => {
                        let child_info =
                            self.visit(child, min_pos_in_group, inside_zero_rep, enclosing_group)?;
                        // Absent stopper doesn't consume any characters itself
                        min_size = 0;
                        const_size = true;
                        hard = true;
                        children.push(child_info);
                    }
                    Clear => {
                        // Range clear doesn't consume any characters
                        min_size = 0;
                        const_size = true;
                        hard = true;
                    }
                }
            }
            Expr::DefineGroup { ref definitions } => {
                // DEFINE groups don't match anything themselves, but we still need to
                // visit the definitions to assign group numbers and analyze any nested content.
                // The DEFINE block itself is not hard - it matches nothing, so it can be
                // delegated (as an empty string) to the underlying engine.
                let def_info = self.visit(definitions, 0, inside_zero_rep, enclosing_group)?;
                min_size = 0;
                const_size = true;
                children.push(def_info);
            }
        };

        // When find_not_empty is active, any node that could produce an empty match
        // must be handled by the VM so it can backtrack past it to find a non-empty match.
        if self.find_not_empty && min_size == 0 && !const_size {
            hard = true;
        }

        Ok(Info {
            expr,
            children,
            capture_groups: CaptureGroupRange(start_group, self.next_group_number),
            min_size,
            const_size,
            hard,
            min_pos_in_group,
        })
    }

    /// Check for left-recursive subroutine calls using depth-first search
    fn check_left_recursion(&self, named_groups: &Map<String, usize>) -> Result<()> {
        // Compute which groups are reachable from the root (group 0)
        let reachable_groups = self.compute_reachable_groups();

        // Check each reachable group for left recursion
        for &start_group in self.subroutine_calls.keys() {
            if !reachable_groups.contains(start_group) {
                // Skip unreachable groups
                continue;
            }

            let mut visited = BitSet::new();
            let mut recursion_stack = BitSet::new();
            if self.dfs_check_left_recursion(start_group, &mut visited, &mut recursion_stack)? {
                // Found left recursion
                // Build reverse mapping from group number to group name (if any)
                // so we can give friendly error messages
                let mut group_names: Map<usize, String> = Map::new();
                for (name, &group_num) in named_groups.iter() {
                    group_names.insert(group_num, name.clone());
                }

                let group_desc = if let Some(name) = group_names.get(&start_group) {
                    format!("group '{}' ({})", name, start_group)
                } else {
                    format!("group {}", start_group)
                };
                return Err(Error::CompileError(Box::new(
                    CompileError::LeftRecursiveSubroutineCall(group_desc),
                )));
            }
        }
        Ok(())
    }

    /// Check for recursive subroutine calls that can never terminate
    fn check_unbounded_recursion(&self, root_expr: &'a Expr) -> Result<()> {
        let mut memo = Map::new();
        if self.group_can_terminate(0, root_expr, &mut BitSet::new(), &mut memo) {
            return Ok(());
        }

        Err(Error::CompileError(Box::new(
            CompileError::NeverEndingRecursion,
        )))
    }

    fn group_can_terminate(
        &self,
        group: usize,
        root_expr: &'a Expr,
        recursion_stack: &mut BitSet,
        memo: &mut Map<usize, bool>,
    ) -> bool {
        if let Some(&can_terminate) = memo.get(&group) {
            return can_terminate;
        }

        if recursion_stack.contains(group) {
            return false;
        }

        let expr = if group == 0 {
            root_expr
        } else if let Some(&group_expr) = self.group_exprs.get(&group) {
            group_expr
        } else {
            return true;
        };

        recursion_stack.insert(group);
        let can_terminate = self.expr_can_terminate(expr, root_expr, recursion_stack, memo);
        recursion_stack.remove(group);
        memo.insert(group, can_terminate);
        can_terminate
    }

    fn expr_can_terminate(
        &self,
        expr: &'a Expr,
        root_expr: &'a Expr,
        recursion_stack: &mut BitSet,
        memo: &mut Map<usize, bool>,
    ) -> bool {
        match expr {
            Expr::Concat(children) => children
                .iter()
                .all(|child| self.expr_can_terminate(child, root_expr, recursion_stack, memo)),
            Expr::Alt(children) => children
                .iter()
                .any(|child| self.expr_can_terminate(child, root_expr, recursion_stack, memo)),
            Expr::Group(child) => self.expr_can_terminate(child, root_expr, recursion_stack, memo),
            Expr::LookAround(child, _) => {
                self.expr_can_terminate(child, root_expr, recursion_stack, memo)
            }
            Expr::AtomicGroup(child) => {
                self.expr_can_terminate(child, root_expr, recursion_stack, memo)
            }
            Expr::Repeat { child, lo, .. } => {
                *lo == 0 || self.expr_can_terminate(child, root_expr, recursion_stack, memo)
            }
            Expr::Conditional {
                condition,
                true_branch,
                false_branch,
            } => {
                self.expr_can_terminate(false_branch, root_expr, recursion_stack, memo)
                    || (self.expr_can_terminate(condition, root_expr, recursion_stack, memo)
                        && self.expr_can_terminate(true_branch, root_expr, recursion_stack, memo))
            }
            Expr::SubroutineCall(target_group) => {
                self.group_can_terminate(*target_group, root_expr, recursion_stack, memo)
            }
            Expr::Absent(absent) => {
                use crate::Absent::*;
                match absent {
                    Repeater(child) | Stopper(child) => {
                        self.expr_can_terminate(child, root_expr, recursion_stack, memo)
                    }
                    Expression { absent, exp } => {
                        self.expr_can_terminate(absent, root_expr, recursion_stack, memo)
                            && self.expr_can_terminate(exp, root_expr, recursion_stack, memo)
                    }
                    Clear => true,
                }
            }
            Expr::BackrefWithRelativeRecursionLevel { .. } => true,
            Expr::DefineGroup { definitions } => {
                self.expr_can_terminate(definitions, root_expr, recursion_stack, memo)
            }
            _ => true,
        }
    }

    /// A group is reachable if it's executed from root (not inside {0}) or called from a reachable group
    fn compute_reachable_groups(&self) -> BitSet {
        let mut reachable = BitSet::new();
        let mut to_visit = Vec::new();

        // Start from root (group 0)
        // Group 0 is always reachable
        reachable.insert(0);
        to_visit.push(0);

        // Also mark groups that are directly executed from root (not inside {0})
        for group in self.root_groups.iter() {
            if !reachable.contains(group) {
                reachable.insert(group);
                to_visit.push(group);
            }
        }

        // Propagate reachability through subroutine calls
        while let Some(group) = to_visit.pop() {
            if let Some(calls) = self.subroutine_calls.get(&group) {
                for call_info in calls {
                    if !reachable.contains(call_info.target_group) {
                        reachable.insert(call_info.target_group);
                        to_visit.push(call_info.target_group);
                    }
                }
            }
        }

        reachable
    }

    /// Depth-first search to detect left recursion
    /// Returns true if left recursion is detected
    fn dfs_check_left_recursion(
        &self,
        group: usize,
        visited: &mut BitSet,
        recursion_stack: &mut BitSet,
    ) -> Result<bool> {
        if recursion_stack.contains(group) {
            // We found a cycle. Since we only follow calls at position 0 (see below),
            // reaching a group already in the recursion stack means we have a left-recursive cycle.
            return Ok(true);
        }

        if visited.contains(group) {
            return Ok(false);
        }

        visited.insert(group);
        recursion_stack.insert(group);

        // Check all subroutine calls from this group
        if let Some(calls) = self.subroutine_calls.get(&group) {
            for call_info in calls {
                // Only consider calls at position 0 (potential left recursion)
                if call_info.min_pos == 0
                    && self.dfs_check_left_recursion(
                        call_info.target_group,
                        visited,
                        recursion_stack,
                    )?
                {
                    return Ok(true);
                }
            }
        }

        recursion_stack.remove(group);
        Ok(false)
    }
}

fn literal_const_size(_: &str, _: bool) -> bool {
    // Right now, regex doesn't do sophisticated case folding,
    // test below will fail when that changes, then we need to
    // do something fancier here.
    true
}

/// Recursively collect all Group expressions and their capture group indices
fn collect_groups<'a>(
    expr: &'a Expr,
    next_group_number: &mut usize,
    groups: &mut Map<usize, &'a Expr>,
) {
    match expr {
        Expr::Group(inner) => {
            let current_group = *next_group_number;
            *next_group_number += 1;
            groups.insert(current_group, inner.as_ref());
            // Continue recursing to find nested groups
            collect_groups(inner.as_ref(), next_group_number, groups);
        }
        _ => {
            // Recurse into all children
            for child in expr.children_iter() {
                collect_groups(child, next_group_number, groups);
            }
        }
    }
}

/// Context options for [`analyze`].
#[derive(Debug, Clone, Default)]
pub struct AnalyzeContext {
    /// When `true`, capture group numbering starts at 0 rather than 1 (i.e. there is an explicit
    /// outer capture group 0 in the pattern rather than an implicit one added by the compiler).
    pub explicit_capture_group_0: bool,
    /// When `true`, sub-expressions that could produce an empty match (`min_size == 0` and not
    /// `const_size`) are promoted to `hard` so that the VM can backtrack past them to find a
    /// non-empty match.
    pub find_not_empty: bool,
}

/// Analyze the parsed expression to determine whether it requires fancy features.
pub fn analyze<'a>(tree: &'a ExprTree, ctx: AnalyzeContext) -> Result<Info<'a>> {
    let explicit_capture_group_0 = ctx.explicit_capture_group_0;
    let find_not_empty = ctx.find_not_empty;

    // Check that numeric capture group references (backrefs and subroutine calls) and named groups are not mixed
    if tree.numbered_groups_ignored
        && tree.numeric_capture_group_references
        && !tree.named_groups.is_empty()
    {
        return Err(Error::CompileError(Box::new(
            CompileError::NamedBackrefOnly,
        )));
    }

    let start_group = if explicit_capture_group_0 { 0 } else { 1 };

    // pre-populate groups if subroutines are present to handle forward references
    let group_exprs = if tree.contains_subroutines {
        let mut groups = Map::new();
        let mut next_group_number = start_group;
        collect_groups(&tree.expr, &mut next_group_number, &mut groups);
        groups
    } else {
        Map::new()
    };

    let mut analyzer = Analyzer {
        backrefs: &tree.backrefs,
        next_group_number: start_group,
        group_info: Map::new(),
        subroutine_calls: Map::new(),
        root_groups: BitSet::new(),
        group_exprs,
        analyzing_groups: BitSet::new(),
        find_not_empty,
    };

    let analyzed = analyzer.visit(&tree.expr, 0, false, 0)?;
    // With start_group == 1 (no explicit group 0) the valid backref range is 1..=total_groups.
    // With start_group == 0 (explicit group 0) group 0 is the whole match and inner groups are
    // numbered 1..=total_groups-1, so the valid range is 0..=total_groups-1.
    let max_valid_group = if explicit_capture_group_0 {
        tree.total_groups.saturating_sub(1)
    } else {
        tree.total_groups
    };
    // Out-of-range group numbers are not in the BitSet (to avoid huge allocations), so check the
    // dedicated field first.
    if let Some(group) = tree.out_of_range_backref {
        return Err(Error::CompileError(Box::new(CompileError::InvalidBackref(
            group,
        ))));
    }
    for group in tree.backrefs.iter() {
        if group < start_group || group > max_valid_group {
            return Err(Error::CompileError(Box::new(CompileError::InvalidBackref(
                group,
            ))));
        }
    }

    // Check for left-recursive and unterminating subroutine calls (only if subroutines are present)
    if tree.contains_subroutines {
        analyzer.check_left_recursion(&tree.named_groups)?;
        analyzer.check_unbounded_recursion(&tree.expr)?;
    }

    Ok(analyzed)
}

/// Determine if the expression will always only ever match at position 0.
/// Note that false negatives are possible - it can return false even if it could be anchored.
/// This should therefore only be treated as an optimization.
pub fn can_compile_as_anchored(root_expr: &Expr) -> bool {
    use crate::Assertion;

    match root_expr {
        Expr::Concat(children) => match children[0] {
            Expr::Assertion(assertion) => assertion == Assertion::StartText,
            Expr::ContinueFromPreviousMatchEnd => true,
            _ => false,
        },
        Expr::Assertion(assertion) => *assertion == Assertion::StartText,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{analyze, AnalyzeContext};
    // use super::literal_const_size;
    use crate::parse::ExprTree;
    use crate::{can_compile_as_anchored, CompileError, Error, Expr};
    use matches::assert_matches;

    #[cfg_attr(feature = "track_caller", track_caller)]
    fn assert_analyze_ok(tree: &ExprTree, ctx: AnalyzeContext) {
        match analyze(tree, ctx) {
            Ok(_) => {}
            Err(e) => panic!("expected analyze to succeed, but got error: {:?}", e),
        }
    }

    #[cfg_attr(feature = "track_caller", track_caller)]
    fn assert_compile_error<T, F>(result: crate::Result<T>, check: F)
    where
        F: FnOnce(&CompileError) -> bool,
    {
        match result {
            Err(Error::CompileError(ref e)) if check(e) => {}
            other => panic!(
                "expected a matching CompileError, but got: {:?}",
                other.err()
            ),
        }
    }

    #[cfg_attr(feature = "track_caller", track_caller)]
    fn assert_invalid_backref(
        pattern: &str,
        explicit_capture_group_0: bool,
        expected_group: usize,
    ) {
        let tree = Expr::parse_tree(pattern).unwrap();
        assert_compile_error(
            analyze(
                &tree,
                AnalyzeContext {
                    explicit_capture_group_0,
                    ..Default::default()
                },
            ),
            |e| matches!(e, CompileError::InvalidBackref(g) if *g == expected_group),
        );
    }

    // #[test]
    // fn case_folding_safe() {
    //     let re = regex::Regex::new("(?i:ß)").unwrap();
    //     if re.is_match("SS") {
    //         assert!(!literal_const_size("ß", true));
    //     }

    //     // Another tricky example, Armenian ECH YIWN
    //     let re = regex::Regex::new("(?i:\\x{0587})").unwrap();
    //     if re.is_match("\u{0565}\u{0582}") {
    //         assert!(!literal_const_size("\u{0587}", true));
    //     }
    // }

    #[test]
    fn invalid_backref_zero() {
        assert_invalid_backref(r".\0", false, 0);
        assert_invalid_backref(r".\0", true, 0);
        assert_invalid_backref(r"(.)\0", false, 0);
        assert_invalid_backref(r"(.)\0", true, 0);
        assert_invalid_backref(r"(.)\0\1", false, 0);
    }

    #[test]
    fn invalid_backref_no_captures() {
        assert_invalid_backref(r"aa\1", false, 1);
        assert_invalid_backref(r"aaaa\2", false, 2);
    }

    #[test]
    fn invalid_backref_unreasonably_large_number() {
        // A group number that is a valid usize but far exceeds the number of groups in the
        // pattern. The resolver inserts it into the BitSet as-is; the analyzer catches it via
        // the total_groups bound check.
        assert_invalid_backref(r".\1999999999", false, 1999999999);
    }

    #[test]
    fn invalid_backref_with_captures() {
        assert_invalid_backref(r"a(a)\2", false, 2);
        assert_invalid_backref(r"a(a)\2\1", false, 2);
    }

    #[test]
    fn invalid_backref_with_captures_explict_capture_group_zero() {
        assert_invalid_backref(r"(a(b)\2)c", true, 2);
        assert_invalid_backref(r"(a(b)\1\2)c", true, 2);
        assert_invalid_backref(r"(a\1)b", true, 1);
        assert_invalid_backref(r"(a(b))\2", true, 2);
    }

    #[test]
    fn unresolved_subroutine_call_error_takes_precedence_over_invalid_backref() {
        // Regression test for issue where encountering an unresolved subroutine call would
        // cause the analyzer to stop visiting groups, leading to an incomplete group count.
        // This would then cause a misleading "Invalid back reference" error instead of
        // the correct error.
        let tree = Expr::parse_tree(r"(?<a>a)(?<b>b)\g<no_exist>(?<c>c)\k<a>\k<c>").unwrap();
        let result = analyze(&tree, AnalyzeContext::default());

        // Should get the unresolved subroutine call error, not an invalid backref error
        assert_compile_error(
            result,
            |e| matches!(e, CompileError::SubroutineCallTargetNotFound(s, _) if s.contains("no_exist")),
        );
    }

    #[test]
    fn allow_analysis_of_self_backref() {
        // even if it will never match, see issue 103
        assert!(!analyze(
            &Expr::parse_tree(r"(.\1)").unwrap(),
            AnalyzeContext::default()
        )
        .is_err());
        assert!(!analyze(
            &Expr::parse_tree(r"((.\1))").unwrap(),
            AnalyzeContext {
                explicit_capture_group_0: true,
                ..Default::default()
            }
        )
        .is_err());
        assert!(!analyze(
            &Expr::parse_tree(r"(([ab]+)\1b)").unwrap(),
            AnalyzeContext::default()
        )
        .is_err());
        // in the following scenario it can match
        assert!(!analyze(
            &Expr::parse_tree(r"(([ab]+?)(?(1)\1| )c)+").unwrap(),
            AnalyzeContext::default(),
        )
        .is_err());
    }

    #[test]
    fn allow_backref_even_when_capture_group_occurs_after_backref() {
        assert!(!analyze(
            &Expr::parse_tree(r"\1(.)").unwrap(),
            AnalyzeContext::default()
        )
        .is_err());
        assert!(!analyze(
            &Expr::parse_tree(r"(\1(.))").unwrap(),
            AnalyzeContext {
                explicit_capture_group_0: true,
                ..Default::default()
            }
        )
        .is_err());
    }

    #[test]
    fn valid_backref_occurs_after_capture_group() {
        assert!(!analyze(
            &Expr::parse_tree(r"(.)\1").unwrap(),
            AnalyzeContext::default()
        )
        .is_err());
        assert!(!analyze(
            &Expr::parse_tree(r"((.)\1)").unwrap(),
            AnalyzeContext {
                explicit_capture_group_0: true,
                ..Default::default()
            }
        )
        .is_err());

        assert!(!analyze(
            &Expr::parse_tree(r"((.)\2\2)\1").unwrap(),
            AnalyzeContext::default()
        )
        .is_err());
        assert!(!analyze(
            &Expr::parse_tree(r"(.)\1(.)\2").unwrap(),
            AnalyzeContext::default()
        )
        .is_err());
        assert!(!analyze(
            &Expr::parse_tree(r"(.)foo(.)\2").unwrap(),
            AnalyzeContext::default()
        )
        .is_err());
        assert!(!analyze(
            &Expr::parse_tree(r"(.)(foo)(.)\3\2\1").unwrap(),
            AnalyzeContext::default(),
        )
        .is_err());
        assert!(!analyze(
            &Expr::parse_tree(r"(.)(foo)(.)\3\1").unwrap(),
            AnalyzeContext::default()
        )
        .is_err());
        assert!(!analyze(
            &Expr::parse_tree(r"(.)(foo)(.)\2\1").unwrap(),
            AnalyzeContext::default()
        )
        .is_err());
    }

    #[test]
    fn feature_not_yet_supported() {
        let tree = &Expr::parse_tree(r"(a)\k<1-0>").unwrap();
        assert_compile_error(analyze(tree, AnalyzeContext::default()), |e| {
            matches!(e, CompileError::FeatureNotYetSupported(_))
        });
    }

    #[test]
    fn subroutine_call_undefined() {
        let tree = &Expr::parse_tree(r"\g<wrong_name>(?<different_name>a)").unwrap();
        assert_compile_error(analyze(tree, AnalyzeContext::default()), |e| {
            matches!(e, CompileError::SubroutineCallTargetNotFound(_, _))
        });
    }

    #[test]
    fn subroutine_call_undefined_by_name_message() {
        let tree = &Expr::parse_tree(r"\g<wrong_name>(?<different_name>a)").unwrap();
        assert_compile_error(
            analyze(tree, AnalyzeContext::default()),
            |e| matches!(e, CompileError::SubroutineCallTargetNotFound(s, _) if s.contains("wrong_name")),
        );
    }

    #[test]
    fn subroutine_call_undefined_by_number() {
        use crate::parse::NamedGroups;
        use bit_set::BitSet;
        // ByNumber is always resolved by the parser so we must construct the ExprTree directly
        let tree = ExprTree {
            expr: Expr::AstNode(
                crate::AstNode::SubroutineCall(crate::CaptureGroupTarget::ByNumber(99)),
                0,
            ),
            backrefs: BitSet::new(),
            named_groups: NamedGroups::default(),
            numeric_capture_group_references: false,
            contains_subroutines: true,
            self_recursive: false,
            total_groups: 0,
            out_of_range_backref: None,
            numbered_groups_ignored: false,
        };
        assert_compile_error(
            analyze(&tree, AnalyzeContext::default()),
            |e| matches!(e, CompileError::SubroutineCallTargetNotFound(s, _) if s.contains("99")),
        );
    }

    #[test]
    fn subroutine_call_undefined_by_relative() {
        use crate::parse::NamedGroups;
        use bit_set::BitSet;
        // Relative can only fail to resolve on integer overflow (essentially never),
        // so we must construct the ExprTree directly
        let tree = ExprTree {
            expr: Expr::AstNode(
                crate::AstNode::SubroutineCall(crate::CaptureGroupTarget::Relative(-1)),
                0,
            ),
            backrefs: BitSet::new(),
            named_groups: NamedGroups::default(),
            numeric_capture_group_references: false,
            contains_subroutines: true,
            self_recursive: false,
            total_groups: 0,
            out_of_range_backref: None,
            numbered_groups_ignored: false,
        };
        assert_compile_error(
            analyze(&tree, AnalyzeContext::default()),
            |e| matches!(e, CompileError::SubroutineCallTargetNotFound(s, _) if s.contains("-1")),
        );
    }

    #[test]
    fn unresolved_ast_node_error() {
        use crate::parse::NamedGroups;
        use bit_set::BitSet;
        // Construct a tree with an unresolved Backref AstNode (not a SubroutineCall),
        // which should produce an UnresolvedAstNode error containing the node's debug representation
        let tree = ExprTree {
            expr: Expr::AstNode(
                crate::AstNode::Backref {
                    target: crate::CaptureGroupTarget::ByNumber(1),
                    casei: false,
                    relative_recursion_level: None,
                },
                0,
            ),
            backrefs: BitSet::new(),
            named_groups: NamedGroups::default(),
            numeric_capture_group_references: false,
            contains_subroutines: false,
            self_recursive: false,
            total_groups: 0,
            out_of_range_backref: None,
            numbered_groups_ignored: false,
        };
        assert_compile_error(
            analyze(&tree, AnalyzeContext::default()),
            |e| matches!(e, CompileError::UnresolvedAstNode(_, s) if s.contains("Backref")),
        );
    }

    #[test]
    fn numeric_capture_group_references_cannot_be_used_with_named_groups() {
        use crate::parse_flags::FLAG_IGNORE_NUMBERED_GROUPS_WHEN_NAMED_GROUPS_EXIST;

        // Test case 1: Named group followed by numeric backref (no alternation)
        let tree = Expr::parse_tree_with_flags(
            r"(?<name>a)\1",
            FLAG_IGNORE_NUMBERED_GROUPS_WHEN_NAMED_GROUPS_EXIST,
        )
        .unwrap();
        assert_compile_error(analyze(&tree, AnalyzeContext::default()), |e| {
            matches!(e, CompileError::NamedBackrefOnly)
        });

        // Test case 2: Numeric backref followed by named group
        let tree = Expr::parse_tree_with_flags(
            r"(a)\1(?<name>b)",
            FLAG_IGNORE_NUMBERED_GROUPS_WHEN_NAMED_GROUPS_EXIST,
        )
        .unwrap();
        assert_compile_error(analyze(&tree, AnalyzeContext::default()), |e| {
            matches!(e, CompileError::NamedBackrefOnly)
        });

        // Test case 3: Alternation with numeric backref and named group in first branch
        let tree = Expr::parse_tree_with_flags(
            r"(?<name>a)\1|b",
            FLAG_IGNORE_NUMBERED_GROUPS_WHEN_NAMED_GROUPS_EXIST,
        )
        .unwrap();
        assert_compile_error(analyze(&tree, AnalyzeContext::default()), |e| {
            matches!(e, CompileError::NamedBackrefOnly)
        });

        // Test case 4: Alternation with named group in first branch, numeric backref in second
        let tree = Expr::parse_tree_with_flags(
            r"(?<name>a)|\1",
            FLAG_IGNORE_NUMBERED_GROUPS_WHEN_NAMED_GROUPS_EXIST,
        )
        .unwrap();
        assert_compile_error(analyze(&tree, AnalyzeContext::default()), |e| {
            matches!(e, CompileError::NamedBackrefOnly)
        });

        // Test case 5: Numbered group containing named group, with numeric backref
        let tree = Expr::parse_tree_with_flags(
            r"(a|(?<name>b))\1",
            FLAG_IGNORE_NUMBERED_GROUPS_WHEN_NAMED_GROUPS_EXIST,
        )
        .unwrap();
        assert_compile_error(analyze(&tree, AnalyzeContext::default()), |e| {
            matches!(e, CompileError::NamedBackrefOnly)
        });

        // Test case 6: Multiple branches with named groups and numeric backrefs
        let tree = Expr::parse_tree_with_flags(
            r"(?<x>a)|(?<y>b)|\1",
            FLAG_IGNORE_NUMBERED_GROUPS_WHEN_NAMED_GROUPS_EXIST,
        )
        .unwrap();
        assert_compile_error(analyze(&tree, AnalyzeContext::default()), |e| {
            matches!(e, CompileError::NamedBackrefOnly)
        });

        // Test case 7: Numeric subroutine call with named group
        let tree = Expr::parse_tree_with_flags(
            r"(?<foo>\w+)\g<1>",
            FLAG_IGNORE_NUMBERED_GROUPS_WHEN_NAMED_GROUPS_EXIST,
        )
        .unwrap();
        assert_compile_error(analyze(&tree, AnalyzeContext::default()), |e| {
            matches!(e, CompileError::NamedBackrefOnly)
        });

        // Test case 8: Named group with numeric subroutine call in alternation
        let tree = Expr::parse_tree_with_flags(
            r"(?<foo>a)|\g<1>",
            FLAG_IGNORE_NUMBERED_GROUPS_WHEN_NAMED_GROUPS_EXIST,
        )
        .unwrap();
        assert_compile_error(analyze(&tree, AnalyzeContext::default()), |e| {
            matches!(e, CompileError::NamedBackrefOnly)
        });

        // Positive cases - these should work

        // Only numeric backrefs and subroutine calls, no named groups
        let tree = Expr::parse_tree_with_flags(
            r"(a)(b+)\1\g<2>",
            FLAG_IGNORE_NUMBERED_GROUPS_WHEN_NAMED_GROUPS_EXIST,
        )
        .unwrap();
        assert_analyze_ok(&tree, AnalyzeContext::default());

        // Only named groups and named backrefs and named subroutine calls
        let tree = Expr::parse_tree_with_flags(
            r"(?<name>a)\k<name>\g<name>",
            FLAG_IGNORE_NUMBERED_GROUPS_WHEN_NAMED_GROUPS_EXIST,
        )
        .unwrap();
        assert_analyze_ok(&tree, AnalyzeContext::default());

        // Multiple named and numbered groups, no backrefs or subroutine calls
        let tree = Expr::parse_tree_with_flags(
            r"(?<a>a)|(?<b>b)(c)(d+)",
            FLAG_IGNORE_NUMBERED_GROUPS_WHEN_NAMED_GROUPS_EXIST,
        )
        .unwrap();
        assert_analyze_ok(&tree, AnalyzeContext::default());
    }

    #[test]
    fn is_literal() {
        let tree = Expr::parse_tree("abc").unwrap();
        let info = analyze(&tree, AnalyzeContext::default()).unwrap();
        assert_eq!(info.is_literal(), true);
    }

    #[test]
    fn is_literal_with_repeat() {
        let tree = Expr::parse_tree("abc*").unwrap();
        let info = analyze(&tree, AnalyzeContext::default()).unwrap();
        assert_eq!(info.is_literal(), false);
    }

    #[test]
    fn anchored_for_starttext_assertions() {
        let tree = Expr::parse_tree(r"^(\w+)\1").unwrap();
        assert_eq!(can_compile_as_anchored(&tree.expr), true);

        let tree = Expr::parse_tree(r"^").unwrap();
        assert_eq!(can_compile_as_anchored(&tree.expr), true);
    }

    #[test]
    fn anchored_for_continue_from_prev_match_assertions() {
        let tree = Expr::parse_tree(r"\G(\w+)\1").unwrap();
        assert_eq!(can_compile_as_anchored(&tree.expr), true);
    }

    #[test]
    fn backref_inherits_group_size_info() {
        // Test that backrefs properly inherit min_size and const_size from referenced groups
        let tree = Expr::parse_tree(r"(abc)\1").unwrap();
        let info = analyze(&tree, AnalyzeContext::default()).unwrap();
        // The concatenation should have min_size = 3 + 3 = 6 (group + backref)
        assert_eq!(info.min_size, 6);
        assert!(info.const_size);

        // Test with a variable-length group
        let tree = Expr::parse_tree(r"(a+)\1").unwrap();
        let info = analyze(&tree, AnalyzeContext::default()).unwrap();
        // The group has min_size = 1, but const_size = false due to the +
        // So the total should be min_size = 2, const_size = false
        assert_eq!(info.min_size, 2);
        assert!(!info.const_size);

        // Test with optional group
        let tree = Expr::parse_tree(r"(a?)\1").unwrap();
        let info = analyze(&tree, AnalyzeContext::default()).unwrap();
        // Both group and backref can be empty, so min_size = 0
        assert_eq!(info.min_size, 0);
        assert!(!info.const_size);
    }

    #[test]
    fn backref_forward_reference() {
        // Test forward references (backref before group definition)
        // These should use conservative defaults but still work
        let tree = Expr::parse_tree(r"\1(abc)").unwrap();
        let info = analyze(&tree, AnalyzeContext::default()).unwrap();
        // Forward ref gets min_size=0, group gets min_size=3, total=3
        assert_eq!(info.min_size, 3);
        // Forward ref sets const_size=false, so overall is false
        assert!(!info.const_size);
    }

    #[test]
    fn backref_in_lookbehind() {
        assert!(!analyze(
            &Expr::parse_tree(r"(hello)(?<=\b\1)").unwrap(),
            AnalyzeContext::default(),
        )
        .is_err());
        assert!(!analyze(
            &Expr::parse_tree(r"(..)(?<=\1\1)").unwrap(),
            AnalyzeContext::default()
        )
        .is_err());
        assert!(!analyze(
            &Expr::parse_tree(r"(abc)(?<=\1)def").unwrap(),
            AnalyzeContext::default()
        )
        .is_err());
    }

    #[test]
    fn backref_inside_recursed_group_not_supported() {
        let tree = Expr::parse_tree(r"(?<foo>a|\(\g<foo>\)\k<foo>?)").unwrap();
        assert_compile_error(
            analyze(&tree, AnalyzeContext::default()),
            |e| matches!(e, CompileError::FeatureNotYetSupported(s) if s.contains("Backreference") && s.contains("recursed")),
        );

        // Test with numbered groups
        let tree = Expr::parse_tree(r"(\g<1>\1)").unwrap();
        assert_compile_error(
            analyze(&tree, AnalyzeContext::default()),
            |e| matches!(e, CompileError::FeatureNotYetSupported(s) if s.contains("Backreference") && s.contains("recursed")),
        );

        // Another example with alternation inside recursed group
        let tree = Expr::parse_tree(r"(a|\g<1>b\1)").unwrap();
        assert_compile_error(
            analyze(&tree, AnalyzeContext::default()),
            |e| matches!(e, CompileError::FeatureNotYetSupported(s) if s.contains("Backreference") && s.contains("recursed")),
        );
    }

    #[test]
    fn backref_outside_recursed_group_is_allowed() {
        let tree = Expr::parse_tree(r"(?<foo>a|\(\g<foo>\))\k<foo>").unwrap();
        let result = analyze(&tree, AnalyzeContext::default());
        assert!(
            result.is_ok(),
            "Backref outside recursed group should pass analysis"
        );
    }

    #[test]
    fn not_anchored_for_startline_assertions() {
        let tree = Expr::parse_tree(r"(?m)^(\w+)\1").unwrap();
        assert_eq!(can_compile_as_anchored(&tree.expr), false);
    }

    #[test]
    fn min_pos_in_group_calculated_correctly_with_no_groups() {
        let tree = Expr::parse_tree(r"\G").unwrap();
        let info = analyze(&tree, AnalyzeContext::default()).unwrap();
        assert_eq!(info.min_size, 0);
        assert_eq!(info.min_pos_in_group, 0);
        assert!(info.const_size);

        let tree = Expr::parse_tree(r"\G(?=abc)\w+").unwrap();
        let info = analyze(&tree, AnalyzeContext::default()).unwrap();
        // the lookahead itself has min size 0
        assert_eq!(info.children[1].min_size, 0);
        assert!(info.children[1].const_size);
        // the children of the lookahead have min_size 3 from the literal
        assert_eq!(info.children[1].children[0].min_size, 3);
        assert!(info.children[1].children[0].const_size);
        // after lookahead, the position is reset
        assert_eq!(info.children[2].min_pos_in_group, 0);
        assert_eq!(info.children[2].min_size, 1);
        assert_eq!(info.min_pos_in_group, 0);
        assert!(!info.const_size);

        let tree = Expr::parse_tree(r"(?:ab*|cd){2}(?=bar)\w").unwrap();
        let info = analyze(&tree, AnalyzeContext::default()).unwrap();
        // the whole expression has min size 3 (a times 2 plus \w)
        assert_eq!(info.min_size, 3);
        // the min pos of the lookahead is 2
        assert_eq!(info.children[1].min_pos_in_group, 2);
        // after lookahead, the position is reset
        assert_eq!(info.children[2].min_pos_in_group, 2);
        assert_eq!(info.children[2].min_size, 1);
        assert!(!info.const_size);
    }

    #[test]
    fn backtracking_control_verb_is_hard_and_const_size() {
        let tree = Expr::parse_tree(r"(*FAIL)").unwrap();
        let info = analyze(&tree, AnalyzeContext::default()).unwrap();
        assert_eq!(info.min_size, 0);
        assert_eq!(info.min_pos_in_group, 0);
        assert!(info.const_size);
    }

    #[test]
    fn min_pos_in_group_calculated_correctly_with_capture_groups() {
        let tree = Expr::parse_tree(r"a(bc)d(e(f)g)").unwrap();
        let info = analyze(&tree, AnalyzeContext::default()).unwrap();
        assert_eq!(info.min_pos_in_group, 0);
        // before the capture begins, the min pos in group 0 is 1
        assert_eq!(info.children[1].min_pos_in_group, 1);
        // inside capture group 1, the min pos of the Concat inside the group is 0
        assert_matches!(info.children[1].children[0].expr, Expr::Concat(_));
        assert_eq!(info.children[1].children[0].min_pos_in_group, 0);
        assert!(info.children[1].children[0].const_size);
        // inside capture group 1, the min pos of the c inside the group is 1
        assert_matches!(info.children[1].children[0].children[1].expr, Expr::Literal { val, casei: false } if val == "c");
        assert_eq!(info.children[1].children[0].children[1].min_pos_in_group, 1);

        // prove we are looking at the position of the d after capture group 1
        assert_matches!(info.children[2].expr, Expr::Literal { val, casei: false } if val == "d");
        assert_eq!(info.children[2].min_pos_in_group, 3);
        assert_eq!(info.children[2].start_group(), 2);
        assert_eq!(info.children[2].min_size, 1);

        // prove we are looking at the position of the e in capture group 2
        assert_matches!(info.children[3].children[0].children[0].expr, Expr::Literal { val, casei: false } if val == "e");
        assert_eq!(info.children[3].children[0].children[0].min_pos_in_group, 0);
    }

    #[test]
    fn absent_repeater_is_hard_and_not_const_size() {
        let tree = Expr::parse_tree(r"(?~abc)").unwrap();
        let info = analyze(&tree, AnalyzeContext::default()).unwrap();
        assert_eq!(info.min_size, 0);
        assert!(!info.const_size);
        assert!(info.hard);
    }

    #[test]
    fn absent_expression_is_hard_and_not_const_size() {
        let tree = Expr::parse_tree(r"(?~|abc|\d+)").unwrap();
        let info = analyze(&tree, AnalyzeContext::default()).unwrap();
        // min_size comes from exp part (\d+ has min_size 1)
        assert_eq!(info.min_size, 1);
        assert!(!info.const_size);
        assert!(info.hard);
    }

    #[test]
    fn range_clear_is_hard_and_const_size() {
        let tree = Expr::parse_tree(r"(?~|)").unwrap();
        let info = analyze(&tree, AnalyzeContext::default()).unwrap();
        assert_eq!(info.min_size, 0);
        assert!(info.const_size);
        assert!(info.hard);
    }

    #[test]
    fn left_recursive_subroutine_direct() {
        // Direct left recursion: group 1 calls itself at position 0
        let tree = Expr::parse_tree(r"(\g<1>a)").unwrap();
        assert_compile_error(analyze(&tree, AnalyzeContext::default()), |e| {
            matches!(e, CompileError::LeftRecursiveSubroutineCall(_))
        });

        let tree = Expr::parse_tree(r"abc(\g<1>a)").unwrap();
        assert_compile_error(analyze(&tree, AnalyzeContext::default()), |e| {
            matches!(e, CompileError::LeftRecursiveSubroutineCall(_))
        });
    }

    #[test]
    fn not_left_recursive_subroutine_after_group() {
        let tree = Expr::parse_tree(r"(a)\g<1>").unwrap();
        let result = analyze(&tree, AnalyzeContext::default());
        assert!(result.is_ok());

        let tree = Expr::parse_tree(r"(?<test>a)\g<test>").unwrap();
        let result = analyze(&tree, AnalyzeContext::default());
        assert!(result.is_ok());
    }

    #[test]
    fn left_recursive_subroutine_at_start() {
        // Left recursion at start of group: (\g<1>a)
        let tree = Expr::parse_tree(r"(\g<1>a)").unwrap();
        assert_compile_error(analyze(&tree, AnalyzeContext::default()), |e| {
            matches!(e, CompileError::LeftRecursiveSubroutineCall(_))
        });

        let tree = Expr::parse_tree(r"(?<test>\g<test>a)").unwrap();
        assert_compile_error(analyze(&tree, AnalyzeContext::default()), |e| {
            matches!(e, CompileError::LeftRecursiveSubroutineCall(_))
        });
    }

    #[test]
    fn left_recursive_subroutine_indirect() {
        // Indirect left recursion: non-nested subroutine calls to each other
        let tree = Expr::parse_tree(r"(\g<2>)(\g<1>)").unwrap();
        assert_compile_error(analyze(&tree, AnalyzeContext::default()), |e| {
            matches!(e, CompileError::LeftRecursiveSubroutineCall(_))
        });

        let tree = Expr::parse_tree(r"(\g<2>)(\g<1>a)").unwrap();
        assert_compile_error(analyze(&tree, AnalyzeContext::default()), |e| {
            matches!(e, CompileError::LeftRecursiveSubroutineCall(_))
        });
    }

    #[test]
    fn left_recursive_subroutine_with_alternation() {
        // Left recursion through alternation, depending which branch is taken it could be left-recursive
        let tree = Expr::parse_tree(r"(a|\g<1>)").unwrap();
        assert_compile_error(analyze(&tree, AnalyzeContext::default()), |e| {
            matches!(e, CompileError::LeftRecursiveSubroutineCall(_))
        });
    }

    #[test]
    fn unbounded_recursive_after_char() {
        // Not left recursive because subroutine call is after a character was consumed,
        // but still never-ending because the recursive call is mandatory.
        let tree = Expr::parse_tree(r"(a\g<1>)").unwrap();
        assert_compile_error(analyze(&tree, AnalyzeContext::default()), |e| {
            matches!(e, CompileError::NeverEndingRecursion)
        });
    }

    #[test]
    fn bounded_recursive_after_char_is_allowed() {
        let tree = Expr::parse_tree(r"(a\g<1>?)").unwrap();
        let result = analyze(&tree, AnalyzeContext::default());
        assert!(result.is_ok());
    }

    #[test]
    fn not_left_recursive_zero_repetition() {
        // Not left recursive because subroutine call is unreachable
        let tree = Expr::parse_tree(r"(a?\g<1>){0}").unwrap();
        let result = analyze(&tree, AnalyzeContext::default());
        assert!(result.is_ok());
    }

    #[test]
    fn left_recursive_with_both_positions() {
        // Left recursive because \g<1> appears at position 0 in the group even though also at end at position 1
        let tree = Expr::parse_tree(r"(\g<1>a\g<1>)").unwrap();
        assert_compile_error(analyze(&tree, AnalyzeContext::default()), |e| {
            matches!(e, CompileError::LeftRecursiveSubroutineCall(_))
        });
    }

    #[test]
    fn left_recursive_with_lookahead() {
        let tree = Expr::parse_tree(r"((?=a)\g<1>)").unwrap();
        assert_compile_error(analyze(&tree, AnalyzeContext::default()), |e| {
            matches!(e, CompileError::LeftRecursiveSubroutineCall(_))
        });
    }

    #[test]
    fn self_recursive_group_zero() {
        // Self-recursive on group 0 after a character
        let tree = Expr::parse_tree(r"a\g<0>").unwrap();
        // Group 0 calls itself at position 1 (after 'a'), so this is NOT left recursive
        assert_compile_error(analyze(&tree, AnalyzeContext::default()), |e| {
            matches!(e, CompileError::NeverEndingRecursion)
        });
    }

    #[test]
    fn not_left_recursive_forward_call() {
        // Forward subroutine call - not left recursive: \g<1>(a)
        let tree = Expr::parse_tree(r"\g<1>(a)").unwrap();
        let result = analyze(&tree, AnalyzeContext::default());
        // The call happens before the group is defined, but it's at position 0 of group 0 (implicit)
        // which calls group 1. Group 1 doesn't call anything, so no cycle.
        assert!(result.is_ok());
    }

    #[test]
    fn not_left_recursive_group_zero_explicit() {
        // Self-recursive on explicit group 0: (a\g<0>)
        let tree = Expr::parse_tree(r"(a\g<0>)").unwrap();
        assert_compile_error(
            analyze(
                &tree,
                AnalyzeContext {
                    explicit_capture_group_0: true,
                    ..Default::default()
                },
            ),
            |e| matches!(e, CompileError::NeverEndingRecursion),
        );
    }

    #[test]
    fn not_left_recursive_group_zero_subroutine_call_unreachable() {
        let tree = Expr::parse_tree(r"\g<0>{0}abc").unwrap();
        let result = analyze(
            &tree,
            AnalyzeContext {
                explicit_capture_group_0: true,
                ..Default::default()
            },
        );
        assert!(result.is_ok());
    }

    #[test]
    fn left_recursive_group_zero_at_start() {
        // Self-recursive on explicit group 0 at start: (\g<0>a)
        let tree = Expr::parse_tree(r"(\g<0>a)").unwrap();
        let result = analyze(
            &tree,
            AnalyzeContext {
                explicit_capture_group_0: true,
                ..Default::default()
            },
        );
        // With explicit group 0, \g<0> at position 0 is left-recursive
        assert!(result.is_err());
    }

    #[test]
    fn three_way_indirect_recursion() {
        // Three-way indirect recursion where each recursive call is mandatory.
        let tree = Expr::parse_tree(r"(\g<2>)(\g<3>)(a\g<1>)").unwrap();
        // Group 1 -> Group 2 (at pos 0)
        // Group 2 -> Group 3 (at pos 0)
        // Group 3 -> Group 1 (at pos 1, after 'a')
        // This forms a cycle, but the call from group 3 to group 1 is at position 1
        // So it's not left-recursive
        assert_compile_error(analyze(&tree, AnalyzeContext::default()), |e| {
            matches!(e, CompileError::NeverEndingRecursion)
        });
    }

    #[test]
    fn three_way_left_recursive() {
        // Three-way left recursion
        let tree = Expr::parse_tree(r"(\g<2>)(\g<3>)(\g<1>)").unwrap();
        let result = analyze(&tree, AnalyzeContext::default());
        // Group 1 -> Group 2 (at pos 0)
        // Group 2 -> Group 3 (at pos 0)
        // Group 3 -> Group 1 (at pos 0)
        // This forms a left-recursive cycle
        assert!(result.is_err());

        let tree = Expr::parse_tree(r"(\g<2>a)(\g<3>b)(\g<1>c)").unwrap();
        let result = analyze(&tree, AnalyzeContext::default());
        assert!(result.is_err());
    }

    #[test]
    fn left_recursive_with_call_to_defined_group() {
        // Even though the call from Group 1 to Group 2 is inside {0} at root level,
        // Group 1's pattern can still be executed when called from Group 2
        // Group 1 contains a?\g<2> - calls group 2 (when executed)
        // Group 2 contains \g<1> - calls group 1 at position 0
        // This creates a cycle: Group 2 -> Group 1 (at pos 0) -> Group 2 (at pos 0)
        let tree = Expr::parse_tree(r"(a?\g<2>){0}(\g<1>)").unwrap();
        assert_compile_error(analyze(&tree, AnalyzeContext::default()), |e| {
            matches!(e, CompileError::LeftRecursiveSubroutineCall(_))
        });
    }

    #[test]
    fn no_left_recursion_complex_pattern() {
        // Group n (1): |\g<m>\g<n> - calls m then itself, but m has min_size > 0
        // Group m (2): a(b)\g<m> - calls itself after 'ab'
        let tree = Expr::parse_tree(r"(?<n>|\g<m>\g<n>)\z|\zEND (?<m>a(b)\g<m>)").unwrap();
        let result = analyze(&tree, AnalyzeContext::default());

        assert!(
            result.is_ok(),
            "Pattern should not be left-recursive because group m has min_size > 0"
        );
    }

    // Tests for forward-referenced subroutine calls
    // These verify that the analyzer correctly handles subroutine calls to groups
    // that are defined later in the pattern, and that group indexes are computed correctly.

    #[test]
    fn forward_subroutine_call_single_group() {
        // Forward reference: calls group 1 before it's defined
        let tree = Expr::parse_tree(r"\g<1>(.a.)").unwrap();
        let info = analyze(&tree, AnalyzeContext::default()).unwrap();

        // The pattern should have 1 capture group
        assert_eq!(info.start_group(), 1);
        assert_eq!(info.end_group(), 2);

        // Verify the Info has the correct Expr nodes - a Concat with 2 children
        assert!(matches!(info.expr, Expr::Concat(_)));
        assert_eq!(info.children.len(), 2);

        // First child is the SubroutineCall
        assert!(matches!(info.children[0].expr, Expr::SubroutineCall(1)));
        // SubroutineCall should have group range 1..1 (no new groups)
        assert_eq!(info.children[0].start_group(), 1);
        assert_eq!(info.children[0].end_group(), 1);
        // SubroutineCall should get the min_size from the group
        assert_eq!(info.children[0].min_size, 3);
        assert!(info.children[0].const_size);

        // Second child is the group
        assert!(matches!(info.children[1].expr, Expr::Group(_)));
        assert_eq!(info.children[1].start_group(), 1);
        assert_eq!(info.children[1].end_group(), 2);
        // The group itself should also have min_size 3
        assert_eq!(info.children[1].min_size, 3);
        assert!(info.children[1].const_size);

        // Overall pattern should have min_size 6 (subroutine call + group)
        assert_eq!(info.min_size, 6);
        assert!(info.const_size);
    }

    #[test]
    fn forward_subroutine_call_with_multiple_groups() {
        // Forward reference with multiple groups defined after the subroutine call
        let tree = Expr::parse_tree(r"\g<2>(a)(bc+)").unwrap();
        let info = analyze(&tree, AnalyzeContext::default()).unwrap();

        // The pattern should have 2 capture groups
        assert_eq!(info.start_group(), 1);
        assert_eq!(info.end_group(), 3);

        // Verify the Info Expr nodes - it's a Concat with 3 children
        assert!(matches!(info.expr, Expr::Concat(_)));
        assert_eq!(info.children.len(), 3);

        // SubroutineCall to group 2
        assert!(matches!(info.children[0].expr, Expr::SubroutineCall(2)));
        // The subroutine call itself is not in a capture group
        assert_eq!(info.children[0].start_group(), 1);
        assert_eq!(info.children[0].end_group(), 1);
        // SubroutineCall should get the min_size from the group
        assert_eq!(info.children[0].min_size, 2);
        assert!(!info.children[0].const_size);

        // First group (capture group 1)
        assert!(matches!(info.children[1].expr, Expr::Group(_)));
        assert_eq!(info.children[1].start_group(), 1);
        assert_eq!(info.children[1].end_group(), 2);
        assert_eq!(info.children[1].min_size, 1);
        assert!(info.children[1].const_size);

        // Second group (capture group 2)
        assert!(matches!(info.children[2].expr, Expr::Group(_)));
        assert_eq!(info.children[2].start_group(), 2);
        assert_eq!(info.children[2].end_group(), 3);
        assert_eq!(info.children[2].min_size, 2);
        assert!(!info.children[2].const_size);
    }

    #[test]
    fn forward_subroutine_call_with_nested_groups() {
        // Forward reference with multiple nested groups defined after the subroutine call
        let tree = Expr::parse_tree(r"(foo)\g<4>(a(b)?)(c(d))(?!e)").unwrap();
        let info = analyze(&tree, AnalyzeContext::default()).unwrap();

        // The pattern should have 5 capture groups
        assert_eq!(info.start_group(), 1);
        assert_eq!(info.end_group(), 6);

        // Verify the Info Expr nodes - it's a Concat with 5 children
        assert!(matches!(info.expr, Expr::Concat(_)));
        assert_eq!(info.children.len(), 5);

        // SubroutineCall to group 4
        assert!(matches!(info.children[1].expr, Expr::SubroutineCall(4)));
        // The subroutine call itself is not in a capture group. but after group 1
        assert_eq!(info.children[1].start_group(), 2);
        assert_eq!(info.children[1].end_group(), 2);
        // SubroutineCall should get the min_size from the group
        assert_eq!(info.children[1].min_size, 2);
        assert!(info.children[1].const_size);

        // First group after the subroutine call (capture group 2)
        assert!(matches!(info.children[2].expr, Expr::Group(_)));
        assert_eq!(info.children[2].start_group(), 2);
        assert_eq!(info.children[2].end_group(), 4);
        assert_eq!(info.children[2].min_size, 1);
        assert!(!info.children[2].const_size);

        // Second group after the subroutine call (nested inside group 2)
        let group_info = &info.children[2].children[0].children[1].children[0];
        assert!(matches!(group_info.expr, Expr::Group(_)));
        assert_eq!(group_info.start_group(), 3);
        assert_eq!(group_info.end_group(), 4);
        assert_eq!(group_info.min_size, 1);
        assert!(group_info.const_size);

        // Third group after the subroutine call
        assert!(matches!(info.children[3].expr, Expr::Group(_)));
        assert_eq!(info.children[3].start_group(), 4);
        assert_eq!(info.children[3].end_group(), 6);
        assert_eq!(info.children[3].min_size, 2);
        assert!(info.children[3].const_size);

        // Fourth group after the subroutine call (nested inside group 4)
        let group_info = &info.children[3].children[0].children[1];
        assert!(matches!(group_info.expr, Expr::Group(_)));
        assert_eq!(group_info.start_group(), 5);
        assert_eq!(group_info.end_group(), 6);
        assert_eq!(group_info.min_size, 1);
        assert!(group_info.const_size);

        // Negative lookahead should start after group 5
        assert!(matches!(info.children[4].expr, Expr::LookAround(_, _)));
        assert_eq!(info.children[4].start_group(), 6);
        assert_eq!(info.children[4].end_group(), 6);
    }

    #[test]
    fn define_group_is_easy_zero_size() {
        // A DEFINE block should be analyzed as: min_size=0, const_size=true, hard=false
        // It matches nothing itself, so it doesn't need backtracking.
        let tree = Expr::parse_tree(r"(?(DEFINE)(?<word>\w+))").unwrap();
        let info = analyze(&tree, AnalyzeContext::default()).unwrap();

        assert!(matches!(info.expr, Expr::DefineGroup { .. }));
        assert_eq!(info.min_size, 0);
        assert!(info.const_size);
        assert!(!info.hard);
    }

    #[test]
    fn define_group_assigns_group_numbers() {
        // Groups inside a DEFINE block should still be assigned group numbers,
        // so that subroutine calls can reference them.
        let tree = Expr::parse_tree(r"(?(DEFINE)(?<first>a)(?<second>b))").unwrap();
        let info = analyze(&tree, AnalyzeContext::default()).unwrap();

        // The definitions child: a Concat of two groups (groups 1 and 2)
        assert_eq!(info.children[0].start_group(), 1);
        assert_eq!(info.children[0].end_group(), 3);

        // Group numbers are assigned in the usual order
        let tree = Expr::parse_tree(
            r"(abc)(?(DEFINE)(?<second>a)ignored: no group(?<third>b(?<fourth>c)))(?<fifth>d)",
        )
        .unwrap();
        let info = analyze(&tree, AnalyzeContext::default()).unwrap();

        assert_eq!(info.start_group(), 1);
        assert_eq!(info.end_group(), 6);
        assert_eq!(info.children[0].start_group(), 1);
        assert_eq!(info.children[1].children[0].start_group(), 2);
        assert_eq!(info.children[2].start_group(), 5);
    }
}
