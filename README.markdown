[![Build Status](https://travis-ci.org/bfops/bit-svo.svg?branch=master)](https://travis-ci.org/bfops/bit-svo)

`bit-svo` is a Rust sparse voxel octree implementation. It doesn't have a whole lot,
but features will be added as [Playform](https://github.com/bfops/playform) grows.

If you feel like something's missing from here, poke me/file an issue/submit a PR!

Things to note:

  * Voxel widths are exponents of 2.
  * Voxel positions are expressed as multiples of their widths, so that the positions can always be integers. This means that voxels positions are aligned to their widths.
  * The tree width is an exponent of 2 (the same in all dimensions).
  * The tree is always centered around (0, 0, 0).
  * The tree expands dynamically on insert.
  * The benches run at around `1us`/insert through 30 levels of depth (world width of `2^30` voxels).
