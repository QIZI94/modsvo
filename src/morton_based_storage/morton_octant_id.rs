use std::fmt::{Debug, Write};
use morton_encoding::{morton_encode, morton_decode};

use super::super::octant_meta::{OctantPlacement, OctantNeighborDirection};
use super::super::Depth;

#[derive(Hash, Copy, Clone,  Eq, PartialEq, PartialOrd)]
pub struct MortonOctantId(pub u64);

impl MortonOctantId {
	pub const ROOT_OCTANT_ID: MortonOctantId = MortonOctantId(1);
	pub const INVALID_OCTANT_ID: MortonOctantId = MortonOctantId(0);
	pub const MAX_DEPTH: Depth = u16::BITS as Depth;

	pub fn from_xyz(x: u16, y: u16, z: u16, depth: Depth) -> Result<MortonOctantId, ValidationError> {
		Self::from_xyz_array([x, y, z], depth)
	}

	pub fn from_xyz_array(xyz_array: [u16;3], depth: Depth) -> Result<MortonOctantId, ValidationError> {
		let _ = Self::validate_xyz_array_at_depth(xyz_array.map(|axis| axis as i32), depth)?;
		let shift: u8 = depth * 3_u8;
		let morton_code: u64 = morton_encode(xyz_array);
		Ok(
			MortonOctantId{
				0: morton_code | (Self::ROOT_OCTANT_ID.as_morton() << shift)
			}
		)
	}

	pub fn from_morton_code(morton_code: u64) -> Self{
		MortonOctantId(morton_code)
	}
	
	pub fn xyz(&self) -> [u16;3] {
		let shift: u8 = self.compute_depth() * 3_u8;
		let root_mask: u64 = !(Self::ROOT_OCTANT_ID.as_morton() << shift);
		morton_decode(self.as_morton() & root_mask)
	}

	pub fn as_morton(&self) -> u64{
		self.0
	}

	pub fn parent_id(&self) -> MortonOctantId {
		if self.is_root() || !self.is_valid(){
			Self::INVALID_OCTANT_ID
		}
		else{
			Self::from_morton_code(
				parent_of_morton_code(self.as_morton())
			)
		}
	}

	pub fn parent_id_iter(&self) -> MortonParentIdIterator {
		MortonParentIdIterator::from_child(*self)
	}

	pub fn children_ids(&self) -> [Self; OctantPlacement::OCTANTS_COUNT] {
		return children_ids_from_parent_id(self.as_morton());
	}

	pub fn child_id_by_placement(&self, child_octant_placement: OctantPlacement) -> MortonOctantId{
		let octant_placement_index: usize = child_octant_placement as usize;
		let parent_morton = self.as_morton();
		let child_offset: u64 = parent_morton << 3;
		MortonOctantId(child_offset | octant_placement_index as u64)
	}

	pub fn compute_depth(&self) -> Depth {
		depth_from_morton_code(self.as_morton())
	}

	pub fn get_neighbor(&self, neighbor_direction: OctantNeighborDirection) -> Result<MortonOctantId, ValidationError> {
		let depth: Depth = self.compute_depth();
		let [x, y, z] = self.xyz().map(i32::from);
		match neighbor_direction {
			OctantNeighborDirection::U => MortonOctantId::from_xyz_array(
				Self::validate_xyz_array_at_depth([x, y+1, z], depth)?,	depth
			),
			OctantNeighborDirection::D => MortonOctantId::from_xyz_array(
				Self::validate_xyz_array_at_depth([x, y-1, z], depth)?,	depth
			),
			OctantNeighborDirection::N => MortonOctantId::from_xyz_array(
				Self::validate_xyz_array_at_depth([x, y, z+1], depth)?,	depth
			),
			OctantNeighborDirection::S => MortonOctantId::from_xyz_array(
				Self::validate_xyz_array_at_depth([x, y, z-1], depth)?,	depth
			),
			OctantNeighborDirection::E => MortonOctantId::from_xyz_array(
				Self::validate_xyz_array_at_depth([x+1, y, z], depth)?,	depth
			),
			OctantNeighborDirection::W => MortonOctantId::from_xyz_array(
				Self::validate_xyz_array_at_depth([x-1, y, z], depth)?, depth
			),
			OctantNeighborDirection::NE => MortonOctantId::from_xyz_array(
				Self::validate_xyz_array_at_depth([x+1, y, z+1], depth)?, depth
			),
			OctantNeighborDirection::NW => MortonOctantId::from_xyz_array(
				Self::validate_xyz_array_at_depth([x-1, y, z+1], depth)?, depth
			),
			OctantNeighborDirection::SE => MortonOctantId::from_xyz_array(
				Self::validate_xyz_array_at_depth([x+1, y, z-1], depth)?, depth
			),
			OctantNeighborDirection::SW => MortonOctantId::from_xyz_array(
				Self::validate_xyz_array_at_depth([x-1, y, z-1], depth)?, depth
			),
			OctantNeighborDirection::UN => MortonOctantId::from_xyz_array(
				Self::validate_xyz_array_at_depth([x, y+1, z+1], depth)?, depth
			),
			OctantNeighborDirection::US => MortonOctantId::from_xyz_array(
				Self::validate_xyz_array_at_depth([x, y+1, z-1], depth)?, depth
			),
			OctantNeighborDirection::UE => MortonOctantId::from_xyz_array(
				Self::validate_xyz_array_at_depth([x+1, y+1, z], depth)?, depth
			),
			OctantNeighborDirection::UW => MortonOctantId::from_xyz_array(
				Self::validate_xyz_array_at_depth([x-1, y+1, z], depth)?, depth
			),
			OctantNeighborDirection::DN => MortonOctantId::from_xyz_array(
				Self::validate_xyz_array_at_depth([x, y-1, z+1], depth)?, depth
			),
			OctantNeighborDirection::DS => MortonOctantId::from_xyz_array(
				Self::validate_xyz_array_at_depth([x, y-1, z-1], depth)?, depth
			),
			OctantNeighborDirection::DE => MortonOctantId::from_xyz_array(
				Self::validate_xyz_array_at_depth([x+1, y-1, z], depth)?, depth
			),
			OctantNeighborDirection::DW => MortonOctantId::from_xyz_array(
				Self::validate_xyz_array_at_depth([x-1, y-1, z], depth)?, depth
			),
			OctantNeighborDirection::UNE => MortonOctantId::from_xyz_array(
				Self::validate_xyz_array_at_depth([x+1, y+1, z+1], depth)?, depth
			),
			OctantNeighborDirection::UNW => MortonOctantId::from_xyz_array(
				Self::validate_xyz_array_at_depth([x-1, y+1, z+1], depth)?, depth
			),
			OctantNeighborDirection::USE => MortonOctantId::from_xyz_array(
				Self::validate_xyz_array_at_depth([x+1, y+1, z-1], depth)?, depth
			),
			OctantNeighborDirection::USW => MortonOctantId::from_xyz_array(
				Self::validate_xyz_array_at_depth([x-1, y+1, z-1], depth)?, depth
			),
			OctantNeighborDirection::DNE => MortonOctantId::from_xyz_array(
				Self::validate_xyz_array_at_depth([x+1, y-1, z+1], depth)?, depth
			),
			OctantNeighborDirection::DNW => MortonOctantId::from_xyz_array(
				Self::validate_xyz_array_at_depth([x-1, y-1, z+1], depth)?, depth
			),
			OctantNeighborDirection::DSE => MortonOctantId::from_xyz_array(
				Self::validate_xyz_array_at_depth([x+1, y-1, z-1], depth)?, depth
			),
			OctantNeighborDirection::DSW => MortonOctantId::from_xyz_array(	
				Self::validate_xyz_array_at_depth([x-1, y-1, z-1], depth)?, depth
			),	
		}
	}

	pub fn get_facing_neighbors(&self) -> [(Result<MortonOctantId, ValidationError>, OctantNeighborDirection); OctantNeighborDirection::FACING_NEIGHBOR_DIRECTIONS_COUNT] {
		[
			(self.get_neighbor(OctantNeighborDirection::U), OctantNeighborDirection::U),
			(self.get_neighbor(OctantNeighborDirection::D), OctantNeighborDirection::D),
			(self.get_neighbor(OctantNeighborDirection::N), OctantNeighborDirection::N),
			(self.get_neighbor(OctantNeighborDirection::S), OctantNeighborDirection::S),
			(self.get_neighbor(OctantNeighborDirection::E), OctantNeighborDirection::E),
			(self.get_neighbor(OctantNeighborDirection::W), OctantNeighborDirection::W)
		]
	}
	pub fn get_diagonal_neighbors(&self) -> [(Result<MortonOctantId, ValidationError>, OctantNeighborDirection); OctantNeighborDirection::DIAGONAL_NEIGHBOR_DIRECTIONS_COUNT] {
		[
			(self.get_neighbor(OctantNeighborDirection::NE), OctantNeighborDirection::NE),
			(self.get_neighbor(OctantNeighborDirection::NW), OctantNeighborDirection::NW),
			(self.get_neighbor(OctantNeighborDirection::SE), OctantNeighborDirection::SE),
			(self.get_neighbor(OctantNeighborDirection::SW), OctantNeighborDirection::SW),

			(self.get_neighbor(OctantNeighborDirection::UN), OctantNeighborDirection::UN),
			(self.get_neighbor(OctantNeighborDirection::US), OctantNeighborDirection::US),
			(self.get_neighbor(OctantNeighborDirection::UE), OctantNeighborDirection::UE),
			(self.get_neighbor(OctantNeighborDirection::UW), OctantNeighborDirection::UW),

			(self.get_neighbor(OctantNeighborDirection::DN), OctantNeighborDirection::DN),
			(self.get_neighbor(OctantNeighborDirection::DS), OctantNeighborDirection::DS),
			(self.get_neighbor(OctantNeighborDirection::DE), OctantNeighborDirection::DE),
			(self.get_neighbor(OctantNeighborDirection::DW), OctantNeighborDirection::DW),

			(self.get_neighbor(OctantNeighborDirection::UNE), OctantNeighborDirection::UNE),
			(self.get_neighbor(OctantNeighborDirection::UNW), OctantNeighborDirection::UNW),
			(self.get_neighbor(OctantNeighborDirection::USE), OctantNeighborDirection::USE),
			(self.get_neighbor(OctantNeighborDirection::USW), OctantNeighborDirection::USW),

			(self.get_neighbor(OctantNeighborDirection::DNE), OctantNeighborDirection::DNE),
			(self.get_neighbor(OctantNeighborDirection::DNW), OctantNeighborDirection::DNW),
			(self.get_neighbor(OctantNeighborDirection::DSE), OctantNeighborDirection::DSE),
			(self.get_neighbor(OctantNeighborDirection::DSW), OctantNeighborDirection::DSW),
		]
	}

	pub fn get_all_neighbors(&self) -> [(Result<MortonOctantId, ValidationError>, OctantNeighborDirection); OctantNeighborDirection::NEIGHBOR_DIRECTIONS_COUNT] {
		[
			(self.get_neighbor(OctantNeighborDirection::U), OctantNeighborDirection::U),
			(self.get_neighbor(OctantNeighborDirection::D), OctantNeighborDirection::D),
			(self.get_neighbor(OctantNeighborDirection::N), OctantNeighborDirection::N),
			(self.get_neighbor(OctantNeighborDirection::S), OctantNeighborDirection::S),
			(self.get_neighbor(OctantNeighborDirection::E), OctantNeighborDirection::E),
			(self.get_neighbor(OctantNeighborDirection::W), OctantNeighborDirection::W),

			(self.get_neighbor(OctantNeighborDirection::NE), OctantNeighborDirection::NE),
			(self.get_neighbor(OctantNeighborDirection::NW), OctantNeighborDirection::NW),
			(self.get_neighbor(OctantNeighborDirection::SE), OctantNeighborDirection::SE),
			(self.get_neighbor(OctantNeighborDirection::SW), OctantNeighborDirection::SW),

			(self.get_neighbor(OctantNeighborDirection::UN), OctantNeighborDirection::UN),
			(self.get_neighbor(OctantNeighborDirection::US), OctantNeighborDirection::US),
			(self.get_neighbor(OctantNeighborDirection::UE), OctantNeighborDirection::UE),
			(self.get_neighbor(OctantNeighborDirection::UW), OctantNeighborDirection::UW),

			(self.get_neighbor(OctantNeighborDirection::DN), OctantNeighborDirection::DN),
			(self.get_neighbor(OctantNeighborDirection::DS), OctantNeighborDirection::DS),
			(self.get_neighbor(OctantNeighborDirection::DE), OctantNeighborDirection::DE),
			(self.get_neighbor(OctantNeighborDirection::DW), OctantNeighborDirection::DW),

			(self.get_neighbor(OctantNeighborDirection::UNE), OctantNeighborDirection::UNE),
			(self.get_neighbor(OctantNeighborDirection::UNW), OctantNeighborDirection::UNW),
			(self.get_neighbor(OctantNeighborDirection::USE), OctantNeighborDirection::USE),
			(self.get_neighbor(OctantNeighborDirection::USW), OctantNeighborDirection::USW),

			(self.get_neighbor(OctantNeighborDirection::DNE), OctantNeighborDirection::DNE),
			(self.get_neighbor(OctantNeighborDirection::DNW), OctantNeighborDirection::DNW),
			(self.get_neighbor(OctantNeighborDirection::DSE), OctantNeighborDirection::DSE),
			(self.get_neighbor(OctantNeighborDirection::DSW), OctantNeighborDirection::DSW),
		]
	}

	pub fn has_child(&self, child_id: &MortonOctantId) -> Option<OctantPlacement> {
		self.children_ids().into_iter()
			.zip(OctantPlacement::OCTANTS_ORDERED)
			.find(
				|(octant_id, _)|{
					*octant_id == *child_id
				}
			)
			.map(|(_, octant_placement)| octant_placement)
	}

	pub fn max_xyz_value(&self) -> u16 {
		(Self::max_xyz_grid_size_in_depth(self.compute_depth()).unwrap() - 1) as u16
	}

	pub fn max_xyz_grid_size_in_depth(depth: Depth) -> Option<u32> {
		if depth > Self::MAX_DEPTH {
			None
		}
		else {
			Some(0x01_u32 << depth)
		}
	}

	pub fn is_root(&self) -> bool {
		*self == Self::ROOT_OCTANT_ID
	}

	pub fn is_valid(&self) -> bool {
		*self != Self::INVALID_OCTANT_ID
	}

	pub fn validate_xyz_array_at_depth(xyz: [i32;3], depth: Depth) -> Result<[u16;3], ValidationError> {
		let max_grid_size = Self::max_xyz_grid_size_in_depth(depth).ok_or(ValidationError::DepthError(depth))?;
		let max_axis: i32 = (max_grid_size - 1) as i32;
		let maybe_error: [Option<i32>; 3] = xyz.map(
			|axis: i32| {
				if axis < 0 || axis > max_axis {
					Some(axis)
				}
				else {
					None
				}
			}
		);

		if let Some(_) = maybe_error.iter().find(|&maybe_bad_axis|maybe_bad_axis.is_some()) {
			let [x, y, z] = maybe_error;
			Err(ValidationError::AxisError(x, y, z, max_axis))
		}
		else {
			Ok(xyz.map(|axis| axis as u16))
		}
	}
	
}

impl Default for MortonOctantId {
	fn default() -> Self {
		Self::INVALID_OCTANT_ID
	}
}

impl Debug for MortonOctantId {
	fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let depth = self.compute_depth();
		if !self.is_valid() {
			write!(formatter, "MortonOctantId(None)")
		}
		else if self.is_root() {
			write!(formatter, "MortonOctantId(Root == {}): depth({}) ", self.as_morton(), depth)
		}
		else {
			let [x,y,z] = self.xyz();
			let parent_id = self.parent_id().as_morton();
			write!(formatter, "MortonOctantId({}): [X: {}, Y: {}, Z: {}] parent_id({}) depth({}) ",self.as_morton(), x, y, z, parent_id, depth)
		}
    }
}

pub enum ValidationError {
	DepthError(Depth),
	/// Option of X,Y,Z and MAX value
	AxisError(Option<i32>, Option<i32>, Option<i32>, i32)
}

impl ValidationError {
	const MIN_AXIS: i32 = 0;
	const MAX_DEPTH: Depth = MortonOctantId::MAX_DEPTH;
	pub fn is_x_below_limit(&self) -> bool{
		match self {
			Self::AxisError(Some(x), _, _, _) => *x < 0,
			_ => false
		}
	}

	pub fn is_y_below_limit(&self) -> bool{
		match self {
			Self::AxisError(_, Some(y), _, _) => *y < 0,
			_ => false
		}
	}

	pub fn is_z_below_limit(&self) -> bool{
		match self {
			Self::AxisError(_, _, Some(z), _) => *z < 0,
			_ => false
		}
	}

	pub fn is_x_above_limit(&self) -> bool{
		match self {
			Self::AxisError(Some(x), _, _, _) => *x > (u16::MAX as i32),
			_ => false
		}
	}

	pub fn is_y_above_limit(&self) -> bool{
		match self {
			Self::AxisError(_, Some(y), _, _) => *y > (u16::MAX as i32),
			_ => false
		}
	}
	
	pub fn is_z_above_limit(&self) -> bool{
		match self {
			Self::AxisError(_, _, Some(z), _) => *z > (u16::MAX as i32),
			_ => false
		}
	}
}


impl Debug for ValidationError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		const DEBUG_PHRASE: &str = "Some axis are out of bounds";

		let display_compare = |axis_char: char, maybe_axis: Option<i32>, max_value: i32|{
			let mut display_invalid_axis = String::default();
			if let Some(axis) = maybe_axis {
				if axis < Self::MIN_AXIS{
					let _ = write!(display_invalid_axis, ", {}: {} (Min: {})", axis_char, axis, Self::MIN_AXIS);
				}
				else if axis > max_value {
					let _ = write!(display_invalid_axis, ", {}: {} (Max: {})", axis_char, axis, max_value);
				}
				else {
					panic!("{} = {}: Valid coordinates should not be error, this is a logic bug", axis_char, axis)
				}	
			}
			display_invalid_axis
		};
		match self {
			Self::DepthError(depth) => write!(f, "Depth {} is over max value {}.", depth, Self::MAX_DEPTH),
			Self::AxisError(maybe_x, maybe_y, maybe_z, max_value) => {
				write!(f, "{}{}{}{}",DEBUG_PHRASE, display_compare('X', *maybe_x, *max_value), display_compare('Y', *maybe_y, *max_value) ,display_compare('Z', *maybe_z, *max_value))
			}
		}
		
	}
}

pub struct MortonParentIdIterator{
	current_octant_id: MortonOctantId,
	
}

impl MortonParentIdIterator {
	fn from_child(child_id: MortonOctantId) -> Self{
		MortonParentIdIterator{current_octant_id: child_id}
	}
}

impl Iterator for MortonParentIdIterator{
	type Item = MortonOctantId;
	fn next(&mut self) -> Option<Self::Item> {
		if self.current_octant_id.is_root() || !self.current_octant_id.is_valid() {
			None
		}
		else {
			self.current_octant_id = self.current_octant_id.parent_id();
			Some(self.current_octant_id)
		}
	}
	
}

pub fn parent_of_morton_code(child_morton: u64) -> u64 {
    child_morton >> 3
}

pub fn depth_from_morton_code(morton_code: u64) -> Depth {
	((u64::BITS - morton_code.leading_zeros()) / 3) as Depth
}

pub fn max_axis_for_depth(depth: Depth) -> u16{
	0x1 << depth
}

pub fn children_ids_from_parent_id(parent_morton: u64) -> [MortonOctantId; OctantPlacement::OCTANTS_COUNT] {
	let child_offset: u64 = parent_morton << 3;
	[
		MortonOctantId(child_offset | 0),
		MortonOctantId(child_offset | 1),
		MortonOctantId(child_offset | 2),
		MortonOctantId(child_offset | 3),
		MortonOctantId(child_offset | 4),
		MortonOctantId(child_offset | 5),
		MortonOctantId(child_offset | 6),
		MortonOctantId(child_offset | 7),
	]
}



#[cfg(test)]
mod tests{
    use crate::shared::morton_voxel_octree::{octant_meta::{OctantNeighborDirection, OctantPlacement}, Depth};
    use super::MortonOctantId;

	#[test]
	fn test_morton_encoding_decoding(){
		const TEST_DEPTH: Depth = 8;
		const TEST_MAX_AXIS: u16 = 0x01 << TEST_DEPTH;
		for x in 0..TEST_MAX_AXIS{
			for y in 0..TEST_MAX_AXIS{
				for z in 0..TEST_MAX_AXIS{
					let octant_id = MortonOctantId::from_xyz(x, y, z, TEST_DEPTH).unwrap();
					assert_eq!([x, y, z], octant_id.xyz());
				}
			}
		}
	}
	#[test]
	fn test_morton_encoding_decoding_diagonal_max_depth(){
		const TEST_MAX_AXIS: u16 = ((0x01 << MortonOctantId::MAX_DEPTH) as u32 - 1) as u16;
		for axis in 0 ..= TEST_MAX_AXIS{
			let octant_id = MortonOctantId::from_xyz_array([axis, axis, axis], MortonOctantId::MAX_DEPTH).unwrap();
			assert_eq!([axis, axis, axis], octant_id.xyz());
		}		
	}

	#[test]
	fn test_morton_id_nearest_neighbor_at_edges(){
		const TEST_DEPTH: Depth = 16;
		/// [0,0,0]
		{
			//const TEST_MAX_AXIS: u16 = ((0x01 << MortonOctantId::MAX_DEPTH) as u32 - 1) as u16;
			let some_octant_id: MortonOctantId = MortonOctantId::from_xyz_array([0u16, 0u16, 0u16], TEST_DEPTH).unwrap();
			let parent_of_some_octant_id: MortonOctantId = some_octant_id.parent_id();
			let placement_of_some_octant_id: OctantPlacement = parent_of_some_octant_id.has_child(&some_octant_id).unwrap();
			some_octant_id.get_all_neighbors().into_iter()
				.filter_map(
					|(maybe_neighbor, neighbor_direction)|{
						let neighbor_id = maybe_neighbor.ok()?;
						Some((neighbor_id, neighbor_direction))
					}
				)
				.for_each(
					|(neighbor_id, neighbor_direction)|{
						
						let neighbor_placement = parent_of_some_octant_id.has_child(&neighbor_id).unwrap();
						let maybe_valid_neighbor = OctantPlacement::all_neighbors_for(placement_of_some_octant_id).into_iter()
							.find(
								|&(octant_direction, octant_placement)|{
									octant_direction == neighbor_direction && octant_placement == neighbor_placement
								}
							);
						if maybe_valid_neighbor.is_none(){	
							dbg!(OctantPlacement::all_neighbors_for(placement_of_some_octant_id));
							dbg!(some_octant_id);
							dbg!(placement_of_some_octant_id);
							panic!("neighbor_id: {:?}, neighbor_placement: {:?}, neighbor_direction: {:?}",neighbor_id, neighbor_placement, neighbor_direction);
						}

					}
				);
		}

		/// [1,1,1]
		{
			//const TEST_MAX_AXIS: u16 = ((0x01 << MortonOctantId::MAX_DEPTH) as u32 - 1) as u16;
			let max_xyz = MortonOctantId::from_xyz(0, 0, 0, TEST_DEPTH).unwrap().max_xyz_value();
			let some_octant_id: MortonOctantId = MortonOctantId::from_xyz_array([max_xyz, max_xyz, max_xyz], TEST_DEPTH).unwrap();
			let parent_of_some_octant_id: MortonOctantId = some_octant_id.parent_id();
			let placement_of_some_octant_id: OctantPlacement = parent_of_some_octant_id.has_child(&some_octant_id).unwrap();
			some_octant_id.get_all_neighbors().into_iter()
				.filter_map(
					|(maybe_neighbor, neighbor_direction)|{
						let neighbor_id = maybe_neighbor.ok()?;
						Some((neighbor_id, neighbor_direction))
					}
				)
				.for_each(
					|(neighbor_id, neighbor_direction)|{
						
						let neighbor_placement = parent_of_some_octant_id.has_child(&neighbor_id).unwrap();
						let maybe_valid_neighbor = OctantPlacement::all_neighbors_for(placement_of_some_octant_id).into_iter()
							.find(
								|&(octant_direction, octant_placement)|{
									octant_direction == neighbor_direction && octant_placement == neighbor_placement
								}
							);
						if maybe_valid_neighbor.is_none(){	
							dbg!(OctantPlacement::all_neighbors_for(placement_of_some_octant_id));
							dbg!(some_octant_id);
							dbg!(placement_of_some_octant_id);
							panic!("neighbor_id: {:?}, neighbor_placement: {:?}, neighbor_direction: {:?}",neighbor_id, neighbor_placement, neighbor_direction);
						}

					}
				);
		}
			
	}

	#[test]
	fn test_neighbor_all_directions(){
		const TEST_DEPTH: Depth = 16;
		let max_xyz = MortonOctantId::from_xyz(0, 0, 0, TEST_DEPTH).unwrap().max_xyz_value();
		let mid_xyz = max_xyz / 2;

		let middle_octant = MortonOctantId::from_xyz(mid_xyz, mid_xyz, mid_xyz, TEST_DEPTH).unwrap();
		middle_octant.get_all_neighbors().into_iter()
			.filter(|(neighbor_result, _)| neighbor_result.is_err())
			.for_each(
				|(neighbor_result, neighbor_direction)|{
					let error = neighbor_result.unwrap_err();
					panic!("{:?}", error);
				}
			);
	}

	#[test]
	fn test_morton_depth_parents_children(){
		let root_id: MortonOctantId = MortonOctantId::ROOT_OCTANT_ID;
		visit_octant_and_compare_parent_recursively(root_id, 0, 8);
	}

	fn visit_octant_and_compare_parent_recursively(parent_id: MortonOctantId, depth: Depth, max_depth: Depth) {
		let computed_depth: Depth = parent_id.compute_depth();
		assert_eq!(depth, computed_depth);
		if computed_depth > max_depth{
			return;
		} 
		for child_id in parent_id.children_ids(){
			assert_eq!(parent_id, child_id.parent_id());
			visit_octant_and_compare_parent_recursively(child_id, depth + 1, max_depth);
		}
	}

	

	#[test]
	fn test_sides(){
		
		for morton_code  in 0..= 7u64 {
			let a = OctantPlacement::try_from(morton_code as usize).unwrap();
			let [x, y, z]: [u16;3] = morton_encoding::morton_decode(morton_code);
			println!("{} = [{}, {}, {}]",morton_code, x, y, z);
		}
	}

}