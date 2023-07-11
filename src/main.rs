use safe_drive::{
    error::DynError, logger::Logger, pr_info
};
use std::env;

fn main() -> Result<(), DynError> {
    // Create a logger.
    let logger = Logger::new("get_launch_arg");

    let args: Vec<String> = env::args().collect();
    pr_info!(logger,"{:?}", args);

    Ok(())
}
