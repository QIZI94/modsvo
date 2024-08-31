mod hash_sparse_octree;
mod hash_spatial_sparse_octree;
mod hash_octant_storage;




use modsvo::{
		morton_based_storage::hashed_octant_storage,
		octant_meta::OctantPlacement,
		octant_storage_trait::{
			ModifiableOctantStorage, OctantStorage, StorageError
		},
		octree_base::{
			SearchControlFlow, SearchControlFlowResult
		},
		Depth};

pub fn test_modifiable_octant_storage<Storage: ModifiableOctantStorage<Data = u32>>(storage: &mut Storage, repeat: u8) {
	if repeat == 0 {
		return;
	}
	else {
		println!("Running repeat of test_modifiable_octant_storage: {} ...", repeat);
	}
	let assigning_function = |assign_to_placement: OctantPlacement|{
		assign_to_placement as usize as u32
	};

	let reverse_assigning_function = |assign_to_placement: OctantPlacement|{
		(OctantPlacement::OCTANTS_COUNT - 1 - assign_to_placement as usize) as u32
	};

	if storage.get_max_depth() == 0 {
		panic!("Max depth returned '0', this will stop depth and breath search from working correctly, if this is intended just comment this part out.");
	}
	
	let root_data: &mut u32 = storage.get_octant_mut(&storage.get_root_id())
		.expect("Root octant not inserted or inserted.");
	*root_data = 0;
	
	let children_check1: [Storage::OctantId; OctantPlacement::OCTANTS_COUNT] = storage.subdivide(&storage.get_root_id(), assigning_function).unwrap();

	OctantPlacement::OCTANTS_ORDERED.iter()
		.enumerate()
		.for_each(
			|(index, &octant_placement)|{
				let child_data = storage.get_octant(&children_check1[index])
					.expect("Subdivide failed to create all children.");
				// this is testing child octant order
				assert_eq!(*child_data, octant_placement as usize as u32);
			}
		);

	let children_reverse_check: [Storage::OctantId; OctantPlacement::OCTANTS_COUNT] = storage.subdivide(&storage.get_root_id(), reverse_assigning_function).unwrap();

	OctantPlacement::OCTANTS_ORDERED.iter()
		.rev()
		.enumerate()
		.for_each(
			|(index, &octant_placement)|{
				let child_data = storage.get_octant(&children_check1[index])
					.expect("Subdivide failed to create all children.");
				// this is testing child octant order
				assert_eq!(*child_data, octant_placement as usize as u32);
			}
		);
	let grand_child: Storage::OctantId = storage.subdivide(
		&children_reverse_check[OctantPlacement::UPPER_BOTTOM_RIGHT as usize],
		assigning_function).unwrap()
		[OctantPlacement::LOWER_TOP_LEFT as usize];

	const FIRST_INSERT_VALUE: u32 = 11; 
	let Ok((first_insert_child, None)) = storage.insert_octant(&grand_child, OctantPlacement::LOWER_TOP_RIGHT, FIRST_INSERT_VALUE) else {
		panic!("First insert returned data that should not have been added yet.");
	};

	const SECOND_INSERT_VALUE: u32 = 20; 
	let Ok((second_insert_child, Some(old_data))) = storage.insert_octant(&grand_child, OctantPlacement::LOWER_TOP_RIGHT, SECOND_INSERT_VALUE) else {
		panic!("First insert was not successful");
	};
	assert_eq!(old_data, FIRST_INSERT_VALUE);

	let new_data = storage.get_octant(&second_insert_child)
		.expect("Previous insert did not insert data into persistent storage.");

	assert_eq!(*new_data, SECOND_INSERT_VALUE);

	if first_insert_child != second_insert_child{
		panic!("Second insert produced different ID which isn't allowed for usage purposes, when user wants to refer to same data multiple times.");
	}
	const CHANGED_VALUE: u32 = 33;
	let mut_data = storage.get_octant_mut(&second_insert_child).unwrap();
	*mut_data = CHANGED_VALUE;

	let changed_data = storage.get_octant(&second_insert_child).unwrap();
	assert_eq!(*changed_data, CHANGED_VALUE);
	
	assert_eq!(storage.get_octant_depth(&second_insert_child), Some(3));
	assert_eq!(storage.get_octant_depth(&grand_child), Some(2));
	assert_eq!(storage.get_octant_depth(&storage.get_root_id()), Some(0));


	let maybe_correct_child = storage.get_existing_child(&grand_child, OctantPlacement::LOWER_TOP_RIGHT)
		.expect("Wrong child placement when inserting.");
	
	if maybe_correct_child != second_insert_child {
		panic!("Wrong child octant_id.")
	}

	storage.get_existing_children(&grand_child).unwrap()
		.iter()
		.flatten()
		.for_each(
			|child_id|{
				if *child_id != second_insert_child {
					panic!("Wrong children linking.");
				}
			}
		);
	
	storage.get_ancestors_for(&second_insert_child)
		.expect("Couldn't get ancestors.")
		.zip(
			[grand_child, children_reverse_check[OctantPlacement::UPPER_BOTTOM_RIGHT as usize], storage.get_root_id()]
		)
		.for_each(
			|(ancestor_id, reference_id)| {
				if ancestor_id != reference_id{
					panic!("Ancestor linking broken.");
				}
			}
		);
	
	let child_placement = storage.which_child_of(&children_reverse_check[OctantPlacement::UPPER_BOTTOM_RIGHT as usize], &grand_child)
		.expect("which_child_of didn't find placement from valid parent_id and child_id.");

	assert_eq!(child_placement as usize, OctantPlacement::LOWER_TOP_LEFT as usize);

	
	{	
		//const DEPTH_INSERT_VALUE: u32 = 11;
		let mut next_decent_id = storage.get_root_id();
	
		let max_depth = storage.get_max_depth();
		for depth in  0 ..= (max_depth as u16 +1) {
			let insertion_result = storage.insert_octant(&next_decent_id, OctantPlacement::UPPER_TOP_LEFT, SECOND_INSERT_VALUE);
			//dbg!(depth);
			match insertion_result {
				Ok((new_octant_id, _)) => next_decent_id = new_octant_id,
				Err(StorageError::OverMaxDepth(max)) => {
					assert_eq!(max, max_depth);
					if max_depth as u16 == depth {
						break;
					}
					else {
						panic!("Depth error was given sooner than max depth was reached {}", max_depth);
					}
				},
				_ => {
					panic!("Wrong error given.");
				}
			}
			
			next_decent_id  = insertion_result.unwrap().0;

						
		}
		
	}
	
	
	let grand_grand_children = storage.get_existing_children(&grand_child).unwrap();
	storage.remove_octant(&grand_child);
	if let Some(_) = storage.get_octant(&grand_child) {
		panic!("Removing top octant failed.");
	}

	grand_grand_children.iter()
		.flatten()
		.filter_map(
			|child_id|{
				storage.get_octant(child_id)
			}
		)
		.for_each(
			|_|{
				panic!("Removing octant's children failed(children accessible after parent removal).");
			}
		);

	if let Some(_) = storage.get_octant(&second_insert_child) {
		panic!("Removing children octants failed.");
	}

	storage.remove_octant(&storage.get_root_id());

	let root_exist = storage.get_octant(&storage.get_root_id());
	assert_eq!(root_exist, None);

	let root_children_exist = storage.get_existing_children(&storage.get_root_id()).iter()
		.flatten()
		.last()
		.is_some();
	assert_eq!(root_children_exist, false);


	const ROOD_DATA: u32 = 55; 
	storage.insert_root(ROOD_DATA);
	let last_root_data = storage.get_octant(&storage.get_root_id()).unwrap();
	assert_eq!(ROOD_DATA, *last_root_data);

	test_modifiable_octant_storage(storage, repeat - 1);

}
