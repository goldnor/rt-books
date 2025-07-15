## Creating Our First Raytraced Image

If we take that math and hard-code it into our program, we can test our code by placing a small sphere at \\( −1 \\) on the z-axis and then coloring red any pixel that intersects it.

```rust-diff,norun,noplayground
{{ #git diff -h -U999 d38202d1d50207cb9ff4bd6721dd88167e06feb6 2cf21871a39a309bce0edfb75464389bba9782f4 src/main.rs::31 }}
```

**Listing 11:** [[main.rs](https://github.com/goldnor/code/blob/2cf21871a39a309bce0edfb75464389bba9782f4/src/main.rs)] *Rendering a red sphere*

<br>

What we get is this:

<img style="width: 100%; image-rendering: pixelated" src="../../imgs/img-1.03-red-sphere.png" alt="A simple red sphere">

**Image 3:** *A simple red sphere*

<br>
