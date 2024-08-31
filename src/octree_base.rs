use std::collections::VecDeque;


use super::octant_meta::OctantPlacement;
use super::octant_storage_trait::{ModifiableOctantStorage, OctantStorage, OctantStorageAccessorMut, StorageError, StorageResult};
use super::Depth;


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

pub trait OctantIdTypeInfo {
	type OctantId;
}

#[derive(Debug)]
pub struct OctreeBase<Storage>{
	pub octants: Storage,
}

impl <Storage: OctantStorage> OctreeBase<Storage> {

	
	pub fn new_with_storage(storage: Storage) -> Self{
		OctreeBase{
			octants: storage
		}
	}

	pub fn guided_search<F>(&self, octant_id: &Storage::OctantId, guide_func: F)  -> StorageResult<Storage::OctantId>
	where F: FnMut(Depth, &Storage::OctantId) -> Option<OctantPlacement> {
		guided_search_from_storage(&self.octants, &octant_id, guide_func)
	}

	pub fn guided_search_from_root<F>(&self, guide_func: F)  -> StorageResult<Storage::OctantId>
	where F: FnMut(Depth, &Storage::OctantId) -> Option<OctantPlacement> {
		let root_id = self.octants.get_root_id();
		self.guided_search( &root_id, guide_func)
	}

	pub fn guided_search_mut<F>(&mut self, octant_id: &Storage::OctantId, guide_func: F)  -> StorageResult<Storage::OctantId>
	where F: FnMut(Depth, &Storage::OctantId, &mut OctantStorageAccessorMut<Storage>) -> Option<OctantPlacement> {
		guided_search_from_storage_mut(&mut self.octants, &octant_id, guide_func)
	}

	pub fn guided_search_from_root_mut<F>(&mut self, guide_func: F)  -> StorageResult<Storage::OctantId>
	where F: FnMut(Depth, &Storage::OctantId, &mut OctantStorageAccessorMut<Storage>) -> Option<OctantPlacement> {
		let root_id = self.octants.get_root_id();
		self.guided_search_mut(&root_id, guide_func)
	}

	pub fn depth_first_search<F>(&self, octant_id: &Storage::OctantId, search_func: F)  -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
	where F: FnMut(Depth, &Storage::OctantId) -> SearchControlFlow {
		depth_first_search_from_storage(&self.octants, octant_id, search_func)
	}

	pub fn depth_first_search_from_root<F>(&self, search_func: F)  -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
	where F: FnMut(Depth, &Storage::OctantId) -> SearchControlFlow {
		let root_id = self.octants.get_root_id();
		self.depth_first_search(&root_id, search_func)
	}

	pub fn depth_first_search_mut<F>(&mut self, octant_id: &Storage::OctantId, search_func: F)  -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
	where F: FnMut(Depth, &Storage::OctantId, &mut OctantStorageAccessorMut<Storage>) -> SearchControlFlow {
		depth_first_search_from_storage_mut(&mut self.octants, octant_id, search_func)
	}

	pub fn depth_first_search_from_root_mut<F>(&mut self, search_func: F)  -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
	where F: FnMut(Depth, &Storage::OctantId, &mut OctantStorageAccessorMut<Storage>) -> SearchControlFlow {
		let root_id = self.octants.get_root_id();
		self.depth_first_search_mut(&root_id, search_func)
	}

	pub fn breadth_first_search<F>(&self, octant_id: &Storage::OctantId, search_func: F)  -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
	where F: FnMut(Depth, &Storage::OctantId) -> SearchControlFlow {
		breadth_first_search_from_storage(&self.octants, octant_id, search_func)
	}

	pub fn breadth_first_search_from_root<F>(&self, search_func: F)  -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
	where F: FnMut(Depth, &Storage::OctantId) -> SearchControlFlow {
		let root_id = self.octants.get_root_id();
		self.breadth_first_search(&root_id, search_func)
	}

	pub fn breadth_first_search_mut<F>(&mut self, octant_id: &Storage::OctantId, search_func: F)  -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
	where F: FnMut(Depth, &Storage::OctantId, &mut OctantStorageAccessorMut<Storage>) -> SearchControlFlow {
		breadth_first_search_from_storage_mut(&mut self.octants, octant_id, search_func)
	}

	pub fn breadth_first_search_from_root_mut<F>(&mut self, search_func: F)  -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
	where F: FnMut(Depth, &Storage::OctantId, &mut OctantStorageAccessorMut<Storage>) -> SearchControlFlow {
		let root_id = self.octants.get_root_id();
		self.breadth_first_search_mut(&root_id, search_func)
	}

	pub fn breadth_first_iterator(&self) -> BreadthFirstIterator<Storage> {
		BreadthFirstIterator::<Storage>::new(&self.octants)
	}

}

impl <Storage: ModifiableOctantStorage> OctreeBase<Storage> {
	pub fn new_with_root(root_custom_data: Storage::Data) -> Self
	where Storage: Default{
		OctreeBase::new_with_storage(Storage::new_with_root(root_custom_data))
	}

	pub fn drill<F>(&mut self, octant_id: &Storage::OctantId, drill_func: F)  -> StorageResult<Storage::OctantId>
	where F: FnMut(Depth, &Storage::OctantId, &mut OctantStorageAccessorMut<Storage>) -> Option<AssignmentControlFlow<Storage::Data>> {
		drill_from_storage(&mut self.octants, octant_id, drill_func)
	}

	pub fn drill_from_root<F>(&mut self, drill_func: F)  -> StorageResult<Storage::OctantId>
	where F: FnMut(Depth, &Storage::OctantId, &mut OctantStorageAccessorMut<Storage>) -> Option<AssignmentControlFlow<Storage::Data>> {
		let root_id = self.octants.get_root_id();
		self.drill(&root_id, drill_func)
	}

	pub fn subdivide_if<F, U>(&mut self, octant_id: &Storage::OctantId, subdivide_predicate: F) -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
	where
		F: FnMut(Depth, &Storage::OctantId, &mut OctantStorageAccessorMut<Storage>) -> SubdivisionControlFlow<U>,
		U: FnMut(OctantPlacement) -> Storage::Data
	{
		subdivide_if_from_storage(&mut self.octants, octant_id, subdivide_predicate)
	}

	pub fn subdivide_if_from_root<F, U>(&mut self, subdivide_predicate: F) -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
	where
		F: FnMut(Depth, &Storage::OctantId, &mut OctantStorageAccessorMut<Storage>) -> SubdivisionControlFlow<U>,
		U: FnMut(OctantPlacement) -> Storage::Data
	{
		let root_id = self.octants.get_root_id();
		self.subdivide_if( &root_id, subdivide_predicate)
	}

	pub fn subdivide_if_some<F, U>(&mut self, octant_id: &Storage::OctantId, subdivide_predicate: F) -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
	where
		F: FnMut(Depth, &Storage::OctantId, &mut OctantStorageAccessorMut<Storage>) -> SubdivisionControlFlow<U>,
		U: FnMut(OctantPlacement) -> Option<Storage::Data>
	{
		subdivide_if_some_from_storage(&mut self.octants, octant_id, subdivide_predicate)
	}

	pub fn subdivide_if_some_from_root<F, U>(&mut self, subdivide_predicate: F) -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
	where
		F: FnMut(Depth, &Storage::OctantId, &mut OctantStorageAccessorMut<Storage>) -> SubdivisionControlFlow<U>,
		U: FnMut(OctantPlacement) -> Option<Storage::Data>
	{
		let root_id = self.octants.get_root_id();
		self.subdivide_if_some(&root_id, subdivide_predicate)
	}

	
}

/// STATIC(non-constructors), these are not internally used but are for convenience when working with mutable accessors
impl <Storage: OctantStorage> OctreeBase<Storage> {
	pub fn guided_search_from_storage<
		AnyStorage: OctantStorage<
				OctantId = Storage::OctantId,
				ParentIdIterator = Storage::ParentIdIterator,
				Data = Storage::Data
			>,
		F
	>(
		storage: &AnyStorage,
		octant_id: &AnyStorage::OctantId,
		guide_func: F
	) -> StorageResult<AnyStorage::OctantId>
	where F: FnMut(Depth, &AnyStorage::OctantId) -> Option<OctantPlacement> {
		guided_search_from_storage(storage, octant_id, guide_func)
	}

	pub fn guided_search_from_storage_mut<
		AnyStorage: OctantStorage<
				OctantId = Storage::OctantId,
				ParentIdIterator = Storage::ParentIdIterator,
				Data = Storage::Data
			>,
		F
	>(
		storage: &mut AnyStorage,
		octant_id: &AnyStorage::OctantId,
		guide_func: F
	) -> StorageResult<AnyStorage::OctantId>
	where F: FnMut(Depth, &AnyStorage::OctantId, &mut OctantStorageAccessorMut<AnyStorage>) -> Option<OctantPlacement> {
		guided_search_from_storage_mut(storage, octant_id, guide_func)
	}

	pub fn depth_first_search_from_storage<
		AnyStorage: OctantStorage<
				OctantId = Storage::OctantId,
				ParentIdIterator = Storage::ParentIdIterator,
				Data = Storage::Data
			>,
		F
	>(
		storage: &AnyStorage,
		octant_id: &AnyStorage::OctantId,
		search_func: F
	) -> StorageResult<SearchControlFlowResult<AnyStorage::OctantId>>
	where F: FnMut(Depth, &AnyStorage::OctantId) -> SearchControlFlow {
		depth_first_search_from_storage(storage, octant_id, search_func)
	}

	pub fn depth_first_search_from_storage_mut<
		AnyStorage: OctantStorage<
				OctantId = Storage::OctantId,
				ParentIdIterator = Storage::ParentIdIterator,
				Data = Storage::Data
			>,
		F
	>(
		storage: &mut AnyStorage,
		octant_id: &AnyStorage::OctantId,
		search_func: F
	) -> StorageResult<SearchControlFlowResult<AnyStorage::OctantId>>
	where F: FnMut(Depth, &AnyStorage::OctantId, &mut OctantStorageAccessorMut<AnyStorage>) -> SearchControlFlow {
		depth_first_search_from_storage_mut(storage, octant_id, search_func)
	}

	pub fn breadth_first_search_from_storage<
	AnyStorage: OctantStorage<
			OctantId = Storage::OctantId,
			ParentIdIterator = Storage::ParentIdIterator,
			Data = Storage::Data
		>,
		F
	>(
		storage: &AnyStorage,
		octant_id: &AnyStorage::OctantId,
		search_func: F
	) -> StorageResult<SearchControlFlowResult<AnyStorage::OctantId>>
	where F: FnMut(Depth, &AnyStorage::OctantId) -> SearchControlFlow {
		breadth_first_search_from_storage(storage, octant_id, search_func)
	}

	pub fn breadth_first_search_from_storage_mut<
		AnyStorage: OctantStorage<
				OctantId = Storage::OctantId,
				ParentIdIterator = Storage::ParentIdIterator,
				Data = Storage::Data
			>,
		F
	>(
		storage: &mut AnyStorage,
		octant_id: &AnyStorage::OctantId,
		search_func: F
	) -> StorageResult<SearchControlFlowResult<AnyStorage::OctantId>>
	where F: FnMut(Depth, &AnyStorage::OctantId, &mut OctantStorageAccessorMut<AnyStorage>) -> SearchControlFlow {
		breadth_first_search_from_storage_mut(storage, octant_id, search_func)
	}

	pub fn drill_from_storage<
		AnyStorage: ModifiableOctantStorage<
			OctantId = Storage::OctantId,
			ParentIdIterator = Storage::ParentIdIterator,
			Data = Storage::Data
		>,
		F
	>(
		storage: &mut AnyStorage,
		octant_id: &AnyStorage::OctantId,
		drill_func: F
	) -> StorageResult<AnyStorage::OctantId>
	where F: FnMut(Depth, &AnyStorage::OctantId, &mut OctantStorageAccessorMut<AnyStorage>) -> Option<AssignmentControlFlow<AnyStorage::Data>> {
		drill_from_storage(storage, octant_id, drill_func)
	}

	pub fn subdivide_if_from_storage<
		AnyStorage: ModifiableOctantStorage<
			OctantId = Storage::OctantId,
			ParentIdIterator = Storage::ParentIdIterator,
			Data = Storage::Data
		>,
		F,
		U
	>(
		storage: &mut AnyStorage,
		octant_id: &AnyStorage::OctantId,
		subdivide_predicate: F
	) -> StorageResult<SearchControlFlowResult<AnyStorage::OctantId>>
	where
		F: FnMut(Depth, &AnyStorage::OctantId, &mut OctantStorageAccessorMut<AnyStorage>) -> SubdivisionControlFlow<U>,
		U: FnMut(OctantPlacement) -> AnyStorage::Data
	{
		subdivide_if_from_storage(storage, octant_id, subdivide_predicate)
	}

	pub fn subdivide_if_some_from_storage<
		AnyStorage: ModifiableOctantStorage<
			OctantId = Storage::OctantId,
			ParentIdIterator = Storage::ParentIdIterator,
			Data = Storage::Data
		>,
		F,
		U
	>(
		storage :&mut AnyStorage,
		octant_id: &AnyStorage::OctantId,
		subdivide_predicate: F
	) -> StorageResult<SearchControlFlowResult<AnyStorage::OctantId>>
	where
		F: FnMut(Depth, &AnyStorage::OctantId, &mut OctantStorageAccessorMut<AnyStorage>) -> SubdivisionControlFlow<U>,
		U: FnMut(OctantPlacement) -> Option<AnyStorage::Data>
	{
		subdivide_if_some_from_storage(storage, octant_id, subdivide_predicate)
	}

}

impl <Storage: OctantStorage + Default> Default for OctreeBase<Storage> {
	fn default() -> Self {
		OctreeBase{
			octants: Storage::default()
		}
	}
}

impl <Storage: OctantStorage>  OctantIdTypeInfo for OctreeBase<Storage> {
	type OctantId = Storage::OctantId;			
}


pub struct BreadthFirstIterator<'a, Storage: OctantStorage>{
	to_be_visited: VecDeque<(Depth, Storage::OctantId)>,
	octant_storage: &'a Storage
}

impl<'a, Storage: OctantStorage> BreadthFirstIterator<'a, Storage> {
	pub fn new(octant_storage: &'a Storage) -> Self {
		BreadthFirstIterator{
			to_be_visited: VecDeque::from([(0, octant_storage.get_root_id())]),
			octant_storage: octant_storage
		}
	}
}

impl<'a, Storage: OctantStorage> Iterator for BreadthFirstIterator<'a, Storage> {
	type Item = (Depth, Storage::OctantId);
	fn next(&mut self) -> Option<Self::Item> {
		let octant_item: Option<(Depth, Storage::OctantId)> = self.to_be_visited.pop_front();

		if let Some((depth, octant_id)) = &octant_item {
			let child_depth = depth + 1;
			self.octant_storage.get_existing_children(&octant_id).ok()?
				.iter()
				.flatten()
				.for_each(	
					|&child_id|{
						self.to_be_visited.push_back((child_depth, child_id));
					}
				);
			}

		octant_item
	}
}

pub struct BreadthFirstIteratorMut<'a, Storage: OctantStorage>{
	to_be_visited: VecDeque<(Depth, Storage::OctantId)>,
	storage_accessor: OctantStorageAccessorMut<'a, Storage> 
}

impl<'a, Storage: OctantStorage> BreadthFirstIteratorMut<'a, Storage> {
	pub fn new(octant_storage: &'a mut Storage) -> Self {
		BreadthFirstIteratorMut{
			to_be_visited: VecDeque::from([(0, octant_storage.get_root_id())]),
			storage_accessor: OctantStorageAccessorMut::<'a, Storage>::new( octant_storage)
		}
	}
}	

impl<'a, Storage: OctantStorage> Iterator for BreadthFirstIteratorMut<'a, Storage> {
	type Item = (Depth, Storage::OctantId, &'a mut OctantStorageAccessorMut::<'a, Storage>);
	fn next(& mut self) -> Option<Self::Item>{
		
		let (depth, octant_id) = self.to_be_visited.pop_front()?;
		let child_depth = depth + 1;
		self.storage_accessor.get_existing_children(&octant_id).ok()?
			.iter()
			.flatten()
			.for_each(
				|&child_id|{
					self.to_be_visited.push_back((child_depth, child_id));
				}
			);

		// when I'll understand lifetimes better I am going to do this without unsafe if possible,
		// SAFETY: this is safe because lifetime of the iterator is the same as Item
		let safe_accessor = unsafe {&mut *(&mut self.storage_accessor as *mut OctantStorageAccessorMut::<'a, Storage>)};
			
		Some((depth, octant_id, safe_accessor))
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

				storage.insert_octant(&current_octant_id, next_child_placement, new_custom_data);
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

