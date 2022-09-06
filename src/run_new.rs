use std::fs;
use std::path::{Path, PathBuf};

use crate::exec::{exec_cmd_args, exec_to_stdout, prompt};
use crate::prelude::*;
use crate::Error;
use aho_corasick::AhoCorasick;
use clap::ArgMatches;

const DEFAULT_APP_NAME: &str = "awesome-app";
const DEFAULT_WIN_TITLE: &str = "Awesome App";

const FILE_PACKAGE: &str = "package.json";
const FILE_TAURI_CONF: &str = "src-tauri/tauri.conf.json";
const FILE_VAPP: &str = "src-ui/src/views/v-app.ts";

const GIT_TMPL_BASE: &'static str = "https://github.com/jeremychone/rust-awesome-app-template-base.git";

struct Conf<'a> {
	app_name: &'a str,
	win_title: &'a str,
}

pub fn run_new(sub_cmd: &ArgMatches) -> Result<()> {
	check_git()?;

	println!();
	let app_name = prompt(&f!("What is your app name? ({DEFAULT_APP_NAME}): "), Some(DEFAULT_APP_NAME))?;
	let app_title = prompt(
		&f!("What should the window title be? ({DEFAULT_WIN_TITLE}): "),
		Some(DEFAULT_WIN_TITLE),
	)?;

	let app_dir = Path::new(&app_name);

	// check if the dir already exist
	if app_dir.exists() {
		return Err(Error::DirAlreadyExist(s!(app_dir.to_string_lossy())));
	}

	// do the git clone
	let res = exec_cmd_args(None, "git", &["clone", GIT_TMPL_BASE, &app_name], false)?;

	// replace the parts now
	replace_parts(
		app_dir,
		Conf {
			app_name: &app_name,
			win_title: &app_title,
		},
	)?;

	Ok(())
}

fn replace_parts(dir: &Path, conf: Conf) -> Result<()> {
	let file_package = path_joins(dir, FILE_PACKAGE);
	let file_conf = path_joins(dir, FILE_TAURI_CONF);
	let file_v_app = path_joins(dir, FILE_VAPP);

	let patterns = &[DEFAULT_APP_NAME, DEFAULT_WIN_TITLE];
	let ac = AhoCorasick::new(patterns);
	let replace_by = &[conf.app_name, conf.win_title];

	for file in [file_package, file_conf, file_v_app] {
		let content = fs::read_to_string(&file)?;
		let res = ac.replace_all_bytes(content.as_bytes(), replace_by);
		let new_content = std::str::from_utf8(&res).unwrap(); // TODO: Remove the unwwrap

		if content != new_content {
			println!("File updated: '{}'", file.to_string_lossy());
			fs::write(file, new_content)?;
		} else {
			println!("File skipped (nothing changed): '{}'", file.to_string_lossy());
		}
	}

	Ok(())
}

// region:    --- Utils

/// Will split the sub_path by '/' and push then to a new root PathBuf
fn path_joins(root: &Path, sub_path: &str) -> PathBuf {
	let parts = sub_path.split('/');
	let mut path = root.to_owned();
	for part in parts {
		path.push(part)
	}
	path
}

fn check_git() -> Result<()> {
	exec_to_stdout(None, "git", &["--version"], false).or_else(|ex| Err(Error::GitNotPresent))?;
	Ok(())
}
// endregion: --- Utils
