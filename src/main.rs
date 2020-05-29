use gtk::*;
use gio::{ApplicationFlags, ApplicationExt, ApplicationExtManual};
use std::{env, sync::{mpsc}};
use log::*;
use common::{set_logging_panic_hook, init_simple_logger, start_thread_loop, convert_err};
use gui::build_ui;
use message::{CompositeMessage, LogicMessage};
use logic::{LogicState};
use composite::CompositorState;

mod message;
mod gui;
mod logic;
mod composite;
mod common;

fn main() -> Result<(), String> {
    if env::args().len() != 2 {
        println!("USAGE:\nnanomosaic [image]");
        return Ok(());
    }

    init_simple_logger();
    set_logging_panic_hook();

    let archive_path = env::args().nth(1).unwrap();
    start_application(&archive_path)
}

const APP_NAME: &str = "nanomosaic.gtk";

fn start_application(file_name: &str) -> Result<(), String> {
    info!("Starting {}", APP_NAME);
    info!("Input image: {}", file_name);

    let queue_size = 3;

    let app = Application::new(APP_NAME, ApplicationFlags::NON_UNIQUE)
        .expect("Initialization failed...");

    let (logic_tx, logic_rx) = mpsc::sync_channel::<Option<LogicMessage>>(queue_size);
    let (composite_tx, composite_rx) = mpsc::sync_channel::<Option<CompositeMessage>>(queue_size);

    let state_thread = start_thread_loop(logic_rx, LogicState::new(composite_tx.clone()));
    let compositor_thread = start_thread_loop(composite_rx, CompositorState::new());

    let gui_logic_tx = logic_tx.clone();
    let gui_composite_tx = composite_tx.clone();
    let gui_file_name = file_name.to_owned();

    app.connect_startup(move |app|
        build_ui(
            app,
            gui_file_name.clone(),
            gui_logic_tx.clone(),
            gui_composite_tx.clone(),
        )
    );
    
    app.connect_activate(|_| {});
    app.run(&vec![]);

    convert_err(logic_tx.send(None))?;
    convert_err(state_thread.join())?;
    debug!("Logic thread finished");

    convert_err(composite_tx.send(None))?;
    convert_err(compositor_thread.join())?;
    debug!("Compositor thread finished");

    Ok(())
}