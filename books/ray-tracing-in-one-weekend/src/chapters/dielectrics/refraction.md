## Refraction

The hardest part to debug is the refracted ray. I usually first just have all the light refract if there is a refraction ray at all. For this project, I tried to put two glass balls in our scene, and I got this (I have not told you how to do this right or wrong yet, but soon!):

<img style="width: 100%; image-rendering: pixelated" src="../../imgs/img-1.15-glass-first.png" alt="Glass first">

**Image 15:** *Glass first*

<br>

Is that right? Glass balls look odd in real life. But no, it isn’t right. The world should be flipped upside down and no weird black stuff. I just printed out the ray straight through the middle of the image and it was clearly wrong. That often does the job.

