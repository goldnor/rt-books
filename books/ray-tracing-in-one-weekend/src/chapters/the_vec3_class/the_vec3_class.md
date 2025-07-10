# The vec3 Class [^3a]

[^3a]: There are no classes in Rust. They are replaced with structs.

Almost all graphics programs have some class(es) for storing geometric vectors and colors. In many systems these vectors are 4D (3D position plus a homogeneous coordinate for geometry, or RGB plus an alpha transparency component for colors). For our purposes, three coordinates suffice. We’ll use the same class vec3 for colors, locations, directions, offsets, whatever. Some people don’t like this because it doesn’t prevent you from doing something silly, like subtracting a position from a color. They have a good point, but we’re going to always take the “less code” route when not obviously wrong. In spite of this, we do declare two aliases for vec3: point3 and color. Since these two types are just aliases for vec3, you won't get warnings if you pass a color to a function expecting a point3, and nothing is stopping you from adding a point3 to a color, but it makes the code a little bit easier to read and to understand.

We define the vec3 class in the top half of a new vec3.h header file, and define a set of useful vector utility functions in the bottom half:

```rust,norun,noplayground
{{ #git show 400e8e91847d5b2fbde6eb17432cbbe49794e28d:src/vec3.rs }}
```
**Listing 4:** [[vec3.rs](TODO)] *vec3 definitions and helper functions*

We use double here, but some ray tracers use float. double has greater precision and range, but is twice the size compared to float. This increase in size may be important if you're programming in limited memory conditions (such as hardware shaders). Either one is fine — follow your own tastes.

