## An Abstraction for Hittable Objects

Now, how about more than one sphere? While it is tempting to have an array of spheres, a very clean solution is to make an “abstract class” for anything a ray might hit, and make both a sphere and a list of spheres just something that can be hit. What that class should be called is something of a quandary — calling it an “object” would be good if not for “object oriented” programming. “Surface” is often used, with the weakness being maybe we will want volumes (fog, clouds, stuff like that). “hittable” emphasizes the member function that unites them. I don’t love any of these, but we'll go with “hittable”.

This `hittable` abstract class will have a `hit` function that takes in a ray. [^63a] Most ray tracers have found it convenient to add a valid interval for hits \\( t_{min} \\) to \\( t_{max} \\), so the hit only “counts” if \\( t_{min} < t < t_{max} \\). For the initial rays this is positive \\( t \\), but as we will see, it can simplify our code to have an interval \\( t_{min} \\) to \\( t_{max} \\). One design question is whether to do things like compute the normal if we hit something. We might end up hitting something closer as we do our search, and we will only need the normal of the closest thing. I will go with the simple solution and compute a bundle of stuff I will store in some structure. Here’s the abstract class:

[^63a]: A simple Rust Trait will be used instead.

```rust,norun,noplayground
{{ #git show 1791ddbea0fdc81c13e4ef3daaaca7e440cec404:src/hittable.rs }}
```

**Listing 15:** [[hittable.rs](TODO)] *The hittable class*

<br>

And here’s the sphere:

```rust,norun,noplayground
{{ #git show 36e5e545d452e57637150a4e1a8ba2ff93aee52b:src/sphere.rs }}
```

**Listing 16:** [[sphere.rs](TODO)] *The sphere class*

<br>

(Note here that we use the C++ standard function `std::fmax()`, which returns the maximum of the two floating-point arguments. Similarly, we will later use `std::fmin()`, which returns the minimum of the two floating-point arguments.) [^63b]

[^63b]: The Rust standard library provides the functions `f64::max()` and `f64::min()`.
