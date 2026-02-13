# Learning Rust

Exercises during my learning of the [Rust language](https://www.rust-lang.org/)

## Resources

### Books

#### [Programming Rust, 3rd Edition (Early Release)](https://www.oreilly.com/library/view/programming-rust-3rd/9781098176228/) by Jim Blandy, Jason Orendorff, Leonora F. S. Tindall

[![Programming Rust, 3rd Edition](https://www.oreilly.com/covers/urn:orm:book:9781098176228/300w/)](https://www.oreilly.com/library/view/programming-rust-3rd/9781098176228/)

You can find the examples I try from this book in the [examples/programming_rust_3rd](https://github.com/benfante/learning_rust/tree/main/examples/programming_rust_3rd) directory. It's not a pedissequous copy of the examples, but I try to follow along with the book and write my own code based on the concepts and examples presented in the book, usually expanding on them with my own ideas and variations, for better understanding and practice.

The official repositories for the book is available on GitHub at [ProgrammingRust](https://github.com/ProgrammingRust).

## Installing Rust

While you can try Rust online using the [Rust Playground](https://play.rust-lang.org/), it's recommended to install Rust on your local machine for a better learning experience.

To install Rust, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install). The recommended way is to use `rustup`, the Rust toolchain installer.

Briefly, you can install Rust by running the following command in your terminal:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, you can verify that Rust is installed correctly by checking the version:

```sh
rustc --version
```

This should display the installed version of Rust.

## Setting Up the development Environment

To set up your development environment for Rust, you can use any text editor or IDE that supports Rust. Some popular choices include:

- Visual Studio Code with the Rust extension
- IntelliJ IDEA with the Rust plugin
- Sublime Text with Rust packages
- Vim or Neovim with Rust plugins

I recommend using Visual Studio Code with the Rust extension for a good balance of features and ease of use.

The recommended Rust extension for Visual Studio Code is called "rust-analyzer". You can install it from the Extensions marketplace in Visual Studio Code.

You can follow [these instructions](<https://code.visualstudio.com/docs/languages/rust>) to set up Visual Studio Code for Rust development.

## Beginning with Cargo

Cargo is Rust's package manager and build system. It helps you manage your Rust projects, dependencies, and builds.

### Creating a New Rust Project

To create a new Rust project using Cargo, you can run the following command in your terminal:

```sh
cargo new hello_world
```

This will create a new directory called `hello_world` with the following structure:

```hello_world
hello_world/
├── Cargo.toml
└── src
    └── main.rs
```

### Running the Rust Project

To run the Rust project, navigate to the project directory and use the following command:

```sh
cd hello_world
cargo run
```

This will compile and run the Rust program, and you should see the output:

```terminal
Hello, world!
```

### Building the Rust Project

To build the Rust project without running it, you can use the following command:

```sh
cargo build
```

This will compile the project and create an executable in the `target/debug` directory.

If you want to build the project in release mode, which optimizes the code for performance (and size too), you can use the  release profile with the following command:

```sh
cargo build --release
```

### Cleaning the Build Artifacts

To clean the build artifacts, you can use the following command:

```sh
cargo clean
```

This will remove the `target` directory and all the compiled files.

## Dependency Management with Cargo

Cargo allows you to manage dependencies for your Rust projects. You can specify your dependencies in the `Cargo.toml` file.

To add a dependency to your project, you can use the following command:

```sh
cargo add <dependency_name>
```

For example, to add the `actix-web` crate as a dependency, you can run:

```sh
cargo add actix-web
```

This will add the `actix-web` crate to your `Cargo.toml` file under the `[dependencies]` section:

```toml
[dependencies]
actix-web = "4.12.1"
```

You can also specify a specific version or a range of versions for the dependency. For example:

```sh
cargo add actix-web@4.0.0
```

Sometimes the crates you want to use may have optional features that you can enable. You can specify these features when adding the dependency. For example, for the `sarde` crate, you can enable the `derive` feature like this:

```sh
cargo add sarde --features derive
```

This will add the `sarde` crate to your `Cargo.toml` file with the specified feature:

```toml
[dependencies]
sarde = { version = "0.1.0", features = ["derive"] }
```

You can also add multiple features at once by separating them with commas:

```sh
cargo add sarde --features "derive,other_feature"
```

The dependencies (crates, the name for Rust packages, they can be libraries and executables) you add to your project will be downloaded and compiled when you build or run your project. Cargo will handle the dependency resolution and ensure that the correct versions of the crates are used.

The crates can be found on [crates.io](https://crates.io/), which is the Rust community's crate registry. You can search for crates, read their documentation, and find out how to use them in your projects.
