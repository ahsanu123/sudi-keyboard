use std::{
    ops::Deref,
    time::{Duration, Instant},
};

#[derive(Clone, Copy)]
pub enum StateTypeEnum {
    Idle,
    OneClick,
    DoubleClick,
    Send,
    Hold,
}

pub trait StateChangerTrait {
    fn handle_state(&self, state: &State) -> Option<State>;
}

pub struct StateChanger {
    pub conditions: Vec<Box<dyn Fn(&State) -> Option<State>>>,
}

impl StateChangerTrait for StateChanger {
    fn handle_state(&self, state: &State) -> Option<State> {
        for condition in &self.conditions {
            let new_state = condition(state);

            if new_state.is_some() {
                return new_state;
            }
        }

        None
    }
}

impl StateChanger {
    fn new_to_idle() -> Self {
        Self {
            conditions: vec![Box::new(|state| None)],
        }
    }

    fn new_to_one_click() -> Self {
        Self {
            conditions: vec![Box::new(|state| None)],
        }
    }

    fn new_to_double_click() -> Self {
        Self {
            conditions: vec![Box::new(|state| None)],
        }
    }

    fn new_to_send() -> Self {
        Self {
            conditions: vec![Box::new(|state| None)],
        }
    }

    fn new_to_hold() -> Self {
        Self {
            conditions: vec![Box::new(|state| None)],
        }
    }
}

pub trait StateHandlerTrait {
    fn handle(&mut self);
}

pub struct State {
    pub state_type: StateTypeEnum,
    pub keys: Option<Vec<char>>,
    pub timestamp: Instant,
    pub state_changers: Vec<StateChanger>,
}

impl StateHandlerTrait for State {
    fn handle(&mut self) {
        let mut new_state: Option<State> = None;

        for state_changer in &self.state_changers {
            new_state = state_changer.handle_state(self);
            if new_state.is_some() {
                break;
            }
        }

        if new_state.is_some() {
            *self = new_state.unwrap();
        }
    }
}

impl State {
    pub fn create_idle() -> Self {
        Self {
            state_type: StateTypeEnum::Idle,
            keys: None,
            timestamp: Instant::now(),
            state_changers: vec![
                StateChanger::new_to_one_click(),
                StateChanger::new_to_hold(),
            ],
        }
    }

    pub fn create_one_click() -> Self {
        Self {
            state_type: StateTypeEnum::OneClick,
            keys: None,
            timestamp: Instant::now(),
            state_changers: vec![
                StateChanger::new_to_double_click(),
                StateChanger::new_to_send(),
            ],
        }
    }

    pub fn create_double_click() -> Self {
        Self {
            state_type: StateTypeEnum::DoubleClick,
            keys: None,
            timestamp: Instant::now(),
            state_changers: vec![StateChanger::new_to_send()],
        }
    }

    pub fn create_send() -> Self {
        Self {
            state_type: StateTypeEnum::Send,
            keys: None,
            timestamp: Instant::now(),
            state_changers: vec![StateChanger::new_to_hold(), StateChanger::new_to_idle()],
        }
    }

    pub fn create_hold() -> Self {
        Self {
            state_type: StateTypeEnum::Hold,
            keys: None,
            timestamp: Instant::now(),
            state_changers: vec![StateChanger::new_to_idle()],
        }
    }
}

pub struct Executor {
    pub current_state: State,
}

impl Executor {
    pub fn handle(&mut self, keys: Option<Vec<char>>) {
        self.current_state.keys = keys;
        self.current_state.handle();
    }
}

#[cfg(test)]
mod fsm_executorv2 {
    use super::*;

    #[test]
    fn test_idle() {
        let idle_state = State::create_idle();
    }
}
