use std::collections::VecDeque;

use super::octant_meta::OctantPlacement;
use super::octant_meta::Depth;
use super::octant_storage_trait::{
	ModifiableOctantStorage,
	OctantStorage,
	OctantStorageAccessorMut,
	StorageError,
	StorageResult
};



#[derive(Debug)]
pub enum SearchControlFlow {
	/// Continue traversing the tree.
	Continue,
	/// Stop traversing the tree.
	Break,
	/// Stop going deeper into tree structure for current branch.
	Skip
}

impl SearchControlFlow {
	pub fn to_result<OctantId>(&self, octant_id: OctantId) -> SearchControlFlowResult<OctantId>{
		match *self {
			Self::Continue => SearchControlFlowResult::Continue(octant_id),
			Self::Skip => SearchControlFlowResult::Skip(octant_id),
			Self::Break => SearchControlFlowResult::Break(octant_id)
		}
	}
}

#[derive(Debug)]
pub enum SearchControlFlowResult<OctantId> {
	/// Continue traversing the tree.
	Continue(OctantId),
	/// Stop traversing the tree.
	Break(OctantId),
	/// Stop going deeper into tree structure for current branch.
	Skip(OctantId)
}

#[derive(Debug)]
pub enum AssignmentControlFlow<T> {
	/// Assign value for next octant data, create next if doesn't exist.
	AssignNext(OctantPlacement, T),
	/// Assign value for next octant data, but only when next does exist.
	AssignNextExisting(OctantPlacement, T),
	/// Assign value for next octant data, but only when next is have to be created.
	AssignNextWhenNew(OctantPlacement, T),
}

impl<T> AssignmentControlFlow<T> {
	pub fn get_assigned(&self) -> (OctantPlacement, &T){
		match self {
			AssignmentControlFlow::AssignNext(octant_placement, data) => (*octant_placement, data),
			AssignmentControlFlow::AssignNextExisting(octant_placement, data) => (*octant_placement, data),
			AssignmentControlFlow::AssignNextWhenNew(octant_placement, data) => (*octant_placement, data),
		}
	}

	pub fn get_assigned_mut(&mut self) -> (OctantPlacement, &mut T){
		match self {
			AssignmentControlFlow::AssignNext(octant_placement, data) => (*octant_placement, data),
			AssignmentControlFlow::AssignNextExisting(octant_placement, data) => (*octant_placement, data),
			AssignmentControlFlow::AssignNextWhenNew(octant_placement, data) => (*octant_placement, data),
		}
	}

	pub fn take(self) -> (OctantPlacement, T) {
		match self {
			AssignmentControlFlow::AssignNext(octant_placement, data) => (octant_placement, data),
			AssignmentControlFlow::AssignNextExisting(octant_placement, data) => (octant_placement, data),
			AssignmentControlFlow::AssignNextWhenNew(octant_placement, data) => (octant_placement, data),
		}
	}
}

#[derive(Debug)]
pub enum SubdivisionControlFlow<T> {
	/// Continue traversing the tree.
	Continue(T),
	/// Stop traversing the tree.
	Break,
	/// Stop going deeper into tree structure for current branch.
	Skip
}

impl<T> SubdivisionControlFlow<T> {
	pub fn to_result<OctantId>(&self, octant_id: OctantId) -> SearchControlFlowResult<OctantId>{
		match self {
			Self::Continue(_) => SearchControlFlowResult::Continue(octant_id),
			Self::Skip => SearchControlFlowResult::Skip(octant_id),
			Self::Break => SearchControlFlowResult::Break(octant_id)
		}
	}
}


pub fn guided_search_from_storage<Storage: OctantStorage, F>(storage: &Storage, octant_id: &Storage::OctantId, mut guide_func: F)  -> StorageResult<Storage::OctantId>
where F: FnMut(Depth, &Storage::OctantId) -> Option<OctantPlacement> {
	let start_from_depth = storage.get_octant_depth(octant_id).ok_or(StorageError::InvalidOctantId)?;
	let mut current_octant_id = octant_id.clone();
	let max_depth = storage.get_max_depth();
	for depth in start_from_depth ..= max_depth{
		let Some(next_child_octant) = guide_func(depth, &current_octant_id) else {
			return Ok(current_octant_id);	
		};	

		let Ok(next_child_id) = storage.get_existing_child(&current_octant_id, next_child_octant) else {
			return Ok(current_octant_id);
		};

		current_octant_id = next_child_id;
	}
	
	Err(StorageError::OverMaxDepth(max_depth))
	//panic!("Octree traversal is stuck in loop. Check implementation of OctantStorage for bugs.");
}

pub fn guided_search_from_storage_mut<Storage: OctantStorage, F>(storage: &mut Storage, octant_id: &Storage::OctantId, mut guide_func: F)  -> StorageResult<Storage::OctantId>
where F: FnMut(Depth, &Storage::OctantId, &mut OctantStorageAccessorMut<Storage>) -> Option<OctantPlacement> {
	let start_from_depth = storage.get_octant_depth(octant_id).ok_or(StorageError::InvalidOctantId)?;
	let mut current_octant_id = octant_id.clone();
	let max_depth: Depth = storage.get_max_depth();
	let mut storage_accessor = OctantStorageAccessorMut::<Storage>::new(storage);
	for depth in start_from_depth ..= max_depth {
		let Some(next_child_octant) = guide_func(depth, &current_octant_id, &mut storage_accessor) else {
			return Ok(current_octant_id);	
		};	

		let Ok(next_child_id) = storage_accessor.get_existing_child(&current_octant_id, next_child_octant) else {
			return Ok(current_octant_id);
		};

		current_octant_id = next_child_id;
	}

	Err(StorageError::OverMaxDepth(max_depth))
	//panic!("Octree traversal is stuck in loop. Check implementation of OctantStorage for bugs.");
}

pub fn depth_first_search_from_storage<Storage: OctantStorage, F>(storage: &Storage, octant_id: &Storage::OctantId, mut search_func: F)  -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
where F: FnMut(Depth, &Storage::OctantId) -> SearchControlFlow {
	let start_from_depth = storage.get_octant_depth(octant_id).ok_or(StorageError::InvalidOctantId)?;
	depth_first_search_recursive(storage, start_from_depth, octant_id, &mut search_func)
}

fn depth_first_search_recursive<Storage: OctantStorage, F>(storage: &Storage, start_from_depth: Depth, octant_id: &Storage::OctantId, search_func: &mut F)  -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
where F: FnMut(Depth, &Storage::OctantId) -> SearchControlFlow {
	let next_step: SearchControlFlow = search_func(start_from_depth, octant_id);
	match next_step {
		SearchControlFlow::Continue => {
			let max_depth: Depth = storage.get_max_depth();
			if start_from_depth >= max_depth {
				return Err(StorageError::OverMaxDepth(max_depth));
			}

			let child_depth = start_from_depth + 1;
			
			let mut last_result: SearchControlFlowResult<Storage::OctantId> = next_step.to_result(*octant_id);
		
			for child_placement in OctantPlacement::OCTANTS_ORDERED {
				let Ok(child_id) = storage.get_existing_child(octant_id, child_placement) else {
					continue;
				};
				let child_step = depth_first_search_recursive(storage, child_depth, &child_id, search_func)?;
			
					//.expect("Broken children link.");
				match child_step {
					SearchControlFlowResult::Continue(step_id) => {
						last_result = SearchControlFlowResult::Continue(step_id);
					},
					SearchControlFlowResult::Skip(step_id) => {
						last_result = SearchControlFlowResult::Skip(step_id);
					},
					SearchControlFlowResult::Break(step_id) => {
						return Ok(SearchControlFlowResult::Break(step_id));
					}
				}
			}
			return Ok(last_result)
		},
		_ => {
			return Ok(next_step.to_result(*octant_id))
		}
	}			
}

pub fn depth_first_search_from_storage_mut<Storage: OctantStorage, F>(storage: &mut Storage, octant_id: &Storage::OctantId, mut search_func: F)  -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
where F: FnMut(Depth, &Storage::OctantId, &mut OctantStorageAccessorMut<Storage>) -> SearchControlFlow {
	let start_from_depth = storage.get_octant_depth(octant_id).ok_or(StorageError::InvalidOctantId)?;
	depth_first_search_recursive_mut(storage, start_from_depth, octant_id, &mut search_func)
}

fn depth_first_search_recursive_mut<Storage: OctantStorage, F>(storage: &mut Storage, start_from_depth: Depth, octant_id: &Storage::OctantId, search_func: &mut F)  -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
where F: FnMut(Depth, &Storage::OctantId, &mut OctantStorageAccessorMut<Storage>) -> SearchControlFlow {
	let mut storage_accessor = OctantStorageAccessorMut::<Storage>::new(storage);
	let next_step: SearchControlFlow = search_func(start_from_depth, octant_id, &mut storage_accessor);
	match next_step {
		SearchControlFlow::Continue => {
			let max_depth: Depth = storage.get_max_depth();
			if start_from_depth >= max_depth {
				return Err(StorageError::OverMaxDepth(max_depth));
			}
			let child_depth = start_from_depth + 1;

			let mut last_result: SearchControlFlowResult<Storage::OctantId> = next_step.to_result(*octant_id);
		
			for child_placement in OctantPlacement::OCTANTS_ORDERED {
				let Ok(child_id) = storage.get_existing_child(octant_id, child_placement) else {
					continue;
				};
				let child_step = depth_first_search_recursive_mut(storage,child_depth, &child_id, search_func)?;

				match child_step {
					SearchControlFlowResult::Continue(step_id) => {
						last_result = SearchControlFlowResult::Continue(step_id);
					},
					SearchControlFlowResult::Skip(step_id) => {
						last_result = SearchControlFlowResult::Skip(step_id);
					},
					SearchControlFlowResult::Break(step_id) => {
						return Ok(SearchControlFlowResult::Break(step_id));
					}
				}					
			}
			return Ok(last_result)
		},
		_ => {
			return Ok(next_step.to_result(*octant_id))
		}
	}			
}


pub fn breadth_first_search_from_storage<Storage: OctantStorage, F>(storage: &Storage, start_from_id: &Storage::OctantId, mut search_func: F) -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
where F: FnMut(Depth, &Storage::OctantId) -> SearchControlFlow {
	let start_from_depth = storage.get_octant_depth(start_from_id).ok_or(StorageError::InvalidOctantId)?;
	let max_depth: Depth = storage.get_max_depth();
	let mut to_be_visited: VecDeque<(Depth, Storage::OctantId)> = VecDeque::new();
	to_be_visited.push_back((start_from_depth, *start_from_id));
	
	let mut last_visited_step: Option<SearchControlFlowResult<Storage::OctantId>> = None;
	while let Some((depth, octant_id)) = to_be_visited.pop_front() {
		let next_step: SearchControlFlow = search_func(depth, &octant_id);
		match next_step {
			SearchControlFlow::Continue => {
				last_visited_step = Some(next_step.to_result(octant_id));
			},
			SearchControlFlow::Break => return Ok(next_step.to_result(octant_id)),
			SearchControlFlow::Skip => {
				last_visited_step = Some(next_step.to_result(octant_id));
				continue;
			}
		};

		if start_from_depth >= max_depth {
			return Err(StorageError::OverMaxDepth(max_depth));
		}

		let child_depth = depth + 1;
		storage.get_existing_children(&octant_id).expect("Broken children link.")
			.iter()
			.flatten()
			.for_each(
				|&child_id|{
					to_be_visited.push_back((child_depth, child_id));
				}
			);
	}
	last_visited_step.ok_or(StorageError::InvalidOctantId)
}

pub fn breadth_first_search_from_storage_mut<Storage: OctantStorage, F>(storage: &mut Storage, start_from_id: &Storage::OctantId, mut search_func: F) -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
where F: FnMut(Depth, &Storage::OctantId, &mut OctantStorageAccessorMut<Storage>) -> SearchControlFlow {
	let start_from_depth = storage.get_octant_depth(start_from_id).ok_or(StorageError::InvalidOctantId)?;
	let max_depth: Depth = storage.get_max_depth();
	let mut storage_accessor = OctantStorageAccessorMut::<Storage>::new(storage);
	let mut to_be_visited: VecDeque<(Depth, Storage::OctantId)> = VecDeque::new();
	to_be_visited.push_back((start_from_depth, *start_from_id));
	
	let mut last_visited_step: Option<SearchControlFlowResult<Storage::OctantId>> = None;
	while let Some((depth, octant_id)) = to_be_visited.pop_front() {
		let next_step: SearchControlFlow = search_func(depth, &octant_id, &mut storage_accessor);
		match next_step {
			SearchControlFlow::Continue => {
				last_visited_step = Some(next_step.to_result(octant_id));
			},
			SearchControlFlow::Break => return Ok(next_step.to_result(octant_id)),
			SearchControlFlow::Skip => {
				last_visited_step = Some(next_step.to_result(octant_id));
				continue;
			}
		};

		if start_from_depth >= max_depth {
			return Err(StorageError::OverMaxDepth(max_depth));
		}

		let child_depth = depth + 1;
		storage_accessor.get_existing_children(&octant_id).expect("Broken children link.")
			.iter()
			.flatten()
			.for_each(
				|&child_id|{
					to_be_visited.push_back((child_depth, child_id));
				}
			);
	}
	last_visited_step.ok_or(StorageError::InvalidOctantId)
}

pub fn drill_from_storage<Storage: ModifiableOctantStorage, F>(storage: &mut Storage, octant_id: &Storage::OctantId, mut drill_func: F)  -> StorageResult<Storage::OctantId>
where F: FnMut(Depth, &Storage::OctantId, &mut OctantStorageAccessorMut<Storage>) -> Option<AssignmentControlFlow<Storage::Data>> {
	let start_from_depth: Depth = storage.get_octant_depth(&octant_id).ok_or(StorageError::InvalidOctantId)?;
	let mut current_octant_id = octant_id.clone();
	let max_depth: Depth = storage.get_max_depth();

	for depth in start_from_depth ..= max_depth{
		let mut storage_accessor = OctantStorageAccessorMut::<Storage>::new(storage);
		let Some(assignment_controlflow) = drill_func(depth, &current_octant_id, &mut storage_accessor) else {
			return Ok(current_octant_id);
		};

		let next_child_id = match assignment_controlflow {
			AssignmentControlFlow::AssignNext(next_child_placement, new_custom_data) => {
				let (child_id, _) = storage.insert_octant(&current_octant_id, next_child_placement, new_custom_data)?;
				child_id
			}
			AssignmentControlFlow::AssignNextExisting(next_child_placement, mut new_custom_data) => {
				
				let Ok(child_id) = storage.get_existing_child(&current_octant_id, next_child_placement) else {
					return Ok(current_octant_id);
				};
				
				let Some(original_custom_data) = storage.get_octant_mut(&child_id) else {
					return Ok(current_octant_id);
				};

				std::mem::swap(original_custom_data, &mut new_custom_data);
				child_id
			}
			AssignmentControlFlow::AssignNextWhenNew(next_child_placement, new_custom_data) => {
				let Ok(child_id) = storage.get_existing_child(&current_octant_id, next_child_placement) else {
					return Ok(current_octant_id);
				};
				
				if let Some(_) = storage.get_octant(&child_id){
					return Ok(current_octant_id);
				}

				let _ = storage.insert_octant(&current_octant_id, next_child_placement, new_custom_data);
				child_id
			}
		};

		current_octant_id = next_child_id;
	}
	Err(StorageError::OverMaxDepth(max_depth))
	//panic!("Octree traversal is stuck in loop. Check implementation of OctantStorage for bugs.");
}


pub fn subdivide_if_from_storage<Storage, F, U>(storage: &mut Storage, start_from_id: &Storage::OctantId, mut subdivide_predicate: F) -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
where
	Storage: ModifiableOctantStorage,
	F: FnMut(Depth, &Storage::OctantId, &mut OctantStorageAccessorMut<Storage>) -> SubdivisionControlFlow<U>,
	U: FnMut(OctantPlacement) -> Storage::Data
{
	let start_from_depth = storage.get_octant_depth(start_from_id).ok_or(StorageError::InvalidOctantId)?;
	let mut to_be_visited: VecDeque<(Depth, Storage::OctantId)> = VecDeque::from([(start_from_depth, *start_from_id)]);

	let mut last_visited_step: Option<SearchControlFlowResult<Storage::OctantId>> = None;
	while let Some((depth, octant_id)) = to_be_visited.pop_front() {
		let mut storage_accessor = OctantStorageAccessorMut::<Storage>::new(storage);
		let next_step: SubdivisionControlFlow<U> = subdivide_predicate(depth, &octant_id, &mut storage_accessor);

		match next_step {
			SubdivisionControlFlow::Continue(create_data_fn) => {
				last_visited_step = Some(SearchControlFlowResult::Continue(octant_id));
				let subdivision_children: [Storage::OctantId; OctantPlacement::OCTANTS_COUNT] = storage.subdivide(&octant_id, create_data_fn)?;
				let child_depth = depth + 1;
				subdivision_children.iter()
					.for_each(
						|&child_id|{
							to_be_visited.push_back((child_depth, child_id));
						}
					);
			},
			SubdivisionControlFlow::Break => return Ok(next_step.to_result(octant_id)),
			SubdivisionControlFlow::Skip => {
				last_visited_step = Some(next_step.to_result(octant_id));
				continue;
			}
		};
	}
	last_visited_step.ok_or(StorageError::InvalidOctantId)
}

pub fn subdivide_if_some_from_storage<Storage, F, U>(storage: &mut Storage, start_from_id: &Storage::OctantId, mut subdivide_predicate: F) -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
where
	Storage: ModifiableOctantStorage,
	F: FnMut(Depth, &Storage::OctantId, &mut OctantStorageAccessorMut<Storage>) -> SubdivisionControlFlow<U>,
	U: FnMut(OctantPlacement) -> Option<Storage::Data>
{
	let start_from_depth = storage.get_octant_depth(start_from_id).ok_or(StorageError::InvalidOctantId)?;
	let mut to_be_visited: VecDeque<(Depth, Storage::OctantId)> = VecDeque::from([(start_from_depth, *start_from_id)]);

	let mut last_visited_step: Option<SearchControlFlowResult<Storage::OctantId>> = None;
	while let Some((depth, octant_id)) = to_be_visited.pop_front() {
		let mut storage_accessor = OctantStorageAccessorMut::<Storage>::new(storage);
		let next_step: SubdivisionControlFlow<U> = subdivide_predicate(depth, &octant_id, &mut storage_accessor);

		match next_step {
			SubdivisionControlFlow::Continue(mut create_data_maybe_fn) => {
				last_visited_step = Some(SearchControlFlowResult::Continue(octant_id));
				let child_depth = depth + 1;
				let created_children_iter = OctantPlacement::OCTANTS_ORDERED.into_iter()
					.filter_map(
						|child_placement: OctantPlacement|{
							let new_data: Storage::Data = create_data_maybe_fn(child_placement)?;
							Some((child_placement, new_data))
						}
					);
				for (child_placement, child_data) in created_children_iter {
					let (child_id, _) = storage.insert_octant(&octant_id, child_placement, child_data)?;
					to_be_visited.push_back((child_depth, child_id));
				}
			},
			SubdivisionControlFlow::Break => return Ok(next_step.to_result(octant_id)),
			SubdivisionControlFlow::Skip => {
				last_visited_step = Some(next_step.to_result(octant_id));
				continue;
			}
		};
	}
	last_visited_step.ok_or(StorageError::InvalidOctantId)
}

