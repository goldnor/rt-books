## Adding a Progress Indicator

Before we continue, let's add a progress indicator to our output. This is a handy way to track the progress of a long render, and also to possibly identify a run that's stalled out due to an infinite loop or other problem.

Our program outputs the image to the standard output stream (`std::cout`), so leave that alone and instead write to the logging output stream (`std::clog`):[^23a]

[^23a]: The [log crate](https://crates.io/crates/log) with the [env_logger](https://crates.io/crates/env_logger) implementation is a good alternative to `std::clog`. Run `RUST_LOG=info cargo r > image.ppm` for the log output.

```rust-diff,norun,noplayground
{{ #git diff -U99 c2b88fe22dadf9cba6dff369ee0b1834dd9733d4 5c4dfec5c8122b0a6df768027866c4a526da8677 src/main.rs:13:33 }}
```

**Listing 3:** [[main.rs](https://github.com/goldnor/code/blob/5c4dfec5c8122b0a6df768027866c4a526da8677/src/main.rs)] *Main render loop with progress reporting*

<br>

Now when running, you'll see a running count of the number of scanlines remaining. Hopefully this runs so fast that you don't even see it! Don't worry — you'll have lots of time in the future to watch a slowly updating progress line as we expand our ray tracer.
