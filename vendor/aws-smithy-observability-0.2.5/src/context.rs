/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

/// Delineates a logical scope that has some beginning and end
/// (e.g. a function or block of code).
pub trait Scope {
    /// invoke when the scope has ended.
    fn end(&self);
}

/// A cross cutting concern for carrying execution-scoped values across API
/// boundaries (both in-process and distributed).
pub trait Context {
    /// Make this context the currently active context.
    /// The returned handle is used to return the previous
    /// context (if one existed) as active.
    fn make_current(&self) -> &dyn Scope;
}

/// Keeps track of the current [Context].
pub trait ContextManager {
    ///Get the currently active context.
    fn current(&self) -> &dyn Context;
}
