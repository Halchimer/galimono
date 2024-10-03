use beryllium::*;
use std::ops::Deref;

pub struct EventHandler(Vec<fn(&Event) -> ()>);

impl EventHandler {
    pub fn new(&self) -> Option<EventHandler> {
        Some(EventHandler(Vec::new()))
    }

    pub fn run_eventfn(&self) {
        for function in self.0.iter() {}
    }
}

impl Deref for EventHandler {
    type Target = Vec<fn(&Event) -> ()>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
