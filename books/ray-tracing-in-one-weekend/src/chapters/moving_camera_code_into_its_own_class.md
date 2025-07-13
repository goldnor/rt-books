# Moving Camera Code Into Its Own Class

Before continuing, now is a good time to consolidate our camera and scene-render code into a single new class: the `camera` class. The camera class will be responsible for two important jobs:

1. Construct and dispatch rays into the world.
2. Use the results of these rays to construct the rendered image.

In this refactoring, we'll collect the `ray_color()` function, along with the image, camera, and render sections of our main program. The new camera class will contain two public methods `initialize()` and `render()`, plus two private helper methods `get_ray()` and `ray_color()`.

Ultimately, the camera will follow the simplest usage pattern that we could think of: it will be default constructed no arguments, then the owning code will modify the camera's public variables through simple assignment, and finally everything is initialized by a call to the `initialize()` function. This pattern is chosen instead of the owner calling a constructor with a ton of parameters or by defining and calling a bunch of setter methods. Instead, the owning code only needs to set what it explicitly cares about. Finally, we could either have the owning code call `initialize()`, or just have the camera call this function automatically at the start of `render()`. We'll use the second approach. [^7a]

[^7a]: The idiomatic Rust solution for this type of problem is the builder pattern. Important parameters can be set either directly with struct access or via a convenient method chain in the owning code.

After main creates a camera and sets default values, it will call the `render()` method. The `render()` method will prepare the `camera` for rendering and then execute the render loop.

Here's the skeleton of our new `camera` class:

```rust,norun,noplayground
{{ #git show 0c76a7edbf70d48e535178b754a87bbb6571559b:src/camera.rs:[34:36,37,39:42,43,45:48,49,70:73,77,98:101,104,108:] }}
```

**Listing 37:** [[camera.rs](https://github.com/goldnor/code/blob/0c76a7edbf70d48e535178b754a87bbb6571559b/src/camera.rs)] *The camera class skeleton*

<br>

To begin with, let's fill in the `ray_color()` function from `main.cc`:

```rust,norun,noplayground
{{ #git show 0c76a7edbf70d48e535178b754a87bbb6571559b:src/camera.rs:[34,100:] }}
```

**Listing 38:** [[camera.rs](https://github.com/goldnor/code/blob/0c76a7edbf70d48e535178b754a87bbb6571559b/src/camera.rs)] *The camera::ray_color function*

<br>

Now we move almost everything from the `main()` function into our new camera class. The only thing remaining in the `main()` function is the world construction. Here's the camera class with newly migrated code:

```rust,norun,noplayground
{{ #git show 0c76a7edbf70d48e535178b754a87bbb6571559b:src/camera.rs:[:101,104,108:] }}
```

**Listing 39:** [[camera.rs](https://github.com/goldnor/code/blob/0c76a7edbf70d48e535178b754a87bbb6571559b/src/camera.rs)] *The working camera class*

<br>

And here's the much reduced main:

```rust-diff,norun,noplayground
{{ #git diff -U999 d75c2cf12d1747e49e2568b2d4718ea5efea86e5 0c76a7edbf70d48e535178b754a87bbb6571559b src/main.rs:[7,11,15:19,32:37,59,77:] }}
```

**Listing 40:** [[main.rs](https://github.com/goldnor/code/blob/0c76a7edbf70d48e535178b754a87bbb6571559b/src/main.rs)] *The new main, using the new camera*

<br>

Running this newly refactored program should give us the same rendered image as before.



