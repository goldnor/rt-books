## An Interval Class

Before we continue, we'll implement an interval class to manage real-valued intervals with a minimum and a maximum. We'll end up using this class quite often as we proceed.

```rust,norun,noplayground
{{ #git show b97cce927b5c04bc925b77462cef0a4ce6152d4a:src/interval.rs }}
```

**Listing 31:** [[interval.rs](TODO)] *Introducing the new interval class*

<br>

```rust-diff,norun,noplayground
{{ #git diff -h -U999 b97cce927b5c04bc925b77462cef0a4ce6152d4a 7fc2aa1432889497ecb73854719905b0ed276a68 src/prelude.rs:16:20 }}
```

**Listing 32:** [[prelude.rs](TODO)] *Including the new interval class*

<br>

```rust-diff,norun,noplayground
{{ #git diff -h -U999 7fc2aa1432889497ecb73854719905b0ed276a68 d75c2cf12d1747e49e2568b2d4718ea5efea86e5 src/hittable.rs:[29,31:] }}
```

**Listing 33:** [[hittable.rs](TODO)] *hittable::hit() using interval*

<br>

```rust-diff,norun,noplayground
{{ #git diff -h -U999 7fc2aa1432889497ecb73854719905b0ed276a68 d75c2cf12d1747e49e2568b2d4718ea5efea86e5 src/hittable_list.rs:[29,31:] }}
```

**Listing 34:** [[hittable_list.rs](TODO)] *hittable_list::hit() using interval*

<br>

```rust-diff,norun,noplayground
{{ #git diff -h -U999 7fc2aa1432889497ecb73854719905b0ed276a68 d75c2cf12d1747e49e2568b2d4718ea5efea86e5 src/sphere.rs:[25:28,39:51,62:] }}
```

**Listing 35:** [[sphere.rs](TODO)] *sphere using interval*

<br>

```rust-diff,norun,noplayground
{{ #git diff -h -U999 7fc2aa1432889497ecb73854719905b0ed276a68 d75c2cf12d1747e49e2568b2d4718ea5efea86e5 src/main.rs:[7:17] }}
```

**Listing 36:** [[main.rs](TODO)] *The new main using interval*

<br>
