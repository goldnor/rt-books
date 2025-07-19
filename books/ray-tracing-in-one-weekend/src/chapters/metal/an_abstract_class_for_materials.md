## An Abstract Class for Materials

If we want different objects to have different materials, we have a design decision. We could have a universal material type with lots of parameters so any individual material type could just ignore the parameters that don't affect it. This is not a bad approach. Or we could have an abstract material class that encapsulates unique behavior. I am a fan of the latter approach. For our program the material needs to do two things:

1. Produce a scattered ray (or say it absorbed the incident ray).
2. If scattered, say how much the ray should be attenuated.

This suggests the abstract class:

```rust,norun,noplayground
{{ #git show 9e2b407ffc6f22c19f7e3e60baf638c5797f15e2:src/material.rs }}
```

**Listing 58:** [[material.rs](https://github.com/goldnor/code/blob/9e2b407ffc6f22c19f7e3e60baf638c5797f15e2/src/material.rs)] *The material class*

<br>
