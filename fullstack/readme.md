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

# Trade-offs

Two of the big selling points of Rust are performance and reliability. Performance
meaning both runtime speed and memory consumption. Reliability here means
catching bugs at compile time and preventing certain classes of errors entirely
through language design. These goals are often seen as classically at odds with one
another. For example, C lives in a world where performance is of utmost importance,
but reliability is left as an exercise for the implementor.

Rust has no garbage collector and no runtime in the traditional sense. However, most
difficulties of working with manual memory management are taken care of for you
by the compiler. Therefore, you will often hear “zero cost” being used to describe
certain features or abstractions in the language and standard library. This is meant
to imply that neither performance nor reliability has to suffer to achieve a particular
goal. You write high level code and the compiler turns it into the same thing as the
“best” low level implementation.

However, in practice, what Rust really gives you is the tools to make choices about
what trade-offs you want to make. Underlying the design and construction of all
software is a series of trade-offs made knowingly or implicitly. Rust pushes more of
these trade-offs to the surface which can make the initial learning period seem a bit
more daunting especially if you have experience in languages where many trade-offs
are implicit.

This will be a topic that permeates this book, but for now we will highlight some of
these aspects as we make our numbers crate do something more interesting.

**Rust has a few different modes of passing arguments to functions.** The biggest
distinction being that Rust differentiates between:
- a function temporarily having access to a variable (borrowing) and
- having ownership of a variable.