use crate::Uninhabited;
use crate::component::func::{ComponentType, LiftContext, LowerContext};
use crate::component::{Instance, Val};
use crate::runtime::vm::VMStore;
use anyhow::{Result, anyhow};
use core::future::Future;
use core::marker::PhantomData;
use core::pin::pin;
use core::task::{Context, Poll, Waker};
use wasmtime_environ::component::{InterfaceType, RuntimeComponentInstanceIndex};

fn should_have_failed_validation<T>(what: &str) -> Result<T> {
    // This should be unreachable; if we trap here, it indicates a
    // bug in Wasmtime rather than in the guest.
    Err(anyhow!(
        "{what} should have failed validation \
         when `component-model-async` feature disabled"
    ))
}

impl Instance {
    pub(crate) fn poll_and_block<R: Send + Sync + 'static>(
        self,
        _store: &mut dyn VMStore,
        future: impl Future<Output = Result<R>> + Send + 'static,
        _caller_instance: RuntimeComponentInstanceIndex,
    ) -> Result<R> {
        match pin!(future).poll(&mut Context::from_waker(Waker::noop())) {
            Poll::Ready(result) => result,
            Poll::Pending => should_have_failed_validation("async lowered import"),
        }
    }
}

pub(crate) fn lower_future_to_index<U>(
    _rep: u32,
    _cx: &mut LowerContext<'_, U>,
    _ty: InterfaceType,
) -> Result<u32> {
    should_have_failed_validation("use of `future`")
}

pub(crate) fn lower_stream_to_index<U>(
    _rep: u32,
    _cx: &mut LowerContext<'_, U>,
    _ty: InterfaceType,
) -> Result<u32> {
    should_have_failed_validation("use of `stream`")
}

pub(crate) fn lower_error_context_to_index<U>(
    _rep: u32,
    _cx: &mut LowerContext<'_, U>,
    _ty: InterfaceType,
) -> Result<u32> {
    should_have_failed_validation("use of `error-context`")
}

pub struct ErrorContext(Uninhabited);

impl ErrorContext {
    pub(crate) fn into_val(self) -> Val {
        match self.0 {}
    }

    pub(crate) fn linear_lift_from_flat(
        _cx: &mut LiftContext<'_>,
        _ty: InterfaceType,
        _src: &<u32 as ComponentType>::Lower,
    ) -> Result<Self> {
        should_have_failed_validation("use of `error-context`")
    }

    pub(crate) fn linear_lift_from_memory(
        _cx: &mut LiftContext<'_>,
        _ty: InterfaceType,
        _bytes: &[u8],
    ) -> Result<Self> {
        should_have_failed_validation("use of `error-context`")
    }
}

pub struct StreamReader<P> {
    uninhabited: Uninhabited,
    _phantom: PhantomData<P>,
}

impl<P> StreamReader<P> {
    pub(crate) fn into_val(self) -> Val {
        match self.uninhabited {}
    }

    pub(crate) fn linear_lift_from_flat(
        _cx: &mut LiftContext<'_>,
        _ty: InterfaceType,
        _src: &<u32 as ComponentType>::Lower,
    ) -> Result<Self> {
        should_have_failed_validation("use of `stream`")
    }

    pub(crate) fn linear_lift_from_memory(
        _cx: &mut LiftContext<'_>,
        _ty: InterfaceType,
        _bytes: &[u8],
    ) -> Result<Self> {
        should_have_failed_validation("use of `stream`")
    }
}

pub struct FutureReader<P> {
    uninhabited: Uninhabited,
    _phantom: PhantomData<P>,
}

impl<P> FutureReader<P> {
    pub(crate) fn into_val(self) -> Val {
        match self.uninhabited {}
    }

    pub(crate) fn linear_lift_from_flat(
        _cx: &mut LiftContext<'_>,
        _ty: InterfaceType,
        _src: &<u32 as ComponentType>::Lower,
    ) -> Result<Self> {
        should_have_failed_validation("use of `future`")
    }

    pub(crate) fn linear_lift_from_memory(
        _cx: &mut LiftContext<'_>,
        _ty: InterfaceType,
        _bytes: &[u8],
    ) -> Result<Self> {
        should_have_failed_validation("use of `future`")
    }
}
