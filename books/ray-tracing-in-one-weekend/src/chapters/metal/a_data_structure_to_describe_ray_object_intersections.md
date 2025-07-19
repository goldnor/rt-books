## A Data Structure to Describe Ray-Object Intersections

The `hit_record` is to avoid a bunch of arguments so we can stuff whatever info we want in there. You can use arguments instead of an encapsulated type, it’s just a matter of taste. Hittables and materials need to be able to reference the other's type in code so there is some circularity of the references. In C++ we add the line class material; to tell the compiler that material is a class that will be defined later. Since we're just specifying a pointer to the class, the compiler doesn't need to know the details of the class, solving the circular reference issue. [^102a]
[^102a]: In Rust traits needs to be imported using the `use` keyword. Circular references are not a problem in Rust, since the `use` keyword does *not* simply copy the header file in comparison to the C++ `include` macro.

```rust-diff,norun,noplayground
{{ #git diff -U999 9e2b407ffc6f22c19f7e3e60baf638c5797f15e2 c2bd376435e53106f8045a293708f0e5c0f2d549 src/hittable.rs:6:32 }}
```

**Listing 59:** [[hittable.rs](https://github.com/goldnor/code/blob/c2bd376435e53106f8045a293708f0e5c0f2d549/src/hittable.rs)] *Hit record with added material pointer* [^102b]

<br>

[^102b]: `HitRecord` can no longer derive `Default` since `Rc<dyn Material>` can not implement `Default`. Instead, the default material is set to a Lambertian.


`hit_record` is just a way to stuff a bunch of arguments into a class so we can send them as a group. When a ray hits a surface (a particular sphere for example), the material pointer in the `hit_record` will be set to point at the material pointer the sphere was given when it was set up in `main()` when we start. When the `ray_color()` routine gets the `hit_record` it can call member functions of the material pointer to find out what ray, if any, is scattered.

To achieve this, `hit_record` needs to be told the material that is assigned to the sphere.

```rust-diff,norun,noplayground
{{ #git diff -U999 -h c2bd376435e53106f8045a293708f0e5c0f2d549 157464117d578d98dfb765727c52c7b1e176dd22 src/sphere.rs:[11:25,26:32,52:] }}
```

**Listing 60:** [[sphere.rs](https://github.com/goldnor/code/blob/157464117d578d98dfb765727c52c7b1e176dd22/src/sphere.rs)] *Ray-sphere intersection with added material information*

<br>
