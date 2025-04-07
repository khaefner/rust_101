
# Chapter 1: 
## Welcome Aboard the Rust Starfleet Academy 
![logo](Line_Header_Star_Trek.png) 

Welcome, cadets! You've embarked on an exciting journey to master the Rust programming language, a powerful tool that will serve you well in the vast expanse of software development. Here at Starfleet Academy, we believe that even the most complex technologies can be learned with dedication and a bit of fun. Our mission for the next hour? To explore the fundamentals of Rust through the lens of the United Federation of Planets and its incredible starfaring technology.

Think of Rust as the core engineering principle behind our starships – reliable, efficient, and capable of handling the most demanding tasks, from powering warp drives to maintaining delicate life support systems. Just as Starfleet engineers meticulously maintain their vessels, you will learn to craft robust and dependable software with Rust.

This first chapter will lay the groundwork for your journey. We'll start by ensuring you have the necessary tools in your toolkit, much like a Starfleet engineer needs their trusty tricorder. We'll cover compiling your first Rust program and introduce you to Cargo, our standard Starfleet build and dependency management system.

## ![logo](Star_Trek_icon.png) Setting Up Your Starfleet Engineering Station  

Before we can engage our first program, we need to ensure your personal workstation (your computer) is equipped with the Rust toolchain. Think of this as setting up your dedicated engineering console.

1.  **Installing Rust:** The official and recommended way to install Rust is through `rustup`, a command-line tool for managing Rust versions and related tools. Open your terminal (on macOS and Linux) or command prompt (on Windows) and follow the instructions on the official Rust website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

    The installation process will likely involve downloading and running a script. Follow the prompts carefully. During the installation, you'll typically be asked about your preferred installation options. For beginners, the default options are usually the best.

2.  **Verifying the Installation:** Once the installation is complete, you can verify that Rust and its core components, including the Rust compiler (`rustc`) and Cargo, have been installed correctly. Open a new terminal or command prompt window and run the following commands:

    ```bash
    rustc --version
    cargo --version
    ```

    You should see output similar to this (the exact versions might differ):

    ```
    rustc 1.76.0 (07dca489a 2024-02-04)
    cargo 1.76.0 (1a4faecce 2024-02-04)
    ```

    If you see version numbers for both `rustc` and `cargo`, congratulations! Your Starfleet engineering station is ready for its first mission.

##  ![logo](Star_Trek_icon.png) Our First Program: A Simple Federation Greeting

Let's write a simple program to ensure everything is working as expected. We'll create a program that displays a classic Federation greeting.

1.  **Creating a Source File:** Open your favorite text editor or Integrated Development Environment (IDE). Create a new file named `hello_federation.rs`. The `.rs` extension is the standard for Rust source code files.

2.  **Writing the Code:** Inside the `hello_federation.rs` file, type the following code:

    ```rust,editable
    fn main() {
        println!("Greetings from the United Federation of Planets!");
    }
    ```

    Let's break down this simple program:

    * `fn main()`: This line declares a function named `main`. In Rust, the `main` function is the entry point for execution – the first code that runs when your program starts. Think of it as the bridge of your program.
    * `println!("Greetings from the United Federation of Planets!");`: This line calls a macro named `println!` (note the exclamation mark). Macros in Rust are like functions with superpowers. The `println!` macro takes a string literal (the text enclosed in double quotes) as input and prints it to the console. This is our message to the world (or at least, your terminal).

3.  **Saving the File:** Save the `hello_federation.rs` file in a location you can easily access through your terminal or command prompt.

## ![logo](Star_Trek_icon.png) Compiling with `rustc`: Engaging the Primary Systems

Now that we have our source code, we need to compile it into an executable program that your computer can understand. We'll use the Rust compiler, `rustc`, for this initial step.

1.  **Navigating to the Directory:** Open your terminal or command prompt and navigate to the directory where you saved the `hello_federation.rs` file using the `cd` command (change directory). For example, if you saved it in a folder named `rust_projects` on your desktop, you might use a command like:

    ```bash
    cd Desktop/rust_projects
    ```

    (The exact command will depend on your operating system and where you saved the file.)

2.  **Compiling the Code:** Once you are in the correct directory, run the following command:

    ```bash
    rustc hello_federation.rs
    ```

    If your code is correct, you won't see any output in the terminal. This means the compilation was successful. The `rustc` command has taken your `hello_federation.rs` file and generated an executable file.

3.  **Running the Executable:** The name of the executable file will vary depending on your operating system:

    * **Linux and macOS:** An executable file named `hello_federation` will be created in the same directory. You can run it by typing:

        ```bash
        ./hello_federation
        ```

    * **Windows:** An executable file named `hello_federation.exe` will be created. You can run it by typing:

        ```bash
        .\hello_federation.exe
        ```

    You should see the following output:

    ```
    Greetings from the United Federation of Planets!
    ```

    Congratulations, cadet! You have successfully written, compiled, and run your first Rust program. You've engaged the primary systems and received a clear signal!

## ![logo](Star_Trek_icon.png) Introducing Cargo: The Starfleet Standard Build System

While `rustc` is perfectly capable of compiling single-file Rust programs, as our projects become more complex (think of the intricate systems of a Galaxy-class starship), we'll need a more sophisticated way to manage our code, dependencies (external libraries), and build process. This is where Cargo comes in.

Cargo is Rust's build system and package manager. It handles many tasks for you, such as:

* Building your project.
* Downloading and managing dependencies (crates, as Rust packages are called).
* Running tests.
* Publishing your libraries.

Think of Cargo as the central command and control system for your Rust projects, ensuring everything is organized and runs smoothly.

## ![logo](Star_Trek_icon.png) Creating a New Cargo Project: Launching a New Mission

Let's create our first project using Cargo.

1.  **Navigating to Your Projects Directory:** Open your terminal or command prompt and navigate to a directory where you want to store your Rust projects.

2.  **Creating a New Project:** Run the following command:

    ```bash
    cargo new federation_mission
    ```

    This command tells Cargo to create a new project named `federation_mission`. Cargo will create a directory with this name and set up a basic project structure inside it.

3.  **Exploring the Project:** Navigate into the newly created `federation_mission` directory:

    ```bash
    cd federation_mission
    ```

    You will see the following files and directories:

    ```
    Cargo.toml
    src/
    ```

    * `Cargo.toml`: This is the manifest file for your project. It contains metadata about your project, such as its name, version, and dependencies. We'll explore this file in more detail later. Think of it as the mission briefing for your project.
    * `src/`: This directory contains your project's source code.

4.  **Examining the Source Code:** Navigate into the `src` directory:

    ```bash
    cd src
    ```

    You will find a file named `main.rs`. Open this file in your text editor. You'll see the familiar "Hello, world!" program:

    ```rust
    fn main() {
        println!("To boldy go where no one has gone before");
    }
    ```

    Cargo automatically generates this basic program for you when you create a new project.

## ![logo](Star_Trek_icon.png) Building with Cargo: Engaging Warp Drive

Now, let's use Cargo to build our project.

1.  **Navigate Back to the Project Root:** Make sure you are in the root directory of your project (`federation_mission`), where the `Cargo.toml` file is located.

2.  **Running the Build Command:** Execute the following command:

    ```bash
    cargo build
    ```

    Cargo will compile your project and create an executable file. You'll see output similar to this:

    ```
    Compiling federation_mission v0.1.0 (/path/to/federation_mission)
    Finished dev [unoptimized + debuginfo] target(s) in 0.xxs
    ```

    Cargo has created a new directory named `target` in your project's root directory. Inside the `target` directory, you'll find a `debug` subdirectory, and within that, the compiled executable file (named `federation_mission` on Linux/macOS or `federation_mission.exe` on Windows).

## ![logo](Star_Trek_icon.png) Running with Cargo: Executing the Mission

Cargo also provides a convenient command to build and run your project in a single step.

1.  **Still in the Project Root:** Ensure you are in the `federation_mission` directory.

2.  **Running the Run Command:** Execute the following command:

    ```bash
    cargo run
    ```

    Cargo will first compile your project (if it hasn't been already) and then run the resulting executable. You should see the following output:

    ```
    Compiling federation_mission v0.1.0 (/path/to/federation_mission)
    Finished dev [unoptimized + debuginfo] target(s) in 0.xxs
     Running `/path/to/federation_mission/target/debug/federation_mission`
    Hello, world!
    ```

    As you can see, Cargo handled both the compilation and execution, making the process much smoother.

## ![logo](Star_Trek_icon.png) The `Cargo.toml` File: The Mission Manifest

Let's take a closer look at the `Cargo.toml` file in your `federation_mission` project. Open this file in your text editor. You'll see something like this:

```toml
[package]
name = "federation_mission"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at [https://doc.rust-lang.org/cargo/reference/manifest.html](https://doc.rust-lang.org/cargo/reference/manifest.html)

[dependencies]



# Chapter 1  Core Concepts and Borrowing 

## The Main Function

All rust starts in the main function.  

```rust,editable
fn main(){
    println!("Hello WOrld");
}
```

---


