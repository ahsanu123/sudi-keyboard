use coretan::state_machine::*;
use std::time::{Duration, Instant};

use crossterm::{
    event::{Event, KeyCode, poll, read},
    terminal::{disable_raw_mode, enable_raw_mode},
};

fn main() {
    let pba_fsm: Fsm<IdleState> = Fsm::new();

    enable_raw_mode().unwrap();
    loop {
        if poll(Duration::from_millis(10)).unwrap() {
            match read().unwrap() {
                Event::FocusGained => println!("FocusGained"),
                Event::FocusLost => println!("FocusLost"),
                Event::Key(event) => {
                    // println!("{:?}\n\n", event);

                    if event.code == KeyCode::Esc {
                        break;
                    }

                    if event.code == KeyCode::Char('a') {
                        pba_fsm.handle_event();
                    }
                }
                Event::Mouse(event) => println!("{:#?}", event),
                Event::Resize(width, height) => println!("New size {}x{}", width, height),
                Event::Paste(_) => println!("paste"),
            }
        }
    }

    disable_raw_mode().unwrap();
}
