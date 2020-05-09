# Getting started

## Motivation
> We think that basing the Amethyst engine on good and modern principle will allow us to make an open source game engine that can actually be more performant than those engines.

- 1、Modularity
> Modularity is at the core of the Unix philosophy, which proved itself to be an excellent way of developing software over the years.

- 2、Parallelism
> Modern computers, even cheap ones, all have multithreading with multicore CPUs. We expect that over the years, there will be more and more opportunities for parallelism to improve performance.

With a proper parallel engine, we are convinced that your game will be more and more performant over the years without even needing you to update it.

- 3、Data-oriented/Data-driven
> Building your game around the data makes it really easy to prototype and quickly build a game. Complex behaviours like swapping assets during gameplay become a breeze, making testing and balancing a lot faster

# Concepts

## State
> The word "state" can mean a lot of different things in computer science.

A game state is a general and global section of the game.

- 1 LoadingState
- 2 MainMenuState
- 3 GameplayState
- 4 PauseState
- 5 ResultState

while you could effectively insert all the game's logic into a single state GameState, dividing it into multiple parts makes it much easier to reason about and maintain.

## State Manager
> Amethyst has a built-in state manager, which allows easily switching between different States

- Stack 
> The stack concept makes it so you can "push" States on top of each other.

- State Machine
> The concept of State Machine can be pretty complex, but here we will only explain the basics of it. The State Machine is usually composed of two elements: Transitions and Events.

Amethyst has multiple types of transitions

- 1 You can Push a State over another
- 2 You can also Switch a State, which replaces the current State with a new one

Events are what trigger the transitions.

## Life Cycle
> States are only valid for a certain period of time, during which a lot of things can occur. A State contains methods that reflect the most common of those events

- on_start
> When a State is added to the stack, this method is called on it.

- on_stop 
> When a State is removed from the stack, this method is called on it.

- on_pause
> When a State is pushed over the current one, the current one is paused, and this method is called on it.

- on_resume
> When the State that was pushed over the current State is popped, the current one resumes, and this method is called on the now-current State

- handle_event
> Allows easily handling events, like the window closing or a key being pressed.

- fixed_update
> This method is called on the active State at a fixed time interval (1/60th second by default)

- update 
> This method is called on the active State as often as possible by the engine.

- shadow_update
> This method is called as often as possible by the engine on all States which are on the StateMachines Stack, including the active State. Unlike update, this does not return a Trans.

- shadow_fixed_update
> This method is called at a fixed time interval(1/60th second by default) on all States which are on the StateMachines stack, including the active State. Unlike fixed_updates, this does not return a Trans. 

## Game Data
> States can have arbitrary data associated with them. If you need to store data that is tightly coupled to your State, the classic way is to put it in the State's Struct.

## Event Handling
> It regroups multiple types of events that are emitted throughout the engine by default

## Entity and Component

- An Entity represents a single object in your world.
- Component represents one aspect of an object

## Storages
> There are a few storage strategies for different usage scenarios
- DenseVecStorage
> Elements are stored in a contiguous vector
- VecStorage
> Elements are stored into a sparse array
- FlaggedStorage
> Used to keep track of changes of a component

## Resource
> A resource is any type that stores data that you might need for your game AND that is not specific to an entity.

# Math

# Animation

# Controlling System Execution

# Glossary

# Appendix A: Config Files