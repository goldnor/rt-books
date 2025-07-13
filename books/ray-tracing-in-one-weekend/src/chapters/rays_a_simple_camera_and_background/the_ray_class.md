## The ray Class
The one thing that all ray tracers have is a ray class and a computation of what color is seen along a ray. Let’s think of a ray as a function \\( \mathbf{P} (t) = \mathbf{A} + t \mathbf{b} \\). Here \\( \mathbf{P} \\) is a 3D position along a line in 3D. \\( \mathbf{A} \\) is the ray origin and \\( \mathbf{b} \\) is the ray direction. The ray parameter \\( t \\) is a real number (double in the code). Plug in a different \\( t \\) and \\( \mathbf{P} (t) \\) moves the point along the ray. Add in negative \\( t \\) values and you can go anywhere on the 3D line. For positive \\( t \\), you get only the parts in front of \\( \mathbf{A} \\), and this is what is often called a half-line or a ray.

![Linear interpolation](../../imgs/fig-1.02-lerp.jpg)

**Figure 2:** *Linear interpolation*

<br>

We can represent the idea of a ray as a class, and represent the function \\( \mathbf{P} (t) \\) as a function that we'll call `ray::at(t)`:

```rust,norun,noplayground
{{ #git show b54170d548e41ff6ea02b4eceb59269ee20b2a56:src/ray.rs }}
```

**Listing 7:** [[ray.rs](https://github.com/goldnor/code/blob/b54170d548e41ff6ea02b4eceb59269ee20b2a56/src/ray.rs)] *The ray class*

<br>

(For those unfamiliar with C++, the functions ray::origin() and ray::direction() both return an immutable reference to their members. Callers can either just use the reference directly, or make a mutable copy depending on their needs.) [^41a]

[^41a]: The careful reader may have noticed that, in the Rust approach, both the `Vec3` and `Ray` structs implement Copy. This method is generally more common than returning a reference and cloning it for mutability when needed.




