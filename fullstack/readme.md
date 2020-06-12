# Making Your First Rust App

# Getting started
> We are going to build an application in Rust to get a feel for the language and ecosystem. The first step for all new Rust projects is generating a new project. Let's create a new project called numbers:
    
    cargo new numbers

Cargo is the package manager for Rust.It is used as a command line tool to manage dependencies, compile your code, and make packages for distribution. Running cargo new project_name by default is equivalent to cargo new project_name --bin which generates a binary project. Alternatively, we could have run cargo new project_name --lib to generates a library project

# Binary vs. library

- A binary project is one which compiles into an executable file. For binary projects,
you can execute cargo run at the root of your application to compile and run the
executable.

- A library project is one which compiles into an artifact which is shareable and can be used as a dependency in other projects. Running cargo run in a library project will produce an error as cargo cannot figure out what executable you want it to run(because one does not exist). Instead, you would run cargo build the library.

There are different formats which the Rust compiler can generate based on your
configuration settings depending on how you wish to use your library.

The default is to generate an rlib which is a format for use in other Rust projects. This allows your library to have a reduced size for further distribution to other Rust projects while still being able to rely on the standard library and maintain enough information to allow the Rust compiler to type check and link to your code.

Alternative library formats exist for more specialized purposes. For example, the
cdylib format is useful for when you want to produce a dynamic library which can
be linked with C code. This produces a .so, .dylib, or .dll depending on the target
architecture you build for.