## Modeling Light Scatter and Reflectance

Here and throughout these books we will use the term *albedo* (Latin for “whiteness”). Albedo is a precise technical term in some disciplines, but in all cases it is used to define some form of *fractional reflectance*. Albedo will vary with material color and (as we will later implement for glass materials) can also vary with incident viewing direction (the direction of the incoming ray).

Lambertian (diffuse) reflectance can either always scatter and attenuate light according to its reflectance \\( \mathbf{R} \\), or it can sometimes scatter (with probability \\( (1 - \mathbf{R}) \\)) with no attenuation (where a ray that isn't scattered is just absorbed into the material). It could also be a mixture of both those strategies. We will choose to always scatter, so implementing Lambertian materials becomes a simple task:

```rust-diff,norun,noplayground
{{ #git diff -U999 -h 9e2b407ffc6f22c19f7e3e60baf638c5797f15e2 c2bd376435e53106f8045a293708f0e5c0f2d549 src/material.rs:13: }}
```

**Listing 61:** [[material.rs](https://github.com/goldnor/code/blob/c2bd376435e53106f8045a293708f0e5c0f2d549/src/material.rs)] *The new lambertian material class*

<br>

Note the third option: we could scatter with some fixed probability \\( p \\) and have attenuation be \\( albedo/p \\). Your choice.

If you read the code above carefully, you'll notice a small chance of mischief. If the random unit vector we generate is exactly opposite the normal vector, the two will sum to zero, which will result in a zero scatter direction vector. This leads to bad scenarios later on (infinities and NaNs), so we need to intercept the condition before we pass it on.

In service of this, we'll create a new vector method — `vec3::near_zero()` — that returns true if the vector is very close to zero in all dimensions.

The following changes will use the C++ standard library function `std::fabs`, which returns the absolute value of its input. [^103a]

[^103a]: Rust version is `f64::abs`.

```rust-diff,norun,noplayground
{{ #git diff -U999 -h 157464117d578d98dfb765727c52c7b1e176dd22 e811cea593ba76384389c842fc0210124998c5e2 src/vec3.rs:[17,37:48,59] }}
```

**Listing 62:** [[vec3.rs](https://github.com/goldnor/code/blob/e811cea593ba76384389c842fc0210124998c5e2/src/vec3.rs)] *The vec3::near_zero() method*

<br>

```rust-diff,norun,noplayground
{{ #git diff -U999 -h e811cea593ba76384389c842fc0210124998c5e2 5230c29a2d01fd09e037b9c6c326ec4cd88ef791 src/material.rs:24: }}
```

**Listing 63:** [[material.rs](https://github.com/goldnor/code/blob/5230c29a2d01fd09e037b9c6c326ec4cd88ef791/src/material.rs)] *Lambertian scatter, bullet-proof*

<br>
