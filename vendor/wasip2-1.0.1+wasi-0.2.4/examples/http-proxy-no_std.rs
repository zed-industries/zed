use wasip2::http::types::{
    Fields, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};

wasip2::http::proxy::export!(Example);

struct Example;

impl wasip2::exports::http::incoming_handler::Guest for Example {
    fn handle(_request: IncomingRequest, response_out: ResponseOutparam) {
        let resp = OutgoingResponse::new(Fields::new());
        let body = resp.body().unwrap();

        ResponseOutparam::set(response_out, Ok(resp));

        let out = body.write().unwrap();
        out.blocking_write_and_flush(b"Hello, WASI!").unwrap();
        drop(out);

        OutgoingBody::finish(body, None).unwrap();
    }
}
