use std::sync::mpsc::{Receiver};
use std::{thread, thread::JoinHandle};
use log::*;

use crate::message::{LogicMessage};
use super::state::State;

// ================================= PUBLIC ==================================

pub fn start_logic_thread(
    logic_rx: Receiver<Option<LogicMessage>>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut state = match State::new() {
            Ok(state) => state,
            Err(msg) => {
                error!("Could not create app state: {}", msg);
                return;
            }
        };

        loop {
            match logic_rx.recv() {
                Ok(message) => match message {
                    Some(msg) => match state.receive(msg) {
                        Err(msg) => error!(
                            "State: could not process message: {}", msg
                        ),
                        _ => {}                        
                    },
                    None => return,

                },
                Err(_) => {
                    // Channel closed, exit
                    return;
                },
            }
        }
    })
}
