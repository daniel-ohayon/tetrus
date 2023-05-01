use std::{time::{SystemTime, Duration}, collections::HashMap};


// is there a more generic event handling system way we can
// represent this?
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Event {
    GravityDrop,
    UserMove,
    GameOver,
}

pub struct EventLog {
    event_timestamps: HashMap<Event, SystemTime>,
}

impl EventLog {
    pub fn new() -> Self {
        return EventLog {
            event_timestamps: HashMap::new(),
        };
    }

    pub fn did_happen(&self, event: Event) -> bool {
        return self.event_timestamps.contains_key(&event);
    }

    pub fn register_event(&mut self, event: Event) {
        self.event_timestamps.insert(event, SystemTime::now());
    }

    pub fn elapsed_since(&self, event: Event, delay: Duration) -> bool {
        if let Some(ts) = self.event_timestamps.get(&event) {
            return SystemTime::now().duration_since(*ts).unwrap() >= delay;
        } else {
            return true;
        }
    }
}
