use simplelog::{Config, SimpleLogger, CombinedLogger, LevelFilter};

pub fn init_simple_logger() {
    CombinedLogger::init(
        vec![SimpleLogger::new(LevelFilter::Debug, Config::default())]
    ).unwrap();
}
