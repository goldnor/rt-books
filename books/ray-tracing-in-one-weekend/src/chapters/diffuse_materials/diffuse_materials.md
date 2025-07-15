# Diffuse Materials

Now that we have objects and multiple rays per pixel, we can make some realistic looking materials. We’ll start with diffuse materials (also called *matte*).

One question is whether we mix and match geometry and materials (so that we can assign a material to multiple spheres, or vice versa) or if geometry and materials are tightly bound (which could be useful for procedural objects where the geometry and material are linked). We’ll go with separate — which is usual in most renderers — but do be aware that there are alternative approaches.
