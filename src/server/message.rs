use tokio::sync::oneshot;

use super::event::Event;

type Responder<Response> = oneshot::Sender<Response>;

#[derive(Debug)]
pub enum RequestType {
    Push,
    Next,
    Count,
}

#[derive(Debug)]
pub enum ResponseType {
    PushedEmpty,
    Pushed,
    Next,
    NextEmpty,
    Count,
    Error,
}

#[derive(Debug)]
pub struct Request {
    pub request_type: RequestType,
    pub message: String,
    pub event: Option<Event>,
    pub responder: Responder<Response>,
}

#[derive(Debug)]
pub struct Response {
    pub response_type: ResponseType,
    pub message: String,
    pub event: Option<Event>,
}

impl Response {
    pub fn empty(response_type: ResponseType) -> Self {
        Response {
            response_type,
            message: "".into(),
            event: None,
        }
    }
}
