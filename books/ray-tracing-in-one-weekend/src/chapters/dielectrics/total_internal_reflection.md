## Total Internal Reflection

One troublesome practical issue with refraction is that there are ray angles for which no solution is possible using Snell's law. When a ray enters a medium of lower index of refraction at a sufficiently glancing angle, it can refract with an angle greater than 90°. If we refer back to Snell's law and the derivation of \\( \sin \theta' \\):

\\[ \sin \theta' = \frac{\eta}{\eta'} \cdot \sin \theta \\]

If the ray is inside glass and outside is air (\\( \eta = 1.5 \\) and \\( \eta' = 1.0 \\)):

\\[ \sin \theta' = \frac{1.5}{1.0} \cdot \sin \theta \\]

The value of \\( \sin \theta' \\) cannot be greater than 1. So, if,

\\[ \frac{1.5}{1.0} \cdot \sin \theta > 1.0 \\]

the equality between the two sides of the equation is broken, and a solution cannot exist. If a solution does not exist, the glass cannot refract, and therefore must reflect the ray:

```rust,norun,noplayground
{{ #git show c5841df4f85f94dc87ccaadc6f50eb63ec71d868:src/material.rs:[87:89,90:92,93] }}
```

**Listing 74:** [[material.rs](https://github.com/goldnor/code/blob/c5841df4f85f94dc87ccaadc6f50eb63ec71d868/src/material.rs)] *Determining if the ray can refract*

<br>

Here all the light is reflected, and because in practice that is usually inside solid objects, it is called *total internal reflection*. This is why sometimes the water-to-air boundary acts as a perfect mirror when you are submerged — if you're under water looking up, you can see things above the water, but when you are close to the surface and looking sideways, the water surface looks like a mirror.

We can solve for `sin_theta` using the trigonometric identities:

\\[ \sin \theta = \sqrt{ 1 - \cos^2 \theta } \\]

and

\\[ \cos \theta = \mathbf{R} \cdot \mathbf{n} \\]

```rust,norun,noplayground
{{ #git show c5841df4f85f94dc87ccaadc6f50eb63ec71d868:src/material.rs:[84:89,90:92,93] }}
```

**Listing 75:** [[material.rs](https://github.com/goldnor/code/blob/c5841df4f85f94dc87ccaadc6f50eb63ec71d868/src/material.rs)] *Determining if the ray can refract*

<br>

And the dielectric material that always refracts (when possible) is:

```rust-diff,norun,noplayground
{{ #git diff -U999 -h e7d30179cee4fc7242cc3a887dc6d7a7eb6b86f0 2a5e5e9ad9a9e5481fbc28a40c14c707af2cea51 src/material.rs:77: }}
```

**Listing 76:** [[material.rs](https://github.com/goldnor/code/blob/2a5e5e9ad9a9e5481fbc28a40c14c707af2cea51/src/material.rs)] *Dielectric material class with reflection*

<br>

Attenuation is always 1 — the glass surface absorbs nothing.

If we render the prior scene with the new `dielectric::scatter()` function, we see … no change. Huh?

Well, it turns out that given a sphere of material with an index of refraction greater than air, there's no incident angle that will yield total internal reflection — neither at the ray-sphere entrance point nor at the ray exit. This is due to the geometry of spheres, as a grazing incoming ray will always be bent to a smaller angle, and then bent back to the original angle on exit.

So how can we illustrate total internal reflection? Well, if the sphere has an index of refraction less than the medium it's in, then we can hit it with shallow grazing angles, getting total external reflection. That should be good enough to observe the effect.

We'll model a world filled with water (index of refraction approximately 1.33), and change the sphere material to air (index of refraction 1.00) — an air bubble! To do this, change the left sphere material's index of refraction to

\\[ \frac{\text{index of refraction of air}}{\text{index of refraction of water}} \\]

```rust-diff,norun,noplayground
{{ #git diff -U999 -h 2a5e5e9ad9a9e5481fbc28a40c14c707af2cea51 4f80203ca763361671fcf137bb1e996c54954705 src/main.rs:16:21 }}
```

**Listing 77:** [[main.rs](https://github.com/goldnor/code/blob/4f80203ca763361671fcf137bb1e996c54954705/src/main.rs)] *Left sphere is an air bubble in water*

<br>

This change yields the following render:

<img style="width: 100%; image-rendering: pixelated" src="../../imgs/img-1.17-air-bubble-total-reflection.png" alt="Air bubble sometimes refracts, sometimes reflects">

**Image 17:** *Air bubble sometimes refracts, sometimes reflects*

<br>

Here you can see that more-or-less direct rays refract, while glancing rays reflect.
