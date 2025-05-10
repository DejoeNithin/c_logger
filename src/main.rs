use c_logger::init_logger;

fn main() {
    let _ = init_logger();
    // env_logger::init();

    log::debug!("Debugging!!!");
    log::warn!("Warning!!!");
    log::info!("Info!!!");
    log::error!("Error!!!");
    log::trace!("Tracing!!!");
}