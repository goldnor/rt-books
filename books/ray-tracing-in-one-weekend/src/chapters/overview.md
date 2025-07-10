# Overview

I’ve taught many graphics classes over the years. Often I do them in ray tracing, because you are forced to write all the code, but you can still get cool images with no API. I decided to adapt my course notes into a how-to, to get you to a cool program as quickly as possible. It will not be a full-featured ray tracer, but it does have the indirect lighting which has made ray tracing a staple in movies. Follow these steps, and the architecture of the ray tracer you produce will be good for extending to a more extensive ray tracer if you get excited and want to pursue that.

When somebody says “ray tracing” it could mean many things. What I am going to describe is technically a path tracer, and a fairly general one. While the code will be pretty simple (let the computer do the work!) I think you’ll be very happy with the images you can make.

I’ll take you through writing a ray tracer in the order I do it, along with some debugging tips. By the end, you will have a ray tracer that produces some great images. You should be able to do this in a weekend. If you take longer, don’t worry about it. I use C++ as the driving language, but you don’t need to.[^1a] However, I suggest you do, because it’s fast, portable, and most production movie and video game renderers are written in C++. Note that I avoid most “modern features” of C++, but inheritance and operator overloading are too useful for ray tracers to pass on.[^1b]

[^1a]: There is where Rust comes into play - a fast, reliable language with better ergonomics. This is an unbiased opinion of course \*caugh\*.

[^1b]: Inheritance is not supported in Rust. In many places simple composition and traits will do the trick.

I do not provide the code online, but the code is real and I show all of it except for a few straightforward operators in the vec3 class. I am a big believer in typing in code to learn it, but when code is available I use it, so I only practice what I preach when the code is not available. So don’t ask!
I have left that last part in because it is funny what a 180 I have done. Several readers ended up with subtle errors that were helped when we compared code. So please do type in the code, but you can find the finished source for each book in the [RayTracing project on GitHub](TODO).

A note on the implementing code for these books — our philosophy for the included code prioritizes the following goals:

- The code should implement the concepts covered in the books.
- We use C++, but as simple as possible. Our programming style is very C-like, but we take advantage of modern features where it makes the code easier to use or understand.
- Our coding style continues the style established from the original books as much as possible, for continuity.
- Line length is kept to 96 characters per line, to keep lines consistent between the codebase and code listings in the books.[^1c]

[^1c]: [rustfmt](https://github.com/rust-lang/rustfmt) will be used instead, as it is shipped with most IDEs.

The code thus provides a baseline implementation, with tons of improvements left for the reader to enjoy. There are endless ways one can optimize and modernize the code; we prioritize the simple solution.

We assume a little bit of familiarity with vectors (like dot product and vector addition). If you don’t know that, do a little review. If you need that review, or to learn it for the first time, check out the online *[Graphics Codex](https://graphicscodex.com/)* by Morgan McGuire, *Fundamentals of Computer Graphics* by Steve Marschner and Peter Shirley, or *Computer Graphics: Principles and Practice* by J.D. Foley and Andy Van Dam.

See the [project README](TODO) file for information about this project, the repository on GitHub, directory structure, building & running, and how to make or reference corrections and contributions.

See our [Further Reading wiki page](TODO) for additional project related resources.

These books have been formatted to print well directly from your browser. We also include PDFs of each book [with each release](TODO), in the “Assets” section.

If you want to communicate with us, feel free to send us an email at:

- Peter Shirley, [ptrshrl@gmail.com](mailto:ptrshrl@gmail.com)
- Steve Hollasch, [steve@hollasch.net](mailto:steve@hollasch.net)
- Trevor David Black, [trevordblack@trevord.black](mailto:trevordblack@trevord.black)

Finally, if you run into problems with your implementation, have general questions, or would like to share your own ideas or work, see the GitHub Discussions forum on the GitHub project.

Thanks to everyone who lent a hand on this project. You can find them in the acknowledgments section at the end of this book.

Let’s get on with it!
