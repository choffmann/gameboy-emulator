use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Event {
    Write,
    Read,
}

#[derive(Clone)]
pub struct Subject {
    pub value: u8,
    pub address: u16,
    pub memory: [u8; 0xFFFF],
}

pub type Subscriber = fn(subject: Subject);

#[derive(Default, Clone)]
pub struct Publisher {
    events: HashMap<Event, Vec<Subscriber>>,
}

impl Publisher {
    pub fn subscribe(&mut self, event: Event, listener: Subscriber) {
        self.events.entry(event.clone()).or_default();
        self.events.get_mut(&event).unwrap().push(listener);
    }

    pub fn unsubscribe(&mut self, event: Event, listener: Subscriber) {
        self.events.get_mut(&event).unwrap().retain(|&x| x != listener);
    }

    pub fn notify(&self, event: Event, subject: Subject) {
        let listeners = self.events.get(&event);
        if let Some(listeners) = listeners {
            for listener in listeners {
                listener(subject.clone());
            }
        }
    }
}