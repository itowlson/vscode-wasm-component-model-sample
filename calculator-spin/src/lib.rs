#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::http::incoming_handler::{Guest, IncomingRequest, ResponseOutparam};
use bindings::wasi::http::types::{Fields, OutgoingBody, OutgoingResponse};

struct Component;

impl Guest for Component {
    fn handle(_request: IncomingRequest, response_out: ResponseOutparam) {
        let eng = bindings::vscode::example::types::Engine::new();
        eng.push_operand(1);
        eng.push_operand(2);
        eng.push_operation(bindings::vscode::example::types::Operation::Add);
        let result = eng.execute().to_string();

        let response = OutgoingResponse::new(Fields::new());
        let body = response.body().unwrap();
        ResponseOutparam::set(response_out, Ok(response));

        let resp_stm = body.write().unwrap();
        resp_stm.write(result.as_bytes()).unwrap();
        resp_stm.flush().unwrap();
        OutgoingBody::finish(body, None).unwrap();
    }
}

bindings::export!(Component with_types_in bindings);
