## Common Constants and Utility Functions

We need some math constants that we conveniently define in their own header file. For now we only need infinity, but we will also throw our own definition of pi in there, which we will need later. We'll also throw common useful constants and future utility functions in here. This new header, `rtweekend.h`, will be our general main header file. [^67a]

[^67a]: In Rust it is common to create a prelude for common types, which we will do here instead. Note however, that there are at the momentan no [plan to include a custom prelude as a language feature](https://github.com/rust-lang/rfcs/pull/890), instead we need to import the prelude with `use crate::prelude::*`.

```rust,norun,noplayground
{{ #git show 9e1b0c216e67d1bcf41de6ee6c449758c9cc33e5:src/prelude.rs }}
```

**Listing 24:** [[prelude.rs](TODO)] *The rtweekend.h common header*

<br>

Program files will include `rtweekend.h` first, so all other header files (where the bulk of our code will reside) can implicitly assume that `rtweekend.h` has already been included. Header files still need to explicitly include any other necessary header files. We'll make some updates with these assumptions in mind.

```rust,norun,noplayground
// nothing changes
```

**Listing 25:** [[color.rs](TODO)] *Assume rtweekend.h inclusion for color.h* [^67b]

<br>

[^67b]: There is no need to use the prelude in `color.rs` only for the `Vec3` struct. The listing is still included to match the numbering of the original book series.

```rust-diff,norun,noplayground
{{ #git diff -U999 9e1b0c216e67d1bcf41de6ee6c449758c9cc33e5 a83dfcb94c1741454b03fe57b8dc56e97a47c0cc src/hittable.rs:5:10 }}
```

**Listing 26:** [[hittable.rs](TODO)] *Assume rtweekend.h inclusion for hittable.h*

<br>

```rust-diff,norun,noplayground
{{ #git diff -U999 a83dfcb94c1741454b03fe57b8dc56e97a47c0cc fd505c6bc51cd1887e71b1ce44fe0ee32a1fb198 src/hittable_list.rs:5:12 }}
```

**Listing 27:** [[hittable_list.rs](TODO)] *Assume rtweekend.h inclusion for hittable_list.h*

<br>

```rust-diff,norun,noplayground
{{ #git diff -U999 fd505c6bc51cd1887e71b1ce44fe0ee32a1fb198 d57e0ab6d5de95b5d3105a150a69c552e4eeb167 src/sphere.rs:5:11 }}
```

**Listing 28:** [[sphere.rs](TODO)] *Assume rtweekend.h inclusion for sphere.h*

<br>

```rust-diff,norun,noplayground
// nothing changes
```

**Listing 29:** [[vec3.rs](TODO)] *Assume rtweekend.h inclusion for vec3.h*

<br>

And now the new main:

```rust-diff,norun,noplayground
{{ #git diff -U999 d57e0ab6d5de95b5d3105a150a69c552e4eeb167 e19dd26432882f6c779f25632318690a3be5a4ac src/main.rs:[5:12,17,19:22,26:83,84:91,92:96,97:] }}
```

**Listing 30:** [[main.rs](TODO)] *The new main with hittables*

<br>

This yields a picture that is really just a visualization of where the spheres are located along with their surface normal. This is often a great way to view any flaws or specific characteristics of a geometric model.

<img style="width: 100%" src="../../imgs/img-1.05-normals-sphere-ground.png" alt=" Resulting render of normals-colored sphere with ground">

**Image 5:** *Resulting render of normals-colored sphere with ground*

<br>






