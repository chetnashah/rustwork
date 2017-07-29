For beginning, following : https://doc.rust-lang.org/book/getting-started.html

### Tools

rustup : the toolchain manager like virtualenv for python,
and nvm for node.

cargo : the package manager like npm.

rustfmt : formats rust code, like prettier.

racer : the autocomplete engine

This is the project directory is where Cargo.toml lives,
like package.json for js.

for stuff like
READMEs, license info, and anything not related to code
like config files, project info files

Code should always live in src/ placed
in the project root directory.

Configuration file: Cargo.toml (yes, capital C)

Build command : cargo build
Doing a build will create a Cargo.lock file
Cargo uses Cargo.lock file to track dependencies in
your application

Run command : cargo run

Project template generation command (like npm init):
(But it will automatically make it a git repo also)

1. To make binary executable application:
   cargo new project_name --bin

2. To make library
   cargo new project_name
   

### Learning Tutorial

http://aml3.github.io/RustTutorial/html/toc.html

Full of examples: https://rustbyexample.com/hello/comment.html
