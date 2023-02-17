mod args;
mod fmt;
mod logger;

use crate::args::Args;
use crate::fmt::write;
use crate::logger::setup_logging;
use clap::Parser;
use std::process;
use termcolor::{Color, StandardStream};

fn main() {
    let args: Args = Args::parse();
    setup_logging(args.verbosity_level);
    log::debug!("Config: {:?}", args);
    log::info!("This is a log message");

    let mut stdout = StandardStream::stdout(args.use_colors());
    write(&mut stdout, "Hello, world!\n", Some(Color::Red));
}
