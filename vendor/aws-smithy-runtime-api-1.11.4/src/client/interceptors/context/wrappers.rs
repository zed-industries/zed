/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use super::{Error, Input, InterceptorContext, Output};
use crate::client::interceptors::context::{Request, Response};
use crate::client::orchestrator::OrchestratorError;
use std::fmt::Debug;

macro_rules! impl_from_interceptor_context {
    (ref $wrapper:ident) => {
        impl<'a, I, O, E> From<&'a InterceptorContext<I, O, E>> for $wrapper<'a, I, O, E> {
            fn from(inner: &'a InterceptorContext<I, O, E>) -> Self {
                Self { inner }
            }
        }
    };
    (mut $wrapper:ident) => {
        impl<'a, I, O, E> From<&'a mut InterceptorContext<I, O, E>> for $wrapper<'a, I, O, E> {
            fn from(inner: &'a mut InterceptorContext<I, O, E>) -> Self {
                Self { inner }
            }
        }
    };
}

macro_rules! expect {
    ($self:ident, $what:ident) => {
        $self.inner.$what().expect(concat!(
            "`",
            stringify!($what),
            "` wasn't set in the underlying interceptor context. This is a bug."
        ))
    };
}

//
// BeforeSerializationInterceptorContextRef
//

/// Interceptor context for the `read_before_execution` and `read_before_serialization` hooks.
///
/// Only the input is available at this point in the operation.
#[derive(Debug)]
pub struct BeforeSerializationInterceptorContextRef<'a, I = Input, O = Output, E = Error> {
    inner: &'a InterceptorContext<I, O, E>,
}

impl_from_interceptor_context!(ref BeforeSerializationInterceptorContextRef);

impl<I, O, E> BeforeSerializationInterceptorContextRef<'_, I, O, E> {
    /// Returns a reference to the input.
    pub fn input(&self) -> &I {
        expect!(self, input)
    }

    /// Downgrade this wrapper struct, returning the underlying InterceptorContext.
    ///
    /// There's no good reason to use this unless you're writing tests or you have to
    /// interact with an API that doesn't support the context wrapper structs.
    pub fn inner(&self) -> &'_ InterceptorContext<I, O, E> {
        self.inner
    }
}

//
// BeforeSerializationInterceptorContextMut
//

/// Interceptor context for the `modify_before_serialization` hook.
///
/// Only the input is available at this point in the operation.
#[derive(Debug)]
pub struct BeforeSerializationInterceptorContextMut<'a, I = Input, O = Output, E = Error> {
    inner: &'a mut InterceptorContext<I, O, E>,
}

impl_from_interceptor_context!(mut BeforeSerializationInterceptorContextMut);

impl<I, O, E> BeforeSerializationInterceptorContextMut<'_, I, O, E> {
    /// Returns a reference to the input.
    pub fn input(&self) -> &I {
        expect!(self, input)
    }

    /// Returns a mutable reference to the input.
    pub fn input_mut(&mut self) -> &mut I {
        expect!(self, input_mut)
    }

    /// Downgrade this wrapper struct, returning the underlying InterceptorContext.
    ///
    /// There's no good reason to use this unless you're writing tests or you have to
    /// interact with an API that doesn't support the context wrapper structs.
    pub fn inner(&self) -> &'_ InterceptorContext<I, O, E> {
        self.inner
    }

    /// Downgrade this wrapper struct, returning the underlying InterceptorContext.
    ///
    /// There's no good reason to use this unless you're writing tests or you have to
    /// interact with an API that doesn't support the context wrapper structs.
    pub fn inner_mut(&mut self) -> &'_ mut InterceptorContext<I, O, E> {
        self.inner
    }
}

//
// BeforeSerializationInterceptorContextRef
//

/// Interceptor context for several hooks in between serialization and transmission.
///
/// Only the request is available at this point in the operation.
#[derive(Debug)]
pub struct BeforeTransmitInterceptorContextRef<'a, I = Input, O = Output, E = Error> {
    inner: &'a InterceptorContext<I, O, E>,
}

impl_from_interceptor_context!(ref BeforeTransmitInterceptorContextRef);

impl<I, O, E> BeforeTransmitInterceptorContextRef<'_, I, O, E> {
    /// Returns a reference to the transmittable request for the operation being invoked.
    pub fn request(&self) -> &Request {
        expect!(self, request)
    }

    /// Downgrade this wrapper struct, returning the underlying InterceptorContext.
    ///
    /// There's no good reason to use this unless you're writing tests or you have to
    /// interact with an API that doesn't support the context wrapper structs.
    pub fn inner(&self) -> &'_ InterceptorContext<I, O, E> {
        self.inner
    }
}

//
// BeforeSerializationInterceptorContextMut
//

/// Interceptor context for several hooks in between serialization and transmission.
///
/// Only the request is available at this point in the operation.
#[derive(Debug)]
pub struct BeforeTransmitInterceptorContextMut<'a, I = Input, O = Output, E = Error> {
    inner: &'a mut InterceptorContext<I, O, E>,
}

impl_from_interceptor_context!(mut BeforeTransmitInterceptorContextMut);

impl<I, O, E> BeforeTransmitInterceptorContextMut<'_, I, O, E> {
    /// Returns a reference to the transmittable request for the operation being invoked.
    pub fn request(&self) -> &Request {
        expect!(self, request)
    }

    /// Returns a mutable reference to the transmittable request for the operation being invoked.
    pub fn request_mut(&mut self) -> &mut Request {
        expect!(self, request_mut)
    }

    /// Downgrade this wrapper struct, returning the underlying InterceptorContext.
    ///
    /// There's no good reason to use this unless you're writing tests or you have to
    /// interact with an API that doesn't support the context wrapper structs.
    pub fn inner(&self) -> &'_ InterceptorContext<I, O, E> {
        self.inner
    }

    /// Downgrade this wrapper struct, returning the underlying InterceptorContext.
    ///
    /// There's no good reason to use this unless you're writing tests or you have to
    /// interact with an API that doesn't support the context wrapper structs.
    pub fn inner_mut(&mut self) -> &'_ mut InterceptorContext<I, O, E> {
        self.inner
    }
}

//
// BeforeDeserializationInterceptorContextRef
//

/// Interceptor context for hooks before deserializing the response.
///
/// Only the response is available at this point in the operation.
#[derive(Debug)]
pub struct BeforeDeserializationInterceptorContextRef<'a, I = Input, O = Output, E = Error> {
    inner: &'a InterceptorContext<I, O, E>,
}

impl_from_interceptor_context!(ref BeforeDeserializationInterceptorContextRef);

impl<I, O, E> BeforeDeserializationInterceptorContextRef<'_, I, O, E> {
    /// Returns a reference to the response.
    pub fn response(&self) -> &Response {
        expect!(self, response)
    }

    /// Downgrade this wrapper struct, returning the underlying InterceptorContext.
    ///
    /// There's no good reason to use this unless you're writing tests or you have to
    /// interact with an API that doesn't support the context wrapper structs.
    pub fn inner(&self) -> &'_ InterceptorContext<I, O, E> {
        self.inner
    }
}

//
// BeforeDeserializationInterceptorContextMut
//

/// Interceptor context for hooks before deserializing the response.
///
/// Only the response is available at this point in the operation.
pub struct BeforeDeserializationInterceptorContextMut<'a, I = Input, O = Output, E = Error> {
    inner: &'a mut InterceptorContext<I, O, E>,
}

impl_from_interceptor_context!(mut BeforeDeserializationInterceptorContextMut);

impl<I, O, E> BeforeDeserializationInterceptorContextMut<'_, I, O, E> {
    /// Returns a reference to the response.
    pub fn response(&self) -> &Response {
        expect!(self, response)
    }

    /// Returns a mutable reference to the response.
    pub fn response_mut(&mut self) -> &mut Response {
        expect!(self, response_mut)
    }

    /// Downgrade this wrapper struct, returning the underlying InterceptorContext.
    ///
    /// There's no good reason to use this unless you're writing tests or you have to
    /// interact with an API that doesn't support the context wrapper structs.
    pub fn inner(&self) -> &'_ InterceptorContext<I, O, E> {
        self.inner
    }

    /// Downgrade this wrapper struct, returning the underlying InterceptorContext.
    ///
    /// There's no good reason to use this unless you're writing tests or you have to
    /// interact with an API that doesn't support the context wrapper structs.
    pub fn inner_mut(&mut self) -> &'_ mut InterceptorContext<I, O, E> {
        self.inner
    }
}

//
// AfterDeserializationInterceptorContextRef
//

/// Interceptor context for hooks after deserializing the response.
///
/// The response and the deserialized output or error are available at this point in the operation.
pub struct AfterDeserializationInterceptorContextRef<'a, I = Input, O = Output, E = Error> {
    inner: &'a InterceptorContext<I, O, E>,
}

impl_from_interceptor_context!(ref AfterDeserializationInterceptorContextRef);

impl<I, O, E> AfterDeserializationInterceptorContextRef<'_, I, O, E> {
    /// Returns a reference to the response.
    pub fn response(&self) -> &Response {
        expect!(self, response)
    }

    /// Returns a reference to the deserialized output or error.
    pub fn output_or_error(&self) -> Result<&O, &OrchestratorError<E>> {
        expect!(self, output_or_error)
    }

    /// Downgrade this wrapper struct, returning the underlying InterceptorContext.
    ///
    /// There's no good reason to use this unless you're writing tests or you have to
    /// interact with an API that doesn't support the context wrapper structs.
    pub fn inner(&self) -> &'_ InterceptorContext<I, O, E> {
        self.inner
    }
}

//
// FinalizerInterceptorContextRef
//

/// Interceptor context for finalization hooks.
///
/// This context is used by the `read_after_attempt` and `read_after_execution` hooks
/// that are all called upon both success and failure, and may have varying levels
/// of context available depending on where a failure occurred if the operation failed.
pub struct FinalizerInterceptorContextRef<'a, I = Input, O = Output, E = Error> {
    inner: &'a InterceptorContext<I, O, E>,
}

impl_from_interceptor_context!(ref FinalizerInterceptorContextRef);

impl<I, O, E> FinalizerInterceptorContextRef<'_, I, O, E> {
    /// Returns the operation input.
    pub fn input(&self) -> Option<&I> {
        self.inner.input.as_ref()
    }

    /// Returns the serialized request.
    pub fn request(&self) -> Option<&Request> {
        self.inner.request.as_ref()
    }

    /// Returns the raw response.
    pub fn response(&self) -> Option<&Response> {
        self.inner.response.as_ref()
    }

    /// Returns the deserialized operation output or error.
    pub fn output_or_error(&self) -> Option<Result<&O, &OrchestratorError<E>>> {
        self.inner.output_or_error.as_ref().map(|o| o.as_ref())
    }

    /// Downgrade this wrapper struct, returning the underlying InterceptorContext.
    ///
    /// There's no good reason to use this unless you're writing tests or you have to
    /// interact with an API that doesn't support the context wrapper structs.
    pub fn inner(&self) -> &'_ InterceptorContext<I, O, E> {
        self.inner
    }
}

//
// FinalizerInterceptorContextMut
//

/// Interceptor context for finalization hooks.
///
/// This context is used by the `modify_before_attempt_completion` and `modify_before_completion` hooks
/// that are all called upon both success and failure, and may have varying levels
/// of context available depending on where a failure occurred if the operation failed.
pub struct FinalizerInterceptorContextMut<'a, I = Input, O = Output, E = Error> {
    inner: &'a mut InterceptorContext<I, O, E>,
}

impl_from_interceptor_context!(mut FinalizerInterceptorContextMut);

impl<I, O, E> FinalizerInterceptorContextMut<'_, I, O, E> {
    /// Returns the operation input.
    pub fn input(&self) -> Option<&I> {
        self.inner.input.as_ref()
    }

    /// Returns the serialized request.
    pub fn request(&self) -> Option<&Request> {
        self.inner.request.as_ref()
    }

    /// Returns the raw response.
    pub fn response(&self) -> Option<&Response> {
        self.inner.response.as_ref()
    }

    /// Returns the deserialized operation output or error.
    pub fn output_or_error(&self) -> Option<Result<&O, &OrchestratorError<E>>> {
        self.inner.output_or_error.as_ref().map(|o| o.as_ref())
    }

    /// Mutably returns the operation input.
    pub fn input_mut(&mut self) -> Option<&mut I> {
        self.inner.input.as_mut()
    }

    /// Mutably returns the serialized request.
    pub fn request_mut(&mut self) -> Option<&mut Request> {
        self.inner.request.as_mut()
    }

    /// Mutably returns the raw response.
    pub fn response_mut(&mut self) -> Option<&mut Response> {
        self.inner.response.as_mut()
    }

    /// Mutably returns the deserialized operation output or error.
    pub fn output_or_error_mut(&mut self) -> Option<&mut Result<O, OrchestratorError<E>>> {
        self.inner.output_or_error.as_mut()
    }

    /// Downgrade this wrapper struct, returning the underlying InterceptorContext.
    ///
    /// There's no good reason to use this unless you're writing tests or you have to
    /// interact with an API that doesn't support the context wrapper structs.
    pub fn inner(&self) -> &'_ InterceptorContext<I, O, E> {
        self.inner
    }

    /// Downgrade this wrapper struct, returning the underlying InterceptorContext.
    ///
    /// There's no good reason to use this unless you're writing tests or you have to
    /// interact with an API that doesn't support the context wrapper structs.
    pub fn inner_mut(&mut self) -> &'_ mut InterceptorContext<I, O, E> {
        self.inner
    }
}
