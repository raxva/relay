use raxva::error::Error;
use raxva::globals::GLOBALS;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Get args (config path)
    let mut args = env::args();
    if args.len() <= 1 {
        panic!("USAGE: chorus <config_path>");
    }
    let _ = args.next(); // ignore program name
    let config_path = args.next().unwrap();

    let config = raxva::load_config(&config_path)?;

    raxva::setup_logging(&config);

    // Log host name
    log::info!(target: "Server", "HOSTNAME = {}", config.hostname);

    raxva::setup_store(&config)?;

    let _ = GLOBALS.store.get().unwrap().sync();

    Ok(())
}
