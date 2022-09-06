#![allow(unused)]

use crate::prelude::*;
use clap::ArgMatches;
use cmd_app::cmd_app;
use exec::exec_to_stdout;
use run_new::run_new;

mod cmd_app;
mod error;
mod exec;
mod prelude;
mod run_new;
mod utils;

fn main() {
	match cmd_run() {
		Ok(_) => (),
		Err(err) => println!("FAIL - {err}"),
	}
}

fn cmd_run() -> Result<()> {
	let app = cmd_app().get_matches();

	match app.subcommand() {
		Some(("new", sub_cmd)) => run_new(sub_cmd)?,
		_ => {
			// needs cmd_app version as the orginal got consumed by get_matches
			cmd_app().print_long_help()?;
			println!("\n");
		}
	}

	Ok(())
}
