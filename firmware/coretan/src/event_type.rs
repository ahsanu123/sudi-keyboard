use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq)]
pub enum HandlerTypeEnum {
    BatteryEvent,
    KeyEvent,
    HostEvent,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    BatteryEvent(Option<BatteryEventData>),
    KeyEvent(Option<KeyEventData>),
    HostEvent(Option<HostEventData>),
}

impl Event {
    pub fn get_type(&self) -> HandlerTypeEnum {
        match self {
            Event::BatteryEvent(_) => HandlerTypeEnum::BatteryEvent,
            Event::KeyEvent(_) => HandlerTypeEnum::KeyEvent,
            Event::HostEvent(_) => HandlerTypeEnum::HostEvent,
        }
    }
}

pub trait EventTrait {
    type DataType;
    type EventType;

    fn get_data(self) -> Self::DataType;
}

pub trait CorTrait: Debug {
    type HandlingType;

    fn handling_type(&self) -> Self::HandlingType;
    fn handle(&self, event: &Event);
}

#[derive(Debug, PartialEq, Eq)]
pub struct BatteryEventData {
    pub percent: i32,
}

impl EventTrait for BatteryEventData {
    type DataType = BatteryEventData;
    type EventType = Event;

    fn get_data(self) -> Self::DataType {
        self
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct KeyEventData {
    pub character: char,
}

impl EventTrait for KeyEventData {
    type DataType = KeyEventData;
    type EventType = Event;

    fn get_data(self) -> Self::DataType {
        self
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HostEventData {
    pub data: char,
}

#[derive(Default, Debug)]
pub struct BatteryEventHandler;

#[derive(Default, Debug)]
pub struct KeyEventHandler;

impl CorTrait for BatteryEventHandler {
    type HandlingType = HandlerTypeEnum;

    fn handle(&self, event: &Event) {
        if let Event::BatteryEvent(data) = event {
            println!("handling KeyEvent {data:#?}");
        }
    }

    fn handling_type(&self) -> Self::HandlingType {
        HandlerTypeEnum::BatteryEvent
    }
}

impl CorTrait for KeyEventHandler {
    type HandlingType = HandlerTypeEnum;

    fn handle(&self, event: &Event) {
        if let Event::KeyEvent(data) = event {
            println!("handling KeyEvent {data:#?}");
        }
    }

    fn handling_type(&self) -> Self::HandlingType {
        HandlerTypeEnum::KeyEvent
    }
}
