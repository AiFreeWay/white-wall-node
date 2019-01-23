type EventId = u8;
type EventData = [u8];

struct Event {
    event_id: EventType,
    event_data: EventData,
}

impl Event {
    
    pub fn build_new_event(event_data: EventData) -> Event {
        Event { 0, event_data }
    }
}