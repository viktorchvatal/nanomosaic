use gtk::*;
use gio::{ApplicationFlags, ApplicationExt, ApplicationExtManual};
use std::{env, sync::{mpsc}};
use log::*;
use panic::set_logging_panic_hook;
use logger::init_simple_logger;
use gui::build_ui;

mod panic;
mod logger;
mod message;
mod gui;
mod logic;

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

    let app = Application::new(APP_NAME, ApplicationFlags::NON_UNIQUE)
        .expect("Initialization failed...");

    app.connect_startup(move |app|
        build_ui(
            app,
        )
    );
    
    app.connect_activate(|_| {});
    app.run(&vec![]);
    
    Ok(())
}