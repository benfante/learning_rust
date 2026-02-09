# Learning Rust

Exercises during my learning of the [Rust language](https://www.rust-lang.org/)

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
