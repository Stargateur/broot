#[macro_use] extern crate log;
#[macro_use] extern crate cli_log;

fn main() {
    init_cli_log!();
    match broot::cli::run() {
        Ok(Some(launchable)) => {
            debug!("launching {:#?}", launchable);
            if let Err(e) = launchable.execute(None) {
                warn!("Failed to launch {:?}", &launchable);
                warn!("Error: {:?}", e);
                eprintln!("{}", e);
            }
        }
        Ok(None) => {}
        Err(e) => {
            // this usually happens when the passed path isn't of a directory
            warn!("Error: {}", e);
            eprintln!("{}", e);
        }
    };
    info!("bye");
}
