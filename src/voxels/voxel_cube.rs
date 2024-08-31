use glam::{Vec3, Vec3A, Vec4, U16Vec3};
//use serde::{Deserialize, Serialize};

use super::super::octant_meta::{OctantPlacement, OctantNeighborDirection};
use super::super::{Depth, voxel_trait::Voxel};

pub type CornerPlacement = OctantPlacement;
pub const CORNER_COUNT: usize = OctantPlacement::OCTANTS_COUNT;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpatialRelation {
	/// Relation doesn't share any common space
	Separate,
	/// Intersecting in relation
	Intersecting,
	/// Fully enclosed in relation
	Contained,
}

#[derive(Debug, Clone, Copy, /*Serialize, Deserialize*/)]
pub struct VolumetricCube{
	center_and_radius_simd: Vec4
}

impl VolumetricCube  {


	pub const fn new_with_radius(half_extent: f32) -> Self{
		Self{
			center_and_radius_simd: Vec4::new(0., 0., 0., half_extent)
		}
	}

	pub fn new(center: Vec3A, half_extent: f32) -> Self{
		Self {
			center_and_radius_simd: center.extend(half_extent)
		}
	}

	pub const fn new_const(x: f32, y: f32, z: f32, half_extent: f32) -> Self{
		Self {
			center_and_radius_simd: Vec4::new(x, y, z, half_extent)
		}
	}

	pub fn center(&self) -> Vec3A {
		self.center_and_radius_simd.into()
	}

	pub fn max(&self) -> Vec3A {
		self.center() + self.half_extent()
	}

	pub fn min(&self) -> Vec3A {
		self.center() - self.half_extent()
	}

	pub fn half_extent(&self) -> f32 {
		self.center_and_radius_simd.w
	}

	pub fn get_single_corner(&self, corner_placement: CornerPlacement) -> Vec3A{
		let min = self.min();
		let max = self.max();

		match corner_placement {
			CornerPlacement::LOWER_BOTTOM_LEFT => {
				min
			},
			CornerPlacement::LOWER_TOP_LEFT => {
				Vec3A::new(min.x, min.y, max.z)
			},
			CornerPlacement::LOWER_BOTTOM_RIGHT => {
				Vec3A::new(max.x, min.y, min.z)
			},
			CornerPlacement::LOWER_TOP_RIGHT => {
				Vec3A::new(max.x, min.y, max.z)
			},
			CornerPlacement::UPPER_BOTTOM_LEFT => {
				Vec3A::new(min.x, max.y, min.z)
			},
			CornerPlacement::UPPER_TOP_LEFT => {
				Vec3A::new(min.x, max.y, max.z)
			},
			
			CornerPlacement::UPPER_BOTTOM_RIGHT => {
				Vec3A::new(max.x, max.y, min.z)
			},			
			CornerPlacement::UPPER_TOP_RIGHT => {
				max
			}
		}
	}

	pub fn get_corners(&self) -> [Vec3A; OctantPlacement::OCTANTS_COUNT] {
		let ordered_placement = CornerPlacement::OCTANTS_ORDERED;
		[
			self.get_single_corner(ordered_placement[0]),
			self.get_single_corner(ordered_placement[1]),
			self.get_single_corner(ordered_placement[2]),
			self.get_single_corner(ordered_placement[3]),

			self.get_single_corner(ordered_placement[4]),
			self.get_single_corner(ordered_placement[5]),
			self.get_single_corner(ordered_placement[6]),
			self.get_single_corner(ordered_placement[7]),
		]
	}

	pub const fn get_corners_per_face(&self, spatial_direction: OctantNeighborDirection) -> [CornerPlacement; Self::CORNERS_PER_FACE] {
		Self::SPATIAL_DIRECTIONAL_FACE_CORNERS[spatial_direction as usize]
	}

	pub fn contains_point(&self, point: Vec3A) -> bool {
        let min_bound = self.min();
        let max_bound = self.max();

        return point.x >= min_bound.x
            && point.x <= max_bound.x
            && point.y >= min_bound.y
            && point.y <= max_bound.y
            && point.z >= min_bound.z
            && point.z <= max_bound.z;
    }

	
	pub fn set_center(&mut self, center: Vec3A) {
		self.center_and_radius_simd = center.extend(self.center_and_radius_simd.w);
	}

	pub fn set_half_extent(&mut self, half_extent: f32) {
		self.center_and_radius_simd.w = half_extent;
	}

	pub fn with_center(self, center: Vec3A) -> Self {
		Self{
			center_and_radius_simd: center.extend(self.center_and_radius_simd.w)
		}
	}

	pub fn with_half_extent(mut self, half_extent: f32) -> Self {
		self.center_and_radius_simd.w = half_extent;
		Self{
			center_and_radius_simd: self.center_and_radius_simd
		} 
	}

    pub fn expand(&mut self, point: Vec3A) {
        let distance_to_point = (point - self.center())
			.abs()
			.to_array()
			.iter()
			.fold(
				0.0_f32,
				|current_max, axis|{
					axis.max(current_max)
				}
			);

        self.set_half_extent(self.half_extent().max(distance_to_point));
    }

	pub fn expand_in_direction(&mut self, _point: Vec3) {
		unimplemented!()	
	}

	pub fn get_octant_position(&self, octant_placement: OctantPlacement) -> Vec3A{
		let half_radius: f32 = self.half_extent() * 0.5;
		self.center() + Self::SPATIAL_POSITION[octant_placement as usize] * half_radius
	}


	pub fn guess_octant(&self, point: Vec3A) -> OctantPlacement{
		let dir = point - self.center();
		match (
			dir.x.is_sign_positive(),
			dir.y.is_sign_positive(),
			dir.z.is_sign_positive()
		){
			(false, true,   true) => OctantPlacement::UPPER_TOP_LEFT,
			(true,  true,   true) => OctantPlacement::UPPER_TOP_RIGHT,
			(true,  true,  false) => OctantPlacement::UPPER_BOTTOM_RIGHT,
			(false, true,  false) => OctantPlacement::UPPER_BOTTOM_LEFT,
			(false, false,  true) => OctantPlacement::LOWER_TOP_LEFT,
			(true,  false,  true) => OctantPlacement::LOWER_TOP_RIGHT,
			(true,  false, false) => OctantPlacement::LOWER_BOTTOM_RIGHT,
			(false, false, false) => OctantPlacement::LOWER_BOTTOM_LEFT,
		}
	}

	pub fn subdivision_depth(&self, initial_half_extent: f32) -> Depth {
        let ratio = initial_half_extent / self.half_extent();
        let depth = (ratio.log2()) as Depth; // Add 1 because depth starts from 0
        depth
    }

	pub fn collides_with_sphere(&self, sphere_origin: Vec3A, sphere_radius: f32) -> SpatialRelation {
		if self.intersects_sphere(sphere_origin, sphere_radius) {
			if self.is_inside_sphere(sphere_origin, sphere_radius) {
				SpatialRelation::Contained
			}
			else {
				SpatialRelation::Intersecting
			}
		}
		else {
			SpatialRelation::Separate
		}
	}
	
	pub fn intersects_sphere(&self, sphere_origin: Vec3A, sphere_radius: f32) -> bool {

		let total_distance_squared = self.center()
			.to_array()
			.iter()
			.zip(sphere_origin.to_array().iter())
			.fold(
				0.0_f32, 
				|sum_distance_squared,(center_axis,  sphere_axis)| {
					let axis_distance = (sphere_axis - center_axis).abs();
					let clamped_distance = axis_distance - self.half_extent();

					sum_distance_squared + clamped_distance.max(0.0).powi(2)
				}
			);

		total_distance_squared <= sphere_radius.powi(2)
	}

	pub fn is_inside_sphere(&self, sphere_origin: Vec3A, sphere_radius: f32) -> bool {
		let sphere_radius_squared = sphere_radius * sphere_radius;
		for corner in self.get_corners().iter(){
			//println!("corner: {:?} sq: {} vs sphere: {}" ,corner,corner.distance(sphere_origin) ,sphere_radius);
			if corner.distance_squared(sphere_origin) > sphere_radius_squared {
				return false;
			}
		}

		true
	}

	pub fn collides_with_box(&self, other: &Self) -> SpatialRelation {
		if self.intersect_box(other) {
			if self.is_inside_box(other) {
				SpatialRelation::Contained
			}
			else {
				SpatialRelation::Intersecting
			}
		}
		else {
			SpatialRelation::Separate
		}
	}

    pub fn intersect_box(&self, other: &Self) -> bool {
		let self_max = self.max();
		let self_min = self.min();
		let other_max = other.max();
		let other_min = other.min();

        if self_max.x < other_min.x || self_min.x > other_max.x {
            return false;
        }

        if self_max.y < other_min.y || self_min.y > other_max.y {
            return false;
        }

        if self_max.z < other_min.z || self_min.z > other_max.z {
            return false;
        }

        true
    }

	pub fn is_inside_box(&self, other: &Self) -> bool {
		let self_max = self.max();
		let self_min = self.min();
		let other_max = other.max();
		let other_min = other.min();

		return 
			other_min.x <= self_min.x &&
			other_max.x >= self_max.x &&
			other_min.y <= self_min.y &&
			other_max.y >= self_max.y &&
			other_min.z <= self_min.z &&
			other_max.z >= self_max.z 
	}


	pub fn grid_position(&self, position: Vec3A, depth_of_subdivision: Depth) -> U16Vec3 {
        // Calculate relative position of current center with respect to original center
        let relative_position = (position - self.center()) / self.half_extent();

        // Calculate grid size at the specified depth
        let grid_size = 1 << depth_of_subdivision; // Equivalent to 2^depth

        // Map relative position to grid indices
        let grid_indices = U16Vec3::new(
            ((relative_position.x + 1.0) * 0.5 * grid_size as f32) as u16,
            ((relative_position.y + 1.0) * 0.5 * grid_size as f32) as u16,
            ((relative_position.z + 1.0) * 0.5 * grid_size as f32) as u16,
        );

        // Clamp grid indices to fit within the range [0, grid_size - 1]
        let clamped_indices = U16Vec3::new(
            grid_indices.x.min(grid_size - 1),
            grid_indices.y.min(grid_size - 1),
            grid_indices.z.min(grid_size - 1),
        );

        clamped_indices
    }
	const HASH_REOLUTION: u32 = 10;
	pub fn hash_point(&self, position: Vec3A) -> u32 {
        // Calculate relative position of current center with respect to original center
		let farthest_point = self.get_corners()[0].distance(self.center()); 

        let relative_position = (position - self.center()) / self.half_extent() / farthest_point;
		

    	let a: u32 = 1<<VolumetricCube::HASH_REOLUTION;// or 1024
        
        
        (relative_position.x * a as f32) as u32 +
        (relative_position.y * a as f32) as u32	* a +
        (relative_position.z * a as f32) as u32	* a * a
        
    }

	pub fn unhash_point(&self, mut hash: u32) -> Vec3A{
		let farthest_point = self.get_corners()[0].distance(self.center()); 

		let a: u32 = 1<<VolumetricCube::HASH_REOLUTION;// or 1024
        
		
	


        let mask: u32 = (1<<VolumetricCube::HASH_REOLUTION + 1) -1;
        let x: f32 = (hash & mask) as f32;
        hash /= a;
        let y: f32 = (hash & mask) as f32;
		hash /= a;
        let z: f32 = (hash & mask) as f32;

        Vec3A::new(x, y, z) * farthest_point
	}


	pub fn sub_volumetric_cube_from_grid_position(&self, depth: Depth, grid_position: U16Vec3) -> Self {
        // Calculate grid size at the specified depth
        let grid_size = 1 << depth; // Equivalent to 2^depth

        // Map grid position to relative corner positions
        let relative_position = Vec3A::new(
            (grid_position.x as f32 / grid_size as f32) * 2.0 - 1.0,
            (grid_position.y as f32 / grid_size as f32) * 2.0 - 1.0,
            (grid_position.z as f32 / grid_size as f32) * 2.0 - 1.0,
        );

        // Calculate current center and half extent
        let current_center = self.center() + Vec3A::new(
            relative_position.x * self.half_extent(),
            relative_position.y * self.half_extent(),
            relative_position.z * self.half_extent(),
        );
        let current_half_extent = self.half_extent() / (grid_size as f32);

        VolumetricCube::new(current_center, current_half_extent)
    }

	pub const fn get_spatial_neighbor_direction(spatial_direction: OctantNeighborDirection) -> Vec3A{
		Self::SPATIAL_NEIGHBOR_DIRECTION[spatial_direction as usize]
	}

}

impl Voxel for VolumetricCube {
	fn make_sub_voxel(&self, sub_voxel_placement: OctantPlacement) -> Self {

		let new_half_extend: f32 = self.half_extent() * 0.5;
		let new_center = self.center() + Self::SPATIAL_POSITION[sub_voxel_placement as usize] * new_half_extend;

		Self{
			center_and_radius_simd: new_center.extend(new_half_extend)
		}
		
	}
}

// constants
impl VolumetricCube {
	pub const CORNERS_PER_FACE: usize = 4;
	
	const SPATIAL_POSITION: [Vec3A; OctantPlacement::OCTANTS_COUNT] = [
		Vec3A::new(-1.0, -1.0, -1.0),// LOWER_BOTTOM_LEFT
		Vec3A::new(-1.0, -1.0, 1.0), // LOWER_TOP_LEFT
		Vec3A::new(1.0,  -1.0, -1.0),// LOWER_BOTTOM_RIGHT
		Vec3A::new(1.0,  -1.0, 1.0), // LOWER_TOP_RIGHT
		
		

		Vec3A::new(-1.0, 1.0, 1.0),  // UPPER_TOP_LEFT
		Vec3A::new(1.0,  1.0, 1.0),  // UPPER_TOP_RIGHT
		Vec3A::new(1.0,  1.0, -1.0), // UPPER_BOTTOM_RIGHT
		Vec3A::new(-1.0, 1.0, -1.0), // UPPER_BOTTOM_LEFT


	];
	
	const SPATIAL_NEIGHBOR_DIRECTION: [Vec3A; OctantNeighborDirection::FACING_NEIGHBOR_DIRECTIONS_COUNT] = [
		// UP
		Vec3A::new(0.0, 1.0, 0.0),
		// DOWN
		Vec3A::new(0.0, -1.0, 0.0),
		// FRONT
		Vec3A::new(0.0, 0.0, 1.0),
		// BACK
		Vec3A::new(0.0, 0.0, -1.0),
		// RIGHT
		Vec3A::new(1.0, 0.0, 0.0),
		// LEFT
		Vec3A::new(-1.0, 0.0, 0.0),
	];

	const SPATIAL_DIRECTIONAL_FACE_CORNERS: [[CornerPlacement; Self::CORNERS_PER_FACE]; OctantNeighborDirection::FACING_NEIGHBOR_DIRECTIONS_COUNT] = [
		// UP
		[			
			CornerPlacement::UPPER_TOP_LEFT,
			CornerPlacement::UPPER_BOTTOM_LEFT,
			CornerPlacement::UPPER_TOP_RIGHT,
			CornerPlacement::UPPER_BOTTOM_RIGHT,
		],
		
		// DOWN
		[
			CornerPlacement::LOWER_TOP_LEFT,
			CornerPlacement::LOWER_TOP_RIGHT,
			CornerPlacement::LOWER_BOTTOM_LEFT,
			CornerPlacement::LOWER_BOTTOM_RIGHT,
		],

	

		// NORTH
		[
			CornerPlacement::UPPER_TOP_LEFT, // 0
			CornerPlacement::UPPER_TOP_RIGHT, // 2
			CornerPlacement::LOWER_TOP_LEFT, // 1
			CornerPlacement::LOWER_TOP_RIGHT, // 3

			//face_indices[0] = unique_vertices.get(&corner_id0)?.as_index();
			//face_indices[1] = unique_vertices.get(&corner_id1)?.as_index();
			//face_indices[2] = unique_vertices.get(&corner_id3)?.as_index();

			//face_indices[3] = unique_vertices.get(&corner_id0)?.as_index();
			//face_indices[4] = unique_vertices.get(&corner_id3)?.as_index();
			//face_indices[5] = unique_vertices.get(&corner_id2)?.as_index();
			
						
		],

		// SOUTH
		[
			CornerPlacement::LOWER_BOTTOM_LEFT,
			CornerPlacement::LOWER_BOTTOM_RIGHT,
			CornerPlacement::UPPER_BOTTOM_LEFT,
			CornerPlacement::UPPER_BOTTOM_RIGHT,
		],

		// EAST
		[
			CornerPlacement::UPPER_BOTTOM_LEFT,	
			CornerPlacement::LOWER_BOTTOM_LEFT,
			CornerPlacement::UPPER_TOP_LEFT,
			CornerPlacement::LOWER_TOP_LEFT,			
		],

		// WEST
		[
			CornerPlacement::LOWER_BOTTOM_RIGHT,
			CornerPlacement::LOWER_TOP_RIGHT,
			CornerPlacement::UPPER_BOTTOM_RIGHT,	
			CornerPlacement::UPPER_TOP_RIGHT,
		]
		
	];
}