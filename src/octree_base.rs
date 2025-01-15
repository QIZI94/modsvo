use std::collections::VecDeque;


use super::octant_meta::OctantPlacement;
use super::octant_storage_trait::{
	ModifiableOctantStorage,
	OctantStorage,
	OctantStorageAccessorMut,
	StorageResult
};
use super::octant_meta::Depth;
pub use super::octant_storage_utils::*;

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

	pub fn breadth_first_iterator_mut(&mut self) -> BreadthFirstIteratorMut<Storage> {
		BreadthFirstIteratorMut::<Storage>::new(&mut self.octants)
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
