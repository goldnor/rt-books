## Generating Pixels with Multiple Samples

For a single pixel composed of multiple samples, we'll select samples from the area surrounding the pixel and average the resulting light (color) values together.

First we'll update the `write_color()` function to account for the number of samples we use: we need to find the average across all of the samples that we take. To do this, we'll add the full color from each iteration, and then finish with a single division (by the number of samples) at the end, before writing out the color. To ensure that the color components of the final result remain within the proper \\( [0,1] \\) bounds, we'll add and use a small helper function: `interval::clamp(x)`. [^82a] [^82b]

[^82a]: For this purpose, version 1.50 of Rust introduced the `f64::clamp(self, min, max)` [function](https://doc.rust-lang.org/std/primitive.f64.html#method.clamp) (the C++17 standard introduced a similar [function](https://en.cppreference.com/w/cpp/algorithm/clamp.html) called `std::clamp(v, lo, hi)`).

[^82b]: The function is const, meaning it can be used to initialise const variables. This will be demonstrated in the next listing.

```rust-diff,norun,noplayground
{{ #git diff -h -U999 04fbc211149bf6fdbd4eba19a23449a2e687c616 c2937087b619981d108b6774b790af1d59316329 src/interval.rs:[17,40:] }}
```

**Listing 43:** [[interval.rs](https://github.com/goldnor/code/blob/c2937087b619981d108b6774b790af1d59316329/src/interval.rs)] *The interval::clamp() utility function*

<br>

Here's the updated `write_color()` function that incorporates the interval clamping function:

```rust-diff,norun,noplayground
{{ #git diff -h -U999 c2937087b619981d108b6774b790af1d59316329 f9b02376bc5c4d4f7337d6fef6903506086925da src/color.rs }}
```

**Listing 44:** [[color.rs](https://github.com/goldnor/code/blob/f9b02376bc5c4d4f7337d6fef6903506086925da/src/color.rs)] *The multi-sample write_color() function*

<br>

Now let's update the camera class to define and use a new `camera::get_ray(i,j)` function, which will generate different samples for each pixel. This function will use a new helper function `sample_square()` that generates a random sample point within the unit square centered at the origin. We then transform the random sample from this ideal square back to the particular pixel we're currently sampling.

```rust-diff,norun,noplayground
{{ #git diff -h -U999 f9b02376bc5c4d4f7337d6fef6903506086925da 8f64151623f1e58e249d00f11a8cac23523fec1b src/camera.rs:[7:46,57:105,123:149,154,158,162:] }}
```

**Listing 45:** [[camera.rs](https://github.com/goldnor/code/blob/8f64151623f1e58e249d00f11a8cac23523fec1b/src/camera.rs)] *Camera with samples-per-pixel parameter*

<br>

(In addition to the new `sample_square()` function above, you'll also find the function `sample_disk()` in the Github source code. This is included in case you'd like to experiment with non-square pixels, but we won't be using it in this book. sample_disk() depends on the function random_in_unit_disk() which is defined later on.)

Main is updated to set the new camera parameter.

```rust-diff,norun,noplayground
{{ #git diff -h -U999 8f64151623f1e58e249d00f11a8cac23523fec1b d4b3a2498a7ed1a3a51ade2c425683a7ff4ad018 src/main.rs:[7,14:] }}
```

**Listing 46:** [[main.rs](https://github.com/goldnor/code/blob/d4b3a2498a7ed1a3a51ade2c425683a7ff4ad018/src/main.rs)] *Setting the new samples-per-pixel parameter*

<br>

Zooming into the image that is produced, we can see the difference in edge pixels.

<img style="width: 100%; image-rendering: pixelated" src="../../imgs/img-1.06-antialias-before-after.png" alt="Before and after antialiasing">
<!-- ![x](../../imgs/img-1.06-antialias-before-after.png) -->

**Image 6:** *Before and after antialiasing*

<br>



