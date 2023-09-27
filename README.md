# RustRayTracer
- A simple raytacer following Peter Shirley's [Raytracing In One Weekend](https://raytracing.github.io/).
- Features multi-threaded rendering using [rayon](https://crates.io/crates/rayon), a custom (albeit naive) HDR environment map solution, and support for a handful of common tonemappers.
- For better or for worse all memory used for rendering lives on the stack.

# Gallery
Basic diffusive material lit by a pretty sunset
<img src="https://github.com/DavJCosby/RustRayTracer/blob/master/raytracer/renders/first_sky.png?raw=true" width="75%">

Metallic materials with varying roughness
<img src="https://github.com/DavJCosby/RustRayTracer/blob/master/raytracer/renders/fuzzy-metal.png?raw=true" width="75%">

Colored dialectric glass material
<img src="https://github.com/DavJCosby/RustRayTracer/blob/master/raytracer/renders/blue-glass.png?raw=true" width="75%">

Final demo scene from the book
<img src="https://github.com/DavJCosby/RustRayTracer/blob/master/raytracer/renders/final-multithreaded.png?raw=true" width="75%">
<br>*That glass shader is definitely wrong...*

Same scene with an environment map applied
<img src="https://github.com/DavJCosby/RustRayTracer/blob/master/raytracer/renders/final-env-multithreaded.png?raw=true" width="75%">

Couldn't figure out how to implement importance sampling for the enviroment maps, so small and bright areas tend to create tons of fireflies :(
<img src="https://github.com/DavJCosby/RustRayTracer/blob/master/raytracer/renders/fireflies.png?raw=true" width="75%">
