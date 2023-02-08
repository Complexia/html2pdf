use std::env;

use clap::Parser;
use html2pdf::{run, Error, PdfOptions};
use log::{debug, warn};

fn main() -> Result<(), Error> {
    let env_log = env::var("RUST_LOG");
    if let Ok(level) = env_log {
        pretty_env_logger::init();
        debug!("RUST_LOG is {level}");
    } else {
        env::set_var("RUST_LOG", "info");
        pretty_env_logger::init();
        warn!("No RUST_LOG environment variable found, set log to 'info'")
    }

    let pdf_options = PdfOptions::parse();
    debug!("Options: {pdf_options:#?}");

    // Let's go
    run(&pdf_options, None)
}
