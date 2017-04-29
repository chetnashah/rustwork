For beginning, following : https://doc.rust-lang.org/book/getting-started.html


This is the project directory,

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
   


