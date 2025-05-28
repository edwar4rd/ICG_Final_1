# Ray Tracing in One Weekend

This repository contains an implementation of a simple ray tracer in Rust, following the tutorial [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

![final image of the project](images/final-image.png)

## Features

- Basic path tracing engine
- Sphere and only spheres
- Diffuse and reflective materials
- Parallel rendering with Rayon (`--features rayon`)

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- Cargo (comes with a default Rust installation)

### Build & Run

Use the `--release` flag (and probably also lower the sample count per pixel) or you'll wait ages.

```bash
git clone https://github.com/edwar4rd/ICG_Final_1
cd ICG_Final_1
cargo run --release > image.ppm
```

The output image will be saved as `image.ppm`.
Use something like `ffmpeg` to turn that to something more portable.

## Credits

- Based on the tutorial by Peter Shirley
- README and assisted by GitHub Copilot

## Reference

- [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) by Peter Shirley

## License

This project is released under the [CC0 1.0 Universal (Public Domain Dedication)](https://creativecommons.org/publicdomain/zero/1.0/). You can copy, modify, and distribute this work, even for commercial purposes, without asking permission.

I think this license is appropriate since I didn't did much work outside of translating everything to Rust, and the original tutorial is also in CC0.
