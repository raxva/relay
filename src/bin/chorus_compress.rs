use raxva::error::Error;
use std::env;
use std::io::BufRead;
fn main() -> Result<(), Error> {
    // Get args (config path)
    let mut args = env::args();
    if args.len() <= 1 {
        panic!("USAGE: chorus_compress <config_path>");
    }
    let _ = args.next(); // ignore program name
    let config_path = args.next().unwrap();

    let config = raxva::load_config(config_path)?;

    raxva::setup_logging(&config);

    println!("Chorus must NOT be running when you do this.");
    println!("Proceed? (break out with ^C, or press <ENTER> to proceed)");
    let stdin = std::io::stdin();
    let _ = stdin.lock().lines().next().unwrap().unwrap();

    let store = raxva::setup_store_and_return(&config)?;

    let pre_stats = store.stats()?;
    println!("{:?}", pre_stats);

    let new_store = unsafe { store.rebuild()? };

    let post_stats = new_store.stats()?;
    println!("{:?}", post_stats);

    Ok(())
}
