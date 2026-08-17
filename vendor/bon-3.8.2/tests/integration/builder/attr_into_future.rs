/// [`core::future::IntoFuture`] relies on [`Box`]. Also this trait was
/// introduced in Rust 1.64, while `bon`'s MSRV is 1.59 at the time of this
/// writing.
#[cfg(any(feature = "std", feature = "alloc"))]
#[rustversion::since(1.64)]
mod tests {
    use crate::prelude::*;
    use core::future::{ready, IntoFuture};
    use core::marker::PhantomData;

    async fn assert_send<B>(builder: B) -> B::Output
    where
        B: IntoFuture + Send,
        B::IntoFuture: Send,
    {
        #[expect(clippy::incompatible_msrv)]
        let fut = builder.into_future();
        let _: &dyn Send = &fut;
        fut.await
    }

    #[expect(clippy::future_not_send)]
    async fn non_send_future() {
        // By keeping `Rc` across an await point, we force the compiler to store it
        // as part of the future's state machine struct and thus we make it non-Send
        let non_send = PhantomData::<Rc<()>>;

        ready(()).await;

        let _ = &non_send;
    }

    mod test_fn {
        use super::*;

        #[tokio::test]
        async fn basic() {
            #[builder(derive(IntoFuture(Box)))]
            async fn simple_async_fn(value: u32) -> u32 {
                ready(value * 2).await
            }

            // Test direct call.
            let builder = simple_async_fn().value(21).call();
            assert_eq!(assert_send(builder).await, 42);

            // Test using IntoFuture with await.
            let builder = simple_async_fn().value(21);
            assert_eq!(assert_send(builder).await, 42);
        }

        #[tokio::test]
        async fn non_send() {
            #[builder(derive(IntoFuture(Box, ?Send)))]
            #[expect(clippy::future_not_send)]
            async fn non_send_async_fn(value: u32) -> u32 {
                non_send_future().await;
                // This future can be !Send.
                ready(value * 2).await
            }

            // Test with non-Send future.
            let result = non_send_async_fn().value(21).await;

            assert_eq!(result, 42);
        }

        #[tokio::test]
        async fn result() {
            #[builder(derive(IntoFuture(Box)))]
            async fn async_with_result(value: u32) -> Result<u32, &'static str> {
                ready(if value > 0 {
                    Ok(value * 2)
                } else {
                    Err("Value must be positive")
                })
                .await
            }

            // Test successful case.
            let builder = async_with_result().value(21);
            assert_eq!(assert_send(builder).await.unwrap(), 42);

            // Test error case.
            let builder = async_with_result().value(0);

            assert_send(builder).await.unwrap_err();
        }

        #[tokio::test]
        async fn into_future_with_optional() {
            #[builder(derive(IntoFuture(Box)))]
            async fn optional_param(#[builder(default = 100)] value: u32) -> u32 {
                ready(value).await
            }

            // Test with value.
            let builder = optional_param().value(42);
            assert_eq!(assert_send(builder).await, 42);

            // Test without value (using default).
            let builder = optional_param();
            assert_eq!(assert_send(builder).await, 100);
        }

        #[tokio::test]
        async fn references_in_params() {
            struct Dummy;

            #[builder(derive(IntoFuture(Box)))]
            async fn sut<'named1, 'named2>(
                _x1: &Dummy,
                _x2: &Dummy,
                x3: &'named1 Dummy,
                x4: &'named2 Dummy,
            ) -> &'named2 Dummy {
                let _: &'named1 Dummy = x3;
                ready(x4).await
            }

            // Store the dummy struct in local variables to make sure no `'static`
            // lifetime promotion happens
            let local_x1 = Dummy;
            let local_x2 = Dummy;
            let local_x3 = Dummy;
            let local_x4 = Dummy;

            let builder = sut()
                .x1(&local_x1)
                .x2(&local_x2)
                .x3(&local_x3)
                .x4(&local_x4);

            let &Dummy = assert_send(builder).await;
        }

        #[tokio::test]
        async fn anon_lifetime_in_return_type() {
            struct Dummy;

            #[builder(derive(IntoFuture(Box)))]
            async fn sut(x1: &Dummy) -> &Dummy {
                ready(x1).await
            }

            // Store the dummy struct in local variables to make sure no `'static`
            // lifetime promotion happens
            let local_x1 = Dummy;

            let builder = sut().x1(&local_x1);

            let &Dummy = assert_send(builder).await;
        }

        #[tokio::test]
        async fn complex_generics() {
            struct Dummy;

            #[builder(derive(IntoFuture(Box)))]
            async fn sut<'a: 'b, 'b, T: Sync + 'a + 'b, U: Sync + 'a, const N: usize>(
                x1: &'a T,
                x2: &'b U,
            ) -> (&'a T, &'b U, usize) {
                async {}.await;
                (x1, x2, N)
            }

            // Store the dummy struct in local variables to make sure no `'static`
            // lifetime promotion happens
            let local_x1 = Dummy;
            let local_x2 = Dummy;

            let builder = sut::<_, _, 42>().x1(&local_x1).x2(&local_x2);

            let (&Dummy, &Dummy, usize) = assert_send(builder).await;
            assert_eq!(usize, 42);
        }
    }

    mod test_method {
        use super::*;

        #[tokio::test]
        async fn basic() {
            struct Calculator;

            #[bon]
            impl Calculator {
                #[builder]
                #[builder(derive(IntoFuture(Box)))]
                async fn multiply(a: u32, b: u32) -> u32 {
                    ready(a * b).await
                }
            }

            // Test using IntoFuture on impl method.
            let builder = Calculator::multiply().a(6).b(7);
            assert_eq!(assert_send(builder).await, 42);
        }

        #[tokio::test]
        async fn non_send() {
            struct Sut;

            #[bon]
            impl Sut {
                #[builder(derive(IntoFuture(Box, ?Send)))]
                #[expect(clippy::future_not_send)]
                async fn sut(self, value: u32) -> u32 {
                    non_send_future().await;

                    // This future can be !Send.
                    ready(value * 2).await
                }
            }

            // Test with non-Send future.
            let result = Sut.sut().value(21).await;
            assert_eq!(result, 42);
        }

        #[tokio::test]
        async fn references_in_params() {
            struct Dummy;

            #[bon]
            impl Dummy {
                #[builder(derive(IntoFuture(Box)))]
                async fn sut<'named1, 'named2>(
                    &self,
                    _x1: &Self,
                    _x2: &Self,
                    x3: &'named1 Self,
                    x4: &'named2 Self,
                ) -> &'named2 Self {
                    let _: &'named1 Self = x3;
                    ready(x4).await
                }
            }

            // Store the dummy struct in local variables to make sure no `'static`
            // lifetime promotion happens
            let local_self = Dummy;
            let local_x1 = Dummy;
            let local_x2 = Dummy;
            let local_x3 = Dummy;
            let local_x4 = Dummy;

            let builder = local_self
                .sut()
                .x1(&local_x1)
                .x2(&local_x2)
                .x3(&local_x3)
                .x4(&local_x4);

            let _: &Dummy = assert_send(builder).await;
        }

        #[tokio::test]
        async fn anon_lifetime_in_return_type() {
            struct Dummy;

            #[bon]
            impl Dummy {
                #[builder(derive(IntoFuture(Box)))]
                async fn sut(&self, _x1: &Self) -> &Self {
                    ready(self).await
                }
            }

            // Store the dummy struct in local variables to make sure no `'static`
            // lifetime promotion happens
            let local_self = Dummy;
            let local_x1 = Dummy;

            let builder = local_self.sut().x1(&local_x1);

            let _: &Dummy = assert_send(builder).await;
        }

        #[tokio::test]
        async fn complex_generics() {
            struct Dummy;

            #[bon]
            impl Dummy {
                #[builder(derive(IntoFuture(Box)))]
                async fn sut<'a: 'b, 'b, T: Sync + 'a + 'b, U: Sync + 'a, const N: usize>(
                    x1: &'a T,
                    x2: &'b U,
                ) -> (&'a T, &'b U, usize) {
                    async {}.await;
                    (x1, x2, N)
                }
            }

            // Store the dummy struct in local variables to make sure no `'static`
            // lifetime promotion happens
            let local_x1 = Dummy;
            let local_x2 = Dummy;

            let builder = Dummy::sut::<_, _, 42>().x1(&local_x1).x2(&local_x2);

            let (&Dummy, &Dummy, usize) = assert_send(builder).await;
            assert_eq!(usize, 42);
        }
    }
}
