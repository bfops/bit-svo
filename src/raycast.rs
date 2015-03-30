use std::cmp::Ordering;

use super::{TreeBody, Branches, VoxelBounds};

// Time-of-intersection. Implements `Ord` for sanity reasons;
// let's hope the floating-points are all valid.
#[derive(Copy, Debug, PartialEq, PartialOrd)]
struct TOI(pub f32);

impl Eq for TOI {}

impl Ord for TOI {
  fn cmp(&self, other: &TOI) -> Ordering {
    self.partial_cmp(other).unwrap()
  }
}

#[derive(Debug, Copy)]
/// Information about a ray entering a voxel.
pub struct Entry {
  /// Index of a side of a rectangular-prismic voxel.
  side: usize,
  // (Roughly) when the side was intersected.
  toi: TOI,
}

impl Entry {
  pub fn from_exit(exit: Exit) -> Entry {
    Entry {
      side: 
        if exit.side < 3 {
          exit.side + 3
        } else {
          exit.side - 3
        },
      toi: exit.toi,
    }
  }
}
 
#[derive(Debug, Copy)]
/// Information about a ray exit a voxel.
pub struct Exit {
  /// Index of a side of a rectangular-prismic voxel.
  side: usize,
  // (Roughly) when the side was intersected.
  toi: TOI,
}

// TODO: Audit all the divisions for divide-by-zeros.

pub fn cast_ray_branches<'a, T, MakeBounds>(
  this: &'a Branches<T>,
  origin: [f32; 3],
  direction: [f32; 3],
  mut entry: Option<Entry>,
  mut coords: [usize; 3],
  make_bounds: &mut MakeBounds,
) -> Result<(VoxelBounds, &'a T), Exit>
  where MakeBounds: FnMut([usize; 3]) -> VoxelBounds,
{
  loop {
    let child = this.get(coords[0], coords[1], coords[2]);
    let bounds = make_bounds(coords);

    match cast_ray(child, origin, direction, bounds, entry) {
      Ok(r) => return Ok(r),
      Err(exit) => {
        let dim = exit.side % 3;
        if direction[dim] < 0.0 {
          if coords[dim] == 0 {
            return Err(exit)
          }
          coords[dim] = 0;
        } else {
          if coords[dim] == 1 {
            return Err(exit)
          }
          coords[dim] = 1;
        }
        entry = Some(Entry::from_exit(exit));
      },
    }
  }
}

/// Precondition: the ray passes through `this`.
pub fn cast_ray<'a, T>(
  this: &'a TreeBody<T>,
  origin: [f32; 3],
  direction: [f32; 3],
  bounds: VoxelBounds,
  entry: Option<Entry>,
) -> Result<(VoxelBounds, &'a T), Exit> {
  match this {
    &TreeBody::Empty => {
      let sides = [
        bounds.x,
        bounds.y,
        bounds.z,
        bounds.x + 1,
        bounds.y + 1,
        bounds.z + 1,
      ];

      let next_toi = |(side, &bound): (usize, &i32)| {
        let dim = side % 3;
        let bound = bound as f32 * bounds.size();
        if direction[dim] == 0.0 {
          None
        } else {
          let toi = (bound - origin[dim]) / direction[dim];
          if entry.map(|entry| entry.toi.0 <= toi).unwrap_or(toi >= 0.0) {
            Some(Exit {
              side: side,
              toi: TOI(toi),
            })
          } else {
            None
          }
        }
      };

      let exit =
        match entry {
          None =>
            sides.iter()
            .enumerate()
            .filter_map(next_toi)
            .min_by(|&exit| exit.toi).unwrap(),
          Some(entry) =>
            sides.iter()
            .enumerate()
            .filter(|&(i, _)| i != entry.side)
            .filter_map(next_toi)
            .min_by(|&exit| exit.toi).unwrap(),
        };
      Err(exit)
    },
    &TreeBody::Leaf(ref leaf) => Ok((bounds, leaf)),
    &TreeBody::Branch(ref b) => {
      let mid = [
        (bounds.x as f32 + 0.5) * bounds.size(),
        (bounds.y as f32 + 0.5) * bounds.size(),
        (bounds.z as f32 + 0.5) * bounds.size(),
      ];

      let mut make_bounds = |coords: [usize; 3]| {
        let mut bounds = bounds;
        bounds.lg_size -= 1;
        bounds.x <<= 1;
        bounds.y <<= 1;
        bounds.z <<= 1;
        bounds.x += coords[0] as i32;
        bounds.y += coords[1] as i32;
        bounds.z += coords[2] as i32;
        bounds
      };

      let entry_toi = entry.map(|entry| entry.toi.0).unwrap_or(0.0);
      let intersect = [
        origin[0] + entry_toi*direction[0],
        origin[1] + entry_toi*direction[1],
        origin[2] + entry_toi*direction[2],
      ];

      cast_ray_branches(
        b,
        origin,
        direction,
        entry,
        [
          if intersect[0] >= mid[0] {1} else {0},
          if intersect[1] >= mid[1] {1} else {0},
          if intersect[2] >= mid[2] {1} else {0},
        ],
        &mut make_bounds,
      )
    }
  }
}
