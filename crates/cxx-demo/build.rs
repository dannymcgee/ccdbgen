use std::{fs::File, io::Write};

const TOOL_PATH: &str = "D:/@experimental/cxx-rs-compile-commands/target/debug/clang.exe";
const CC_PATH: &str = "D:/@experimental/cxx-rs-compile-commands/.vscode/compile_commands.json";

fn main() -> anyhow::Result<()> {
	build_with(Some(TOOL_PATH))?;
	build_with(None)?;

	Ok(())
}

fn build_with(tool: Option<&str>) -> anyhow::Result<()> {
	let mut build = cxx_build::bridge("src/main.rs");

	build
		.file("src/blobstore.cc")
		.flag_if_supported("-std=c++17");

	if let Some(tool) = tool {
		let mut f = File::create(CC_PATH)?;
		f.write_all(b"[\n")?;
		drop(f);

		build.compiler(tool);
		let _ = build.try_compile("cxx-demo");
	} else {
		build.try_compile("cxx-demo")?;
	}

	if tool.is_some() {
		let mut f = File::options().append(true).open(CC_PATH)?;
		f.write_all(b"\n]\n")?;
	}

	Ok(())
}
