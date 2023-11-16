This is an experiment to generate a [`compile_commands.json`](https://clang.llvm.org/docs/JSONCompilationDatabase.html)
database from a Rust build script.

The resulting database is basically useless
except for feeding into [`clangd`](https://marketplace.visualstudio.com/items?itemName=llvm-vs-code-extensions.vscode-clangd)
for C++ language support in VS Code (or, I suppose, other editors with LSP
support), but that was my use case, so... mission accomplished. (Knowing C++
though, it'll probably crumble like the sad little house of cards it is as soon
as I try to use it in a real project.)
