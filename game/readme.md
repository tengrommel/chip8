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

# State Manager

Amethyst has a built-in state manager, which allows easily switching between different States. It is based on the concept of a pushdown-automaton, which is a combination of a Stack and a State Machine.

# State 
> The Stack concept makes it so you can "push" States on top of each other.

If we take the pong example of earlier, you can push the PauseState over the GameplayState.

When you want to go out of pause, you pop the PauseState out of the stack and you are back into the GameplayState, just as you left it.

# State Machine
> The concept of State Machine can be pretty complex, but here we will only explain the basics of it. The State Machine is usually composed of two elements: Transitions and Events.

Transitions are simply the "switching" between two states.

For example, from LoadingState, go to state MainMenuState.

Amethyst has multiple types of transitions

- You can Push a State over another
   
- You can also Switch a State, which replaces the current State with a new one

**Events are what trigger the transitions. In the case of amethyst, it is the different methods called on the State. Continue reading to learn about them.**

# Life Cycle
> States are only valid for a certain period of time, during which a lot of things can occur. A State contains methods methods methods that reflect the most common of those events:

- on_start: When a State is added to the stack, this method is called on it.
- on_stop: When a State is removed from the stack, this method is called on it.
- on_pause: When a State is pushed over the current one, the current one is paused, and this method is called on it.
- on_resume: When the State that was pushed over the current State is popped, the current one resumes, and this method is called on the now-current State.
- handle_event: Allows easily handling events, like the window closing or a key being pressed.
- fixed_update: This method is called on the active State at a fixed time interval (1/60th second by default).
- update: This method is called on the active State as often as possible by the engine
- shadow_update: This method is called as often as possible by the engine on all States which are on the StateMachines stack, including the active State. Unlike update, this does not return a Trans.
- shadow_fixed_update: This method is called at a fixed time interval (1/60th second by default) on all States which are on the StateMachines stack, including the active State. Unlike fixed_update, this does not return a Trans.

If you aren't using SimpleState or EmptyState, you must implement the update method to call data.data.update(&mut data.world).

# Game Data

States can have arbitrary data associated with them. If you need to store data that is tightly coupled to your State, the classic way is to put it in the State's struct.

States also have internal data, which is any type T. In most cases, the two following are the most used: () and GameData.

() means that there is no data associated with this State. This is usually used for tests and not for actual games. GameData is the de-facto standard. It is a struct containing a Dispatcher. This will be discussed later.

When calling your State's methods, the engine will pass a StateData struct which contains both the World (which will also be discussed later) and the Game Data type that you chose

# Code 

Yes! It's finally time to get some code in here!

Here will just be a small code snippet that shows the basics of State's usage. For more advanced examples, see the following pong tutorial.
