# Straal
Straal is a rust-based linear algebra library, mainly aimed at applications in rendering and physics.
In the future, there may be additions to also do other types of calculations.

## Supported Types:
### Vectors of various sizes, both normal and Integer variants:
* `(I)Vec2-4` in i32, i64, f32 and f64
### Square Matrices of Various sizes:
* `Mat2-4` in f32 and f64
### Quaternions
* `Quat` in f32 and f64

## Features:
* In an attempt to follow GLSL, all "standard" operations are done through operator overloads
* Very easy to use with Glium
* Matrices have a few extra operations, such as transpose, determinant, adjoint, and inverse.
* Matrices also have a few ease of use functions to make rotations and translations much easier to work with.
* Lots of cross-casts are offered through the `From` trait, to easily get from one type to another

## Things to watch out for:
* Just like GLSL, multiplying 2 vectors with the `*` operator, does not give the dot product, but a component-wise product.
* The matrices are row-major, this is not the same as GLSL, uniforms are automatically converted to column major as they are passed into OpenGL, but be careful with it.

## To Do:
* A bunch of built-in opengl functions, like reflect, refract, mix, clamp, etc.
* Complex Number type
* Potentially a Geometry (primitives, tests, etc.) component.
