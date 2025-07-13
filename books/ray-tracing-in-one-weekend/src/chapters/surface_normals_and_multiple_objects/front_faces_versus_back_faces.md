## Front Faces Versus Back Faces

The second design decision for normals is whether they should always point out. At present, the normal found will always be in the direction of the center to the intersection point (the normal points out). If the ray intersects the sphere from the outside, the normal points against the ray. If the ray intersects the sphere from the inside, the normal (which always points out) points with the ray. Alternatively, we can have the normal always point against the ray. If the ray is outside the sphere, the normal will point outward, but if the ray is inside the sphere, the normal will point inward.

![Possible directions for sphere surface-normal geometry](../../imgs/fig-1.07-normal-sides.jpg)

**Figure 7:** *Possible directions for sphere surface-normal geometry*

<br>

We need to choose one of these possibilities because we will eventually want to determine which side of the surface that the ray is coming from. This is important for objects that are rendered differently on each side, like the text on a two-sided sheet of paper, or for objects that have an inside and an outside, like glass balls.

If we decide to have the normals always point out, then we will need to determine which side the ray is on when we color it. We can figure this out by comparing the ray with the normal. If the ray and the normal face in the same direction, the ray is inside the object, if the ray and the normal face in the opposite direction, then the ray is outside the object. This can be determined by taking the dot product of the two vectors, where if their dot is positive, the ray is inside the sphere.

```rust,norun,noplayground
{{ #git show 82ebc96f39c4382174634b94a3425093ac1e6daf:src/hittable.rs:[20:22,24:26,28] }}
```

**Listing 17:** *Comparing the ray and the normal*

<br>

If we decide to have the normals always point against the ray, we won't be able to use the dot product to determine which side of the surface the ray is on. Instead, we would need to store that information:

```rust,norun,noplayground
{{ #git show 82ebc96f39c4382174634b94a3425093ac1e6daf:src/hittable.rs:19:29 }}
```

**Listing 18:** *Remembering the side of the surface*

<br>

We can set things up so that normals always point “outward” from the surface, or always point against the incident ray. This decision is determined by whether you want to determine the side of the surface at the time of geometry intersection or at the time of coloring. In this book we have more material types than we have geometry types, so we'll go for less work and put the determination at geometry time. This is simply a matter of preference, and you'll see both implementations in the literature.

We add the `front_face` bool to the `hit_record` class. We'll also add a function to solve this calculation for us: `set_face_normal()`. For convenience we will assume that the vector passed to the new `set_face_normal()` function is of unit length. We could always normalize the parameter explicitly, but it's more efficient if the geometry code does this, as it's usually easier when you know more about the specific geometry.

```rust-diff,norun,noplayground
{{ #git diff -U999 -h 36e5e545d452e57637150a4e1a8ba2ff93aee52b a1c2d4fad73108c42bb6612c0bdb02d2a9c6a7ed src/hittable.rs:11:32 }}
```

**Listing 19:** [[hittable.rs](https://github.com/goldnor/code/blob/a1c2d4fad73108c42bb6612c0bdb02d2a9c6a7ed/src/hittable.rs)] *Adding front-face tracking to hit_record*

<br>

And then we add the surface side determination to the class:

```rust-diff,norun,noplayground
{{ #git diff -U999 -h a1c2d4fad73108c42bb6612c0bdb02d2a9c6a7ed e54430867656af9ca129dfd3350fe4f0ead18d19 src/sphere.rs:[26:28,48:64] }}
```

**Listing 20:** [[sphere.rs](https://github.com/goldnor/code/blob/e54430867656af9ca129dfd3350fe4f0ead18d19/src/sphere.rs)] *The sphere class with normal determination*

<br>


