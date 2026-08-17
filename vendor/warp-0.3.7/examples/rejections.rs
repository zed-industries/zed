#![deny(warnings)]

use std::convert::Infallible;
use std::error::Error;
use std::num::NonZeroU16;

use serde_derive::{Deserialize, Serialize};
use warp::http::StatusCode;
use warp::{reject, Filter, Rejection, Reply};

/// Rejections represent cases where a filter should not continue processing
/// the request, but a different filter *could* process it.
#[tokio::main]
async fn main() {
    let math = warp::path!("math" / u16);
    let div_with_header = math
        .and(warp::get())
        .and(div_by())
        .map(|num: u16, denom: NonZeroU16| {
            warp::reply::json(&Math {
                op: format!("{} / {}", num, denom),
                output: num / denom.get(),
            })
        });

    let div_with_body =
        math.and(warp::post())
            .and(warp::body::json())
            .map(|num: u16, body: DenomRequest| {
                warp::reply::json(&Math {
                    op: format!("{} / {}", num, body.denom),
                    output: num / body.denom.get(),
                })
            });

    let routes = div_with_header.or(div_with_body).recover(handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

/// Extract a denominator from a "div-by" header, or reject with DivideByZero.
fn div_by() -> impl Filter<Extract = (NonZeroU16,), Error = Rejection> + Copy {
    warp::header::<u16>("div-by").and_then(|n: u16| async move {
        if let Some(denom) = NonZeroU16::new(n) {
            Ok(denom)
        } else {
            Err(reject::custom(DivideByZero))
        }
    })
}

#[derive(Deserialize)]
struct DenomRequest {
    pub denom: NonZeroU16,
}

#[derive(Debug)]
struct DivideByZero;

impl reject::Reject for DivideByZero {}

// JSON replies

/// A successful math operation.
#[derive(Serialize)]
struct Math {
    op: String,
    output: u16,
}

/// An API error serializable to JSON.
#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

// This function receives a `Rejection` and tries to return a custom
// value, otherwise simply passes the rejection along.
async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(DivideByZero) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = "DIVIDE_BY_ZERO";
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        // This error happens if the body could not be deserialized correctly
        // We can use the cause to analyze the error and customize the error message
        message = match e.source() {
            Some(cause) => {
                if cause.to_string().contains("denom") {
                    "FIELD_ERROR: denom"
                } else {
                    "BAD_REQUEST"
                }
            }
            None => "BAD_REQUEST",
        };
        code = StatusCode::BAD_REQUEST;
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        // We can handle a specific error, here METHOD_NOT_ALLOWED,
        // and render it however we want
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
