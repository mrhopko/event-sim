use std::collections::BinaryHeap;

use super::event::Event;
use super::message::Request;
use super::message::RequestType;
use super::message::Response;
use super::message::ResponseType;

pub struct EventBus {
    pub heap: BinaryHeap<Event>,
    buffer: Vec<Event>,
}

impl EventBus {
    pub fn new() -> Self {
        EventBus {
            heap: BinaryHeap::new(),
            buffer: Vec::new(),
        }
    }

    pub fn request(&mut self, request: &Request) -> Response {
        match request.request_type {
            RequestType::Push => self.push(request),
            RequestType::Next => self.next(),
            RequestType::Count => self.count(),
        }
    }

    /// Get the next message from the bus
    pub fn next(&mut self) -> Response {
        if let Some(event) = self.heap.pop() {
            self.buffer.push(event.clone());
            return Response {
                response_type: ResponseType::Next,
                message: "next".to_string(),
                event: Some(event),
            };
        }

        Response {
            response_type: ResponseType::NextEmpty,
            message: "next_empty".to_string(),
            event: None,
        }
    }

    /// push a message to the bus
    pub fn push(&mut self, request: &Request) -> Response {
        if let Some(event) = request.event.clone() {
            self.heap.push(event);
            return Response {
                response_type: ResponseType::Pushed,
                message: "pushed".to_string(),
                event: None,
            };
        }
        Response {
            response_type: ResponseType::PushedEmpty,
            message: "pushed_empty".to_string(),
            event: None,
        }
    }

    pub fn count(&mut self) -> Response {
        let count = self.heap.len();
        Response {
            response_type: ResponseType::Count,
            message: count.to_string(),
            event: None,
        }
    }
}
