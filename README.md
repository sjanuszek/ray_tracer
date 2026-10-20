Ray Tracer
==========

A Rust path tracer inspired by Peter Shirley's *Ray Tracing in One Weekend* series. It renders 3D scenes with materials, textures, lighting, depth of field, motion blur, participating media, and BVH acceleration. Rendering is parallelized with Rayon.

Requirements
------------
- Rust and Cargo
- `earthmap.jpg` in the working directory when rendering scene 2

Build and Run
-------------
Run with four arguments: scene, image width, samples per pixel, and maximum ray depth.

    cargo run --release -- <scene> <width> <samples> <depth> > image.ppm

Example:

    cargo run --release -- 6 400 100 50 > image.ppm

The image is written as PPM (P3) to standard output; progress is printed to standard error. Convert `image.ppm` to PNG with an image tool if needed.

Scenes
------
0. Bouncing spheres
1. Checkered spheres
2. Earth
3. Perlin-noise spheres
4. Quads
5. Simple light
6. Cornell box
7. Cornell smoke

The numeric arguments are required. Rendering time depends on image width, sample count, ray depth, and hardware.

