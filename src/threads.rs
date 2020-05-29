use std::sync::mpsc::{Receiver};
use std::{thread, thread::JoinHandle};
use log::*;
use crate::message::MessageReceiver;

pub fn start_thread_loop<'a, TS, TM>(receiver: Receiver<Option<TM>>, state: TS) -> JoinHandle<()>
where TM: Send + 'static, TS: MessageReceiver<TM> + Send + 'static {
    thread::spawn(move || {
        let mut state: TS = state;

        loop {
            match receiver.recv() {
                Ok(message) => match message {
                    Some(msg) => match state.receive(msg) {
                        // Processing error, log details and continue
                        Err(msg) => error!(
                            "Could not process message: {}", msg
                        ),
                        _ => {}                        
                    },
                    // Empty message sent, exit
                    None => return,

                },
                Err(_) => {
                    // Channel has been already closed, exit
                    return;
                },
            }
        }
    })
}