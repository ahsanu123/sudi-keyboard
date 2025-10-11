use coretan::event_type::*;

fn main() {
    let events = vec![
        Event::BatteryEvent(Some(BatteryEventData { percent: 10 })),
        Event::KeyEvent(Some(KeyEventData { character: 'C' })),
        Event::BatteryEvent(Some(BatteryEventData { percent: 50 })),
        Event::KeyEvent(Some(KeyEventData { character: 'A' })),
        Event::HostEvent(Some(HostEventData { data: 'A' })),
    ];

    let event_handlers: [&dyn CorTrait<HandlingType = HandlerTypeEnum>; 2] =
        [&KeyEventHandler::default(), &BatteryEventHandler::default()];

    for event in events {
        let handler = event_handlers
            .iter()
            .find(|&&pr| pr.handling_type() == event.get_type());

        if handler.is_none() {
            panic!("cant find event handler for {event:#?}")
        }
    }
}
