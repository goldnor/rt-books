## Some New C++ Features [^66a]

[^66a]: This chapter can be safely skipped when the code of last chapter is clear.

The `hittable_list` class code uses some C++ features that may trip you up if you're not normally a C++ programmer: `vector`, `shared_ptr`, and `make_shared`. [^66b]

[^66b]: The Rust equivalents are `Vec`, `Rc` and a simple `new` method of the reference counting smart pointer.

 <!-- It is possible to use `Arc` instead for `Rc` if using multithreading and insert a `Cell` type to allow for interior mutability, but since the scene is setup once and does not change over the course of the program lifetime a simple `Rc` is sufficient. -->

`shared_ptr<type>` is a pointer to some allocated type, with reference-counting semantics. Every time you assign its value to another shared pointer (usually with a simple assignment), the reference count is incremented. As shared pointers go out of scope (like at the end of a block or function), the reference count is decremented. Once the count goes to zero, the object is safely deleted. [^66c]

[^66c]: Here we use `Rc`. In contrast to the C++ `shared_ptr`, the contained value of an `Rc` is inmutable. Enclosing the value with a cell based type like for example `RefCell` would allow for interior mutability. However in this case, all objects are created on startup and do not change over the course of the program lifetime which is why a simple `Rc` is sufficient.

Typically, a shared pointer is first initialized with a newly-allocated object, something like this:

```rust,norun,nodiff
let double_ptr: Rc<double> = Rc::new(0.37);
let vec3_ptr: Rc<Vec3> = Rc::new(Vec3::new(1.414214, 2.718281, 1.618034));
let sphere_ptr: Rc<Sphere> = Rc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0));
```

**Listing 22:** *An example allocation using shared_ptr*

<br>

`make_shared<thing>(thing_constructor_params ...)` allocates a new instance of type thing, using the constructor parameters. It returns a `shared_ptr<thing>`. [^66d]

[^66d]: Rust has no constructors, instead it is convention to fill structs with the `new` method, the implementation of traits like `Default`, `From` or similar, or to use any method that returns `Self`. So technically something like `make_shared` does not exist in Rust.

Since the type can be automatically deduced by the return type of `make_shared<type>(...)`, the above lines can be more simply expressed using C++'s `auto` type specifier: [^66e]

[^66e]: The type annotations in the last listing were not nessecary, Rust's `let` declarations can in this case infere the types.

```rust,norun,nodiff
let double_ptr = Rc::new(0.37);
let vec3_ptr = Rc::new(Vec3::new(1.414214, 2.718281, 1.618034));
let sphere_ptr = Rc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0));
```

**Listing 23:**  *An example allocation using shared_ptr with auto type*

<br>

We'll use shared pointers in our code, because it allows multiple geometries to share a common instance (for example, a bunch of spheres that all use the same color material), and because it makes memory management automatic and easier to reason about.

`std::shared_ptr` is included with the `<memory>` header.[^66f]

[^66f]: `Rc` is found in `std::rc::Rc`.

The second C++ feature you may be unfamiliar with is `std::vector`. This is a generic array-like collection of an arbitrary type. Above, we use a collection of pointers to `hittable`. `std::vector` automatically grows as more values are added: `objects.push_back(object)` adds a value to the end of the `std::vector` member variable `objects`.

`std::vector` is included with the ``<vector>` header. [^66g]

[^66g]: `Vec` is included in Rust's std prelude and does have to be included to be used.

Finally, the `using` statements in [listing 21](a_list_of_hittable_objects.md) tell the compiler that we'll be getting `shared_ptr` and `make_shared` from the `std` library, so we don't need to prefix these with `std::` every time we reference them.
