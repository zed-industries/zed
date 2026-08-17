/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use fastrand::Rng;
use http_1x::{HeaderName, HeaderValue};

use aws_smithy_runtime_api::box_error::BoxError;
use aws_smithy_runtime_api::client::interceptors::context::BeforeTransmitInterceptorContextMut;
use aws_smithy_runtime_api::client::interceptors::Intercept;
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
use aws_smithy_types::config_bag::{ConfigBag, Storable, StoreReplace};
#[cfg(feature = "test-util")]
pub use test_util::{NoInvocationIdGenerator, PredefinedInvocationIdGenerator};

#[allow(clippy::declare_interior_mutable_const)] // we will never mutate this
const AMZ_SDK_INVOCATION_ID: HeaderName = HeaderName::from_static("amz-sdk-invocation-id");

/// A generator for returning new invocation IDs on demand.
pub trait InvocationIdGenerator: Debug + Send + Sync {
    /// Call this function to receive a new [`InvocationId`] or an error explaining why one couldn't
    /// be provided.
    fn generate(&self) -> Result<Option<InvocationId>, BoxError>;
}

/// Dynamic dispatch implementation of [`InvocationIdGenerator`]
#[derive(Clone, Debug)]
pub struct SharedInvocationIdGenerator(Arc<dyn InvocationIdGenerator>);

impl SharedInvocationIdGenerator {
    /// Creates a new [`SharedInvocationIdGenerator`].
    pub fn new(gen: impl InvocationIdGenerator + 'static) -> Self {
        Self(Arc::new(gen))
    }
}

impl InvocationIdGenerator for SharedInvocationIdGenerator {
    fn generate(&self) -> Result<Option<InvocationId>, BoxError> {
        self.0.generate()
    }
}

impl Storable for SharedInvocationIdGenerator {
    type Storer = StoreReplace<Self>;
}

/// An invocation ID generator that uses random UUIDs for the invocation ID.
#[derive(Debug, Default)]
pub struct DefaultInvocationIdGenerator {
    rng: Mutex<Rng>,
}

impl DefaultInvocationIdGenerator {
    /// Creates a new [`DefaultInvocationIdGenerator`].
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a [`DefaultInvocationIdGenerator`] with the given seed.
    pub fn with_seed(seed: u64) -> Self {
        Self {
            rng: Mutex::new(Rng::with_seed(seed)),
        }
    }
}

impl InvocationIdGenerator for DefaultInvocationIdGenerator {
    fn generate(&self) -> Result<Option<InvocationId>, BoxError> {
        let mut rng = self.rng.lock().unwrap();
        let mut random_bytes = [0u8; 16];
        rng.fill(&mut random_bytes);

        let id = uuid::Builder::from_random_bytes(random_bytes).into_uuid();
        Ok(Some(InvocationId::new(id.to_string())))
    }
}

/// This interceptor generates a UUID and attaches it to all request attempts made as part of this operation.
#[non_exhaustive]
#[derive(Debug, Default)]
pub struct InvocationIdInterceptor {
    default: DefaultInvocationIdGenerator,
}

impl InvocationIdInterceptor {
    /// Creates a new `InvocationIdInterceptor`
    pub fn new() -> Self {
        Self::default()
    }
}

impl Intercept for InvocationIdInterceptor {
    fn name(&self) -> &'static str {
        "InvocationIdInterceptor"
    }

    fn modify_before_retry_loop(
        &self,
        _ctx: &mut BeforeTransmitInterceptorContextMut<'_>,
        _runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        let gen = cfg
            .load::<SharedInvocationIdGenerator>()
            .map(|gen| gen as &dyn InvocationIdGenerator)
            .unwrap_or(&self.default);
        if let Some(id) = gen.generate()? {
            cfg.interceptor_state().store_put::<InvocationId>(id);
        }

        Ok(())
    }

    fn modify_before_transmit(
        &self,
        ctx: &mut BeforeTransmitInterceptorContextMut<'_>,
        _runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        let headers = ctx.request_mut().headers_mut();
        if let Some(id) = cfg.load::<InvocationId>() {
            headers.append(AMZ_SDK_INVOCATION_ID, id.0.clone());
        }
        Ok(())
    }
}

/// InvocationId provides a consistent ID across retries
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvocationId(HeaderValue);

impl InvocationId {
    /// Create an invocation ID with the given value.
    ///
    /// # Panics
    /// This constructor will panic if the given invocation ID is not a valid HTTP header value.
    pub fn new(invocation_id: String) -> Self {
        Self(
            HeaderValue::try_from(invocation_id)
                .expect("invocation ID must be a valid HTTP header value"),
        )
    }
}

impl Storable for InvocationId {
    type Storer = StoreReplace<Self>;
}

#[cfg(feature = "test-util")]
mod test_util {
    use super::*;

    impl InvocationId {
        /// Create a new invocation ID from a `&'static str`.
        pub fn new_from_str(uuid: &'static str) -> Self {
            InvocationId(HeaderValue::from_static(uuid))
        }
    }

    /// A "generator" that returns [`InvocationId`]s from a predefined list.
    #[derive(Debug)]
    pub struct PredefinedInvocationIdGenerator {
        pre_generated_ids: Arc<Mutex<Vec<InvocationId>>>,
    }

    impl PredefinedInvocationIdGenerator {
        /// Given a `Vec<InvocationId>`, create a new [`PredefinedInvocationIdGenerator`].
        pub fn new(mut invocation_ids: Vec<InvocationId>) -> Self {
            // We're going to pop ids off of the end of the list, so we need to reverse the list or else
            // we'll be popping the ids in reverse order, confusing the poor test writer.
            invocation_ids.reverse();

            Self {
                pre_generated_ids: Arc::new(Mutex::new(invocation_ids)),
            }
        }
    }

    impl InvocationIdGenerator for PredefinedInvocationIdGenerator {
        fn generate(&self) -> Result<Option<InvocationId>, BoxError> {
            Ok(Some(
                self.pre_generated_ids
                    .lock()
                    .expect("this will never be under contention")
                    .pop()
                    .expect("testers will provide enough invocation IDs"),
            ))
        }
    }

    /// A "generator" that always returns `None`.
    #[derive(Debug, Default)]
    pub struct NoInvocationIdGenerator;

    impl NoInvocationIdGenerator {
        /// Create a new [`NoInvocationIdGenerator`].
        pub fn new() -> Self {
            Self
        }
    }

    impl InvocationIdGenerator for NoInvocationIdGenerator {
        fn generate(&self) -> Result<Option<InvocationId>, BoxError> {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use aws_smithy_runtime_api::client::interceptors::context::{
        BeforeTransmitInterceptorContextMut, Input, InterceptorContext,
    };
    use aws_smithy_runtime_api::client::interceptors::Intercept;
    use aws_smithy_runtime_api::client::orchestrator::HttpRequest;
    use aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder;
    use aws_smithy_types::config_bag::ConfigBag;

    use super::*;

    fn expect_header<'a>(
        context: &'a BeforeTransmitInterceptorContextMut<'_>,
        header_name: &str,
    ) -> &'a str {
        context.request().headers().get(header_name).unwrap()
    }

    #[test]
    fn default_id_generator() {
        let rc = RuntimeComponentsBuilder::for_tests().build().unwrap();
        let mut ctx = InterceptorContext::new(Input::doesnt_matter());
        ctx.enter_serialization_phase();
        ctx.set_request(HttpRequest::empty());
        let _ = ctx.take_input();
        ctx.enter_before_transmit_phase();

        let mut cfg = ConfigBag::base();
        let interceptor = InvocationIdInterceptor::new();
        let mut ctx = Into::into(&mut ctx);
        interceptor
            .modify_before_retry_loop(&mut ctx, &rc, &mut cfg)
            .unwrap();
        interceptor
            .modify_before_transmit(&mut ctx, &rc, &mut cfg)
            .unwrap();

        let expected = cfg.load::<InvocationId>().expect("invocation ID was set");
        let header = expect_header(&ctx, "amz-sdk-invocation-id");
        assert_eq!(expected.0, header, "the invocation ID in the config bag must match the invocation ID in the request header");
        // UUID should include 32 chars and 4 dashes
        assert_eq!(header.len(), 36);
    }

    #[cfg(feature = "test-util")]
    #[test]
    fn custom_id_generator() {
        use aws_smithy_types::config_bag::Layer;
        let rc = RuntimeComponentsBuilder::for_tests().build().unwrap();
        let mut ctx = InterceptorContext::new(Input::doesnt_matter());
        ctx.enter_serialization_phase();
        ctx.set_request(HttpRequest::empty());
        let _ = ctx.take_input();
        ctx.enter_before_transmit_phase();

        let mut cfg = ConfigBag::base();
        let mut layer = Layer::new("test");
        layer.store_put(SharedInvocationIdGenerator::new(
            PredefinedInvocationIdGenerator::new(vec![InvocationId::new(
                "the-best-invocation-id".into(),
            )]),
        ));
        cfg.push_layer(layer);

        let interceptor = InvocationIdInterceptor::new();
        let mut ctx = Into::into(&mut ctx);
        interceptor
            .modify_before_retry_loop(&mut ctx, &rc, &mut cfg)
            .unwrap();
        interceptor
            .modify_before_transmit(&mut ctx, &rc, &mut cfg)
            .unwrap();

        let header = expect_header(&ctx, "amz-sdk-invocation-id");
        assert_eq!("the-best-invocation-id", header);
    }
}
