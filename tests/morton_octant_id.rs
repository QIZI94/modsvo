#[cfg(test)]
mod tests{	
	use modsvo::octant_meta::{OctantPlacement, Depth};
	use modsvo::morton_based_storage::morton_octant_id::MortonOctantId;


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
		// [0,0,0]
		{
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

		// [1,1,1]
		{
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
				|(neighbor_result, _neighbor_direction)|{
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
			let _ = OctantPlacement::try_from(morton_code as usize).unwrap();
			let [x, y, z]: [u16;3] = morton_encoding::morton_decode(morton_code);
			println!("{} = [{}, {}, {}]",morton_code, x, y, z);
		}
	}

	#[test]
	fn test_longest_common_ancestor(){
		let root_id: MortonOctantId = MortonOctantId::ROOT_OCTANT_ID;
		let first_children = root_id.children_ids();
		let second_child = first_children[3].children_ids()[3].children_ids()[4].children_ids()[6];

		let forth_child = second_child.children_ids()[3];
		let sixth_child = second_child.children_ids()[5];

		let eighth_child = sixth_child.children_ids()[0];
		let tenth_child = forth_child.children_ids()[6];

		let result = eighth_child.nearest_common_ancestor_with(tenth_child);
		assert_eq!(result, second_child);

	}
	#[test]
	fn test_children_by_id_and_placement(){
		let deeper_octant_id = MortonOctantId::ROOT_OCTANT_ID.children_ids()[2];
		let deepest_octant_id = deeper_octant_id.children_ids()[3];
		
		const TEST_CHILD_PLACEMENT: OctantPlacement = OctantPlacement::UPPER_BOTTOM_RIGHT;

		let test_child_id = deepest_octant_id.child_id_by_placement(TEST_CHILD_PLACEMENT);
		let test_child_placement = deepest_octant_id.child_placement_by_id(test_child_id);

		assert_eq!(test_child_placement, Some(TEST_CHILD_PLACEMENT));
		assert_eq!(deepest_octant_id.child_placement_by_id(deeper_octant_id), None);
		assert_eq!(deepest_octant_id.child_placement_by_id(MortonOctantId::INVALID_OCTANT_ID), None);
	}

}