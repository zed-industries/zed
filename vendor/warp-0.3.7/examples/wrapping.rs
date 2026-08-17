#![deny(warnings)]
use warp::Filter;

fn hello_wrapper<F, T>(
    filter: F,
) -> impl Filter<Extract = (&'static str,)> + Clone + Send + Sync + 'static
where
    F: Filter<Extract = (T,), Error = std::convert::Infallible> + Clone + Send + Sync + 'static,
    F::Extract: warp::Reply,
{
    warp::any()
        .map(|| {
            println!("before filter");
        })
        .untuple_one()
        .and(filter)
        .map(|_arg| "wrapped hello world")
}

#[tokio::main]
async fn main() {
    // Match any request and return hello world!
    let routes = warp::any()
        .map(|| "hello world")
        .boxed()
        .recover(|_err| async { Ok("recovered") })
        // wrap the filter with hello_wrapper
        .with(warp::wrap_fn(hello_wrapper));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
