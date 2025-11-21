# hrusty-arkanoid
Simple arkanoid game written in Hrust with SDL3
## What is Hrust?
Like Crust, it is Rust that is actually fun! <br />
What is different?
 - No use of rust std (but libc or SDL3 stdlib is allowed)
 - Many unsafe code
 - Use of C libs
## What about collisions?
I tried to make precise collisions by approximating collision time using binary search and processing collisions like sub-frames. But what about situations when a small object "teleported" through another between frames? <br />
Updated: Added a new experimental deep collision detection.
We can calculate distance between two objects (rects). So we can find distance delta in a very small time (f32 epsilon). If distance was decreasing and then became increasing (again binary search), there is chance that there was a collision at the smallest distance.
## Dependencies
Everything is already bundled
- [SDL](https://github.com/libsdl-org/SDL) <br />
- [upng](https://github.com/FREEWING-JP/upng/tree/feature/add_index_color_pallet) <br />
- [miniaudio](https://github.com/mackron/miniaudio)
## TODO
Test on platforms other than windows
