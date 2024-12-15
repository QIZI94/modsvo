use std::collections::VecDeque;

use super::voxel_trait::Voxel;
use super::octant_meta::OctantPlacement;
use super::octant_meta::Depth;
use super::octree_base::OctreeBase;
use super::octant_storage_trait::{ModifiableOctantStorage, OctantStorage, OctantStorageAccessorMut, StorageError, StorageResult, };
use super::octant_storage_utils::{SearchControlFlow, SearchControlFlowResult, AssignmentControlFlow, SubdivisionControlFlow};

pub fn guided_search_from_storage<Storage: OctantStorage, Volumetric: Voxel, F>(storage: &Storage, octant_id: &Storage::OctantId, voxel: &Volumetric, mut guided_fn: F) -> StorageResult<Storage::OctantId>
where F: FnMut(Depth, &Storage::OctantId, &Volumetric)  -> Option<OctantPlacement> {
	let mut current_voxel: Volumetric = voxel.clone();
	OctreeBase::<Storage>::guided_search_from_storage(
		storage,
		octant_id,
		|depth, id|{
			let next_octant_placement = guided_fn(depth, id, &voxel)?;
			current_voxel = current_voxel.make_sub_voxel(next_octant_placement);
			Some(next_octant_placement)
		}
	)
}

pub fn guided_search_from_storage_mut<Storage: OctantStorage, Volumetric: Voxel, F>(storage: &mut Storage, octant_id: &Storage::OctantId, voxel: &Volumetric, mut guided_fn: F) -> StorageResult<Storage::OctantId>
where F: FnMut(Depth, &Storage::OctantId, &Volumetric, &mut OctantStorageAccessorMut<Storage>)  -> Option<OctantPlacement> {
	let mut current_voxel: Volumetric = voxel.clone();
	OctreeBase::<Storage>::guided_search_from_storage_mut(
		storage,
		octant_id,
		|depth, id, storage_accessor|{
			let next_octant_placement = guided_fn(depth, id, &voxel, storage_accessor)?;
			current_voxel = current_voxel.make_sub_voxel(next_octant_placement);
			Some(next_octant_placement)
		}
	)
}

pub fn depth_first_search_from_storage<Storage: OctantStorage, Volumetric: Voxel, F>(storage: &Storage, octant_id: &Storage::OctantId, voxel: &Volumetric, mut search_func: F)  -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
where F: FnMut(Depth, &Storage::OctantId, &Volumetric) -> SearchControlFlow {
	let start_from_depth = storage.get_octant_depth(octant_id).ok_or(StorageError::InvalidOctantId)?;
	depth_first_search_recursive(storage, start_from_depth, octant_id, voxel, &mut search_func)
}

fn depth_first_search_recursive<Storage: OctantStorage, Volumetric: Voxel, F>(storage: &Storage, start_from_depth: Depth, octant_id: &Storage::OctantId, voxel: &Volumetric, search_func: &mut F)  -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
where F: FnMut(Depth, &Storage::OctantId, &Volumetric) -> SearchControlFlow {
	let next_step: SearchControlFlow = search_func(start_from_depth, octant_id, voxel);
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
				let child_voxel = voxel.make_sub_voxel(child_placement);
				let child_step = depth_first_search_recursive(storage, child_depth, &child_id, &child_voxel, search_func)?;
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

pub fn depth_first_search_from_storage_mut<Storage: OctantStorage, Volumetric: Voxel, F>(storage: &mut Storage, octant_id: &Storage::OctantId, voxel: &Volumetric, mut search_func: F)  -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
where F: FnMut(Depth, &Storage::OctantId, &Volumetric, &mut OctantStorageAccessorMut<Storage>) -> SearchControlFlow {
	let start_from_depth = storage.get_octant_depth(octant_id).ok_or(StorageError::InvalidOctantId)?;
	depth_first_search_recursive_mut(storage, start_from_depth, octant_id, voxel, &mut search_func)
}

fn depth_first_search_recursive_mut<Storage: OctantStorage, Volumetric: Voxel, F>(storage: &mut Storage, start_from_depth: Depth, octant_id: &Storage::OctantId, voxel: &Volumetric, search_func: &mut F)  -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
where F: FnMut(Depth, &Storage::OctantId, &Volumetric, &mut OctantStorageAccessorMut<Storage>) -> SearchControlFlow {
	let mut storage_accessor = OctantStorageAccessorMut::<Storage>::new(storage);
	let next_step: SearchControlFlow = search_func(start_from_depth, octant_id, voxel, &mut storage_accessor);
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
				let child_voxel = voxel.make_sub_voxel(child_placement);
				let child_step = depth_first_search_recursive_mut(storage, child_depth, &child_id, &child_voxel, search_func)?;
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

pub fn breadth_first_search_from_storage<Storage: OctantStorage, Volumetric: Voxel, F>(storage: &Storage, start_from_id: &Storage::OctantId, octant_voxel: &Volumetric, mut search_func: F) -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
where F: FnMut(Depth, &Storage::OctantId, &Volumetric) -> SearchControlFlow {
	let start_from_depth = storage.get_octant_depth(start_from_id).ok_or(StorageError::InvalidOctantId)?;
	let max_depth: Depth = storage.get_max_depth();
	let mut to_be_visited: VecDeque<(Depth, Storage::OctantId, Volumetric)> = VecDeque::new();
	to_be_visited.push_back((start_from_depth, *start_from_id, *octant_voxel));
	
	let mut last_visited_step: Option<SearchControlFlowResult<Storage::OctantId>> = None;
	while let Some((depth, octant_id, voxel)) = to_be_visited.pop_front() {
		let next_step = search_func(depth, &octant_id, &voxel);	
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
			.zip(OctantPlacement::OCTANTS_ORDERED)
			.filter_map(
				|(&maybe_child_id, child_placement)|{
					let child_id: Storage::OctantId = maybe_child_id?;
					Some((child_id, child_placement))
				}
			)
			.for_each(
				|(child_id, child_placement)|{
					let child_voxel = voxel.make_sub_voxel(child_placement);
					to_be_visited.push_back((child_depth, child_id, child_voxel));
				}
			);
	}
	last_visited_step.ok_or(StorageError::InvalidOctantId)
}

pub fn breadth_first_search_from_storage_mut<Storage: OctantStorage, Volumetric: Voxel, F>(storage: &mut Storage, start_from_id: &Storage::OctantId, octant_voxel: &Volumetric, mut search_func: F) -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
where F: FnMut(Depth, &Storage::OctantId, &Volumetric, &mut OctantStorageAccessorMut<Storage>) -> SearchControlFlow {
	let start_from_depth: Depth = storage.get_octant_depth(start_from_id).ok_or(StorageError::InvalidOctantId)?;
	let max_depth: Depth = storage.get_max_depth();
	let mut storage_accessor = OctantStorageAccessorMut::<Storage>::new(storage);
	
	let mut to_be_visited: VecDeque<(Depth, Storage::OctantId, Volumetric)> = VecDeque::new();
	to_be_visited.push_back((start_from_depth, *start_from_id, *octant_voxel));
	
	let mut last_visited_step: Option<SearchControlFlowResult<Storage::OctantId>> = None;
	while let Some((depth, octant_id, voxel)) = to_be_visited.pop_front() {
		let next_step = search_func(depth, &octant_id, &voxel, &mut storage_accessor);
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
			.zip(OctantPlacement::OCTANTS_ORDERED)
			.filter_map(
				|(&maybe_child_id, child_placement)|{
					let child_id: Storage::OctantId = maybe_child_id?;
					Some((child_id, child_placement))
				}
			)
			.for_each(
				|(child_id, child_placement)|{
					let child_voxel = voxel.make_sub_voxel(child_placement);
					to_be_visited.push_back((child_depth, child_id, child_voxel));
				}
			);
	}
	last_visited_step.ok_or(StorageError::InvalidOctantId)
}


pub fn drill_from_storage<Storage: ModifiableOctantStorage, Volumetric: Voxel, F>(storage: &mut Storage, octant_id: &Storage::OctantId, octant_voxel: &Volumetric, mut drill_func: F)  -> StorageResult<Storage::OctantId>
where F: FnMut(Depth, &Storage::OctantId, &Volumetric, &mut OctantStorageAccessorMut<Storage>) -> Option<AssignmentControlFlow<Storage::Data>> {
	let mut current_voxel: Volumetric = octant_voxel.clone();
	OctreeBase::<Storage>::drill_from_storage(
		storage,
		octant_id,
		|depth, id, storage_accessor|{
			let next_octant_assignment = drill_func(depth, id, &current_voxel, storage_accessor)?;
			let (next_octant_placement, _) = next_octant_assignment.get_assigned();
			current_voxel = current_voxel.make_sub_voxel(next_octant_placement);
			Some(next_octant_assignment)
		}
	)
}


pub fn subdivide_if_from_storage<Storage, Volumetric: Voxel, F, U>(storage: &mut Storage, start_from_id: &Storage::OctantId, octant_voxel: &Volumetric, mut subdivide_predicate: F) -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
where
	Storage: ModifiableOctantStorage,
	F: FnMut(Depth, &Storage::OctantId, &Volumetric, &mut OctantStorageAccessorMut<Storage>) -> SubdivisionControlFlow<U>,
	U: FnMut(OctantPlacement) -> Storage::Data
{
	let start_from_depth = storage.get_octant_depth(start_from_id).ok_or(StorageError::InvalidOctantId)?;
	let mut to_be_visited: VecDeque<(Depth, Storage::OctantId, Volumetric)> = VecDeque::from([(start_from_depth, *start_from_id, *octant_voxel)]);

	let mut last_visited_step: Option<SearchControlFlowResult<Storage::OctantId>> = None;
	while let Some((depth, octant_id, voxel)) = to_be_visited.pop_front() {
		let mut storage_accessor = OctantStorageAccessorMut::<Storage>::new(storage);
		let next_step: SubdivisionControlFlow<U> = subdivide_predicate(depth, &octant_id, &voxel, &mut storage_accessor);

		match next_step {
			SubdivisionControlFlow::Continue(create_data_fn) => {
				last_visited_step = Some(SearchControlFlowResult::Continue(octant_id));
				let subdivision_children: [Storage::OctantId; OctantPlacement::OCTANTS_COUNT] = storage.subdivide(&octant_id, create_data_fn)?;
				let child_depth = depth + 1;
				subdivision_children.iter()
					.zip(OctantPlacement::OCTANTS_ORDERED)
					.for_each(
						|(&child_id, child_placement)|{
							let child_voxel = voxel.make_sub_voxel(child_placement);
							to_be_visited.push_back((child_depth, child_id, child_voxel));
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

pub fn subdivide_if_some_from_storage<Storage, Volumetric: Voxel, F, U>(storage: &mut Storage, start_from_id: &Storage::OctantId, octant_voxel: &Volumetric, mut subdivide_predicate: F) -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
where
	Storage: ModifiableOctantStorage,
	F: FnMut(Depth, &Storage::OctantId, &Volumetric, &mut OctantStorageAccessorMut<Storage>) -> SubdivisionControlFlow<U>,			
	U: FnMut(OctantPlacement) -> Option<Storage::Data>
{
	let start_from_depth = storage.get_octant_depth(start_from_id).ok_or(StorageError::InvalidOctantId)?;
	let mut to_be_visited: VecDeque<(Depth, Storage::OctantId, Volumetric)> = VecDeque::from([(start_from_depth, *start_from_id, *octant_voxel)]);

	let mut last_visited_step: Option<SearchControlFlowResult<Storage::OctantId>> = None;
	while let Some((depth, octant_id, voxel)) = to_be_visited.pop_front() {
		let mut storage_accessor = OctantStorageAccessorMut::<Storage>::new(storage);
		let next_step: SubdivisionControlFlow<U> = subdivide_predicate(depth, &octant_id, &voxel, &mut storage_accessor);

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
					let child_voxel = voxel.make_sub_voxel(child_placement);
					to_be_visited.push_back((child_depth, child_id, child_voxel));
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
