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

# Wrapping up

The Rust language is designed to allow fine grained control over performance and reliability while writing code at a high level of abstraction. 

The cost is thus learning the abstractions and dealing with the cognitive load of making choices as you design a program. These considerations are important for production quality code as you profile and optimize where you find bottlenecks. However, following the standards of the community will get you the majority of the way to high quality applications.

We will not belabor all of the nuance in the finer points of making every decision along the way.Our goal is to provide a solid foundation upon which you can explore alternatives as necessary by using the standard library documentation.

# Making A Web App With Actix

# Web Ecosystem
> One area where Rust stands out is in the building of web servers.

Rust has its origins at Mozilla primarily as a tool for building a new browser engine.

The existing engine being written in C++ combined with the syntactical similarities encourages the idea the Rust was meant to be a replacement for C++. There is obviously some truth to this, but in many ways this characterization sells Rust’s potential short. While it is capable of being a systems programming language, there are a plethora of language features that make it suitable for innumerable programming tasks, including building web servers.

There are a few different layers to the web programming stack. Primarily we are concerned here with the application layer which is comparable to where Django, Rails, and Express live in Python, Ruby, and NodeJS, respectively

The ecosystem around web application development in Rust is still quite nascent despite the Rust language hitting 1.0 in 2015. Much of the underlying infrastructure for building concurrent programs took until 2019 to reach a maturity sufficient to be included in the stable version of the standard library.

*However, most of the ecosystem has coalesced around similar ideas which take advantage of Rust's particular features.*

# Actix
> The Actix project is actually a group of projects which define an actor system as well as a framework for building web applications. The web framework is aptly named actix-web.

It has been built on top of futures and async primitives from the beginning. It also runs on the stable version of the compiler.

