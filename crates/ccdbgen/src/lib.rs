use std::{env, fs::File, io::Write};

use cc::Build;

pub trait GenerateCompileCommandsDb {
	fn generate_compile_commands(&self) -> anyhow::Result<()>;
}

impl GenerateCompileCommandsDb for Build {
	fn generate_compile_commands(&self) -> anyhow::Result<()> {
		let mut build = self.clone();

		build.compiler("ccdbgen");

		let cwd = env::current_dir()?;
		let ccdb_path = cwd.join("compile_commands.json");
		{
			let mut f = File::create(&ccdb_path)?;
			f.write_all(b"[\n")?;
		}

		// This will absolutely fail, because we're not actually compiling
		// anything, so we're not outputting any objects to link.
		let _ = build.try_compile("ccdb");

		{
			let mut f = File::options().append(true).open(ccdb_path)?;
			f.write_all(b"\n]\n")?;
		}

		Ok(())
	}
}
