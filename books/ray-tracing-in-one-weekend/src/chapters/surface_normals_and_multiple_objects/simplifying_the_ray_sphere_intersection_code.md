## Simplifying the Ray-Sphere Intersection Code

Let’s revisit the ray-sphere function:

```rust,norun,noplayground
{{ #git show 47441cbbe4cb464b6f29e82801d47aba101092f2:src/main.rs:6:15 }}
```

**Listing 13:** [[main.rs](TODO)] *Ray-sphere intersection code (before)*

<br>

First, recall that a vector dotted with itself is equal to the squared length of that vector.

Second, notice how the equation for \\( b \\) has a factor of negative two in it. Consider what happens to the quadratic equation if \\( b = 2h \\):

\\[ \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}\\]
\\[= \frac{-(-2h) \pm \sqrt{(-2h)^2 - 4ac}}{2a}\\]
\\[= \frac{2h \pm 2 \sqrt{h^2 - ac}}{2a}\\]
\\[= \frac{h \pm \sqrt{h^2 - ac}}{a}\\]

This simplifies nicely, so we'll use it. So solving for \\( h \\):

\\[b = -2 \mathbf{d} \cdot (\mathbf{C} - \mathbf{Q}) \\]
\\[b = -2h \\]
\\[h = \frac{b}{-2} = \mathbf{d} \cdot (\mathbf{C} - \mathbf{Q}) \\]

Using these observations, we can now simplify the sphere-intersection code to this:

```rust-diff,norun,noplayground
{{ #git diff -U999 -h 47441cbbe4cb464b6f29e82801d47aba101092f2 f878b49faaf7958cf88bf4748416acf7cfd61408 src/main.rs:11:25 }}
```

**Listing 14:** [[main.rs](TODO)] *Ray-sphere intersection code (after)*

<br>
