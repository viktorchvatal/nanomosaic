use ::log::error;
use std::{thread, panic::{PanicInfo, set_hook}, any::Any};
use backtrace::Backtrace;

pub fn set_logging_panic_hook() {
    set_hook(Box::new(|info| log_panic(info)));
}

fn log_panic(info: &PanicInfo<'_>) {
    let thread = thread::current();
    let thread_name = thread.name().unwrap_or("unnamed").to_owned();
    let file = info.location().map(|loc| loc.file());
    let line = info.location().map(|loc| loc.line());

    error!(
        "Thread '{}' panicked at '{}': {}:{}{:?}",
        thread_name, 
        any_to_string(info.payload()), 
        file.unwrap_or("?"), 
        line.unwrap_or(0u32), 
        Backtrace::new()
    );
}

fn any_to_string(value: &dyn Any) -> String {
    format!("{}",
        match value.downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match value.downcast_ref::<String>() {
                Some(s) => &**s,
                None => "No string representation",
            }
        }
    )
}