## Schlick Approximation

Now real glass has reflectivity that varies with angle — look at a window at a steep angle and it becomes a mirror. There is a big ugly equation for that, but almost everybody uses a cheap and surprisingly accurate polynomial approximation by Christophe Schlick. This yields our full glass material:

```rust-diff,norun,noplayground
{{ #git diff -U999 -h 4f80203ca763361671fcf137bb1e996c54954705 bdb3324e3e3a832a0aa528a60ecffb1a2b5eb862 src/material.rs:64: }}
```

**Listing 78:** [[material.rs](https://github.com/goldnor/code/blob/bdb3324e3e3a832a0aa528a60ecffb1a2b5eb862/src/material.rs)] *Full glass material*

<br>
