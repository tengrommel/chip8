# Amethyst

# Presentation

Howdy! This book will teach you everything you need to know about building video games and interactive simulations with the Amethyst game engine. This engine is written entirely in Rust, a safe and fast systems programming language, and sports a clean and modern design. More correctly, though, Amethyst is actually a collection of separate libraries and tools that collectively make up a game engine.

Amethyst is free and open source software, distributed under a dual license of MIT and Apache. This means that the engine is given to you at no cost and its source code is completely yours to tinker with. The code is available on GitHub. Contributions and feature requests will always be welcomed!


# Getting started
>This book is split into several sections, with this introduction being the first. The others are:

- Getting Started – Prepare your computer for Amethyst development.
- Concepts – An overview of the concepts used in Amethyst. Recommended.
- Pong Tutorial – Build a basic pong game in Rust.
- Math – A quick introduction to doing math with Amethyst.
- Animation – Explains the architecture of the amethyst_animation crate.
- Controlling System Execution – Shows you how to structure more complex games that need to change the System graph.
- Glossary – Defines special terms used throughout the book.
- Appendix A: Config Files – Shows you how to define your data in RON files.

Read the crate-level API documentation for more details.

# Motivation

Most of us have worked with quite a few game engines over the years, namely Unity, Unreal Engine, JMonkeyEngine and many more. While they all are pretty solid solutions if you want to build a quality game, each have their own pros and cons that you have to weigh before using them, especially in regards to performance and scalability.

*We think that basing the Amethyst engine on good and modern principles will allow us to make an open source game engine that can actually be more performant than those engines.* Those principles are:

- 1.Modularity
>Modularity is at the core of the Unix philosophy, which proved itself to be an excellent way of developing software over the years. You will always be free to use the built-in modules, or to write your own and integrate them easily into the engine. Since modules are small and well integrated, it is easier to reason about what they do and how they relate to other modules.

- 2.Parallelism

>Modern computers, even cheap ones, all have multithreading with multicore CPUs. We expect that over the years, there will be more and more opportunities for parallelism to improve performance. With a proper parallel engine, we are convinced that your game will be more and more performant over the years without even needing you to update it.

- 3.Data-oriented/Data-driven
>Building your game around the data makes it really easy to prototype and quickly build a game. Complex behaviours like swapping assets during gameplay become a breeze, making testing and balancing a lot faster.

# Getting started

## Setting up Amethyst
>You can either use the Amethyst CLI or cargo to set up your project.

Amethyst CLI (Easiest)

If you wish to use the Amethyst cli tool, you can install it like so

    cargo install amethyst_tools
and then run

    amethyst new <game-name>
you should get Cargo.toml, src/main.rs and config/display.ron.


Cargo (Manual)
>In case you're doing this with cargo, here's what you need to do:

- Add amethyst as dependency in your Cargo.toml.
- Create a config folder and put a display.ron in it.
- (Optional) Copy the code from one of amethyst's examples.

Depending on the book version that you choose to read, make sure that the amethyst version in your Cargo.toml matches that.

For the released crates.io version, you should have something like this:

    [dependencies]
    amethyst = "LATEST_CRATES.IO_VERSION"

If you want to use the latest unreleased changes, your Cargo.toml file should look like this:

    [dependencies]
    amethyst = { git = "https://github.com/amethyst/amethyst", rev = "COMMIT_HASH" }


# Concepts behind Amethyst
>Amethyst uses quite a few concepts that you might not be familiar with. This section of the book explains what they are, how they work and how they relate to each other.

- If you are a practical person and want to quickly get into the code, you can skip to the pong tutorial section of the book, which is focused on practice. That said, reading this section is suggested, as it can be hard to understand the examples without knowing the theory presented here.

- If you don't understand how something works in amethyst, knowing the concepts presented here will help you understand how some implementations are made.

# State
>What is a state?

>The word "state" can mean a lot of different things in computer science. In the case of amethyst, it is used to represent the "game state".

A game state is a general and global section of the game.

## Example

As an example, let's say you are making a pong game.

- When the user opens up the game, it first loads all the assets and shows a loading screen.
- Then, the main menu shows up, asking you if you want to start a game in single or multiplayer.
- Once you select an option, the game displays the paddles and the ball and starts playing.
- By pressing escape, you can toggle the "pause" menu.
- Once the score limit is reached, a result screen is shown with a button to go back to the main menu.

The game can be divided into different states

- LoadingState
- MainMenuState
- PauseState
- ResultState