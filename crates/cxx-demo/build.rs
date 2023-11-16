use ccdbgen::GenerateCompileCommandsDb;

fn main() -> anyhow::Result<()> {
	let mut build = cxx_build::bridge("src/main.rs");
	build
		.file("src/blobstore.cc")
		.target("x86_64-pc-windows-msvc")
		.std("c++17");

	build.generate_compile_commands()?;
	build.try_compile("cxx-demo")?;

	Ok(())
}
