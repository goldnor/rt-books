# Positionable Camera

Cameras, like dielectrics, are a pain to debug, so I always develop mine incrementally. First, let’s allow for an adjustable field of view (*fov*). This is the visual angle from edge to edge of the rendered image. Since our image is not square, the fov is different horizontally and vertically. I always use vertical fov. I also usually specify it in degrees and change to radians inside a constructor — a matter of personal taste.
