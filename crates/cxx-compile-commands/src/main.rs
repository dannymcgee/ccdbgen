use std::{
	fmt::Write as _,
	fs::{self, File},
	io::Write as _,
	path::Path,
};

use anyhow::anyhow;
use serde_json::json;

const CC_PATH: &str = "D:/@experimental/cxx-rs-compile-commands/.vscode/compile_commands.json";

// D:/@experimental/cxx-rs-compile-commands/target/debug/cxx-compile-commands.exe
fn main() -> anyhow::Result<()> {
	let path = Path::new(CC_PATH);
	let content = fs::read_to_string(path)?;

	let mut buf = String::new();
	let mut out = File::options().append(true).open(path)?;

	if content.ends_with('}') {
		buf.push_str(",\n");
	}

	let mut args = std::env::args().collect::<Vec<_>>();
	if let Some(arg) = args.get_mut(0) {
		if arg.ends_with("clang.exe") {
			*arg = "C:/Program Files/LLVM/bin/clang++.exe".into();
		}
	}

	let cwd = std::env::current_dir()?;
	let cwd = cwd
		.to_str()
		.ok_or_else(|| anyhow!("Failed to convert cwd to string!"))?;

	let file = args.iter().last().unwrap();

	let entry = json! {{
		"directory": cwd,
		"arguments": args,
		"file": file,
	}};

	write!(buf, "{}", serde_json::to_string_pretty(&entry)?)?;
	write!(out, "{buf}")?;

	// let mut buf = String::new();

	// for arg in std::env::args() {
	// 	write!(buf, "{arg} ")?;
	// }
	// writeln!(buf)?;
	// writeln!(out, "{buf}")?;

	Ok(())
}
