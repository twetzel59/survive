# #survive
A simple survival game, where you must find water and avoid freezing, etc.

## How to Play

### Controls
* Move around with WASD and change heading with the mouse. Go to the edges of the world to make it scroll.
* Click on trees to get wood. You must be in close range!
* Press 'C' to create a bonfire. Your mouse must be close to the player.

### Day/Night Cycle
There is a day/night cycle. It runs rather fast. The current time is indicated in the upper right on a sundial.

### Hydration
* You must avoid dehydration by standing in a water area of the world, especially in daylight hours.

### Temperature
* Avoid freezing by building a bonfire as soon as possible. At night, the temperature will fall rapidly.
* During the day, your temperature will rise again, but not enough to get by more than 3 days. After that, the loss will cascade enough to cause death by exposure.

### About
I created this to practice Rust. It isn't that "fun", but it taught me about lifetimes and traits. The world is generated with noise, and is a pseudo-random world. However, the same seed is currently used, so the world is generated the same way each time. This may change in the future. The Rust binding for SFML, a C++ OpenGL frontend for 2D rendering, is used for graphics, windowing and events.

