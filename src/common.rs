use log::*;
use std::fmt::Debug;

pub fn convert_err<T, E: Debug>(input: Result<T, E>) -> Result<T, String> {
    match input {
        Ok(x) => Ok(x),
        Err(err) => Err(format!("{:?}", err))
    }
}

pub fn log_err<T: Debug>(result: Result<(), T>) {
    if let Err(err) = result {
        warn!("Runtime error: {:?}", err);
    }
}