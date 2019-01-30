pub type EventId = String;
pub type EventData = Vec<u8>;

pub struct Event {
    event_id: EventId,
    event_data: EventData
}

impl Event {
    
    pub fn build_new_event(event_data: EventData) -> Event {
        return Event { 
            event_id: String::new(), 
            event_data: event_data
        }
    }
}