## Mirrored Light Reflection

For polished metals the ray won’t be randomly scattered. The key question is: How does a ray get reflected from a metal mirror? Vector math is our friend here:

![Ray reflection](../../imgs/fig-1.15-reflection.jpg)

**Figure 15:** *Ray reflection*

<br>

The reflected ray direction in red is just \\( \mathbf{v} + 2 \mathbf{b} \\). In our design, \\( \mathbf{n} \\) is a unit vector (length one), but \\( \mathbf{v} \\) may not be. To get the vector \\( \mathbf{b} \\), we scale the normal vector by the length of the projection of \\( \mathbf{v} \\) onto \\( \mathbf{n} \\), which is given by the dot product \\( \mathbf{v} \cdot \mathbf{n} \\). (If \\( \mathbf{n} \\) were not a unit vector, we would also need to divide this dot product by the length of \\( \mathbf{n} \\).) Finally, because \\( \mathbf{b} \\) points *into* the surface, and we want \\( \mathbf{b} \\) to point *out* of the surface, we need to negate this projection length.

Putting everything together, we get the following computation of the reflected vector:

```rust-diff,norun,noplayground
{{ #git diff -U999 -h 5230c29a2d01fd09e037b9c6c326ec4cd88ef791 5454ed7161f531954e1b14fb97bd52d11b7331a4 src/vec3.rs:[211:213,219:225] }}
```

**Listing 64:** [[vec3.rs](https://github.com/goldnor/code/blob/5454ed7161f531954e1b14fb97bd52d11b7331a4/src/vec3.rs)] *vec3 reflection function*

<br>

The metal material just reflects rays using that formula:

```rust-diff,norun,noplayground
{{ #git diff -U999 -h 5454ed7161f531954e1b14fb97bd52d11b7331a4 c5fbfbf8809f5009c7c5953bcb1bcae1493267de src/material.rs:[24,27,38:] }}
```

**Listing 65:** [[material.rs](https://github.com/goldnor/code/blob/c5fbfbf8809f5009c7c5953bcb1bcae1493267de/src/material.rs)] *Metal material with reflectance function*

<br>

We need to modify the `ray_color()` function for all of our changes:

```rust-diff,norun,noplayground
{{ #git diff -U999 -h c5fbfbf8809f5009c7c5953bcb1bcae1493267de 6e7548e7e604786c7ee88cc924ef6015cb6b49d7 src/camera.rs:[48,154:] }}
```

**Listing 66:** [[camera.rs](https://github.com/goldnor/code/blob/6e7548e7e604786c7ee88cc924ef6015cb6b49d7/src/camera.rs)] *Ray color with scattered reflectance*

<br>

Now we'll update the sphere constructor to initialize the material pointer `mat`:

```rust-diff,norun,noplayground
{{ #git diff -U999 -h 6e7548e7e604786c7ee88cc924ef6015cb6b49d7 29d828944db9e970983d3e7a5b2b3ad827e3991b src/sphere.rs:[5:11,18:31] }}
```

**Listing 67:** [[sphere.rs](https://github.com/goldnor/code/blob/29d828944db9e970983d3e7a5b2b3ad827e3991b/src/sphere.rs)] *Initializing sphere with a material*

<br>
