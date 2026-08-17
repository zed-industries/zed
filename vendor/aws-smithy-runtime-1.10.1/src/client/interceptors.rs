/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_runtime_api::box_error::BoxError;
use aws_smithy_runtime_api::client::interceptors::context::{
    BeforeSerializationInterceptorContextRef, BeforeTransmitInterceptorContextMut,
    FinalizerInterceptorContextMut, FinalizerInterceptorContextRef,
};
use aws_smithy_runtime_api::client::interceptors::context::{
    Error, Input, InterceptorContext, Output,
};
use aws_smithy_runtime_api::client::interceptors::{
    Intercept, InterceptorError, SharedInterceptor,
};
use aws_smithy_runtime_api::client::orchestrator::HttpRequest;
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
use aws_smithy_types::body::SdkBody;
use aws_smithy_types::config_bag::ConfigBag;
use aws_smithy_types::error::display::DisplayErrorContext;
use std::error::Error as StdError;
use std::fmt;
use std::marker::PhantomData;

macro_rules! interceptor_impl_fn {
    (mut $interceptor:ident) => {
        pub(crate) fn $interceptor(
            self,
            ctx: &mut InterceptorContext,
            runtime_components: &RuntimeComponents,
            cfg: &mut ConfigBag,
        ) -> Result<(), InterceptorError> {
            tracing::trace!(concat!(
                "running `",
                stringify!($interceptor),
                "` interceptors"
            ));
            let mut result: Result<(), (&str, BoxError)> = Ok(());
            let mut ctx = ctx.into();
            for interceptor in self.into_iter() {
                if let Some(interceptor) = interceptor.if_enabled(cfg) {
                    if let Err(new_error) =
                        interceptor.$interceptor(&mut ctx, runtime_components, cfg)
                    {
                        if let Err(last_error) = result {
                            tracing::debug!(
                                "{}::{}: {}",
                                last_error.0,
                                stringify!($interceptor),
                                DisplayErrorContext(&*last_error.1)
                            );
                        }
                        result = Err((interceptor.name(), new_error));
                    }
                }
            }
            result.map_err(|(name, err)| InterceptorError::$interceptor(name, err))
        }
    };
    (ref $interceptor:ident) => {
        pub(crate) fn $interceptor(
            self,
            ctx: &InterceptorContext,
            runtime_components: &RuntimeComponents,
            cfg: &mut ConfigBag,
        ) -> Result<(), InterceptorError> {
            tracing::trace!(concat!(
                "running `",
                stringify!($interceptor),
                "` interceptors"
            ));
            let mut result: Result<(), (&str, BoxError)> = Ok(());
            let ctx = ctx.into();
            for interceptor in self.into_iter() {
                if let Some(interceptor) = interceptor.if_enabled(cfg) {
                    if let Err(new_error) = interceptor.$interceptor(&ctx, runtime_components, cfg)
                    {
                        if let Err(last_error) = result {
                            tracing::debug!(
                                "{}::{}: {}",
                                last_error.0,
                                stringify!($interceptor),
                                DisplayErrorContext(&*last_error.1)
                            );
                        }
                        result = Err((interceptor.name(), new_error));
                    }
                }
            }
            result.map_err(|(name, err)| InterceptorError::$interceptor(name, err))
        }
    };
}

#[derive(Debug)]
pub(crate) struct Interceptors<I> {
    interceptors: I,
}

impl<I> Interceptors<I>
where
    I: Iterator<Item = SharedInterceptor>,
{
    pub(crate) fn new(interceptors: I) -> Self {
        Self { interceptors }
    }

    fn into_iter(self) -> impl Iterator<Item = ConditionallyEnabledInterceptor> {
        self.interceptors.map(ConditionallyEnabledInterceptor)
    }

    pub(crate) fn read_before_execution(
        self,
        operation: bool,
        ctx: &InterceptorContext<Input, Output, Error>,
        cfg: &mut ConfigBag,
    ) -> Result<(), InterceptorError> {
        tracing::trace!(
            "running {} `read_before_execution` interceptors",
            if operation { "operation" } else { "client" }
        );
        let mut result: Result<(), (&str, BoxError)> = Ok(());
        let ctx: BeforeSerializationInterceptorContextRef<'_> = ctx.into();
        for interceptor in self.into_iter() {
            if let Some(interceptor) = interceptor.if_enabled(cfg) {
                if let Err(new_error) = interceptor.read_before_execution(&ctx, cfg) {
                    if let Err(last_error) = result {
                        tracing::debug!(
                            "{}::{}: {}",
                            last_error.0,
                            "read_before_execution",
                            DisplayErrorContext(&*last_error.1)
                        );
                    }
                    result = Err((interceptor.name(), new_error));
                }
            }
        }
        result.map_err(|(name, err)| InterceptorError::read_before_execution(name, err))
    }

    interceptor_impl_fn!(mut modify_before_serialization);
    interceptor_impl_fn!(ref read_before_serialization);
    interceptor_impl_fn!(ref read_after_serialization);
    interceptor_impl_fn!(mut modify_before_retry_loop);
    interceptor_impl_fn!(ref read_before_attempt);
    interceptor_impl_fn!(mut modify_before_signing);
    interceptor_impl_fn!(ref read_before_signing);
    interceptor_impl_fn!(ref read_after_signing);
    interceptor_impl_fn!(mut modify_before_transmit);
    interceptor_impl_fn!(ref read_before_transmit);
    interceptor_impl_fn!(ref read_after_transmit);
    interceptor_impl_fn!(mut modify_before_deserialization);
    interceptor_impl_fn!(ref read_before_deserialization);
    interceptor_impl_fn!(ref read_after_deserialization);

    pub(crate) fn modify_before_attempt_completion(
        self,
        ctx: &mut InterceptorContext<Input, Output, Error>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), InterceptorError> {
        tracing::trace!("running `modify_before_attempt_completion` interceptors");
        let mut result: Result<(), (&str, BoxError)> = Ok(());
        let mut ctx: FinalizerInterceptorContextMut<'_> = ctx.into();
        for interceptor in self.into_iter() {
            if let Some(interceptor) = interceptor.if_enabled(cfg) {
                if let Err(new_error) =
                    interceptor.modify_before_attempt_completion(&mut ctx, runtime_components, cfg)
                {
                    if let Err(last_error) = result {
                        tracing::debug!(
                            "{}::{}: {}",
                            last_error.0,
                            "modify_before_attempt_completion",
                            DisplayErrorContext(&*last_error.1)
                        );
                    }
                    result = Err((interceptor.name(), new_error));
                }
            }
        }
        result.map_err(|(name, err)| InterceptorError::modify_before_attempt_completion(name, err))
    }

    pub(crate) fn read_after_attempt(
        self,
        ctx: &InterceptorContext<Input, Output, Error>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), InterceptorError> {
        tracing::trace!("running `read_after_attempt` interceptors");
        let mut result: Result<(), (&str, BoxError)> = Ok(());
        let ctx: FinalizerInterceptorContextRef<'_> = ctx.into();
        for interceptor in self.into_iter() {
            if let Some(interceptor) = interceptor.if_enabled(cfg) {
                if let Err(new_error) =
                    interceptor.read_after_attempt(&ctx, runtime_components, cfg)
                {
                    if let Err(last_error) = result {
                        tracing::debug!(
                            "{}::{}: {}",
                            last_error.0,
                            "read_after_attempt",
                            DisplayErrorContext(&*last_error.1)
                        );
                    }
                    result = Err((interceptor.name(), new_error));
                }
            }
        }
        result.map_err(|(name, err)| InterceptorError::read_after_attempt(name, err))
    }

    pub(crate) fn modify_before_completion(
        self,
        ctx: &mut InterceptorContext<Input, Output, Error>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), InterceptorError> {
        tracing::trace!("running `modify_before_completion` interceptors");
        let mut result: Result<(), (&str, BoxError)> = Ok(());
        let mut ctx: FinalizerInterceptorContextMut<'_> = ctx.into();
        for interceptor in self.into_iter() {
            if let Some(interceptor) = interceptor.if_enabled(cfg) {
                if let Err(new_error) =
                    interceptor.modify_before_completion(&mut ctx, runtime_components, cfg)
                {
                    if let Err(last_error) = result {
                        tracing::debug!(
                            "{}::{}: {}",
                            last_error.0,
                            "modify_before_completion",
                            DisplayErrorContext(&*last_error.1)
                        );
                    }
                    result = Err((interceptor.name(), new_error));
                }
            }
        }
        result.map_err(|(name, err)| InterceptorError::modify_before_completion(name, err))
    }

    pub(crate) fn read_after_execution(
        self,
        ctx: &InterceptorContext<Input, Output, Error>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), InterceptorError> {
        tracing::trace!("running `read_after_execution` interceptors");
        let mut result: Result<(), (&str, BoxError)> = Ok(());
        let ctx: FinalizerInterceptorContextRef<'_> = ctx.into();
        for interceptor in self.into_iter() {
            if let Some(interceptor) = interceptor.if_enabled(cfg) {
                if let Err(new_error) =
                    interceptor.read_after_execution(&ctx, runtime_components, cfg)
                {
                    if let Err(last_error) = result {
                        tracing::debug!(
                            "{}::{}: {}",
                            last_error.0,
                            "read_after_execution",
                            DisplayErrorContext(&*last_error.1)
                        );
                    }
                    result = Err((interceptor.name(), new_error));
                }
            }
        }
        result.map_err(|(name, err)| InterceptorError::read_after_execution(name, err))
    }
}

/// A interceptor wrapper to conditionally enable the interceptor based on
/// [`DisableInterceptor`](aws_smithy_runtime_api::client::interceptors::DisableInterceptor)
struct ConditionallyEnabledInterceptor(SharedInterceptor);
impl ConditionallyEnabledInterceptor {
    fn if_enabled(&self, cfg: &ConfigBag) -> Option<&dyn Intercept> {
        if self.0.enabled(cfg) {
            Some(&self.0)
        } else {
            None
        }
    }
}

/// Interceptor that maps the request with a given function.
pub struct MapRequestInterceptor<F, E> {
    f: F,
    _phantom: PhantomData<E>,
}

impl<F, E> fmt::Debug for MapRequestInterceptor<F, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MapRequestInterceptor")
    }
}

impl<F, E> MapRequestInterceptor<F, E> {
    /// Creates a new `MapRequestInterceptor`.
    pub fn new(f: F) -> Self {
        Self {
            f,
            _phantom: PhantomData,
        }
    }
}

impl<F, E> Intercept for MapRequestInterceptor<F, E>
where
    F: Fn(HttpRequest) -> Result<HttpRequest, E> + Send + Sync + 'static,
    E: StdError + Send + Sync + 'static,
{
    fn name(&self) -> &'static str {
        "MapRequestInterceptor"
    }

    fn modify_before_signing(
        &self,
        context: &mut BeforeTransmitInterceptorContextMut<'_>,
        _runtime_components: &RuntimeComponents,
        _cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        let mut request = HttpRequest::new(SdkBody::taken());
        std::mem::swap(&mut request, context.request_mut());
        let mut mapped = (self.f)(request)?;
        std::mem::swap(&mut mapped, context.request_mut());

        Ok(())
    }
}

/// Interceptor that mutates the request with a given function.
pub struct MutateRequestInterceptor<F> {
    f: F,
}

impl<F> fmt::Debug for MutateRequestInterceptor<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MutateRequestInterceptor")
    }
}

impl<F> MutateRequestInterceptor<F> {
    /// Creates a new `MutateRequestInterceptor`.
    pub fn new(f: F) -> Self {
        Self { f }
    }
}

impl<F> Intercept for MutateRequestInterceptor<F>
where
    F: Fn(&mut HttpRequest) + Send + Sync + 'static,
{
    fn name(&self) -> &'static str {
        "MutateRequestInterceptor"
    }

    fn modify_before_signing(
        &self,
        context: &mut BeforeTransmitInterceptorContextMut<'_>,
        _runtime_components: &RuntimeComponents,
        _cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        let request = context.request_mut();
        (self.f)(request);

        Ok(())
    }
}

#[cfg(all(test, feature = "test-util"))]
mod tests {
    use super::*;
    use aws_smithy_runtime_api::box_error::BoxError;
    use aws_smithy_runtime_api::client::interceptors::context::{
        BeforeTransmitInterceptorContextRef, Input, InterceptorContext,
    };
    use aws_smithy_runtime_api::client::interceptors::{
        disable_interceptor, Intercept, SharedInterceptor,
    };
    use aws_smithy_runtime_api::client::runtime_components::{
        RuntimeComponents, RuntimeComponentsBuilder,
    };
    use aws_smithy_types::config_bag::ConfigBag;

    #[derive(Debug)]
    struct TestInterceptor;
    impl Intercept for TestInterceptor {
        fn name(&self) -> &'static str {
            "TestInterceptor"
        }
    }

    #[test]
    fn test_disable_interceptors() {
        #[derive(Debug)]
        struct PanicInterceptor;
        impl Intercept for PanicInterceptor {
            fn name(&self) -> &'static str {
                "PanicInterceptor"
            }

            fn read_before_transmit(
                &self,
                _context: &BeforeTransmitInterceptorContextRef<'_>,
                _rc: &RuntimeComponents,
                _cfg: &mut ConfigBag,
            ) -> Result<(), BoxError> {
                Err("boom".into())
            }
        }
        let rc = RuntimeComponentsBuilder::for_tests()
            .with_interceptor(SharedInterceptor::new(PanicInterceptor))
            .with_interceptor(SharedInterceptor::new(TestInterceptor))
            .build()
            .unwrap();

        let mut cfg = ConfigBag::base();
        let interceptors = Interceptors::new(rc.interceptors());
        assert_eq!(
            interceptors
                .into_iter()
                .filter(|i| i.if_enabled(&cfg).is_some())
                .count(),
            2
        );

        Interceptors::new(rc.interceptors())
            .read_before_transmit(
                &InterceptorContext::new(Input::doesnt_matter()),
                &rc,
                &mut cfg,
            )
            .expect_err("interceptor returns error");
        cfg.interceptor_state()
            .store_put(disable_interceptor::<PanicInterceptor>("test"));
        assert_eq!(
            Interceptors::new(rc.interceptors())
                .into_iter()
                .filter(|i| i.if_enabled(&cfg).is_some())
                .count(),
            1
        );
        // shouldn't error because interceptors won't run
        Interceptors::new(rc.interceptors())
            .read_before_transmit(
                &InterceptorContext::new(Input::doesnt_matter()),
                &rc,
                &mut cfg,
            )
            .expect("interceptor is now disabled");
    }
}
