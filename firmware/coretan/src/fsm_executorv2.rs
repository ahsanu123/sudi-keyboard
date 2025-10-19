use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq, Debug)]
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

impl std::fmt::Debug for StateChanger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StateChanger")
            .field("conditions_len", &self.conditions.len())
            .finish()
    }
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

// only change timestamp if state is changed
impl StateChanger {
    fn new_to_idle() -> Self {
        Self {
            conditions: vec![Box::new(|state| None)],
        }
    }

    fn new_to_one_click() -> Self {
        Self {
            conditions: vec![Box::new(|state| {
                match state.state_type {
                    StateTypeEnum::Idle => {
                        if state.keys.is_some() {
                            return Some(State::create_one_click(state.keys.clone()));
                        }
                    }
                    _ => {}
                }

                None
            })],
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

#[derive(Debug)]
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

    pub fn create_one_click(keys: Option<Vec<char>>) -> Self {
        Self {
            state_type: StateTypeEnum::OneClick,
            keys,
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

impl Default for Executor {
    fn default() -> Self {
        Self {
            current_state: State::create_idle(),
        }
    }
}

#[cfg(test)]
mod fsm_executorv2 {
    use tokio::time::sleep;

    use super::*;

    type EventOnTime = Box<dyn Fn(Duration) -> Option<Vec<char>>>;

    fn change_event(duration: Duration, events: &[EventOnTime]) -> Option<Vec<char>> {
        let mut current_condition: Option<Vec<char>> = None;
        for event in events {
            current_condition = event(duration);
            if current_condition.is_some() {
                break;
            }
        }

        current_condition
    }

    #[test]
    fn test_idle_to_one_click() {
        let keys = Some(vec!['c']);

        let mut executor = Executor::default();
        executor.handle(keys);

        let current_state = executor.current_state;

        println!("{current_state:#?}");
        assert!(matches!(current_state.state_type, StateTypeEnum::OneClick))
    }

    #[tokio::test]
    async fn test_loop_events() {
        let events_on_time: [EventOnTime; 1] = [Box::new(|duration| {
            if duration < Duration::from_millis(10) {
                return None;
            } else if duration > Duration::from_millis(10) && duration <= Duration::from_millis(15)
            {
                return Some(vec!['c', 'a']);
            } else if duration > Duration::from_millis(15) {
                return None;
            }
            None
        })];

        let mut executor = Executor::default();
        let mut current_keys: Option<Vec<char>> = None;
        let start_time = Instant::now();

        loop {
            executor.handle(current_keys.clone());

            sleep(Duration::from_millis(1)).await;

            let elapsed_time = start_time.elapsed();
            let current_state = &executor.current_state;
            println!("{elapsed_time:?} => {current_state:?}");

            current_keys = change_event(elapsed_time, &events_on_time);

            if elapsed_time > Duration::from_millis(20) {
                break;
            }
        }
    }
}
