# Some Random Number Utilities [^81a]

[^81a]: The utility functions, such as `degrees_to_radians`, were not mentioned in [chapter 6.7](../surface_normals_and_multiple_objects/common_constants_and_utility_functions.md) since Rust already provides this functionality by default. The utility functions for randomness described in this chapter will also not be included in the code, as they only wrap functions implemented in the Rust [rand crate](https://crates.io/crates/rand) (see [listing 42](#l42)).

We're going to need a random number generator that returns real random numbers. This function should return a canonical random number, which by convention falls in the range \\( 0 \le n < 1 \\). The “less than” before the \\( 1 \\) is important, as we will sometimes take advantage of that.

A simple approach to this is to use the `std::rand()` function that can be found in `<cstdlib>`, which returns a random integer in the range `0` and `RAND_MAX`. Hence we can get a real random number as desired with the following code snippet, added to `rtweekend.h`:

```rust,norun,noplayground
// Utility Functions

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees.to_radians()
}

#[inline]
pub fn random_double() -> f64 {
    // Returns a random real in [0,1).
    rand::random::<i32>() as f64 / (i32::MAX as f64 + 1.0)
}

#[inline]
pub fn random_double_range(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max).
    min + (max - min) * random_double()
}
```

**Listing 41:** [[prelude.rs](https://github.com/goldnor/code/blob/48d27302c1d726208b8f952050bcca45e53dc756/src/prelude.rs)] *random_double() functions*

C++ did not traditionally have a standard random number generator, but newer versions of C++ have addressed this issue with the `<random>` header (if imperfectly according to some experts). [^81b] If you want to use this, you can obtain a random number with the conditions we need as follows:

[^81b]: As far as I am aware, the Rust implementation is perfectly fine.

<a name="l42"></a>
```rust,norun,noplayground
// Utility Functions

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees.to_radians()
}

#[inline]
pub fn random_double() -> f64 {
    // Returns a random real in [0,1).
    rand::random()
}

#[inline]
pub fn random_double_range(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max).
    rand::random_range(min..max)
}
```

**Listing 42:** [[prelude.rs](https://github.com/goldnor/code/blob/48d27302c1d726208b8f952050bcca45e53dc756/src/prelude.rs)] *random_double() functions*
