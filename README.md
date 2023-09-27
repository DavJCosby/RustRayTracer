# RustRayTracer
A simple raytacer following Peter Shirley's [Raytracing In One Weekend](https://raytracing.github.io/). Features multi-threaded rendering using [rayon](https://crates.io/crates/rayon), a custom (albeit naive) HDR environment map solution, and support for a handful of common tonemappers. For better or for worse, all memory used for rendering lives on the stack.

# Gallery
Basic diffusive material lit by a pretty sunset <br>
<img src="https://github.com/DavJCosby/RustRayTracer/blob/master/raytracer/renders/first_sky.png?raw=true" width="75%">

Metallic materials with varying roughness <br>
<img src="https://github.com/DavJCosby/RustRayTracer/blob/master/raytracer/renders/fuzzy-metal.png?raw=true" width="75%">

Colored dialectric glass material <br>
<img src="https://github.com/DavJCosby/RustRayTracer/blob/master/raytracer/renders/blue-glass.png?raw=true" width="75%">

Final demo scene from the book <br>
<img src="https://github.com/DavJCosby/RustRayTracer/blob/master/raytracer/renders/final-multithreaded.png?raw=true" width="75%">
<br>*That glass shader is definitely wrong...*

Same scene with an environment map applied <br>
<img src="https://github.com/DavJCosby/RustRayTracer/blob/master/raytracer/renders/final-env-multithreaded.png?raw=true" width="75%">

Couldn't figure out how to implement importance sampling for the enviroment maps, so small and bright areas tend to create tons of fireflies :( <br>
<img src="https://github.com/DavJCosby/RustRayTracer/blob/master/raytracer/renders/fireflies.png?raw=true" width="75%">
