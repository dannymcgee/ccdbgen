use std::{
	env,
	fmt::Write as _,
	fs::{self, File},
	io::Write as _,
};

use serde_json::json;

fn main() -> anyhow::Result<()> {
	let cwd = env::current_dir()?;
	let ccdb_path = cwd.join("compile_commands.json");

	let content = fs::read_to_string(&ccdb_path)?;

	let mut buf = String::new();
	let mut out = File::options().append(true).open(&ccdb_path)?;

	if content.ends_with('}') {
		buf.push_str(",\n");
	}

	let args = env::args().collect::<Vec<_>>();
	let file = args.iter().last().unwrap();
	let entry = json! {{
		"directory": cwd,
		"arguments": &args[1..],
		"file": file,
	}};

	write!(buf, "{}", serde_json::to_string_pretty(&entry)?)?;
	write!(out, "{buf}")?;

	Ok(())
}
