## Fixing Shadow Acne

There’s also a subtle bug that we need to address. A ray will attempt to accurately calculate the intersection point when it intersects with a surface. Unfortunately for us, this calculation is susceptible to floating point rounding errors which can cause the intersection point to be ever so slightly off. This means that the origin of the next ray, the ray that is randomly scattered off of the surface, is unlikely to be perfectly flush with the surface. It might be just above the surface. It might be just below the surface. If the ray's origin is just below the surface then it could intersect with that surface again. Which means that it will find the nearest surface at \\( 𝑡 = 0.00000001 \\) or whatever floating point approximation the hit function gives us. The simplest hack to address this is just to ignore hits that are very close to the calculated intersection point:

```rust-diff,norun,noplayground
{{ #git diff -U999 -h 7be08e72109304cab1df8c2d9d6a9a17fe9a936b f4051c25e9322b269695ab51a919c084a0caa440 src/camera.rs:[48,154:] }}
```

**Listing 54:** [[camera.rs](TODO)] *Calculating reflected ray origins with tolerance*

<br>

This gets rid of the shadow acne problem. Yes it is really called that. Here's the result:

<img style="width: 100%; image-rendering: pixelated" src="../../imgs/img-1.09-no-acne.png" alt="Diffuse sphere with no shadow acne">

**Image 9:** *Diffuse sphere with no shadow acne*

<br>
