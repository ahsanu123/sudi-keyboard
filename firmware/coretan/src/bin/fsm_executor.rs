use coretan::fsm_executor::{FsmExecutor, FsmExecutorTrait};
use std::time::Duration;

use crossterm::{
    event::{Event, KeyCode, poll, read},
    terminal::{disable_raw_mode, enable_raw_mode},
};

fn main() {
    let mut executor = FsmExecutor::default();

    enable_raw_mode().unwrap();
    loop {
        if poll(Duration::from_millis(10)).unwrap() {
            match read().unwrap() {
                Event::FocusGained => println!("FocusGained"),
                Event::FocusLost => println!("FocusLost"),
                Event::Key(event) => {
                    // println!("{:?}", event.code);

                    if event.code == KeyCode::Esc {
                        break;
                    }

                    executor.handle(Some(event.code));
                }
                Event::Mouse(event) => println!("{:#?}", event),
                Event::Resize(width, height) => println!("New size {}x{}", width, height),
                Event::Paste(_) => println!("paste"),
            }
        } else {
            executor.handle(None);
        }
    }

    disable_raw_mode().unwrap();
}
