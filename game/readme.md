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

    extern crate amethyst;
    use amethyst::prelude::*;
    
    struct GameplayState {
        /// The `State`-local data. Usually you will not have anything.
        /// In this case, we have the number of players here.
        player_count: u8,
    }
    
    impl SimpleState for GameplayState {
        fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
            println!("Number of players: {}", self.player_count);
        }
    }

That's a lot of code, indeed!

We first declare the State's struct GameplayState.

In this case, we give it some data: player_count, a byte.

Then, we implement the SimpleState trait for our GameplayState. SimpleState is a shorthand for State<GameData<'static, 'static>, ()> where GameData is the internal shared data between states.

# Switching State
> Now, if we want to change to a second state, how do we do it?

Well, we'll need to use one of the methods that return the Trans type.

Those are:
- handle_event
- fixed_update
- update

Let's use handle_event to go to the PausedState and come back by pressing the "Escape" key.

    extern crate amethyst;
    use amethyst::prelude::*;
    use amethyst::input::{VirtualKeyCode, is_key_down};
    
    struct GameplayState;
    struct PausedState;
    
    // This time around, we are using () instead of GameData, because we don't have any `System`s that need to be updated.
    // (They are covered in the dedicated section of the book.)
    // Instead of writing `State<(), StateEvent>`, we can instead use `EmptyState`.
    impl EmptyState for GameplayState {
        fn handle_event(&mut self, _data: StateData<()>, event: StateEvent) -> EmptyTrans {
            if let StateEvent::Window(event) = &event {
                if is_key_down(&event, VirtualKeyCode::Escape) {
                    // Pause the game by going to the `PausedState`.
                    return Trans::Push(Box::new(PausedState));
                }
            }
    
            // Escape isn't pressed, so we stay in this `State`.
            Trans::None
        }
    }
    
    impl EmptyState for PausedState {
        fn handle_event(&mut self, _data: StateData<()>, event: StateEvent) -> EmptyTrans {
            if let StateEvent::Window(event) = &event {
                if is_key_down(&event, VirtualKeyCode::Escape) {
                    // Go back to the `GameplayState`.
                    return Trans::Pop;
                }
            }
    
            // Escape isn't pressed, so we stay in this `State`.
            Trans::None
        }
    }

# Event Handling
> As you already saw, we can handle events from the handle_event method. But what is this weired StateEvent all about?

Well, it is simply an enum. It regroups multiple types of events that are emitted throughout the engine by default. To change the set of events that the state receives, you create a new event enum and derive EventReader for that type.

    #[macro_use] extern crate amethyst;
    use amethyst::prelude::*;
    use amethyst::ui::UiEvent;
    use amethyst::input::{VirtualKeyCode, is_key_down};
    use amethyst::winit::Event;
    
    // These imports are required for the #[derive(EventReader)] code to build
    use amethyst::core::{
        ecs::{Read, SystemData, World},
        shrev::{ReaderId, EventChannel},
        EventReader
    };
    
    #[derive(Clone, Debug)]
    pub struct AppEvent {
        data: i32,
    }
    
    #[derive(Debug, EventReader, Clone)]
    #[reader(MyEventReader)]
    pub enum MyEvent {
        Window(Event),
        Ui(UiEvent),
        App(AppEvent),
    }
    
    struct GameplayState;
    
    impl State<(), MyEvent> for GameplayState {
        fn handle_event(&mut self, _data: StateData<()>, event: MyEvent) -> Trans<(), MyEvent> {
            match event {
                MyEvent::Window(_) => {}, // Events related to the window and inputs.
                MyEvent::Ui(_) => {}, // Ui event. Button presses, mouse hover, etc...
                MyEvent::App(ev) => println!("Got an app event: {:?}", ev),
            };
    
            Trans::None
        }
    }
    
    fn main() {}

To make Application aware of the change to which events to send to the state, you also need to supply both the event type, and the EventReader type (the name you give in the #[reader(SomeReader)] derive attribute) when the Application is created.
 
 
 **This is done by replacing Application::build (or Application::new) with CoreApplication::<_, MyEvent, MyEventReader>::build() (or CoreApplication::<_, MyEvent, MyEventReader>::new()).**
 
# Entity and Component
>What are Entity and Component?

An Entity represents a single object in your world. Component represents one aspect of an object. For example, a bottle of water has a shape, a volume, a color and is made of a material (usually plastic). In this example, the bottle is the entity, and the properties are components.

# Entity and Component in Amethyst
>In an inheritance design, entity usually contains components. All the data and methods related to an entity are stored within. However, in the ECS design, entity is just a general purpose object. In fact, the implementation of Entity in Amethyst is simply:

    struct Entity(u32, Generation);

where u32 is the id of the entity and generation is used to check if the entity has been deleted.

*Entity s are stored in a special container EntitiesRes. Whereas the data associated with the entities are grouped into components and stored in the designated storages.*

Consider an example where you have three objects: two bottles and a person.

    object	    x	    y	    shape     color	  name
    Bottle A	150.0	202.1	"round"	  "red"	
    Bottle B	570.0	122.0	"square"  "white"	
    Person C	100.5	300.8			          "Peter"

We can separate bottle's properties into PositionComponent and BottleComponent, and person's properties into PositionComponent and PersonComponent. Here's an illustration of how the three objects would be stored.

**entities do not store data. Nor do they know any information about their components. 
They serve the purpose of object identification and tracking object existence. The component storage stores all the data and their connection to entities.**

# EntitiesRes
> Even though the structure of the entity is pretty simple, entity manipulation is very sophisticated and crucial to game performance. This is why entities are handled exclusively by the struct EntitiesRes. EntitiesRes provides two ways for creating/deleting entities:

- Immediate creation/deletion, used for game setup or clean up
- Lazy creation/deletion, used in the game play state. It updates entities in batch at the end of each game loop. This is also referred to as atomic creation/deletion

You will see how these methods are used in later chapters.

# Declaring a component
> To declare a component, you first declare the relevant underlying data:

    extern crate amethyst;
    use amethyst::core::math::{Isometry3, Vector3};
    
    /// This `Component` describes the shape of an `Entity`
    enum Shape {
        Sphere { radius: f32 },
        Cuboid { height: f32, width: f32, depth: f32 },
    }
    
    /// This `Component` describes the transform of an `Entity`
    pub struct Transform {
        /// Translation + rotation value
        iso: Isometry3<f32>,
        /// Scale vector
        scale: Vector3<f32>,
    }


and then you implement the Component trait for them:

    use amethyst::ecs::{Component, DenseVecStorage, FlaggedStorage};
    
    impl Component for Shape {
        type Storage = DenseVecStorage<Self>;
    }
    
    impl Component for Transform {
        type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
    }
    
The storage type will determine how you store the component, but it will not initialize the storage. Storage is initialized when you register a component in World or when you use that component in a System.

# Storages
> There are a few storage strategies for different usage scenarios. The most commonly used types are DenseVecStorage, VecStorage and FlaggedStorage.

- *DenseVecStorage*: Elements are stored in a contiguous vector. No empty space is left between Component S, allowing a lowered memory usage for big components.
- *VecStorage*: Elements are stored into a sparse array. The entity id is the same as the index of component. If your component is small (<= 16 bytes) or is carried by most entities, this is preferable over DenseVecStorage.
- *FlaggedStorage*: Used to keep track of changes of a component. Useful for caching purposes.


    DenseVecStorage ( entity_id maps to data_id )
    data        data	data	data	data	...
    data_id     0	    2	    3	    1	    ...
    entity_id   0       1       5       9       ...

    VecStorage ( entity_id = data index, can be empty )
    data   data	data	empty	data	...

For more information, see the specs storage reference and the "Storages" section of the specs book.

There are a bunch more storages, and deciding which one is the best isn't trivial and should be done based on careful benchmarking. A general rule is: if your component is used in over 30% of entities, use VecStorage. If you don't know which one you should use, DenseVecStorage is a good default. It will need more memory than VecStorage for pointer-sized components, but it will perform well for most scenarios.

# Tags
> Components can also be used to "tag" entities. The usual way to do it is to create an empty struct, and implement Component using NullStorage as the Storage type for it. Null storage means that it is not going to take memory space to store those components.

# Resource
> A resource is any type that stores data that you might need for your game AND that is not specific to an entity. 

For example, the score of a pong game is global to the whole game and isn't owned by any of the entities (paddle, ball and even the ui score text).

# Creating a resource
> Resource are stored in the World container.

Adding a resource to a World instance is done like this:

    extern crate amethyst;
    use amethyst::ecs::World;
    
    struct MyResource {
        pub game_score: i32,
    }
    
    fn main() {
        let mut world = World::empty();
        
        let my = MyResource {
            game_score: 0,
        };
        
        world.insert(my);
    }

# Fetching a resource(from World)
> Fetching a resource can be done like this:

    // try_fetch returns a Option<Fetch<MyResource>>
    let fetched = world.try_fetch::<MyResource>();
    if let Some(fetched_resource) = fetched {
        //dereference Fetch<MyResource> to access data
        assert_eq!(*fetched_resource, MyResource{ game_score: 0, });
    } else {
        println!("No MyResource present in `World`");
    }
    
If you want to get a resource and create it if it doesn't exist.

    // If the resource isn't inside `World`, 
    // it will insert the instance we created earlier.
    let fetched = world.entry::<MyResource>().or_insert_with(|| my);


If you want to change a resource that is already inside of World:

      // try_fetch_mut returns a Option<FetchMut<MyResource>>
      let fetched = world.try_fetch_mut::<MyResource>();
      if let Some(mut fetched_resource) = fetched {
        assert_eq!(fetched_resource.game_score, 0);
        fetched_resource.game_score = 10;
        assert_eq!(fetched_resource.game_score, 10);
      } else {
        println!("No MyResource present in `World`");
      }
      
# Deleting a resource
> There is no method to properly "delete" a resource added to the world. The usual method to achieve something similar is to add an Option<MyResource> and to set it to None when you want to delete it.

# Storages, part2
> A Component'S Storage is a resource. The components are "attached" to entities, but as said previously, they are not "owned" by the entities at the implementation level. 

By storing them into Storage S and by having Storage be placed inside World, it allows global access to all of the components at runtime with minimal effort.

Actually accessing the components inside Storages will be covered in the world and system sections of the book.

**WARNING**: If you try to fetch the component directly, you will not get the storage. You will get a Default::default() instance of that component. To get the Storage resource that HOLDS all the MyComponent instances, you need to fetch ReadStorage<MyComponent>.

# World
> A World is a container for resources, with some helper functions that make your life easier. This chapter will showcase those functions and their usage.

# Adding a resource

    use amethyst::ecs::{World, WorldExt};
    
    // A simple struct with no data.
    struct MyResource;
    
    fn main() {
        // We create a new `World` instance.
        let mut world = World::new();
        
        // We create our resource.
        let my = MyResource;
        
        // We add the resource to the world.
        world.insert(my);
    }

# Fetching a resource
> Here's how to fetch a read-only resource. Be aware that this method panics if the resource isn't inserted into Resources.

    let my = world.read_resource::<MyResource>();
    
If you are not sure that the resource will be present, use the methods available on Resources, as shown in the resource chapter.

    let my = world.entry::<MyResource>().or_insert_with(|| MyResource);

# Modifying a resource

    let mut my = world.write_resource::<MyResource>();
    
# Creating entities
> You first start by creating the entity builder. Then, you can add components to your entity. Finally, you call the build() method on the entity builder to get the actual entity. Please note that *in order to use this syntax, you need to import the amethyst::prelude::Builder trait.*

    world.register::<MyComponent>();
    use amethyst::prelude::Builder;
    
    let mut entity_builder = world.create_entity();
    entity_builder = entity_builder.with(MyComponent);
    let my_entity = entity_builder.build();

Shorter version:
    
    use amethyst::prelude::Builder;
    let my_entity = world
        .create_entity()
        .with(MyComponet)
        .build();

Internally, the World interacts with EntitiesRes, which is a resource holding the entities inside of Resources.

# Accessing a Component

    // Create an `Entity` with `MyComponent`.
    // `World` will implicitly write to the component's storage in `Resources`.
    let my_entity = world.create_entity().with(MyComponent).build();
        
    // Get a ReadStorage<MyComponent>
    let storage = world.read_storage::<MyComponent>();
        
    // Get the actual component from the storage.
    let my = storage.get(my_entity).expect("Failed to get component for entity");
    
# Modifying a Component
> This is almost the same as accessing a component:
    
    let my_entity = world.create_entity().with(MyComponent).build();
    let mut storage = world.write_storage::<MyComponent>();
    let mut my = storage.get_mut(my_entity).expect("Failed to get component for entity");
    
# Getting all entities
> It is pretty rare to use this, but can be useful in some occasions.
    
    let entities = world.entities();

# Delete an entity

Single:

    world.delete_entity(my_entity).expect("Failed to delete entity. Was it already removed?");
    
Multiple:
    
    world.delete_entities(entity_vec.as_slice()).expect("Failed to delete entities from specified list.");

All:
    
    world.delete_all();

**Note**: Entities are lazily deleted, which means that deletion only happens at the end of the frame and not immediately when calling the delete method.

# Check if the entity was deleted

    let is_alive = world.is_alive(my_entity);
    
# Exec

> This is just to show that this feature exists. It is normal to not understand what it does until you read the system chapter

Sometimes, you will want to create an entity where you need to fetch resources to create the correct components for it. There is a function that acts as a shorthand for this:

    world.exec(|mut data: SomeSystemData| {
            data.do_something();
    });