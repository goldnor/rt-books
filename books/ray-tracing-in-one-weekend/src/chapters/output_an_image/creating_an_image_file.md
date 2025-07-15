## Creating an Image File

Because the file is written to the standard output stream, you'll need to redirect it to an image file. Typically this is done from the command-line by using the `>` redirection operator.

On Windows, you'd get the debug build from CMake running this command:[^22a]

[^22a]: We are not using CMake; we are relying on the standard cargo build tool.

```shell
cargo b
```

Then run your newly-built program like so:[^22b]

[^22b]: You can skip the building step since it is part of the run command (or `r` for short).

```shell
cargo r > image.ppm
```

Later, it will be better to run optimized builds for speed. In that case, you would build like this:

```shell
cargo b -r
```

and would run the optimized program like this:

```shell
cargo r -r
```

The examples above assume that you are building with CMake, using the same approach as the CMakeLists.txt file in the included source. Use whatever build environment (and language) you're most comfortable with.

On Mac or Linux, release build, you would launch the program like this:[^22c]

[^22c]: It is the same as for Windows.

```shell
cargo r > image.ppm
```

Complete building and running instructions can be found in the [project README](https://github.com/goldnor/rt-books/blob/main/README.md).

Opening the output file (in `ToyViewer` on my Mac, but try it in your favorite image viewer and Google “ppm viewer” if your viewer doesn’t support it) shows this result:

<img style="width: 100%; image-rendering: pixelated" src="../../imgs/img-1.01-first-ppm-image.png" alt="First PPM image">

**Image 1:** *First PPM image*

<br>

Hooray! This is the graphics “hello world”. If your image doesn’t look like that, open the output file in a text editor and see what it looks like. It should start something like this:

```ppm
P3
256 256
255
0 0 0
1 0 0
2 0 0
3 0 0
4 0 0
5 0 0
6 0 0
7 0 0
8 0 0
9 0 0
10 0 0
11 0 0
12 0 0
...
```
**Listing 2:** *First image output*

<br>

If your PPM file doesn't look like this, then double-check your formatting code. If it *does* look like this but fails to render, then you may have line-ending differences or something similar that is confusing your image viewer. To help debug this, you can find a file `test.ppm` in the `images` directory of the Github project. This should help to ensure that your viewer can handle the PPM format and to use as a comparison against your generated PPM file.

Some readers have reported problems viewing their generated files on Windows. In this case, the problem is often that the PPM is written out as UTF-16, often from PowerShell. If you run into this problem, see [Discussion 1114](https://github.com/RayTracing/raytracing.github.io/discussions/1114) for help with this issue.

If everything displays correctly, then you're pretty much done with system and IDE issues — everything in the remainder of this series uses this same simple mechanism for generated rendered images.

If you want to produce other image formats, I am a fan of `stb_image.h`, a header-only image library available on GitHub at [https://github.com/nothings/stb](https://github.com/nothings/stb).[^22d]

[^22d]: Rust crate alternativ: [https://crates.io/crates/stb_image](https://crates.io/crates/stb_image).
