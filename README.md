# chip8 rust game

# The game

- It will be a 2D game, with one player(a cat) on the left and one player on the right.
- WASD keys control the movement of the left player, and the arrow keys control the right player
- A ball will be fed from the middle, and each player has to bounce the ball back to the opponent using its head and body.
- You score when the ball touches the ground on the opponent's side.
- There will be music and sound effects.

# Amethyst ESC Pattern

> Amethyst is a game engine that is built on the entity-component-system(ESC) pattern.

The core idea of ECS is to promote composition over inheritance.

- First are entities.

  > Entities are objects in the game like the player, the monsters, and the trees.

- Implementing all the aspects of the entities in one piece of code will quickly become unmanageable.

- Instead, you'll learn how to separate each aspect into components, and attach components onto entities, creating the game object from a collection of components.

following components

- Attack: Attack power and range
- Transform: Keep track of the location
- Collision: Detect collision
- Health: Keep track of the health and death

following entities

- Player: Attack + Transform + Collision + Health
- Monster: Attack + Transform + Collision + Health
- Tree: Transform + Collision

Finally, to make the game move, you'll implement systems to update each component.

- Movement: Moves the entities and updates their Transform.
- Input: Takes user input, updates the player's location, and perform attacks
- Collision: Checks for collisions and stop the entities from crossing each other; may also incur damage.
- Attack: When an attack happens, reduces the health of the victim based on the attacker's attack power.
