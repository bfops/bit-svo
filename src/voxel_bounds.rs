#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct VoxelBounds {
  /// x-coordinate as a multiple of 2^lg_size.
  pub x: i32,
  /// y-coordinate as a multiple of 2^lg_size.
  pub y: i32,
  /// z-coordinate as a multiple of 2^lg_size.
  pub z: i32,
  /// The log_2 of the voxel's size.
  pub lg_size: i16,
}

impl VoxelBounds {
  /// Convenience function to create `VoxelBounds`.
  /// N.B. That the input coordinates should be divided by (2^lg_size).
  pub fn new(x: i32, y: i32, z: i32, lg_size: i16) -> VoxelBounds {
    let ret =
      VoxelBounds {
        x: x,
        y: y,
        z: z,
        lg_size: lg_size,
      };
    ret
  }

  /// The width of this voxel.
  #[inline(always)]
  pub fn size(&self) -> f32 {
    if self.lg_size >= 0 {
      (1 << self.lg_size) as f32
    } else {
      1.0 / (1 << -self.lg_size) as f32
    }
  }
}
