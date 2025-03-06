# RUST

Cargo is the rust Package manager
Cargo is also the Rust's build system

Git files won’t be generated if you run `cargo new` within an existing Git repository; you can override this behavior by using `cargo new --vcs=git.`

This file is in the `TOML (Tom’s Obvious, Minimal Language)` format, which is Cargo’s configuration format.

In Rust, packages of code are referred to as crates.

**IN rust it is important to understand that each package is a crate**

Cargo expects your source files to live inside the src directory.

The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code.

One easy way to get that `Cargo.toml` file is to run `cargo init`, which will create it for you automatically.

when `cargo build` is ran it creates a executable under the `target/debug/PackageName`

the default build is a debug build, Cargo puts the binary in a directory named debug. You can run the executable with this command:
`./target/debug/PackageName #`

Running `cargo build` for the `first time` also causes Cargo to create a new file at the top level: `Cargo.lock`. This file keeps track of the exact versions of dependencies in your project.

Cargo also provides a command called `cargo check`. This command quickly checks your code to make sure it compiles but doesn’t produce an executable.

When your project is finally ready for release, you can use `cargo build --release` to compile it with optimizations. This command will create an executable in `target/release` instead of target/debug. The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile. This is why there are two different profiles: one for development, when you want to rebuild quickly and often, and another for building the final program you’ll give to a user that won’t be rebuilt repeatedly and that will run as fast as possible. If you’re benchmarking your code’s running time, be sure to run cargo build --release and benchmark with the executable in target/release.

**Refer to the actual code file to learn more, only generic info will be put here**
