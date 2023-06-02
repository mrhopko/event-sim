use std::{cmp::Ordering, collections::HashMap};

#[derive(Clone, Debug)]
pub enum EventType {
    EmptyEvent,
}

#[derive(Clone, Debug)]
pub struct Event {
    event_type: EventType,
    at: u128,
    data: Option<HashMap<String, Vec<u8>>>,
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.at).cmp(&self.at)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.at == other.at
    }
}

impl Eq for Event {}

impl Event {
    pub fn empty() -> Event {
        Event {
            event_type: EventType::EmptyEvent,
            at: 0,
            data: None,
        }
    }
}
