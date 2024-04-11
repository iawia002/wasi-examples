#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::http::incoming_handler::Guest;
use bindings::wasi::http::types::{
    FieldKey, FieldValue, Fields, IncomingRequest, OutgoingBody, OutgoingResponse,
    ResponseOutparam, StatusCode,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

struct Component;

bindings::export!(Component with_types_in bindings);

#[derive(Serialize, Deserialize)]
struct Data {
    path: String,
    query: HashMap<String, String>,
}

impl Guest for Component {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        let path = request.path_with_query().unwrap_or_default();
        let url = Url::parse(format!("http://127.0.0.1{}", path).as_str()).unwrap();

        let path = url.path();
        let query_pairs = url.query_pairs();
        let query_map: HashMap<String, String> = query_pairs.into_owned().collect();

        let data = Data {
            path: path.to_string(),
            query: query_map,
        };
        match serde_json::to_vec(&data) {
            Ok(s) => write_json_response(response, s.as_slice()),
            Err(err) => write_error_response(
                response,
                400,
                format!("serialize data to JSON failed: {err}\n",).as_bytes(),
            ),
        }
    }
}

fn write_json_response(response: ResponseOutparam, data: &[u8]) {
    let headers = Fields::from_list(&[(
        FieldKey::from("Content-Type"),
        FieldValue::from("application/json"),
    )])
    .expect("outgoing headers");
    let resp = OutgoingResponse::new(headers);
    let body = resp.body().expect("outgoing response");
    ResponseOutparam::set(response, Ok(resp));

    let out = body.write().expect("outgoing stream");
    out.blocking_write_and_flush(data)
        .expect("writing response");
    drop(out);
    OutgoingBody::finish(body, None).unwrap();
}

fn write_error_response(response: ResponseOutparam, status_code: StatusCode, error_message: &[u8]) {
    let headers = Fields::from_list(&[(
        FieldKey::from("Content-Type"),
        FieldValue::from("text/plain"),
    )])
    .expect("outgoing headers");
    let resp = OutgoingResponse::new(headers);
    resp.set_status_code(status_code).expect("");
    let body = resp.body().expect("outgoing response");
    ResponseOutparam::set(response, Ok(resp));

    let out = body.write().expect("outgoing stream");
    out.blocking_write_and_flush(error_message)
        .expect("writing response");
    drop(out);
    OutgoingBody::finish(body, None).unwrap();
}
