//-
// Copyright 2019 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::std_facade::{fmt, Arc};
use core::mem;

use crate::strategy::traits::*;
use crate::test_runner::*;

/// Represents a value tree that is initialized on the first call to any
/// methods.
///
/// This is used to defer potentially expensive generation to shrinking time. It
/// is public only to allow APIs to expose it as an intermediate value.
pub struct LazyValueTree<S: Strategy> {
    state: LazyValueTreeState<S>,
}

enum LazyValueTreeState<S: Strategy> {
    Initialized(S::Tree),
    Uninitialized {
        strategy: Arc<S>,
        runner: TestRunner,
    },
    Failed,
}

impl<S: Strategy> LazyValueTree<S> {
    /// Create a new value tree where initial generation is deferred until
    /// `maybe_init` is called.
    pub(crate) fn new(strategy: Arc<S>, runner: &mut TestRunner) -> Self {
        let runner = runner.partial_clone();
        Self {
            state: LazyValueTreeState::Uninitialized { strategy, runner },
        }
    }

    /// Create a new value tree that has already been initialized.
    pub(crate) fn new_initialized(value_tree: S::Tree) -> Self {
        Self {
            state: LazyValueTreeState::Initialized(value_tree),
        }
    }

    /// Returns a reference to the inner value tree if initialized.
    pub(crate) fn as_inner(&self) -> Option<&S::Tree> {
        match &self.state {
            LazyValueTreeState::Initialized(v) => Some(v),
            LazyValueTreeState::Uninitialized { .. }
            | LazyValueTreeState::Failed => None,
        }
    }

    /// Returns a mutable reference to the inner value tree if uninitialized.
    pub(crate) fn as_inner_mut(&mut self) -> Option<&mut S::Tree> {
        match &mut self.state {
            LazyValueTreeState::Initialized(v) => Some(v),
            LazyValueTreeState::Uninitialized { .. }
            | LazyValueTreeState::Failed => None,
        }
    }

    /// Try initializing the value tree.
    pub(crate) fn maybe_init(&mut self) {
        if !self.is_uninitialized() {
            return;
        }

        let state = mem::replace(&mut self.state, LazyValueTreeState::Failed);
        match state {
            LazyValueTreeState::Uninitialized {
                strategy,
                mut runner,
            } => {
                match strategy.new_tree(&mut runner) {
                    Ok(v) => {
                        let _ = mem::replace(
                            &mut self.state,
                            LazyValueTreeState::Initialized(v),
                        );
                    }
                    Err(_) => {
                        // self.state is set to Failed above. Keep it that way.
                    }
                }
            }
            LazyValueTreeState::Initialized(_) | LazyValueTreeState::Failed => {
                unreachable!("can only reach here if uninitialized")
            }
        }
    }

    /// Whether this value tree still needs to be initialized.
    pub(crate) fn is_uninitialized(&self) -> bool {
        match &self.state {
            LazyValueTreeState::Uninitialized { .. } => true,
            LazyValueTreeState::Initialized(_) | LazyValueTreeState::Failed => {
                false
            }
        }
    }

    /// Whether the value tree was successfully initialized.
    pub(crate) fn is_initialized(&self) -> bool {
        match &self.state {
            LazyValueTreeState::Initialized(_) => true,
            LazyValueTreeState::Uninitialized { .. }
            | LazyValueTreeState::Failed => false,
        }
    }
}

impl<S: Strategy> Clone for LazyValueTree<S>
where
    S::Tree: Clone,
{
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}

impl<S: Strategy> fmt::Debug for LazyValueTree<S>
where
    S::Tree: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("LazyValueTree")
            .field("state", &self.state)
            .finish()
    }
}

impl<S: Strategy> Clone for LazyValueTreeState<S>
where
    S::Tree: Clone,
{
    fn clone(&self) -> Self {
        use LazyValueTreeState::*;

        match self {
            Initialized(v) => Initialized(v.clone()),
            Uninitialized { strategy, runner } => Uninitialized {
                strategy: Arc::clone(strategy),
                runner: runner.clone(),
            },
            Failed => Failed,
        }
    }
}

impl<S: Strategy> fmt::Debug for LazyValueTreeState<S>
where
    S::Tree: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LazyValueTreeState::Initialized(value_tree) => {
                f.debug_tuple("Initialized").field(value_tree).finish()
            }
            LazyValueTreeState::Uninitialized { strategy, .. } => f
                .debug_struct("Uninitialized")
                .field("strategy", strategy)
                .finish(),
            LazyValueTreeState::Failed => write!(f, "Failed"),
        }
    }
}
