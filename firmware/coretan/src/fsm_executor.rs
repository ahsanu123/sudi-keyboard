use crossterm::event::KeyCode;
use std::time::Duration;
use std::time::Instant;

use crate::fsm_executor::idle_state::IdleState;

pub trait FsmExecutorTrait {
    fn handle(&mut self, key: Option<KeyCode>);
}

pub const MAX_STATE_LEN: usize = 10;

#[derive(Clone, Copy, Debug)]
pub enum StateTypeEnum {
    // Idle(idle_state::IdleState),
    Idle,
    OneClick,
    DoubleClick,
    // CapitalUntil,
    SendData,
}

#[derive(Debug, Clone, Copy)]
pub enum ActiveStateTypeEnum {
    Idle(Option<idle_state::IdleState>),
    OneClick(Option<one_click_state::OneClickState>),
    DoubleClick(Option<double_click_state::DoubleClickState>),
    SendData(Option<char>),
}

pub trait StateTrait {
    fn get_last_state(self) -> ActiveStateTypeEnum;
    fn get_key(self) -> Option<KeyCode>;
    fn trigger_enter(
        self,
        key: Option<KeyCode>,
        last_state: &ActiveStateTypeEnum,
        time: Instant,
    ) -> ActiveStateTypeEnum;
    fn trigger_leave(
        self,
        key: Option<KeyCode>,
        last_state: &ActiveStateTypeEnum,
        time: Instant,
    ) -> ActiveStateTypeEnum;
    fn get_last_time_stamp(self) -> Instant;
}

#[derive(Clone, Copy, Debug)]
pub struct State {
    pub from: [Option<StateTypeEnum>; MAX_STATE_LEN],
    pub to: [Option<StateTypeEnum>; MAX_STATE_LEN],
}

pub mod idle_state {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub struct IdleState {
        pub base: State,
        pub time_stamp: Instant,
        pub key: Option<KeyCode>,
    }

    impl StateTrait for IdleState {
        fn get_last_state(self) -> ActiveStateTypeEnum {
            todo!()
        }

        fn trigger_enter(
            self,
            key: Option<KeyCode>,
            last_state: &ActiveStateTypeEnum,
            time: Instant,
        ) -> ActiveStateTypeEnum {
            if key.is_some() && matches!(last_state, ActiveStateTypeEnum::Idle(_)) {
                println!("inside trigger_enter => {key:#?}");
                return ActiveStateTypeEnum::OneClick(Some(one_click_state::OneClickState {
                    time_stamp: time,
                    key,
                    ..Default::default()
                }));
            }
            ActiveStateTypeEnum::Idle(Some(IdleState::default()))
        }

        fn trigger_leave(
            self,
            key: Option<KeyCode>,
            last_state: &ActiveStateTypeEnum,
            time: Instant,
        ) -> ActiveStateTypeEnum {
            todo!()
        }

        fn get_last_time_stamp(self) -> Instant {
            self.time_stamp
        }

        fn get_key(self) -> Option<KeyCode> {
            todo!()
        }
    }

    impl Default for IdleState {
        fn default() -> Self {
            let mut from: [Option<StateTypeEnum>; MAX_STATE_LEN] = [None; MAX_STATE_LEN];
            let mut to: [Option<StateTypeEnum>; MAX_STATE_LEN] = [None; MAX_STATE_LEN];

            from[0] = Some(StateTypeEnum::OneClick);
            from[1] = Some(StateTypeEnum::DoubleClick);
            from[2] = Some(StateTypeEnum::SendData);

            to[0] = Some(StateTypeEnum::OneClick);
            to[1] = Some(StateTypeEnum::DoubleClick);

            Self {
                base: State { from, to },
                time_stamp: Instant::now(),
                key: None,
            }
        }
    }
}

pub mod one_click_state {

    use super::*;

    #[derive(Clone, Copy, Debug)]
    pub struct OneClickState {
        pub base: State,
        pub time_stamp: Instant,
        pub key: Option<KeyCode>,
    }

    impl StateTrait for OneClickState {
        fn trigger_enter(
            self,
            key: Option<KeyCode>,
            last_state: &ActiveStateTypeEnum,
            time: Instant,
        ) -> ActiveStateTypeEnum {
            // to send
            let duration = self.get_last_time_stamp().elapsed();

            if matches!(last_state, ActiveStateTypeEnum::OneClick(_))
                && duration > Duration::from_secs(1)
                && key.is_some()
            {
                let char = key.unwrap().as_char().unwrap();

                return ActiveStateTypeEnum::SendData(Some(char));
            }

            ActiveStateTypeEnum::Idle(None)
        }

        fn trigger_leave(
            self,
            key: Option<KeyCode>,
            last_state: &ActiveStateTypeEnum,
            time: Instant,
        ) -> ActiveStateTypeEnum {
            todo!()
        }

        fn get_last_state(self) -> ActiveStateTypeEnum {
            todo!()
        }

        fn get_last_time_stamp(self) -> Instant {
            self.time_stamp
        }

        fn get_key(self) -> Option<KeyCode> {
            todo!()
        }
    }

    impl Default for OneClickState {
        fn default() -> Self {
            let mut from: [Option<StateTypeEnum>; MAX_STATE_LEN] = [None; MAX_STATE_LEN];
            let mut to: [Option<StateTypeEnum>; MAX_STATE_LEN] = [None; MAX_STATE_LEN];

            from[0] = Some(StateTypeEnum::OneClick);
            from[1] = Some(StateTypeEnum::Idle);

            to[0] = Some(StateTypeEnum::OneClick);
            to[1] = Some(StateTypeEnum::DoubleClick);
            to[1] = Some(StateTypeEnum::SendData);

            Self {
                base: State { from, to },
                time_stamp: Instant::now(),
                key: None,
            }
        }
    }
}

pub mod double_click_state {
    use super::*;

    #[derive(Clone, Copy, Debug)]
    pub struct DoubleClickState {
        pub base: State,
        pub time_stamp: Instant,
        pub key: Option<KeyCode>,
    }

    impl StateTrait for DoubleClickState {
        fn trigger_enter(
            self,
            key: Option<KeyCode>,
            last_state: &ActiveStateTypeEnum,
            time: Instant,
        ) -> ActiveStateTypeEnum {
            todo!()
        }

        fn trigger_leave(
            self,
            key: Option<KeyCode>,
            last_state: &ActiveStateTypeEnum,
            time: Instant,
        ) -> ActiveStateTypeEnum {
            todo!()
        }

        fn get_last_state(self) -> ActiveStateTypeEnum {
            todo!()
        }

        fn get_last_time_stamp(self) -> Instant {
            self.time_stamp
        }

        fn get_key(self) -> Option<KeyCode> {
            todo!()
        }
    }

    impl Default for DoubleClickState {
        fn default() -> Self {
            let mut from: [Option<StateTypeEnum>; MAX_STATE_LEN] = [None; MAX_STATE_LEN];
            let mut to: [Option<StateTypeEnum>; MAX_STATE_LEN] = [None; MAX_STATE_LEN];

            from[0] = Some(StateTypeEnum::OneClick);

            to[1] = Some(StateTypeEnum::SendData);

            Self {
                base: State { from, to },
                time_stamp: Instant::now(),
                key: None,
            }
        }
    }
}

pub struct FsmExecutor {
    last_state: ActiveStateTypeEnum,
    active_state: ActiveStateTypeEnum,
}

impl FsmExecutorTrait for FsmExecutor {
    fn handle(&mut self, key: Option<KeyCode>) {
        match &self.active_state {
            ActiveStateTypeEnum::Idle(idle_state) => {
                if idle_state.is_some() {
                    self.last_state = self.active_state;
                    self.active_state =
                        idle_state
                            .unwrap()
                            .trigger_enter(key, &self.last_state, Instant::now());

                    let is_one_click =
                        matches!(self.active_state, ActiveStateTypeEnum::OneClick(_));

                    println!("{is_one_click:#?}");
                }
            }
            ActiveStateTypeEnum::OneClick(one_click_state) => {
                if one_click_state.is_some() {
                    self.last_state = self.active_state;
                    self.active_state = one_click_state.unwrap().trigger_enter(
                        key,
                        &self.last_state,
                        Instant::now(),
                    );
                }
            }
            ActiveStateTypeEnum::DoubleClick(double_click_state) => {
                if double_click_state.is_some() {
                    self.last_state = self.active_state;
                    self.active_state = double_click_state.unwrap().trigger_enter(
                        key,
                        &self.last_state,
                        Instant::now(),
                    );
                }
            }
            ActiveStateTypeEnum::SendData(sended_char) => {
                println!("{sended_char:#?}");

                self.last_state = self.active_state;
                self.active_state = ActiveStateTypeEnum::Idle(Some(IdleState::default()));
            }
        }
    }
}

impl Default for FsmExecutor {
    fn default() -> Self {
        Self {
            active_state: ActiveStateTypeEnum::Idle(Some(idle_state::IdleState::default())),
            last_state: ActiveStateTypeEnum::Idle(Some(idle_state::IdleState::default())),
        }
    }
}
