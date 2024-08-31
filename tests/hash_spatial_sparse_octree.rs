

#[cfg(test)]
mod tests{			
	#[derive(Debug, Default)]
	struct NoData;
    use glam::Vec3A;

    use modsvo::{
		octant_meta::OctantPlacement, octant_storage_trait::{ModifiableOctantStorage, OctantStorage, OctantStorageAccessor}, octree_base::{SearchControlFlow, SearchControlFlowResult}, voxels::voxel_cube::VolumetricCube, Depth, SparseOctreeHashed, SpatialSparseOctreeHashed,
	};

	use modsvo::morton_based_storage::morton_octant_id::MortonOctantId;
	use modsvo::octree_base::OctantIdTypeInfo;
	#[test]
	fn test_depth_first_search(){
		type OctantId =  <SpatialSparseOctreeHashed<NoData> as OctantIdTypeInfo>::OctantId;
		let mut octree: SpatialSparseOctreeHashed<NoData> = SpatialSparseOctreeHashed::with_root_voxel(VolumetricCube::new(Vec3A::new(0.0,0.0, 0.0), 1.0));
		let root_voxel = *octree.get_root_voxel();

		let depth_first_visit_order  = [
			1,
			8,
			9,
			72,
			73,
			74,
			592,
			593,
			594,
			595,
			596,
			597,
			598,
			599,
			4792,
			4793,
			4794,
			4795,
			4796,
			4797,
			38376,
			38377,
			38378,
			38379,
			38380,
			38381,
			38382,
			38383,
			4798,
			4799,
			75,
			76,
			77,
			78,
			79,
			10,
			11,	
			12,
			13,
			14,
			15,
		];

		let mut depth_first_visit_order_iter_all = depth_first_visit_order.iter()
			.map(|morton_code| *morton_code);



		let level1 = octree.octants_mut().subdivide_with_default(&MortonOctantId::ROOT_OCTANT_ID).unwrap()[1];
		let level2 = octree.octants_mut().subdivide_with_default(&level1).unwrap()[2];
		let level3 = octree.octants_mut().subdivide_with_default(&level2).unwrap()[7];
		let level4 = octree.octants_mut().subdivide_with_default(&level3).unwrap()[5];
		octree.octants_mut().subdivide_with_default(&level4).unwrap()[6];

		// Test control-flow Break
		let SearchControlFlowResult::Continue(continue_step_id) = octree.depth_first_search_from_root(
			&mut |depth, octant_id: &MortonOctantId, octant_voxel|{
				let correct_morton_code = depth_first_visit_order_iter_all.next().expect("Depth first search visited more octants than defined.");
				assert_eq!(octant_id.as_morton(), correct_morton_code, "Depth first search visited octant with morton code different from defined.");
				assert_eq!(octant_id.compute_depth() as u8, depth);
				assert_eq!(octant_voxel.subdivision_depth(root_voxel.half_extent()) as u8, depth);
				SearchControlFlow::Continue
			}
		).expect("Broken root link,") else {
			panic!("Wrong last step.");
		};
		assert_eq!(continue_step_id.as_morton(), *depth_first_visit_order.last().unwrap());

		// Test control-flow Break
		const BREAKING_INDEX: usize = 30;
		let mut depth_first_visit_order_iter_break = depth_first_visit_order.iter()
			.enumerate()
			.filter(|(index, _)| *index <= BREAKING_INDEX)
			.map(|(_, morton_code)| *morton_code);

		let SearchControlFlowResult::Break(break_step_id) = octree.depth_first_search_from_root(
			&mut |depth, octant_id, octant_voxel|{
				let correct_morton_code = depth_first_visit_order_iter_break.next().expect("Depth first search visited more octants than defined.");
				assert_eq!(octant_id.as_morton(), correct_morton_code, "Depth first search visited octant with morton code different from defined.");
				assert_eq!(octant_id.compute_depth() as u8, depth);
				assert_eq!(octant_voxel.subdivision_depth(root_voxel.half_extent()) as u8, depth);
				if depth_first_visit_order[BREAKING_INDEX] == octant_id.as_morton(){
					SearchControlFlow::Break
				}
				else {
					SearchControlFlow::Continue
				}
			}
		).expect("Broken root link,") else {
			panic!("Wrong last step.");
		};
		assert_eq!(break_step_id.as_morton(), depth_first_visit_order[BREAKING_INDEX]);


		// Test control-flow Skip
		const SKIP_AFTER_DEPTH: Depth = 3;
		let mut depth_first_visit_order_iter_skip = depth_first_visit_order.iter()
			.map(|morton_code| MortonOctantId(*morton_code))
			.filter(|morton_code| morton_code.compute_depth() <= SKIP_AFTER_DEPTH)
			.map(| morton_code| morton_code.as_morton());

		let depth_first_last_visit = depth_first_visit_order.iter()
			.map(|morton_code| MortonOctantId(*morton_code))
			.filter(|morton_code| morton_code.compute_depth() <= SKIP_AFTER_DEPTH)
			.map(| morton_code| morton_code.as_morton())
			.last()
			.unwrap();

		let mut continue_skipping: bool = false;
		let SearchControlFlowResult::Skip(skip_step_id) = octree.depth_first_search_from_root(
			&mut |depth, octant_id, octant_voxel|{
				let correct_morton_code = depth_first_visit_order_iter_skip.next().expect("Depth first search visited more octants than defined.");
				assert_eq!(octant_id.as_morton(), correct_morton_code, "Depth first search visited octant with morton code different from defined.");
				assert_eq!(octant_id.compute_depth() as u8, depth);
				assert_eq!(octant_voxel.subdivision_depth(root_voxel.half_extent()) as u8, depth);
				if continue_skipping || octant_id.compute_depth() >= SKIP_AFTER_DEPTH{
					continue_skipping = true;
					SearchControlFlow::Skip
				}
				else {
					SearchControlFlow::Continue
				}
			}
		).expect("Broken root link,") else {
			panic!("Wrong last step.");
		};
		assert_eq!(skip_step_id.as_morton(), depth_first_last_visit);
	}

	#[test]
	fn test_depth_first_search_mut(){
		type OctantId =  <SpatialSparseOctreeHashed<NoData> as OctantIdTypeInfo>::OctantId;
		let mut octree: SpatialSparseOctreeHashed<NoData> = SpatialSparseOctreeHashed::with_root_voxel(VolumetricCube::new(Vec3A::new(0.0,0.0, 0.0), 1.0));
		let root_voxel = *octree.get_root_voxel();

		let depth_first_visit_order  = [
			1,
			8,
			9,
			72,
			73,
			74,
			592,
			593,
			594,
			595,
			596,
			597,
			598,
			599,
			4792,
			4793,
			4794,
			4795,
			4796,
			4797,
			38376,
			38377,
			38378,
			38379,
			38380,
			38381,
			38382,
			38383,
			4798,
			4799,
			75,
			76,
			77,
			78,
			79,
			10,
			11,	
			12,
			13,
			14,
			15,
		];

		let mut depth_first_visit_order_iter_all = depth_first_visit_order.iter()
			.map(|morton_code| *morton_code);



		let level1 = octree.octants_mut().subdivide_with_default(&MortonOctantId::ROOT_OCTANT_ID).unwrap()[1];
		let level2 = octree.octants_mut().subdivide_with_default(&level1).unwrap()[2];
		let level3 = octree.octants_mut().subdivide_with_default(&level2).unwrap()[7];
		let level4 = octree.octants_mut().subdivide_with_default(&level3).unwrap()[5];
		octree.octants_mut().subdivide_with_default(&level4).unwrap()[6];
	
		// Test control-flow Break
		let SearchControlFlowResult::Continue(continue_step_id) = octree.depth_first_search_from_root_mut(
			&mut |depth, octant_id: &MortonOctantId, octant_voxel, _|{
				let correct_morton_code = depth_first_visit_order_iter_all.next().expect("Depth first search visited more octants than defined.");
				assert_eq!(octant_id.as_morton(), correct_morton_code, "Depth first search visited octant with morton code different from defined.");
				assert_eq!(octant_id.compute_depth() as u8, depth);
				assert_eq!(octant_voxel.subdivision_depth(root_voxel.half_extent()) as u8, depth);
				SearchControlFlow::Continue
			}
		).expect("Broken root link,") else {
			panic!("Wrong last step.");
		};
		assert_eq!(continue_step_id.as_morton(), *depth_first_visit_order.last().unwrap());

		// Test control-flow Break
		const BREAKING_INDEX: usize = 30;
		let mut depth_first_visit_order_iter_break = depth_first_visit_order.iter()
			.enumerate()
			.filter(|(index, _)| *index <= BREAKING_INDEX)
			.map(|(_, morton_code)| *morton_code);

		let SearchControlFlowResult::Break(break_step_id) = octree.depth_first_search_from_root_mut(
			&mut |depth, octant_id, octant_voxel, _|{
				let correct_morton_code = depth_first_visit_order_iter_break.next().expect("Depth first search visited more octants than defined.");
				assert_eq!(octant_id.as_morton(), correct_morton_code, "Depth first search visited octant with morton code different from defined.");
				assert_eq!(octant_id.compute_depth() as u8, depth);
				assert_eq!(octant_voxel.subdivision_depth(root_voxel.half_extent()) as u8, depth);
				if depth_first_visit_order[BREAKING_INDEX] == octant_id.as_morton(){
					SearchControlFlow::Break
				}
				else {
					SearchControlFlow::Continue
				}
			}
		).expect("Broken root link,") else {
			panic!("Wrong last step.");
		};
		assert_eq!(break_step_id.as_morton(), depth_first_visit_order[BREAKING_INDEX]);


		// Test control-flow Skip
		const SKIP_AFTER_DEPTH: Depth = 3;
		let mut depth_first_visit_order_iter_skip = depth_first_visit_order.iter()
			.map(|morton_code| MortonOctantId(*morton_code))
			.filter(|morton_code| morton_code.compute_depth() <= SKIP_AFTER_DEPTH)
			.map(| morton_code| morton_code.as_morton());

		let depth_first_last_visit = depth_first_visit_order.iter()
			.map(|morton_code| MortonOctantId(*morton_code))
			.filter(|morton_code| morton_code.compute_depth() <= SKIP_AFTER_DEPTH)
			.map(| morton_code| morton_code.as_morton())
			.last()
			.unwrap();

		let mut continue_skipping: bool = false;
		let SearchControlFlowResult::Skip(skip_step_id) = octree.depth_first_search_from_root_mut(
			&mut |depth, octant_id, octant_voxel, _|{
				let correct_morton_code = depth_first_visit_order_iter_skip.next().expect("Depth first search visited more octants than defined.");
				assert_eq!(octant_id.as_morton(), correct_morton_code, "Depth first search visited octant with morton code different from defined.");
				assert_eq!(octant_id.compute_depth() as u8, depth);
				assert_eq!(octant_voxel.subdivision_depth(root_voxel.half_extent()) as u8, depth);
				if continue_skipping || octant_id.compute_depth() >= SKIP_AFTER_DEPTH{
					continue_skipping = true;
					SearchControlFlow::Skip
				}
				else {
					SearchControlFlow::Continue
				}
			}
		).expect("Broken root link,") else {
			panic!("Wrong last step.");
		};
		assert_eq!(skip_step_id.as_morton(), depth_first_last_visit);
	}

	#[test]
	fn test_breadth_first_search(){
		type OctantId =  <SpatialSparseOctreeHashed<NoData> as OctantIdTypeInfo>::OctantId;
		let mut octree: SpatialSparseOctreeHashed<NoData> = SpatialSparseOctreeHashed::with_root_voxel(VolumetricCube::new(Vec3A::new(0.0,0.0, 0.0), 1.0));
		let root_voxel = *octree.get_root_voxel();

		let breadth_first_visit_order  = [
			1_u64,
			8,
			9,
			10,
			11,
			12,
			13,
			14,
			15,
			72,
			73,
			74,
			75,
			76,
			77,
			78,
			79,
			592,
			593,
			594,
			595,
			596,
			597,
			598,
			599,
			4792,
			4793,
			4794,
			4795,
			4796,
			4797,
			4798,
			4799,
			38376,
			38377,
			38378,
			38379,
			38380,
			38381,
			38382,
			38383
		];

		let mut breadth_first_visit_order_iter_all = breadth_first_visit_order.iter()
			.map(|morton_code| *morton_code);

		let level1 = octree.octants_mut().subdivide_with_default(&MortonOctantId::ROOT_OCTANT_ID).unwrap()[1];
		let level2 = octree.octants_mut().subdivide_with_default(&level1).unwrap()[2];
		let level3 = octree.octants_mut().subdivide_with_default(&level2).unwrap()[7];
		let level4 = octree.octants_mut().subdivide_with_default(&level3).unwrap()[5];
		octree.octants_mut().subdivide_with_default(&level4).unwrap()[6];
		// Test control-flow Break
		let SearchControlFlowResult::Continue(continue_step_id) = octree.breadth_first_search_from_root(
			&mut |depth, octant_id, octant_voxel|{
				let correct_morton_code = breadth_first_visit_order_iter_all.next().expect("Depth first search visited more octants than defined.");
				assert_eq!(octant_id.as_morton(), correct_morton_code, "Depth first search visited octant with morton code different from defined.");
				assert_eq!(octant_id.compute_depth() as u8, depth);
				assert_eq!(octant_voxel.subdivision_depth(root_voxel.half_extent()) as u8, depth);
				SearchControlFlow::Continue
			}
		).expect("Broken root link,") else {
			panic!("Wrong last step.");
		};
		assert_eq!(continue_step_id.as_morton(), *breadth_first_visit_order.last().unwrap());

		/// Test control-flow Break
		const BREAKING_INDEX: usize = 30;
		let mut breadth_first_visit_order_iter_break = breadth_first_visit_order.iter()
			.enumerate()
			.filter(|(index, _)| *index <= BREAKING_INDEX)
			.map(|(_, morton_code)| *morton_code);

		let SearchControlFlowResult::Break(break_step_id) = octree.breadth_first_search_from_root(
			&mut |depth, octant_id, octant_voxel|{
				let correct_morton_code = breadth_first_visit_order_iter_break.next().expect("Depth first search visited more octants than defined.");
				assert_eq!(octant_id.as_morton(), correct_morton_code, "Depth first search visited octant with morton code different from defined.");
				assert_eq!(octant_id.compute_depth() as u8, depth);
				assert_eq!(octant_voxel.subdivision_depth(root_voxel.half_extent()) as u8, depth);
				if breadth_first_visit_order[BREAKING_INDEX] == octant_id.as_morton(){
					SearchControlFlow::Break
				}
				else {
					SearchControlFlow::Continue
				}
			}
		).expect("Broken root link,") else {
			panic!("Wrong last step.");
		};
		assert_eq!(break_step_id.as_morton(), breadth_first_visit_order[BREAKING_INDEX]);


		/// Test control-flow Skip
		const SKIP_AFTER_DEPTH: Depth = 3;
		let mut breadth_first_visit_order_iter_skip = breadth_first_visit_order.iter()
			.map(|morton_code| MortonOctantId(*morton_code))
			.filter(|morton_code| morton_code.compute_depth() <= SKIP_AFTER_DEPTH)
			.map(| morton_code| morton_code.as_morton());

		let breadth_first_last_visit = breadth_first_visit_order.iter()
			.map(|morton_code| MortonOctantId(*morton_code))
			.filter(|morton_code| morton_code.compute_depth() <= SKIP_AFTER_DEPTH)
			.map(| morton_code| morton_code.as_morton())
			.last()
			.unwrap();

		let SearchControlFlowResult::Skip(skip_step_id) = octree.breadth_first_search_from_root(
			&mut |depth, octant_id, octant_voxel|{
				let correct_morton_code = breadth_first_visit_order_iter_skip.next().expect("Depth first search visited more octants than defined.");
				assert_eq!(octant_id.as_morton(), correct_morton_code, "Depth first search visited octant with morton code different from defined.");
				assert_eq!(octant_id.compute_depth() as u8, depth);
				assert_eq!(octant_voxel.subdivision_depth(root_voxel.half_extent()) as u8, depth);
				if octant_id.compute_depth() >= SKIP_AFTER_DEPTH{
					SearchControlFlow::Skip
				}
				else {
					SearchControlFlow::Continue
				}
			}
		).expect("Broken root link,") else {
			panic!("Wrong last step.");
		};
		assert_eq!(skip_step_id.as_morton(), breadth_first_last_visit);
	}


	#[test]
	fn test_breadth_first_search_mut(){
		type OctantId =  <SpatialSparseOctreeHashed<NoData> as OctantIdTypeInfo>::OctantId;
		let mut octree: SpatialSparseOctreeHashed<NoData> = SpatialSparseOctreeHashed::with_root_voxel(VolumetricCube::new(Vec3A::new(0.0,0.0, 0.0), 1.0));
		let root_voxel = *octree.get_root_voxel();

		let breadth_first_visit_order  = [
			1_u64,
			8,
			9,
			10,
			11,
			12,
			13,
			14,
			15,
			72,
			73,
			74,
			75,
			76,
			77,
			78,
			79,
			592,
			593,
			594,
			595,
			596,
			597,
			598,
			599,
			4792,
			4793,
			4794,
			4795,
			4796,
			4797,
			4798,
			4799,
			38376,
			38377,
			38378,
			38379,
			38380,
			38381,
			38382,
			38383
		];

		let mut breadth_first_visit_order_iter_all = breadth_first_visit_order.iter()
			.map(|morton_code| *morton_code);

		let level1 = octree.octants_mut().subdivide_with_default(&MortonOctantId::ROOT_OCTANT_ID).unwrap()[1];
		let level2 = octree.octants_mut().subdivide_with_default(&level1).unwrap()[2];
		let level3 = octree.octants_mut().subdivide_with_default(&level2).unwrap()[7];
		let level4 = octree.octants_mut().subdivide_with_default(&level3).unwrap()[5];
		octree.octants_mut().subdivide_with_default(&level4).unwrap()[6];
		// Test control-flow Break
		let SearchControlFlowResult::Continue(continue_step_id) = octree.breadth_first_search_from_root_mut(
			|depth, octant_id, octant_voxel, _|{
				let correct_morton_code = breadth_first_visit_order_iter_all.next().expect("Depth first search visited more octants than defined.");
				assert_eq!(octant_id.as_morton(), correct_morton_code, "Depth first search visited octant with morton code different from defined.");
				assert_eq!(octant_id.compute_depth() as u8, depth);
				assert_eq!(octant_voxel.subdivision_depth(root_voxel.half_extent()) as u8, depth);
				SearchControlFlow::Continue
			}
		).expect("Broken root link,") else {
			panic!("Wrong last step.");
		};
		assert_eq!(continue_step_id.as_morton(), *breadth_first_visit_order.last().unwrap());

		/// Test control-flow Break
		const BREAKING_INDEX: usize = 30;
		let mut breadth_first_visit_order_iter_break = breadth_first_visit_order.iter()
			.enumerate()
			.filter(|(index, _)| *index <= BREAKING_INDEX)
			.map(|(_, morton_code)| *morton_code);

		let SearchControlFlowResult::Break(break_step_id) = octree.breadth_first_search_from_root_mut(
			|depth, octant_id, octant_voxel, _|{
				let correct_morton_code = breadth_first_visit_order_iter_break.next().expect("Depth first search visited more octants than defined.");
				assert_eq!(octant_id.as_morton(), correct_morton_code, "Depth first search visited octant with morton code different from defined.");
				assert_eq!(octant_id.compute_depth() as u8, depth);
				assert_eq!(octant_voxel.subdivision_depth(root_voxel.half_extent()) as u8, depth);
				if breadth_first_visit_order[BREAKING_INDEX] == octant_id.as_morton(){
					SearchControlFlow::Break
				}
				else {
					SearchControlFlow::Continue
				}
			}
		).expect("Broken root link,") else {
			panic!("Wrong last step.");
		};
		assert_eq!(break_step_id.as_morton(), breadth_first_visit_order[BREAKING_INDEX]);


		/// Test control-flow Skip
		const SKIP_AFTER_DEPTH: Depth = 3;
		let mut breadth_first_visit_order_iter_skip = breadth_first_visit_order.iter()
			.map(|morton_code| MortonOctantId(*morton_code))
			.filter(|morton_code| morton_code.compute_depth() <= SKIP_AFTER_DEPTH)
			.map(| morton_code| morton_code.as_morton());

		let breadth_first_last_visit = breadth_first_visit_order.iter()
			.map(|morton_code| MortonOctantId(*morton_code))
			.filter(|morton_code| morton_code.compute_depth() <= SKIP_AFTER_DEPTH)
			.map(| morton_code| morton_code.as_morton())
			.last()
			.unwrap();

		let SearchControlFlowResult::Skip(skip_step_id) = octree.breadth_first_search_from_root_mut(
			|depth, octant_id, octant_voxel, _|{
				let correct_morton_code = breadth_first_visit_order_iter_skip.next().expect("Depth first search visited more octants than defined.");
				assert_eq!(octant_id.as_morton(), correct_morton_code, "Depth first search visited octant with morton code different from defined.");
				assert_eq!(octant_id.compute_depth() as u8, depth);
				assert_eq!(octant_voxel.subdivision_depth(root_voxel.half_extent()) as u8, depth);
				if octant_id.compute_depth() >= SKIP_AFTER_DEPTH{
					SearchControlFlow::Skip
				}
				else {
					SearchControlFlow::Continue
				}
			}
		).expect("Broken root link,") else {
			panic!("Wrong last step.");
		};
		assert_eq!(skip_step_id.as_morton(), breadth_first_last_visit);
	}
	use modsvo::morton_based_storage::hashed_octant_storage::HashedOctantStorage;
	#[test]
	fn test_morton_depth_parents_children(){
		type OctantId =  <SparseOctreeHashed<NoData> as OctantIdTypeInfo>::OctantId;
		let mut octree: SpatialSparseOctreeHashed<NoData> = SpatialSparseOctreeHashed::new_with_root(VolumetricCube::new(Vec3A::ZERO, 1.0), NoData);

		let l1 = octree.octants_mut().subdivide_with_default(&MortonOctantId::ROOT_OCTANT_ID).unwrap()[2];
		let level1 = octree.octants_mut().subdivide_with_default(&MortonOctantId::ROOT_OCTANT_ID).unwrap()[1];
		let level2 = octree.octants_mut().subdivide_with_default(&level1).unwrap()[2];
		let level3 = octree.octants_mut().subdivide_with_default(&level2).unwrap()[7];
		let level4 = octree.octants_mut().subdivide_with_default(&level3).unwrap()[5];
		octree.octants_mut().subdivide_with_default(&level4).unwrap()[6];

		//octree = dbg!(octree);
		

		let mut count: usize = 0;
		let root_voxel = *octree.get_root_voxel();
		let a = octree.breadth_first_search_from_root_mut(
			|depth, octant_id: &MortonOctantId, voxel, octant_accessor|{

				for _ in 0..octant_id.compute_depth(){
				//	print!("\t");
				}
				println!("{:?}", octant_id);


				let current_voxel = SpatialSparseOctreeHashed::<NoData>::get_voxel_by_id_from_storage(octant_accessor, octant_id, &root_voxel).unwrap();
				assert_eq!(current_voxel.subdivision_depth(root_voxel.half_extent()), depth);
				
				

				let grid_position = root_voxel.grid_position(current_voxel.center(), depth);
				dbg!(current_voxel);
				dbg!(grid_position);
				dbg!(root_voxel.sub_volumetric_cube_from_grid_position(depth, grid_position));
				
				SearchControlFlow::Continue
				//Some(OctantPlacement::LOWER_BOTTOM_RIGHT)
			}
		);
		println!("\nLast id: {:?}", a.unwrap());
		println!("count: {:?}", count);
		dbg!(level4);
		octree.octants_mut().get_ancestors_for(&level4).unwrap().for_each(
			|octant_id|{
				dbg!(octant_id);
			}
		);
		
		let level4_voxel = dbg!(octree.get_voxel_by_id(&level4).unwrap());
		
		dbg!(level4_voxel.subdivision_depth(octree.get_root_voxel().half_extent()));
		dbg!(level4.compute_depth());

		dbg!(MortonOctantId::max_xyz_grid_size_in_depth(MortonOctantId(1).children_ids()[2].children_ids()[3].children_ids()[6].compute_depth()));
		dbg!(MortonOctantId(1).children_ids()[2].children_ids()[3].children_ids()[6].max_xyz_value());

		dbg!(MortonOctantId::from_xyz_array([0,0, u16::MAX], 16).unwrap().get_all_neighbors());

	}
}