use core::marker::PhantomData;
use std::time::Instant;

pub enum FsmEvent {
    PBA,
}

pub trait StateTrait {
    fn whoami(&self);
    fn static_whoami();
    fn handle_event(&self);
    fn new() -> Self;
}

pub struct IdleState;
pub struct PressedState;
pub struct DoubleClickState;
pub struct OneClickAndHoldState;
pub struct SendState;

pub struct Fsm<TState> {
    time_stamp: u128,
    data: Option<char>,
    _state: PhantomData<TState>,
}

mod idle {

    use super::*;

    impl StateTrait for Fsm<IdleState> {
        fn whoami(&self) {
            println!("I Am State A");
        }

        fn static_whoami() {
            println!("I Am Static State A");
        }

        fn new() -> Self {
            Self {
                _state: PhantomData,
                time_stamp: Instant::now().elapsed().as_millis(),
                data: None,
            }
        }

        fn handle_event(&self) {
            let now = Instant::now().elapsed().as_millis();

            let duraton = now - self.time_stamp;

            if duraton > 1000 {
                self.to_pressed_state(now);
            }
        }
    }

    impl Fsm<IdleState> {
        pub fn to_pressed_state(&self, now: u128) -> Fsm<PressedState> {
            Fsm::<PressedState> {
                _state: PhantomData,
                time_stamp: now,
                data: None,
            }
        }
    }
}

mod pressed {
    use super::*;

    impl StateTrait for Fsm<PressedState> {
        fn whoami(&self) {
            println!("I Am State B");
        }

        fn static_whoami() {
            println!("I Am Static State B");
        }

        fn new() -> Self {
            Self {
                _state: PhantomData,
                time_stamp: Instant::now().elapsed().as_millis(),
                data: None,
            }
        }

        fn handle_event(&self) {
            todo!()
        }
    }

    impl Fsm<PressedState> {
        pub fn to_idle_state(&self, now: u128) -> Fsm<IdleState> {
            Fsm::<IdleState> {
                _state: PhantomData,
                time_stamp: now,
                data: None,
            }
        }

        pub fn to_pressed_state(&self, now: u128) -> Fsm<PressedState> {
            Fsm::<PressedState> {
                _state: PhantomData,
                time_stamp: now,
                data: None,
            }
        }

        pub fn to_double_click_state(&self, now: u128) -> Fsm<DoubleClickState> {
            Fsm::<DoubleClickState> {
                _state: PhantomData,
                time_stamp: now,
                data: None,
            }
        }

        pub fn to_send_state(&self, now: u128) -> Fsm<SendState> {
            Fsm::<SendState> {
                _state: PhantomData,
                time_stamp: now,
                data: None,
            }
        }
    }
}

mod double_click {
    use super::*;

    impl StateTrait for Fsm<DoubleClickState> {
        fn whoami(&self) {
            println!("I Am State DoubleClickState");
        }

        fn static_whoami() {
            println!("I Am Static State DoubleClickState");
        }

        fn new() -> Self {
            Self {
                _state: PhantomData,
                time_stamp: 0,
                data: None,
            }
        }

        fn handle_event(&self) {
            todo!()
        }
    }

    impl Fsm<DoubleClickState> {
        pub fn to_idle_state(&self, now: u128) -> Fsm<IdleState> {
            Fsm::<IdleState> {
                _state: PhantomData,
                time_stamp: now,
                data: None,
            }
        }

        pub fn to_send_state(&self, now: u128) -> Fsm<SendState> {
            Fsm::<SendState> {
                _state: PhantomData,
                time_stamp: now,
                data: None,
            }
        }
    }
}

mod one_click_and_hold {
    use super::*;

    impl StateTrait for Fsm<OneClickAndHoldState> {
        fn whoami(&self) {
            println!("I Am State OneClickAndHoldState");
        }

        fn static_whoami() {
            println!("I Am Static State OneClickAndHoldState");
        }

        fn new() -> Self {
            Self {
                _state: PhantomData,
                time_stamp: 0,
                data: None,
            }
        }

        fn handle_event(&self) {
            todo!()
        }
    }
}

mod send {
    use super::*;

    impl StateTrait for Fsm<SendState> {
        fn whoami(&self) {
            println!("I Am State SendState");
        }

        fn static_whoami() {
            println!("I Am Static State SendState");
        }

        fn new() -> Self {
            Self {
                _state: PhantomData,
                time_stamp: 0,
                data: None,
            }
        }

        fn handle_event(&self) {
            todo!()
        }
    }

    impl Fsm<SendState> {
        pub fn to_idle_state(&self, now: u128) -> Fsm<IdleState> {
            Fsm::<IdleState> {
                _state: PhantomData,
                time_stamp: now,
                data: None,
            }
        }
    }
}
