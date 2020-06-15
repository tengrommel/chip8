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

# System
> A system is where the logic of the game is executed. In practice, it consists of a struct implementing a function executed on every iteration of the game loop, and taking as taking as an argument data about the game.

Systems can be seen as a small unit of logic. All systems are run by the engine together (even in parallel when possible), and do a specialized operation on one or a group of entities.

# Structure

A system struct is a structure implementing the trait amethyst::ecs::System.

Here is a very simple example implementation:

    struct MyFirstSystem;
    
    impl<'a> System<'a> for MyFirstSystem {
        type SystemData = ();
    
        fn run(&mut self, data: Self::SystemData) {
            println!("Hello!");
        }
    }
    
This system will, on every iteration of the game loop, print "Hello!" in the console. This is a pretty boring system as it does not interact at all with the game. Let us spice it up a bit.

# Accessing the context of the game
> In the definition of a system, the requires you to define a type SystemData.

This type defines what data the system will be provided with on each call of its run method. SystemData is only meant to carry information accessible to multiple systems. Data local to a system is usually stored in the system's struct itself instead.

*The Amethyst engine provides useful system data types to use in order to access the context of a game. Here are some of the most important ones:*

- Read<'a, Resource> (respectively Write<'a, Resource>) allows you to obtain an immutable (respectively mutable) reference to a resource of the type you specify. This is guaranteed to not fail as if the resource is not available, it will give you the Default::default() of your resource.

- ReadExpect<'a, Resource> (respectively WriteExpect<'a, Resource>) is a failable alternative to the previous system data, so that you can use resources that do not implement the Default trait.

- ReadStorage<'a, Component> (respectively WriteStorage<'a, Component>) allows you to obtain an immutable (respectively mutable) reference to the entire storage of a certain Component type.

- Entities<'a> allows you to create or destroy entities in the context of a system.


    struct MyFirstSystem;
    
    impl<'a> System<'a> for MyFirstSystem {
        type SystemData = Read<'a, Time>;
    
        fn run(&mut self, data: Self::SystemData) {
            println!("{}", data.delta_seconds());
        }
    }
    
 Here, we get the *amethyst::core::timing::Time* resource to print in the console the time elapsed between two frames. Nice! But that's still a bit boring.
 
# Manipulating storages
>Once you have access to a storage, you can use them in different ways.

## Getting a component of a specific entity

Sometimes, it can be useful to get a component in the storage for a specific entity. This can easily be done using the get or, for mutable storages, get_mut methods.

    struct WalkPlayerUp {
        player: Entity,
    }
    
    impl<'a> System<'a> for WalkPlayerUp {
        type SystemData = WriteStorage<'a, Transform>;
    
        fn run(&mut self, mut transforms: Self::SystemData) {
            transforms.get_mut(self.player).unwrap().prepend_translation_y(0.1);
        }
    }

This system makes the player go up by 0.1 unit every iteration of the game loop! To identify what entity the player is, we stored it beforehand in the system's struct. Then, we get its *Transform* from the transform storage, and move it along the Y axis by 0.1.

# Getting all entities with specific components

Most of the time, you will want to perform logic on all entities with a specific component, or even all entities with a selection of components.

This is possible using the join method. You may be familiar with joining operations if you have ever worked with databases. The join method takes multiple storages, and iterates over all entities that have a component in each of those storages. It works like an "AND" gate. It will return an iterator containing a tuple of all the requested components if they are ALL on the same entity.

**If you join with components A, B and C, only the entities that have ALL those components will be considered.**

Needless to say that you can use it with only one storage to iterate over all entities with a specific component.

Keep in mind that the join method is only available by importing amethyst::ecs::Join.

    

    use amethyst::ecs::Join;
    
    struct MakeObjectsFall;
    
    impl<'a> System<'a> for MakeObjectsFall {
        type SystemData = (
            WriteStorage<'a, Transform>,
            ReadStorage<'a, FallingObject>,
        );
    
        fn run(&mut self, (mut transforms, falling): Self::SystemData) {
            for (mut transform, _) in (&mut transforms, &falling).join() {
                if transform.translation().y > 0.0 {
                    transform.prepend_translation_y(-0.1);
                }
            }
        }
    }

# Manipulating the structure of entities

It may sometimes be interesting to manipulate the structure of entities in a system, such as creating new ones or modifying the component layout of existing ones. This kind of process is done using the Entities<'a> system data.


# Creating new entities in a system
> Creating an entity while in the context of a system is very similar to the way one would create an entity using the World struct. The only difference is that one needs to provide mutable storages of all the components they plan to add to the entity.

    struct SpawnEnemies {
        counter: u32,
    }
    
    impl<'a> System<'a> for SpawnEnemies {
        type SystemData = (
            WriteStorage<'a, Transform>,
            WriteStorage<'a, Enemy>,
            Entities<'a>,
        );
    
        fn run(&mut self, (mut transforms, mut enemies, entities): Self::SystemData) {
            self.counter += 1;
            if self.counter > 200 {
                entities.build_entity()
                    .with(Transform::default(), &mut transforms)
                    .with(Enemy, &mut enemies)
                    .build();
                self.counter = 0;
            }
        }
    }

# Removing an entity
>Deleting an entity is very easy using Entities<'a>

    entities.delete(entity);
    
# Iterating over components with associated entity
> Sometimes, when you iterate over components, you may want to also know what entity you are working with. To do that, you can use the joining operation with Entities<'a>

    struct MakeObjectsFall;
    
    impl<'a> System<'a> for MakeObjectsFall {
        type SystemData = (
            Entities<'a>,
            WriteStorage<'a, Transform>,
            ReadStorage<'a, FallingObject>,
        );
    
        fn run(&mut self, (entities, mut transforms, falling): Self::SystemData) {
            for (e, mut transform, _) in (&*entities, &mut transforms, &falling).join() {
                if transform.translation().y > 0.0 {
                    transform.prepend_translation_y(-0.1);
                } else {
                    entities.delete(e);
                }
            }
        }
    }

# Adding or removing components

You can also insert or remove components from a specific entity. To do that, you need to get a mutable storage of the component you want to modify, and simply do:

    // Add the component
    write_storage.insert(entity, MyComponent);
    
    // Remove the component
    write_storage.remove(entity);

# Changing states through resources
> The data in a resource is available both to systems and states. We can use this to our advantage!

Let's say you have the following two states:

- GameplayState: State in which the game is running
- GameMenuState: State where the game is paused and we interact with a game menu.

The following example shows how to keep track of which state we are currently in. This allows us to do a bit of conditional logic in our systems to determine what to do depending on which state is currently active, and manipulating the states by tracking user actions:

    use amethyst::prelude::*;
    
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum CurrentState {
        MainMenu,
        Gameplay,
    }
    
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum UserAction {
        OpenMenu,
        ResumeGame,
        Quit,
    }
    
    impl Default for CurrentState {
        fn default() -> Self {
            CurrentState::Gameplay
        }
    }
    
    struct Game {
        user_action: Option<UserAction>,
        current_state: CurrentState,
    }
    
    impl Default for Game {
        fn default() -> Self {
            Game {
                user_action: None,
                current_state: CurrentState::default(),
            }
        }
    }
    
    struct GameplayState;
    
    impl SimpleState for GameplayState {
        fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
            // If the `Game` resource has been set up to go back to the menu, push
            // the menu state so that we go back.
    
            let mut game = data.world.write_resource::<Game>();
    
            if let Some(UserAction::OpenMenu) = game.user_action.take() {
                return Trans::Push(Box::new(GameMenuState));
            }
    
            Trans::None
        }
    
        fn on_resume(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
            // mark that the current state is a gameplay state.
            data.world.write_resource::<Game>().current_state = CurrentState::Gameplay;
        }
    }
    
    struct GameMenuState;
    
    impl SimpleState for GameMenuState {
        fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
            let mut game = data.world.write_resource::<Game>();
    
            match game.user_action.take() {
                Some(UserAction::ResumeGame) => Trans::Pop,
                Some(UserAction::Quit) => {
                    // Note: no need to clean up :)
                    Trans::Quit
                },
                _ => Trans::None,
            }
        }
    
        fn on_resume(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
            // mark that the current state is a main menu state.
            data.world.write_resource::<Game>().current_state = CurrentState::MainMenu;
        }
    }
    
# The SystemData trait
> While this is rarely useful, it is possible to create SystemData types.

## System Initialization
> Systems may need to access resources from the World in order to be instantiated. For example, obtaining a ReaderId to an EventChannel that exists in the World. When there is an existing event channel in the World, a System should register itself as a reader of that channel instead of replacing it, as that invalidates all other readers.

In Amethyst, the World that the application begins with is populated with a number of default resources -- event channels, a thread pool, a frame limiter, and so on.

Given the default resources begin with special limits, we need a way to pass the System initialization logic through to the application, including parameters to the System's constructor. This is information the SystemDesc trait captures.

For each System, an implementation of the SystemDesc trait specifies the logic to instantiate the System. For Systems that do not require special initialization logic, the SystemDesc derive automatically implements the SystemDesc trait on the system type itself:

    use amethyst::{
        core::SystemDesc,
        derive::SystemDesc,
        ecs::{System, SystemData, World},
    };
    
    #[derive(SystemDesc)]
    struct SystemName;
    
    impl<'a> System<'a> for SystemName {
        type SystemData = ();
    
        fn run(&mut self, data: Self::SystemData) {
            println!("Hello!");
        }
    }
    
# SystemDesc Derive
> The SystemDesc derive supports the following cases when generating a SystemDesc trait implementation:

- Parameters to pass to the system constructor
- Fields to skip -- defaulted by the system constructor
- Registering a ReaderId for an EventChannel<_> in the World
- Registering a ReaderId to a component's FlaggedStorage
- Inserting a resource into the world

# Passing parameters to system constructor
    
    #[derive(SystemDesc)]
    #[system_desc(name(SystemNameDesc))]
    pub struct SystemName {
        field_0: u32,
        field_1: String,
    }
    
    impl SystemName {
        fn new(field_0: u32, field_1: String) -> Self {
            SystemName { field_0, field_1 }
        }
    }

# Fields to skip -- defaulted by the system constructor

    #[derive(SystemDesc)]
    #[system_desc(name(SystemNameDesc))]
    pub struct SystemName {
        #[system_desc(skip)]
        field_0: u32,
        field_1: String,
    }
    
    impl SystemName {
        fn new(field_1: String) -> Self {
            SystemName { field_0: 123, field_1 }
        }
    }

Note: If there are no field parameters, the SystemDesc implementation will call SystemName::default():

    #[derive(Default, SystemDesc)]
    #[system_desc(name(SystemNameDesc))]
    pub struct SystemName {
        #[system_desc(skip)]
        field_0: u32,
    }

# Registering a ReaderId for an EventChannel<_> in the World

    #[derive(SystemDesc)]
    #[system_desc(name(SystemNameDesc))]
    pub struct SystemName {
        #[system_desc(event_channel_reader)]
        reader_id: ReaderId<UiEvent>,
    }
    
    impl SystemName {
        fn new(reader_id: ReaderId<UiEvent>) -> Self {
            SystemName { reader_id }
        }
    }
    
# Registering a ReaderId to a component's FlaggedStorage

    #[derive(SystemDesc)]
    #[system_desc(name(SystemNameDesc))]
    pub struct SystemName {
        #[system_desc(flagged_storage_reader(UiResize))]
        resize_events_id: ReaderId<ComponentEvent>,
    }
    
    impl SystemName {
        fn new(resize_events_id: ReaderId<ComponentEvent>) -> Self {
            SystemName { resize_events_id }
        }
    }

# Inserting a resource into the World

    pub struct NonDefault;
    
    #[derive(Default, SystemDesc)]
    #[system_desc(insert(NonDefault))]
    pub struct SystemName;
    
    impl<'a> System<'a> for SystemName {
        type SystemData = ReadExpect<'a, NonDefault>;
        fn run(&mut self, data: Self::SystemData) {}
    }

# Implementing the SystemDesc Trait
> If the SystemDesc derive is unable to generate a SystemDesc trait implementation for system initialization, the SystemDesc trait can be implemented manually:

    use amethyst::{
        audio::output::Output,
        core::SystemDesc,
        ecs::{System, SystemData, World},
    };
    
    /// Builds an `AudioSystem`.
    #[derive(Default, Debug)]
    pub struct AudioSystemDesc {
        /// Audio `Output`.
        pub output: Output,
    }
    
    impl<'a, 'b> SystemDesc<'a, 'b, AudioSystem> for AudioSystemDesc {
        fn build(self, world: &mut World) -> AudioSystem {
            <AudioSystem as System<'_>>::SystemData::setup(world);
    
            world.insert(self.output.clone());
    
            AudioSystem(self.output)
        }
    }
    
    // in `main.rs`:
    // let game_data = GameDataBuilder::default()
    //     .with_system_desc(AudioSystemDesc::default(), "", &[]);
    
# Templates

    use amethyst_core::SystemDesc;
    
    /// Builds a `SystemName`.
    #[derive(Default, Debug)]
    pub struct SystemNameDesc;
    
    impl<'a, 'b> SystemDesc<'a, 'b, SystemName> for SystemNameDesc {
        fn build(self, world: &mut World) -> SystemName {
            <SystemName as System<'_>>::SystemData::setup(world);
    
            let arg = unimplemented!("Replace code here");
    
            SystemName::new(arg)
        }
    }

With type parameters:
    
    use std::marker::PhantomData;
    
    use derivative::Derivative;
    
    use amethyst_core::ecs::SystemData;
    use amethyst_core::SystemDesc;
    
    /// Builds a `SystemName`.
    #[derive(Derivative, Debug)]
    #[derivative(Default(bound = ""))]
    pub struct SystemNameDesc<T> {
        marker: PhantomData<T>,
    }
    
    impl<'a, 'b, T> SystemDesc<'a, 'b, SystemName<T>>
        for SystemNameDesc<T>
    where
        T: unimplemented!("Replace me."),
    {
        fn build(self, world: &mut World) -> SystemName<T> {
            <SystemName<T> as System<'_>>::SystemData::setup(world);
    
            let arg = unimplemented!("Replace code here");
    
            SystemName::new(arg)
        }
    }

# Dispatcher
> Dispatchers are the heart of the ECS infrastructure. They are the executors that decide when the Systems will be executed so that they don't walk over each other.

When a dispatcher is created, it is associated with the systems that it will execute. It then generates an execution plan that respects mutability rules while maximizing parallelism.

# Respecting mutability rules
> When a system wants to access a Storage or a resource, they can do so either mutably or immutably. This works just like in Rust: either only one system can request something mutably and no other system can access it, or multiple systems can request something but only immutably.

The dispatcher looks at all the SystemData in the systems and builds execution stages.

If you want to have the best performance possible, you should prefer immutable over mutable whenever it is possible. (Read instead of Write, ReadStorage instead of WriteStorage).
    
# Event Channel
> An EventChannel is a broadcast queue of events. Events may be any type that implements Send + Sync + 'static.


# Creating an event channel

    // In the following examples, `MyEvent` is the event type of the channel.
    #[derive(Debug)]
    pub enum MyEvent {
        A,
        B,
    }
    
    let mut channel = EventChannel::<MyEvent>::new();

# Writing events to the event channel

- Single:
    
    
    channel.single_write(MyEvent::A);
    
- Multiple:
    
    
    channel.iter_write(vec![MyEvent::A, MyEvent::A, MyEvent::B].into_iter());
    
# Reading events

EventChannels guarantee sending events in order to each reader.

To subscribe to events, register a reader against the EventChannel to receive a ReaderId:

    let mut reader_id = channel.register_reader();
    
When reading events, pass the ReaderId in:

    for event in channel.read(&mut reader_id) {
        // The type of the event is inferred from the generic type
        // we assigned to the `EventChannel<MyEvent>` earlier when creating it.
        println!("Received event value of: {:?}", event);
    }

# Patterns
> When using the event channel, we usually re-use the same pattern over and over again to maximize parallelism. It goes as follow:

- Create the event channel and add it to the world during State creation:
  
    
    world.insert(EventChannel::<MyEvent>::new());

- In the producer System, get a mutable reference to your resource:


    type SystemData = Write<'a, EventChannel<MyEvent>>;
    
- In the receiver Systems, you need to store the ReaderId somewhere.


    struct ReceiverSystem {
        // The type inside of ReaderId should be the type of the event you are using.
        reader: Option<ReaderId<MyEvent>>,
    }

- and you also need to get read access:


    type SystemData = Read<'a, EventChannel<MyEvent>>;
    
- Then, in the System's new method:


    impl MySystem {
        pub fn new(world: &mut World) -> Self {
            <Self as System<'_>>::SystemData::setup(world);
            let reader_id = world.fetch_mut::<EventChannel<MyEvent>>().register_reader();
            Self { reader_id }
        }
    }
    
- Finally, you can read events from your System.

  
    impl<'a> amethyst::ecs::System<'a> for MySystem {
        type SystemData = Read<'a, EventChannel<MyEvent>>;
        fn run(&mut self, my_event_channel: Self::SystemData) {
            for event in my_event_channel.read(&mut self.reader_id) {
                println!("Received an event: {:?}", event);
            }
        }
    }