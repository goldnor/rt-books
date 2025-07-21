## Next Steps

You now have a cool ray tracer! What next?

### Book 2: Ray Tracing: The Next Week

The second book in this series builds on the ray tracer you've developed here. This includes new features such as:

* Motion blur — Realistically render moving objects.
* Bounding volume hierarchies — speeding up the rendering of complex scenes.
* Texture maps — placing images on objects.
* Perlin noise — a random noise generator very useful for many techniques.
* Quadrilaterals — something to render besides spheres! Also, the foundation to implement disks, triangles, rings or just about any other 2D primitive.
* Lights — add sources of light to your scene.
* Transforms — useful for placing and rotating objects.
* Volumetric rendering — render smoke, clouds and other gaseous volumes.

### Book 3: Ray Tracing: The Rest of Your Life

This book expands again on the content from the second book. A lot of this book is about improving both the rendered image quality and the renderer performance, and focuses on generating the *right rays* and accumulating them appropriately.

This book is for the reader seriously interested in writing professional-level ray tracers, and/or interested in the foundation to implement advanced effects like subsurface scattering or nested dielectrics.

### Other Directions

There are so many additional directions you can take from here, including techniques we haven't (yet?) covered in this series. These include:

**Triangles** — Most cool models are in triangle form. The model I/O is the worst and almost everybody tries to get somebody else’s code to do this. This also includes efficiently handling large meshes of triangles, which present their own challenges.

**Parallelism** — Run \\( N \\) copies of your code on \\( N \\) cores with different random seeds. Average the \\( N \\) runs. This averaging can also be done hierarchically where \\( N/2 \\) pairs can be averaged to get \\( N/4 \\) images, and pairs of those can be averaged. That method of parallelism should extend well into the thousands of cores with very little coding.

**Shadow Rays** — When firing rays at light sources, you can determine exactly how a particular point is shadowed. With this, you can render crisp or soft shadows, adding another degreee of realism to your scenes.

Have fun, and please send me your cool images!
