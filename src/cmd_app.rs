use clap::{crate_version, Arg, Command};

pub fn cmd_app() -> Command<'static> {
	Command::new("awesome-app")
		.version(&crate_version!()[..])
		.about("Awesome Desktop App Scaffolder")
		.subcommand(sub_new())
}

fn sub_new() -> Command<'static> {
	Command::new("new").about("Build new tauri app from template base")
}
