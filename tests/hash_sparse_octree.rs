


#[cfg(test)]
mod tests{			
	#[derive(Debug, Default)]
	struct NoData;
    use glam::Vec3A;
	
	

    use modsvo::{octant_meta::OctantPlacement,octant_storage_trait::{OctantStorage, ModifiableOctantStorage}, octree_base::{SearchControlFlow, SearchControlFlowResult, SubdivisionControlFlow}, voxels::voxel_cube::VolumetricCube, Depth, SparseOctreeHashed};

	use modsvo::morton_based_storage::morton_octant_id::MortonOctantId;
	use modsvo::octree_base::OctantIdTypeInfo;

	#[test]
	fn test_depth_first_search(){
		type OctantId =  <SparseOctreeHashed<NoData> as OctantIdTypeInfo>::OctantId;
		let mut octree: SparseOctreeHashed<NoData> = SparseOctreeHashed::default();
		
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



		let level1 = octree.octants.subdivide_with_default(&MortonOctantId::ROOT_OCTANT_ID).unwrap()[1];
		let level2 = octree.octants.subdivide_with_default(&level1).unwrap()[2];
		let level3 = octree.octants.subdivide_with_default(&level2).unwrap()[7];
		let level4 = octree.octants.subdivide_with_default(&level3).unwrap()[5];
		octree.octants.subdivide_with_default(&level4).unwrap()[6];

		// Test control-flow Break
		let SearchControlFlowResult::Continue(continue_step_id) = octree.depth_first_search_from_root(
			|depth, octant_id: &MortonOctantId|{
				let correct_morton_code = depth_first_visit_order_iter_all.next().expect("Depth first search visited more octants than defined.");
				assert_eq!(octant_id.as_morton(), correct_morton_code, "Depth first search visited octant with morton code different from defined.");
				assert_eq!(octant_id.compute_depth() as u8, depth);
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
			|depth, octant_id: &MortonOctantId|{
				let correct_morton_code = depth_first_visit_order_iter_break.next().expect("Depth first search visited more octants than defined.");
				assert_eq!(octant_id.as_morton(), correct_morton_code, "Depth first search visited octant with morton code different from defined.");
				assert_eq!(octant_id.compute_depth() as u8, depth);
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
			|depth, octant_id: &MortonOctantId|{
				let correct_morton_code = depth_first_visit_order_iter_skip.next().expect("Depth first search visited more octants than defined.");
				assert_eq!(octant_id.as_morton(), correct_morton_code, "Depth first search visited octant with morton code different from defined.");
				assert_eq!(octant_id.compute_depth() as u8, depth);
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
		type OctantId =  <SparseOctreeHashed<NoData> as OctantIdTypeInfo>::OctantId;
		let mut octree: SparseOctreeHashed<NoData> = SparseOctreeHashed::default();

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



		let level1 = octree.octants.subdivide_with_default(&MortonOctantId::ROOT_OCTANT_ID).unwrap()[1];
		let level2 = octree.octants.subdivide_with_default(&level1).unwrap()[2];
		let level3 = octree.octants.subdivide_with_default(&level2).unwrap()[7];
		let level4 = octree.octants.subdivide_with_default(&level3).unwrap()[5];
		octree.octants.subdivide_with_default(&level4).unwrap()[6];
	
		// Test control-flow Break
		let SearchControlFlowResult::Continue(continue_step_id) = octree.depth_first_search_from_root_mut(
			|depth, octant_id: &MortonOctantId, _|{
				let correct_morton_code = depth_first_visit_order_iter_all.next().expect("Depth first search visited more octants than defined.");
				assert_eq!(octant_id.as_morton(), correct_morton_code, "Depth first search visited octant with morton code different from defined.");
				assert_eq!(octant_id.compute_depth() as u8, depth);
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
			|depth, octant_id: &MortonOctantId, a|{
				let correct_morton_code = depth_first_visit_order_iter_break.next().expect("Depth first search visited more octants than defined.");
				assert_eq!(octant_id.as_morton(), correct_morton_code, "Depth first search visited octant with morton code different from defined.");
				assert_eq!(octant_id.compute_depth() as u8, depth);
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
			|depth, octant_id: &MortonOctantId, hhh|{
				let correct_morton_code = depth_first_visit_order_iter_skip.next().expect("Depth first search visited more octants than defined.");
				assert_eq!(octant_id.as_morton(), correct_morton_code, "Depth first search visited octant with morton code different from defined.");
				assert_eq!(octant_id.compute_depth() as u8, depth);
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
		type OctantId =  <SparseOctreeHashed<NoData> as OctantIdTypeInfo>::OctantId;
		let mut octree: SparseOctreeHashed<NoData> = SparseOctreeHashed::default();

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

		let level1 = octree.octants.subdivide_with_default(&MortonOctantId::ROOT_OCTANT_ID).unwrap()[1];
		let level2 = octree.octants.subdivide_with_default(&level1).unwrap()[2];
		let level3 = octree.octants.subdivide_with_default(&level2).unwrap()[7];
		let level4 = octree.octants.subdivide_with_default(&level3).unwrap()[5];
		octree.octants.subdivide_with_default(&level4).unwrap()[6];
		// Test control-flow Break
		let SearchControlFlowResult::Continue(continue_step_id) = octree.breadth_first_search_from_root(
			|depth, octant_id: &MortonOctantId|{
				let correct_morton_code = breadth_first_visit_order_iter_all.next().expect("Depth first search visited more octants than defined.");
				assert_eq!(octant_id.as_morton(), correct_morton_code, "Depth first search visited octant with morton code different from defined.");
				assert_eq!(octant_id.compute_depth() as u8, depth);
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
			|depth, octant_id: &MortonOctantId|{
				let correct_morton_code = breadth_first_visit_order_iter_break.next().expect("Depth first search visited more octants than defined.");
				assert_eq!(octant_id.as_morton(), correct_morton_code, "Depth first search visited octant with morton code different from defined.");
				assert_eq!(octant_id.compute_depth() as u8, depth);
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
			|depth, octant_id: &MortonOctantId|{
				let correct_morton_code = breadth_first_visit_order_iter_skip.next().expect("Depth first search visited more octants than defined.");
				assert_eq!(octant_id.as_morton(), correct_morton_code, "Depth first search visited octant with morton code different from defined.");
				assert_eq!(octant_id.compute_depth() as u8, depth);
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
		type OctantId =  <SparseOctreeHashed<NoData> as OctantIdTypeInfo>::OctantId;
		let mut octree: SparseOctreeHashed<NoData> = SparseOctreeHashed::default();

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

		let level1 = octree.octants.subdivide_with_default(&MortonOctantId::ROOT_OCTANT_ID).unwrap()[1];
		let level2 = octree.octants.subdivide_with_default(&level1).unwrap()[2];
		let level3 = octree.octants.subdivide_with_default(&level2).unwrap()[7];
		let level4 = octree.octants.subdivide_with_default(&level3).unwrap()[5];
		octree.octants.subdivide_with_default(&level4).unwrap()[6];
		// Test control-flow Break
		let SearchControlFlowResult::Continue(continue_step_id) = octree.breadth_first_search_from_root_mut(
			|depth, octant_id: &MortonOctantId, _|{
				let correct_morton_code = breadth_first_visit_order_iter_all.next().expect("Depth first search visited more octants than defined.");
				assert_eq!(octant_id.as_morton(), correct_morton_code, "Depth first search visited octant with morton code different from defined.");
				assert_eq!(octant_id.compute_depth() as u8, depth);
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
			|depth, octant_id: &MortonOctantId, _|{
				let correct_morton_code = breadth_first_visit_order_iter_break.next().expect("Depth first search visited more octants than defined.");
				assert_eq!(octant_id.as_morton(), correct_morton_code, "Depth first search visited octant with morton code different from defined.");
				assert_eq!(octant_id.compute_depth() as u8, depth);
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
			|depth, octant_id: &MortonOctantId, _|{
				let correct_morton_code = breadth_first_visit_order_iter_skip.next().expect("Depth first search visited more octants than defined.");
				assert_eq!(octant_id.as_morton(), correct_morton_code, "Depth first search visited octant with morton code different from defined.");
				assert_eq!(octant_id.compute_depth() as u8, depth);
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
	fn test_morton_depth_parents_children(){
		type OctantId =  <SparseOctreeHashed<NoData> as OctantIdTypeInfo>::OctantId;
		let mut octree: SparseOctreeHashed<NoData> = SparseOctreeHashed::default();

		let l1 = octree.octants.subdivide_with_default(&MortonOctantId::ROOT_OCTANT_ID).unwrap()[2];
		let level1 = octree.octants.subdivide_with_default(&MortonOctantId::ROOT_OCTANT_ID).unwrap()[1];
		let level2 = octree.octants.subdivide_with_default(&level1).unwrap()[2];
		let level3 = octree.octants.subdivide_with_default(&level2).unwrap()[7];
		let level4 = octree.octants.subdivide_with_default(&level3).unwrap()[5];
		octree.octants.subdivide_with_default(&level4).unwrap()[6];

		//octree = dbg!(octree);
		

		let mut count: usize = 0;
		let a = octree.breadth_first_search_from_root(
			|depth, octant_id: &MortonOctantId|{
				/*for child_placement in OctantPlacement::OCTANTS_ORDERED {
					if octree.get_existing_child(octant_id, child_placement).is_some() {
						return Some(child_placement);
					}
				}
				count += 1;
				return None;*/
				for _ in 0..octant_id.compute_depth(){
					print!("\t");
				}
				println!("{:?}", octant_id);
				SearchControlFlow::Continue
				//Some(OctantPlacement::UPPER_TOP_RIGHT)
			}
		);
		println!("\nLast id: {:?}", a.unwrap());
		println!("count: {:?}", count);
		dbg!(level4);
		octree.octants.get_ancestors_for(&level4).unwrap().for_each(
			|octant_id|{
				dbg!(octant_id);
			}
		);
		let root_id: MortonOctantId = octree.octants.get_root_id();
			octree.subdivide_if_some(
			&root_id,
			|depth, b, c|{
				if depth >= 3 {
					SubdivisionControlFlow::Skip
				}
				else {
					SubdivisionControlFlow::Continue(move |o: OctantPlacement| Some(NoData))
				}				
			}
		);

		octree.breadth_first_search_from_root(
			|depth, octant_id: &MortonOctantId|{
				/*for child_placement in OctantPlacement::OCTANTS_ORDERED {
					if octree.get_existing_child(octant_id, child_placement).is_some() {
						return Some(child_placement);
					}
				}
				count += 1;
				return None;*/
				for _ in 0..octant_id.compute_depth(){
					print!("\t");
				}
				println!("{:?}", octant_id);
				SearchControlFlow::Continue
				//Some(OctantPlacement::UPPER_TOP_RIGHT)
			}
		);
		//let level4_voxel = dbg!(octree.get_voxel_by_id(&level4).unwrap());
		
		//dbg!(level4_voxel.subdivision_depth(octree.get_root_voxel().half_extent()));
		//dbg!(level4.compute_depth());

		/*
		dbg!(&level4.compute_depth());
		dbg!(octree.get_root_voxel().subdivision_depth(octree.get_root_voxel().half_extent()));
		dbg!(level4_voxel.grid_position(level4_voxel.center(), 4));
		dbg!(octree.octants_mut().subdivide_with_default(&MortonOctantId::ROOT_OCTANT_ID));
		dbg!(MortonOctantId(0).children_ids()[0].children_ids());
		dbg!(MortonOctantId::ROOT_OCTANT_ID.children_ids()[0]);
		dbg!(MortonOctantId::ROOT_OCTANT_ID.as_morton() << 3);
		dbg!(MortonOctantId::ROOT_OCTANT_ID.as_morton() << 3 & !MortonOctantId::ROOT_OCTANT_ID.as_morton() << 3);
		//dbg!(MortonOctantId((2_u64 | 1_u64 | 16_u64).reverse_bits()| 256_u64).children_ids());
		//dbg!((8_u64 | 4_u64 | 2_u64 | 1_u64).reverse_bits());
		*/
	}
}