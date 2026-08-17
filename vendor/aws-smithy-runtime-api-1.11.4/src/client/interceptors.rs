/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Interceptors for clients.
//!
//! Interceptors are operation lifecycle hooks that can read/modify requests and responses.

use crate::box_error::BoxError;
use crate::client::interceptors::context::{
    AfterDeserializationInterceptorContextRef, BeforeDeserializationInterceptorContextMut,
    BeforeDeserializationInterceptorContextRef, BeforeSerializationInterceptorContextMut,
    BeforeSerializationInterceptorContextRef, BeforeTransmitInterceptorContextMut,
    BeforeTransmitInterceptorContextRef, FinalizerInterceptorContextMut,
    FinalizerInterceptorContextRef,
};
use crate::client::runtime_components::sealed::ValidateConfig;
use crate::client::runtime_components::RuntimeComponents;
use aws_smithy_types::config_bag::{ConfigBag, Storable, StoreReplace};
use std::fmt;
use std::marker::PhantomData;
use std::sync::Arc;

pub mod context;
pub mod error;

use crate::impl_shared_conversions;
pub use error::InterceptorError;

macro_rules! interceptor_trait_fn {
    ($name:ident, $phase:ident, $docs:tt) => {
        #[doc = $docs]
        fn $name(
            &self,
            context: &$phase<'_>,
            runtime_components: &RuntimeComponents,
            cfg: &mut ConfigBag,
        ) -> Result<(), BoxError> {
            let (_ctx, _rc, _cfg) = (context, runtime_components, cfg);
            Ok(())
        }
    };
    (mut $name:ident, $phase:ident, $docs:tt) => {
        #[doc = $docs]
        fn $name(
            &self,
            context: &mut $phase<'_>,
            runtime_components: &RuntimeComponents,
            cfg: &mut ConfigBag,
        ) -> Result<(), BoxError> {
            let (_ctx, _rc, _cfg) = (context, runtime_components, cfg);
            Ok(())
        }
    };
}

/// An interceptor allows injecting code into the SDK ’s request execution pipeline.
///
/// ## Terminology:
/// - An execution is one end-to-end invocation against an SDK client.
/// - An attempt is an attempt at performing an execution. By default executions are retried multiple
///   times based on the client ’s retry strategy.
/// - A hook is a single method on the interceptor, allowing injection of code into a specific part
///   of the SDK ’s request execution pipeline. Hooks are either "read" hooks, which make it possible
///   to read in-flight request or response messages, or "read/write" hooks, which make it possible
///   to modify in-flight request or output messages.
pub trait Intercept: fmt::Debug + Send + Sync {
    /// The name of this interceptor, used in error messages for debugging.
    fn name(&self) -> &'static str;

    /// A hook called at the start of an execution, before the SDK
    /// does anything else.
    ///
    /// **When:** This will **ALWAYS** be called once per execution. The duration
    /// between invocation of this hook and `after_execution` is very close
    /// to full duration of the execution.
    ///
    /// **Available Information:** The [`InterceptorContext::input`](context::InterceptorContext::input)
    /// is **ALWAYS** available. Other information **WILL NOT** be available.
    ///
    /// **Error Behavior:** Errors raised by this hook will be stored
    /// until all interceptors have had their `before_execution` invoked.
    /// Other hooks will then be skipped and execution will jump to
    /// `modify_before_completion` with the raised error as the
    /// [`InterceptorContext::output_or_error`](context::InterceptorContext::output_or_error). If multiple
    /// `before_execution` methods raise errors, the latest
    /// will be used and earlier ones will be logged and dropped.
    fn read_before_execution(
        &self,
        context: &BeforeSerializationInterceptorContextRef<'_>,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        let (_ctx, _cfg) = (context, cfg);
        Ok(())
    }

    interceptor_trait_fn!(
        mut modify_before_serialization,
        BeforeSerializationInterceptorContextMut,
        "
        A hook called before the input message is marshalled into a
        transport message.
        This method has the ability to modify and return a new
        request message of the same type.

        **When:** This will **ALWAYS** be called once per execution, except when a
        failure occurs earlier in the request pipeline.

        **Available Information:** The [`InterceptorContext::input`](context::InterceptorContext::input) is
        **ALWAYS** available. This request may have been modified by earlier
        `modify_before_serialization` hooks, and may be modified further by
        later hooks. Other information **WILL NOT** be available.

        **Error Behavior:** If errors are raised by this hook,
        execution will jump to `modify_before_completion` with the raised
        error as the [`InterceptorContext::output_or_error`](context::InterceptorContext::output_or_error).

        **Return Constraints:** The input message returned by this hook
        MUST be the same type of input message passed into this hook.
        If not, an error will immediately be raised.
        "
    );

    interceptor_trait_fn!(
        read_before_serialization,
        BeforeSerializationInterceptorContextRef,
        "
        A hook called before the input message is marshalled
        into a transport
        message.

        **When:** This will **ALWAYS** be called once per execution, except when a
        failure occurs earlier in the request pipeline. The
        duration between invocation of this hook and `after_serialization` is
        very close to the amount of time spent marshalling the request.

        **Available Information:** The [`InterceptorContext::input`](context::InterceptorContext::input) is
        **ALWAYS** available. Other information **WILL NOT** be available.

        **Error Behavior:** If errors are raised by this hook,
        execution will jump to `modify_before_completion` with the raised
        error as the [`InterceptorContext::output_or_error`](context::InterceptorContext::output_or_error).
        "
    );

    interceptor_trait_fn!(
        read_after_serialization,
        BeforeTransmitInterceptorContextRef,
        "
        A hook called after the input message is marshalled into
        a transport message.

        **When:** This will **ALWAYS** be called once per execution, except when a
        failure occurs earlier in the request pipeline. The duration
        between invocation of this hook and `before_serialization` is very
        close to the amount of time spent marshalling the request.

        **Available Information:** The [`InterceptorContext::request`](context::InterceptorContext::request)
        is **ALWAYS** available. Other information **WILL NOT** be available.

        **Error Behavior:** If errors are raised by this hook,
        execution will jump to `modify_before_completion` with the raised
        error as the [`InterceptorContext::output_or_error`](context::InterceptorContext::output_or_error).
        "
    );

    interceptor_trait_fn!(
        mut modify_before_retry_loop,
        BeforeTransmitInterceptorContextMut,
        "
        A hook called before the retry loop is entered. This method
        has the ability to modify and return a new transport request
        message of the same type, except when a failure occurs earlier in the request pipeline.

        **Available Information:** The [`InterceptorContext::request`](context::InterceptorContext::request)
        is **ALWAYS** available. Other information **WILL NOT** be available.

        **Error Behavior:** If errors are raised by this hook,
        execution will jump to `modify_before_completion` with the raised
        error as the [`InterceptorContext::output_or_error`](context::InterceptorContext::output_or_error).

        **Return Constraints:** The transport request message returned by this
        hook MUST be the same type of request message passed into this hook
        If not, an error will immediately be raised.
        "
    );

    interceptor_trait_fn!(
        read_before_attempt,
        BeforeTransmitInterceptorContextRef,
        "
        A hook called before each attempt at sending the transmission
        request message to the service.

        **When:** This will **ALWAYS** be called once per attempt, except when a
        failure occurs earlier in the request pipeline. This method will be
        called multiple times in the event of retries.

        **Available Information:** The [`InterceptorContext::request`](context::InterceptorContext::request)
        is **ALWAYS** available. Other information **WILL NOT** be available. In the event of retries,
        the `InterceptorContext` will not include changes made in previous
        attempts (e.g. by request signers or other interceptors).

        **Error Behavior:** Errors raised by this hook will be stored
        until all interceptors have had their `before_attempt` invoked.
        Other hooks will then be skipped and execution will jump to
        `modify_before_attempt_completion` with the raised error as the
        [`InterceptorContext::output_or_error`](context::InterceptorContext::output_or_error). If multiple
        `before_attempt` methods raise errors, the latest will be used
        and earlier ones will be logged and dropped.
        "
    );

    interceptor_trait_fn!(
        mut modify_before_signing,
        BeforeTransmitInterceptorContextMut,
        "
        A hook called before the transport request message is signed.
        This method has the ability to modify and return a new transport
        request message of the same type.

        **When:** This will **ALWAYS** be called once per attempt, except when a
        failure occurs earlier in the request pipeline. This method may be
        called multiple times in the event of retries.

        **Available Information:** The [`InterceptorContext::request`](context::InterceptorContext::request)
        is **ALWAYS** available. The `http::Request` may have been modified by earlier
        `modify_before_signing` hooks, and may be modified further by later
        hooks. Other information **WILL NOT** be available. In the event of
        retries, the `InterceptorContext` will not include changes made
        in previous attempts (e.g. by request signers or other interceptors).

        **Error Behavior:** If errors are raised by this
        hook, execution will jump to `modify_before_attempt_completion` with
        the raised error as the [`InterceptorContext::output_or_error`](context::InterceptorContext::output_or_error).

        **Return Constraints:** The transport request message returned by this
        hook MUST be the same type of request message passed into this hook

        If not, an error will immediately be raised.
        "
    );

    interceptor_trait_fn!(
        read_before_signing,
        BeforeTransmitInterceptorContextRef,
        "
        A hook called before the transport request message is signed.

        **When:** This will **ALWAYS** be called once per attempt, except when a
        failure occurs earlier in the request pipeline. This method may be
        called multiple times in the event of retries. The duration between
        invocation of this hook and `after_signing` is very close to
        the amount of time spent signing the request.

        **Available Information:** The [`InterceptorContext::request`](context::InterceptorContext::request) is **ALWAYS** available.
        Other information **WILL NOT** be available. In the event of retries,
        the `InterceptorContext` will not include changes made in previous
        attempts (e.g. by request signers or other interceptors).

        **Error Behavior:** If errors are raised by this
        hook, execution will jump to `modify_before_attempt_completion` with
        the raised error as the [`InterceptorContext::output_or_error`](context::InterceptorContext::output_or_error).
        "
    );

    interceptor_trait_fn!(
        read_after_signing,
        BeforeTransmitInterceptorContextRef,
        "
        A hook called after the transport request message is signed.

        **When:** This will **ALWAYS** be called once per attempt, except when a
        failure occurs earlier in the request pipeline. This method may be
        called multiple times in the event of retries. The duration between
        invocation of this hook and `before_signing` is very close to
        the amount of time spent signing the request.

        **Available Information:** The [`InterceptorContext::request`](context::InterceptorContext::request) is **ALWAYS** available.
        Other information **WILL NOT** be available. In the event of retries,
        the `InterceptorContext` will not include changes made in previous
        attempts (e.g. by request signers or other interceptors).

        **Error Behavior:** If errors are raised by this
        hook, execution will jump to `modify_before_attempt_completion` with
        the raised error as the [`InterceptorContext::output_or_error`](context::InterceptorContext::output_or_error).
        "
    );

    interceptor_trait_fn!(
        mut modify_before_transmit,
        BeforeTransmitInterceptorContextMut,
        "
        A hook called before the transport request message is sent to the
        service. This method has the ability to modify and return
        a new transport request message of the same type.

        **When:** This will **ALWAYS** be called once per attempt, except when a
        failure occurs earlier in the request pipeline. This method may be
        called multiple times in the event of retries.

        **Available Information:** The [`InterceptorContext::request`](context::InterceptorContext::request)
        is **ALWAYS** available. The `http::Request` may have been modified by earlier
        `modify_before_transmit` hooks, and may be modified further by later
        hooks. Other information **WILL NOT** be available.
        In the event of retries, the `InterceptorContext` will not include
        changes made in previous attempts (e.g. by request signers or
        other interceptors).

        **Error Behavior:** If errors are raised by this
        hook, execution will jump to `modify_before_attempt_completion` with
        the raised error as the [`InterceptorContext::output_or_error`](context::InterceptorContext::output_or_error).

        **Return Constraints:** The transport request message returned by this
        hook MUST be the same type of request message passed into this hook

        If not, an error will immediately be raised.
        "
    );

    interceptor_trait_fn!(
        read_before_transmit,
        BeforeTransmitInterceptorContextRef,
        "
        A hook called before the transport request message is sent to the
        service.

        **When:** This will **ALWAYS** be called once per attempt, except when a
        failure occurs earlier in the request pipeline. This method may be
        called multiple times in the event of retries. The duration between
        invocation of this hook and `after_transmit` is very close to
        the amount of time spent communicating with the service.
        Depending on the protocol, the duration may not include the
        time spent reading the response data.

        **Available Information:** The [`InterceptorContext::request`](context::InterceptorContext::request)
        is **ALWAYS** available. Other information **WILL NOT** be available. In the event of retries,
        the `InterceptorContext` will not include changes made in previous
        attempts (e.g. by request signers or other interceptors).


        **Error Behavior:** If errors are raised by this
        hook, execution will jump to `modify_before_attempt_completion` with
        the raised error as the [`InterceptorContext::output_or_error`](context::InterceptorContext::output_or_error).
        "
    );

    interceptor_trait_fn!(
        read_after_transmit,
        BeforeDeserializationInterceptorContextRef,
        "
        A hook called after the transport request message is sent to the
        service and a transport response message is received.

        **When:** This will **ALWAYS** be called once per attempt, except when a
        failure occurs earlier in the request pipeline. This method may be
        called multiple times in the event of retries. The duration between
        invocation of this hook and `before_transmit` is very close to
        the amount of time spent communicating with the service.
        Depending on the protocol, the duration may not include the time
        spent reading the response data.

        **Available Information:** The [`InterceptorContext::response`](context::InterceptorContext::response)
        is **ALWAYS** available. Other information **WILL NOT** be available. In the event of retries,
        the `InterceptorContext` will not include changes made in previous
        attempts (e.g. by request signers or other interceptors).

        **Error Behavior:** If errors are raised by this
        hook, execution will jump to `modify_before_attempt_completion` with
        the raised error as the [`InterceptorContext::output_or_error`](context::InterceptorContext::output_or_error).
        "
    );

    interceptor_trait_fn!(
        mut modify_before_deserialization,
        BeforeDeserializationInterceptorContextMut,
        "
        A hook called before the transport response message is unmarshalled.
        This method has the ability to modify and return a new transport
        response message of the same type.

        **When:** This will **ALWAYS** be called once per attempt, except when a
        failure occurs earlier in the request pipeline. This method may be
        called multiple times in the event of retries.

        **Available Information:** The [`InterceptorContext::response`](context::InterceptorContext::response)
        is **ALWAYS** available. The transmit_response may have been modified by earlier
        `modify_before_deserialization` hooks, and may be modified further by
        later hooks. Other information **WILL NOT** be available. In the event of
        retries, the `InterceptorContext` will not include changes made in
        previous attempts (e.g. by request signers or other interceptors).

        **Error Behavior:** If errors are raised by this
        hook, execution will jump to `modify_before_attempt_completion` with
        the raised error as the
        [`InterceptorContext::output_or_error`](context::InterceptorContext::output_or_error).

        **Return Constraints:** The transport response message returned by this
        hook MUST be the same type of response message passed into
        this hook. If not, an error will immediately be raised.
        "
    );

    interceptor_trait_fn!(
        read_before_deserialization,
        BeforeDeserializationInterceptorContextRef,
        "
        A hook called before the transport response message is unmarshalled

        **When:** This will **ALWAYS** be called once per attempt, except when a
        failure occurs earlier in the request pipeline. This method may be
        called multiple times in the event of retries. The duration between
        invocation of this hook and `after_deserialization` is very close
        to the amount of time spent unmarshalling the service response.
        Depending on the protocol and operation, the duration may include
        the time spent downloading the response data.

        **Available Information:** The [`InterceptorContext::response`](context::InterceptorContext::response)
        is **ALWAYS** available. Other information **WILL NOT** be available. In the event of retries,
        the `InterceptorContext` will not include changes made in previous
        attempts (e.g. by request signers or other interceptors).

        **Error Behavior:** If errors are raised by this
        hook, execution will jump to `modify_before_attempt_completion`
        with the raised error as the [`InterceptorContext::output_or_error`](context::InterceptorContext::output_or_error).
        "
    );

    interceptor_trait_fn!(
        read_after_deserialization,
        AfterDeserializationInterceptorContextRef,
        "
        A hook called after the transport response message is unmarshalled.

        **When:** This will **ALWAYS** be called once per attempt, except when a
        failure occurs earlier in the request pipeline. The duration
        between invocation of this hook and `before_deserialization` is
        very close to the amount of time spent unmarshalling the
        service response. Depending on the protocol and operation,
        the duration may include the time spent downloading
        the response data.

        **Available Information:** The [`InterceptorContext::response`](context::InterceptorContext::response)
        and [`InterceptorContext::output_or_error`](context::InterceptorContext::output_or_error)
        are **ALWAYS** available. In the event of retries, the `InterceptorContext` will not include changes made
        in previous attempts (e.g. by request signers or other interceptors).

        **Error Behavior:** If errors are raised by this
        hook, execution will jump to `modify_before_attempt_completion` with
        the raised error as the [`InterceptorContext::output_or_error`](context::InterceptorContext::output_or_error).
        "
    );

    /// A hook called when an attempt is completed. This method has the
    /// ability to modify and return a new output message or error
    /// matching the currently-executing operation.
    ///
    /// **When:** This will **ALWAYS** be called once per attempt, except when a
    /// failure occurs before `before_attempt`. This method may
    /// be called multiple times in the event of retries.
    ///
    /// **Available Information:**
    /// The [`InterceptorContext::input`](context::InterceptorContext::input),
    /// [`InterceptorContext::request`](context::InterceptorContext::request),
    /// [`InterceptorContext::response`](context::InterceptorContext::response), or
    /// [`InterceptorContext::output_or_error`](context::InterceptorContext::output_or_error) **MAY** be available.
    /// If the operation succeeded, the `output` will be available. Otherwise, any of the other
    /// pieces of information may be available depending on where in the operation lifecycle it failed.
    /// In the event of retries, the `InterceptorContext` will not include changes made
    /// in previous attempts (e.g. by request signers or other interceptors).
    ///
    /// **Error Behavior:** If errors are raised by this
    /// hook, execution will jump to `after_attempt` with
    /// the raised error as the [`InterceptorContext::output_or_error`](context::InterceptorContext::output_or_error).
    ///
    /// **Return Constraints:** Any output message returned by this
    /// hook MUST match the operation being invoked. Any error type can be
    /// returned, replacing the response currently in the context.
    fn modify_before_attempt_completion(
        &self,
        context: &mut FinalizerInterceptorContextMut<'_>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        let (_ctx, _rc, _cfg) = (context, runtime_components, cfg);
        Ok(())
    }

    /// A hook called when an attempt is completed.
    ///
    /// **When:** This will **ALWAYS** be called once per attempt, as long as
    /// `before_attempt` has been executed.
    ///
    /// **Available Information:**
    /// The [`InterceptorContext::input`](context::InterceptorContext::input),
    /// [`InterceptorContext::request`](context::InterceptorContext::request),
    /// [`InterceptorContext::response`](context::InterceptorContext::response), or
    /// [`InterceptorContext::output_or_error`](context::InterceptorContext::output_or_error) **MAY** be available.
    /// If the operation succeeded, the `output` will be available. Otherwise, any of the other
    /// pieces of information may be available depending on where in the operation lifecycle it failed.
    /// In the event of retries, the `InterceptorContext` will not include changes made
    /// in previous attempts (e.g. by request signers or other interceptors).
    ///
    /// **Error Behavior:** Errors raised by this hook will be stored
    /// until all interceptors have had their `after_attempt` invoked.
    /// If multiple `after_execution` methods raise errors, the latest
    /// will be used and earlier ones will be logged and dropped. If the
    /// retry strategy determines that the execution is retryable,
    /// execution will then jump to `before_attempt`. Otherwise,
    /// execution will jump to `modify_before_attempt_completion` with the
    /// raised error as the [`InterceptorContext::output_or_error`](context::InterceptorContext::output_or_error).
    fn read_after_attempt(
        &self,
        context: &FinalizerInterceptorContextRef<'_>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        let (_ctx, _rc, _cfg) = (context, runtime_components, cfg);
        Ok(())
    }

    /// A hook called when an execution is completed.
    /// This method has the ability to modify and return a new
    /// output message or error matching the currently - executing
    /// operation.
    ///
    /// **When:** This will **ALWAYS** be called once per execution.
    ///
    /// **Available Information:**
    /// The [`InterceptorContext::input`](context::InterceptorContext::input),
    /// [`InterceptorContext::request`](context::InterceptorContext::request),
    /// [`InterceptorContext::response`](context::InterceptorContext::response), or
    /// [`InterceptorContext::output_or_error`](context::InterceptorContext::output_or_error) **MAY** be available.
    /// If the operation succeeded, the `output` will be available. Otherwise, any of the other
    /// pieces of information may be available depending on where in the operation lifecycle it failed.
    /// In the event of retries, the `InterceptorContext` will not include changes made
    /// in previous attempts (e.g. by request signers or other interceptors).
    ///
    /// **Error Behavior:** If errors are raised by this
    /// hook , execution will jump to `after_attempt` with
    /// the raised error as the [`InterceptorContext::output_or_error`](context::InterceptorContext::output_or_error).
    ///
    /// **Return Constraints:** Any output message returned by this
    /// hook MUST match the operation being invoked. Any error type can be
    /// returned , replacing the response currently in the context.
    fn modify_before_completion(
        &self,
        context: &mut FinalizerInterceptorContextMut<'_>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        let (_ctx, _rc, _cfg) = (context, runtime_components, cfg);
        Ok(())
    }

    /// A hook called when an execution is completed.
    ///
    /// **When:** This will **ALWAYS** be called once per execution. The duration
    /// between invocation of this hook and `before_execution` is very
    /// close to the full duration of the execution.
    ///
    /// **Available Information:**
    /// The [`InterceptorContext::input`](context::InterceptorContext::input),
    /// [`InterceptorContext::request`](context::InterceptorContext::request),
    /// [`InterceptorContext::response`](context::InterceptorContext::response), or
    /// [`InterceptorContext::output_or_error`](context::InterceptorContext::output_or_error) **MAY** be available.
    /// If the operation succeeded, the `output` will be available. Otherwise, any of the other
    /// pieces of information may be available depending on where in the operation lifecycle it failed.
    /// In the event of retries, the `InterceptorContext` will not include changes made
    /// in previous attempts (e.g. by request signers or other interceptors).
    ///
    /// **Error Behavior:** Errors raised by this hook will be stored
    /// until all interceptors have had their `after_execution` invoked.
    /// The error will then be treated as the
    /// [`InterceptorContext::output_or_error`](context::InterceptorContext::output_or_error)
    /// to the customer. If multiple `after_execution` methods raise errors , the latest will be
    /// used and earlier ones will be logged and dropped.
    fn read_after_execution(
        &self,
        context: &FinalizerInterceptorContextRef<'_>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        let (_ctx, _rc, _cfg) = (context, runtime_components, cfg);
        Ok(())
    }
}

/// Interceptor wrapper that may be shared
#[derive(Clone)]
pub struct SharedInterceptor {
    interceptor: Arc<dyn Intercept>,
    check_enabled: Arc<dyn Fn(&ConfigBag) -> bool + Send + Sync>,
}

impl fmt::Debug for SharedInterceptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SharedInterceptor")
            .field("interceptor", &self.interceptor)
            .finish()
    }
}

impl SharedInterceptor {
    /// Create a new `SharedInterceptor` from `Interceptor`.
    pub fn new<T: Intercept + 'static>(interceptor: T) -> Self {
        Self {
            interceptor: Arc::new(interceptor),
            check_enabled: Arc::new(|conf: &ConfigBag| {
                conf.load::<DisableInterceptor<T>>().is_none()
            }),
        }
    }

    /// Checks if this interceptor is enabled in the given config.
    pub fn enabled(&self, conf: &ConfigBag) -> bool {
        (self.check_enabled)(conf)
    }
}

impl ValidateConfig for SharedInterceptor {}

impl Intercept for SharedInterceptor {
    fn name(&self) -> &'static str {
        self.interceptor.name()
    }

    fn modify_before_attempt_completion(
        &self,
        context: &mut FinalizerInterceptorContextMut<'_>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        self.interceptor
            .modify_before_attempt_completion(context, runtime_components, cfg)
    }

    fn modify_before_completion(
        &self,
        context: &mut FinalizerInterceptorContextMut<'_>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        self.interceptor
            .modify_before_completion(context, runtime_components, cfg)
    }

    fn modify_before_deserialization(
        &self,
        context: &mut BeforeDeserializationInterceptorContextMut<'_>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        self.interceptor
            .modify_before_deserialization(context, runtime_components, cfg)
    }

    fn modify_before_retry_loop(
        &self,
        context: &mut BeforeTransmitInterceptorContextMut<'_>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        self.interceptor
            .modify_before_retry_loop(context, runtime_components, cfg)
    }

    fn modify_before_serialization(
        &self,
        context: &mut BeforeSerializationInterceptorContextMut<'_>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        self.interceptor
            .modify_before_serialization(context, runtime_components, cfg)
    }

    fn modify_before_signing(
        &self,
        context: &mut BeforeTransmitInterceptorContextMut<'_>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        self.interceptor
            .modify_before_signing(context, runtime_components, cfg)
    }

    fn modify_before_transmit(
        &self,
        context: &mut BeforeTransmitInterceptorContextMut<'_>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        self.interceptor
            .modify_before_transmit(context, runtime_components, cfg)
    }

    fn read_after_attempt(
        &self,
        context: &FinalizerInterceptorContextRef<'_>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        self.interceptor
            .read_after_attempt(context, runtime_components, cfg)
    }

    fn read_after_deserialization(
        &self,
        context: &AfterDeserializationInterceptorContextRef<'_>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        self.interceptor
            .read_after_deserialization(context, runtime_components, cfg)
    }

    fn read_after_execution(
        &self,
        context: &FinalizerInterceptorContextRef<'_>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        self.interceptor
            .read_after_execution(context, runtime_components, cfg)
    }

    fn read_after_serialization(
        &self,
        context: &BeforeTransmitInterceptorContextRef<'_>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        self.interceptor
            .read_after_serialization(context, runtime_components, cfg)
    }

    fn read_after_signing(
        &self,
        context: &BeforeTransmitInterceptorContextRef<'_>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        self.interceptor
            .read_after_signing(context, runtime_components, cfg)
    }

    fn read_after_transmit(
        &self,
        context: &BeforeDeserializationInterceptorContextRef<'_>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        self.interceptor
            .read_after_transmit(context, runtime_components, cfg)
    }

    fn read_before_attempt(
        &self,
        context: &BeforeTransmitInterceptorContextRef<'_>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        self.interceptor
            .read_before_attempt(context, runtime_components, cfg)
    }

    fn read_before_deserialization(
        &self,
        context: &BeforeDeserializationInterceptorContextRef<'_>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        self.interceptor
            .read_before_deserialization(context, runtime_components, cfg)
    }

    fn read_before_execution(
        &self,
        context: &BeforeSerializationInterceptorContextRef<'_>,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        self.interceptor.read_before_execution(context, cfg)
    }

    fn read_before_serialization(
        &self,
        context: &BeforeSerializationInterceptorContextRef<'_>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        self.interceptor
            .read_before_serialization(context, runtime_components, cfg)
    }

    fn read_before_signing(
        &self,
        context: &BeforeTransmitInterceptorContextRef<'_>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        self.interceptor
            .read_before_signing(context, runtime_components, cfg)
    }

    fn read_before_transmit(
        &self,
        context: &BeforeTransmitInterceptorContextRef<'_>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        self.interceptor
            .read_before_transmit(context, runtime_components, cfg)
    }
}

impl_shared_conversions!(convert SharedInterceptor from Intercept using SharedInterceptor::new);

/// Generalized interceptor disabling interface
///
/// RuntimePlugins can disable interceptors by inserting [`DisableInterceptor<T>`](DisableInterceptor) into the config bag
#[must_use]
#[derive(Debug)]
pub struct DisableInterceptor<T> {
    _t: PhantomData<T>,
    #[allow(unused)]
    cause: &'static str,
}

impl<T> Storable for DisableInterceptor<T>
where
    T: fmt::Debug + Send + Sync + 'static,
{
    type Storer = StoreReplace<Self>;
}

/// Disable an interceptor with a given cause
pub fn disable_interceptor<T: Intercept>(cause: &'static str) -> DisableInterceptor<T> {
    DisableInterceptor {
        _t: PhantomData,
        cause,
    }
}
