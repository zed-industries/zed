#![deny(warnings)]
use std::net::SocketAddr;

use warp::hyper::StatusCode;
use warp::{hyper::Method, reject, Filter, Rejection, Reply};

#[derive(Debug)]
struct MethodError;
impl reject::Reject for MethodError {}

const FOO_METHOD: &'static str = "FOO";
const BAR_METHOD: &'static str = "BAR";

fn method(name: &'static str) -> impl Filter<Extract = (), Error = Rejection> + Clone {
    warp::method()
        .and_then(move |m: Method| async move {
            if m == name {
                Ok(())
            } else {
                Err(reject::custom(MethodError))
            }
        })
        .untuple_one()
}

pub async fn handle_not_found(reject: Rejection) -> Result<impl Reply, Rejection> {
    if reject.is_not_found() {
        Ok(StatusCode::NOT_FOUND)
    } else {
        Err(reject)
    }
}

pub async fn handle_custom(reject: Rejection) -> Result<impl Reply, Rejection> {
    if reject.find::<MethodError>().is_some() {
        Ok(StatusCode::METHOD_NOT_ALLOWED)
    } else {
        Err(reject)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address: SocketAddr = "[::]:3030".parse()?;

    let foo_route = method(FOO_METHOD)
        .and(warp::path!("foo"))
        .map(|| "Success")
        .recover(handle_not_found);

    let bar_route = method(BAR_METHOD)
        .and(warp::path!("bar"))
        .map(|| "Success")
        .recover(handle_not_found);

    warp::serve(foo_route.or(bar_route).recover(handle_custom))
        .run(address)
        .await;

    Ok(())
}
