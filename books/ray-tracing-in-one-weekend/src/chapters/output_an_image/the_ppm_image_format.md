## The PPM Image Format
Whenever you start a renderer, you need a way to see an image. The most straightforward way is to write it to a file. The catch is, there are so many formats. Many of those are complex. I always start with a plain text ppm file. Here’s a nice description from Wikipedia:

![PPM Example](../../imgs/fig-1.01-ppm.jpg)
**Figure 1:** *PPM Example*

Let’s make some C++ code to output such a thing:[^21a]

[^21a]: It is Rust code of course. This won't be annotated anymore.

```rust
{{ #git show c2b88fe22dadf9cba6dff369ee0b1834dd9733d4:src/main.rs }}
```
**Listing 1:** [[main.rs](TODO)] *Creating your first image*

There are some things to note in this code:

1. The pixels are written out in rows.
2. Every row of pixels is written out left to right.
3. These rows are written out from top to bottom.
4. By convention, each of the red/green/blue components are represented internally by real-valued variables that range from 0.0 to 1.0. These must be scaled to integer values between 0 and 255 before we print them out.
5. Red goes from fully off (black) to fully on (bright red) from left to right, and green goes from fully off at the top (black) to fully on at the bottom (bright green). Adding red and green light together make yellow so we should expect the bottom right corner to be yellow.
