## Color Utility Functions
Using our new `vec3` class, we'll create a new `color.h` header file and define a utility function that writes a single pixel's color out to the standard output stream.

```rust,norun,noplayground
{{ #git show 6a4c63a317edcf4bb154fff3f817c561c2ed3aa9:src/color.rs }}
```

**Listing 5:** [[color.rs](TODO)] *color utility functions*

<br>

Now we can change our main to use both of these:

```rust-diff,norun,noplayground
{{ #git diff -h -U99 6a4c63a317edcf4bb154fff3f817c561c2ed3aa9 7e3d601b91c93177f3b3d1ed98aa69d8ecf4ffee src/main.rs }}
```

**Listing 6:** [[main.rs](TODO)] *Final code for the first PPM image*

<br>

And you should get the exact same picture as before.
