use std::collections::VecDeque;

use super::octant_storage_trait::{ModifiableOctantStorage, OctantStorage, OctantStorageAccessorMut, StorageError, StorageResult};
use super::octant_meta::Depth;
use super::octant_meta::OctantPlacement;
use super::{
	octree_base::{OctreeBase, OctantIdTypeInfo},
	octant_storage_utils::{
		SearchControlFlow, SearchControlFlowResult, AssignmentControlFlow, SubdivisionControlFlow
	},
	voxel_trait::Voxel, voxels::voxel_cube::VolumetricCube
};
//use super::octant_storage_utils::*;
use super::octant_storage_voxel_utils::*;

pub struct SpatialOctreeBase<Storage, Volumetric: Voxel = VolumetricCube>{
	pub base:  OctreeBase<Storage>,
	root_voxel: Volumetric
}


impl<Storage: OctantStorage, Volumetric: Voxel>  SpatialOctreeBase<Storage, Volumetric> {
	pub fn new_with_base(root_voxel: Volumetric, base: OctreeBase<Storage>) -> Self{
		SpatialOctreeBase{
			base,
			root_voxel
		}
	}
	
	pub fn octants(&self) -> &Storage {
		self.base.octants()
	}

	pub fn octants_mut(&mut self) -> &mut Storage {
		self.base.octants_mut()
	}
	pub fn get_root_id(&self) -> Storage::OctantId{
		self.base.get_root_id()
	}

	pub fn get_root_voxel(&self) -> &Volumetric {
		&self.root_voxel
	}
	pub fn get_voxel_by_id(&self, octant_id: &Storage::OctantId) -> Option<Volumetric> {
		let root_voxel: &Volumetric = self.get_root_voxel();
		Self::get_voxel_by_id_from_storage(self.octants(), octant_id, root_voxel)
	}

	pub fn guided_search<F>(&self, octant_id: &Storage::OctantId, guided_fn: F) -> StorageResult<Storage::OctantId>
	where F: FnMut(Depth, &Storage::OctantId, &Volumetric)  -> Option<OctantPlacement> {
		let voxel: Volumetric = self.get_voxel_by_id(octant_id).ok_or(StorageError::InvalidOctantId)?;
		guided_search_from_storage(self.octants(), octant_id, &voxel, guided_fn)
	}

	pub fn guided_search_from_root<F>(&self, guided_fn: F) -> StorageResult<Storage::OctantId>
	where F: FnMut(Depth, &Storage::OctantId, &Volumetric)  -> Option<OctantPlacement> {
		let root_id: Storage::OctantId = self.get_root_id();
		let voxel: Volumetric = *self.get_root_voxel();
		guided_search_from_storage(self.octants(), &root_id, &voxel, guided_fn)
	}

	pub fn guided_search_mut<F>(&mut self, octant_id: &Storage::OctantId, guided_fn: F) -> StorageResult<Storage::OctantId>
	where F: FnMut(Depth, &Storage::OctantId, &Volumetric, &mut OctantStorageAccessorMut<Storage>)  -> Option<OctantPlacement> {
		let voxel: Volumetric = self.get_voxel_by_id(octant_id).ok_or(StorageError::InvalidOctantId)?;
		guided_search_from_storage_mut(self.octants_mut(), octant_id, &voxel, guided_fn)
	}

	pub fn guided_search_from_root_mut<F>(&mut self, guided_fn: F) -> StorageResult<Storage::OctantId>
	where F: FnMut(Depth, &Storage::OctantId, &Volumetric, &mut OctantStorageAccessorMut<Storage>)  -> Option<OctantPlacement> {
		let root_id: Storage::OctantId = self.get_root_id();
		let voxel: Volumetric = *self.get_root_voxel();
		guided_search_from_storage_mut(self.octants_mut(), &root_id, &voxel, guided_fn)
	}

	pub fn depth_first_search<F>(&self, octant_id: &Storage::OctantId, search_func: &mut F)  -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
	where F: FnMut(Depth, &Storage::OctantId, &Volumetric) -> SearchControlFlow {
		let voxel: Volumetric = self.get_voxel_by_id(octant_id).ok_or(StorageError::InvalidOctantId)?;
		depth_first_search_from_storage(self.octants(), octant_id, &voxel, search_func)
	}

	pub fn depth_first_search_from_root<F>(&self, search_func: &mut F)  -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
	where F: FnMut(Depth, &Storage::OctantId, &Volumetric) -> SearchControlFlow {
		let root_id = self.get_root_id();
		self.depth_first_search(&root_id, search_func)
	}

	pub fn depth_first_search_mut<F>(&mut self, octant_id: &Storage::OctantId, search_func: &mut F)  -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
	where F: FnMut(Depth, &Storage::OctantId, &Volumetric, &mut OctantStorageAccessorMut<Storage>) -> SearchControlFlow {
		let voxel: Volumetric = self.get_voxel_by_id(octant_id).ok_or(StorageError::InvalidOctantId)?;
		depth_first_search_from_storage_mut(self.octants_mut(), octant_id, &voxel, search_func)
	}

	pub fn depth_first_search_from_root_mut<F>(&mut self, search_func: &mut F)  -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
	where F: FnMut(Depth, &Storage::OctantId, &Volumetric, &mut OctantStorageAccessorMut<Storage>) -> SearchControlFlow {
		let root_id: Storage::OctantId = self.get_root_id();
		self.depth_first_search_mut(&root_id, search_func)
	}


	pub fn breadth_first_search<F>(&self, octant_id: &Storage::OctantId, search_func: &mut F)  -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
	where F: FnMut(Depth, &Storage::OctantId, &Volumetric) -> SearchControlFlow {
		let voxel: Volumetric = self.get_voxel_by_id(octant_id).ok_or(StorageError::InvalidOctantId)?;
		breadth_first_search_from_storage(self.octants(), octant_id, &voxel, search_func)
	}

	pub fn breadth_first_search_from_root<F>(&self, search_func: &mut F)  -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
	where F: FnMut(Depth, &Storage::OctantId, &Volumetric) -> SearchControlFlow {
		let root_id: Storage::OctantId = self.get_root_id();
		self.breadth_first_search(&root_id, search_func)
	}

	pub fn breadth_first_search_mut<F>(&mut self, octant_id: &Storage::OctantId, search_func: F)  -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
	where F: FnMut(Depth, &Storage::OctantId, &Volumetric, &mut OctantStorageAccessorMut<Storage>) -> SearchControlFlow {
		let voxel: Volumetric = self.get_voxel_by_id(octant_id).ok_or(StorageError::InvalidOctantId)?;
		breadth_first_search_from_storage_mut(self.octants_mut(), octant_id, &voxel, search_func)
	}

	pub fn breadth_first_search_from_root_mut<F>(&mut self, search_func: F)  -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
	where F: FnMut(Depth, &Storage::OctantId, &Volumetric, &mut OctantStorageAccessorMut<Storage>) -> SearchControlFlow {
		let root_id: Storage::OctantId = self.get_root_id();
		self.breadth_first_search_mut(&root_id, search_func)
	}

	pub fn breadth_first_iterator(&self) -> BreadthFirstIterator<Storage, Volumetric> {
		let root_voxel: Volumetric = self.get_root_voxel().clone();
		BreadthFirstIterator::new(self.octants(), root_voxel)
	}

	pub fn breadth_first_iterator_mut(&mut self) -> BreadthFirstIteratorMut<Storage, Volumetric> {
		let root_voxel: Volumetric = self.get_root_voxel().clone();
		BreadthFirstIteratorMut::new(self.octants_mut(), root_voxel)
	}
}

impl<Storage: ModifiableOctantStorage, Volumetric: Voxel>  SpatialOctreeBase<Storage, Volumetric> {

	pub fn new_with_root(root_voxel: Volumetric, root_custom_data: Storage::Data) -> Self
	where Storage: Default {
		Self::new_with_base(
			root_voxel,
			OctreeBase::<Storage>::new_with_root(root_custom_data)
		)
	}

	pub fn drill<F>(&mut self, octant_id: &Storage::OctantId, drill_func: F)  -> StorageResult<Storage::OctantId>
	where F: FnMut(Depth, &Storage::OctantId, &Volumetric, &mut OctantStorageAccessorMut<Storage>) -> Option<AssignmentControlFlow<Storage::Data>> {
		let voxel: Volumetric = self.get_voxel_by_id(octant_id).ok_or(StorageError::InvalidOctantId)?;
		drill_from_storage(self.octants_mut(), octant_id, &voxel, drill_func)
	}

	pub fn drill_from_root<F>(&mut self, drill_func: F)  -> StorageResult<Storage::OctantId>
	where F: FnMut(Depth, &Storage::OctantId, &Volumetric, &mut OctantStorageAccessorMut<Storage>) -> Option<AssignmentControlFlow<Storage::Data>> {
		let root_id: Storage::OctantId = self.get_root_id();
		let voxel: Volumetric = self.get_root_voxel().clone();
		drill_from_storage(self.octants_mut(), &root_id, &voxel, drill_func)
	}

	pub fn subdivide_if<F, U>(&mut self, start_from_id: &Storage::OctantId, octant_voxel: &Volumetric, subdivide_predicate: F) -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
	where
		F: FnMut(Depth, &Storage::OctantId, &Volumetric, &mut OctantStorageAccessorMut<Storage>) -> SubdivisionControlFlow<U>,
		U: FnMut(OctantPlacement) -> Storage::Data
	{
		subdivide_if_from_storage(self.octants_mut(), start_from_id, octant_voxel, subdivide_predicate)
	}

	pub fn subdivide_if_some<F, U>(&mut self, start_from_id: &Storage::OctantId, octant_voxel: &Volumetric, subdivide_predicate: F) -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
	where
		F: FnMut(Depth, &Storage::OctantId, &Volumetric, &mut OctantStorageAccessorMut<Storage>) -> SubdivisionControlFlow<U>,
		U: FnMut(OctantPlacement) -> Option<Storage::Data>
	{
		subdivide_if_some_from_storage(self.octants_mut(), start_from_id, octant_voxel, subdivide_predicate)
	}

}

/// STATIC(non-constructors)
impl<Storage: OctantStorage, Volumetric: Voxel>  SpatialOctreeBase<Storage, Volumetric> {
	pub fn get_voxel_by_id_from_storage<AnyStorage: OctantStorage>(
		storage: &AnyStorage,
		octant_id: &AnyStorage::OctantId,
		root_voxel: &Volumetric
	) -> Option<Volumetric> {
		
		compute_voxel_by_id(storage, octant_id, root_voxel)
	}

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
		voxel: &Volumetric,
		guided_fn: F
	) -> StorageResult<AnyStorage::OctantId>
	where F: FnMut(Depth, &AnyStorage::OctantId, &Volumetric)  -> Option<OctantPlacement> {
		guided_search_from_storage(storage, octant_id, voxel, guided_fn)
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
		voxel: &Volumetric,
		guided_fn: F
	) -> StorageResult<AnyStorage::OctantId>
	where F: FnMut(Depth, &AnyStorage::OctantId, &Volumetric, &mut OctantStorageAccessorMut<AnyStorage>)  -> Option<OctantPlacement> {
		guided_search_from_storage_mut(storage, octant_id, voxel, guided_fn)
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
		voxel: &Volumetric,
		search_fn: F
	) -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
	where F: FnMut(Depth, &Storage::OctantId, &Volumetric) -> SearchControlFlow {
		depth_first_search_from_storage(storage, octant_id, voxel, search_fn)
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
		voxel: &Volumetric,
		search_fn: F
	) -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
	where F: FnMut(Depth, &Storage::OctantId, &Volumetric, &mut OctantStorageAccessorMut<AnyStorage>) -> SearchControlFlow {
		depth_first_search_from_storage_mut(storage, octant_id, voxel, search_fn)
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
		voxel: &Volumetric,
		search_fn: F
	) -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
	where F: FnMut(Depth, &Storage::OctantId, &Volumetric) -> SearchControlFlow {
		breadth_first_search_from_storage(storage, octant_id, voxel, search_fn)
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
		voxel: &Volumetric,
		search_fn: F
	) -> StorageResult<SearchControlFlowResult<Storage::OctantId>>
	where F: FnMut(Depth, &Storage::OctantId, &Volumetric, &mut OctantStorageAccessorMut<AnyStorage>) -> SearchControlFlow {
		breadth_first_search_from_storage_mut(storage, octant_id, voxel, search_fn)
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
		voxel: &Volumetric,
		drill_fn: F
	) -> StorageResult<Storage::OctantId>
	where F: FnMut(Depth, &Storage::OctantId, &Volumetric, &mut OctantStorageAccessorMut<AnyStorage>) -> Option<AssignmentControlFlow<Storage::Data>> {
		drill_from_storage(storage, octant_id, voxel, drill_fn)
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
		octant_voxel: Volumetric,
		subdivide_predicate: F
	) -> StorageResult<SearchControlFlowResult<AnyStorage::OctantId>>
	where
		F: FnMut(Depth, &AnyStorage::OctantId, &Volumetric, &mut OctantStorageAccessorMut<AnyStorage>) -> SubdivisionControlFlow<U>,
		U: FnMut(OctantPlacement) -> AnyStorage::Data
	{
		subdivide_if_from_storage(storage, octant_id, &octant_voxel, subdivide_predicate)
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
		octant_voxel: Volumetric,
		subdivide_predicate: F
	) -> StorageResult<SearchControlFlowResult<AnyStorage::OctantId>>
	where
		F: FnMut(Depth, &AnyStorage::OctantId, &Volumetric, &mut OctantStorageAccessorMut<AnyStorage>) -> SubdivisionControlFlow<U>,
		U: FnMut(OctantPlacement) -> Option<AnyStorage::Data>
	{
		subdivide_if_some_from_storage(storage, octant_id, &octant_voxel, subdivide_predicate)
	}
}


impl <Storage: OctantStorage + Default, Volumetric: Voxel> SpatialOctreeBase<Storage, Volumetric> {
	pub fn with_root_voxel(root_voxel: Volumetric) -> Self {
		Self::new_with_base(
			root_voxel,
			OctreeBase::<Storage>::new_with_storage(Storage::default())
		)
	}
}

impl <Storage: OctantStorage, Volumetric: Voxel>  OctantIdTypeInfo for SpatialOctreeBase<Storage, Volumetric> {
	type OctantId = Storage::OctantId;
}

pub fn compute_voxel_by_id<Storage: OctantStorage, Volumetric: Voxel>(
	storage: &Storage,
	octant_id: &Storage::OctantId,
	root_voxel: &Volumetric
) -> Option<Volumetric> {
	let _ = storage.get_octant(octant_id)?;
	let mut parent_iterator: Storage::ParentIdIterator = storage.get_ancestors_for(octant_id)?;
	Some(compute_voxel_by_id_recursive(storage, octant_id, &mut parent_iterator, root_voxel))
}


fn compute_voxel_by_id_recursive<Storage: OctantStorage, Volumetric: Voxel>(
	storage_accessor: &Storage,
	child_id: &Storage::OctantId,
	parent_iterator: &mut Storage::ParentIdIterator,
	root_voxel: &Volumetric
) -> Volumetric {

	if *child_id == storage_accessor.get_root_id(){
		return *root_voxel;
	}

	let parent_id: Storage::OctantId = parent_iterator.next()
		.expect("Broken iterator didn't find root_id in parents.");

	let parent_voxel: Volumetric = {
		if parent_id == storage_accessor.get_root_id() {
			*root_voxel
		}
		else {
			compute_voxel_by_id_recursive(storage_accessor, &parent_id, parent_iterator, root_voxel)
		}
	};

	let child_placement: OctantPlacement = storage_accessor.which_child_of(&parent_id, &child_id)
		.expect("Wrong child-parent when iterating parent iterator.");
	let child_voxel: Volumetric = parent_voxel.make_sub_voxel(child_placement);

	child_voxel
}


pub struct BreadthFirstIterator<'a, Storage: OctantStorage, Volumetric: Voxel>{
	to_be_visited: VecDeque<(Depth, Storage::OctantId, Volumetric)>,
	octant_storage: &'a Storage,
}

impl<'a, Storage: OctantStorage, Volumetric: Voxel> BreadthFirstIterator<'a, Storage, Volumetric> {
	pub fn new(octant_storage: &'a Storage, voxel: Volumetric) -> Self {
		BreadthFirstIterator{
			to_be_visited: VecDeque::from([(0, octant_storage.get_root_id(), voxel)]),
			octant_storage: octant_storage
		}
	}
}

impl<'a, Storage: OctantStorage, Volumetric: Voxel> Iterator for BreadthFirstIterator<'a, Storage, Volumetric> {
	type Item = (Depth, Storage::OctantId, Volumetric);
	fn next(&mut self) -> Option<Self::Item> {
		let octant_item: Option<(Depth, Storage::OctantId, Volumetric)> = self.to_be_visited.pop_front();

		if let Some((depth, octant_id, voxel)) = &octant_item {
			let child_depth = depth + 1;
			self.octant_storage.get_existing_children(&octant_id).ok()?
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
						self.to_be_visited.push_back((child_depth, child_id, child_voxel));
					}
				);
			}

		octant_item
	}
}

pub struct BreadthFirstIteratorMut<'a, Storage: OctantStorage, Volumetric: Voxel>{
	to_be_visited: VecDeque<(Depth, Storage::OctantId, Volumetric)>,
	storage_accessor: OctantStorageAccessorMut<'a, Storage> 
}

impl<'a, Storage: OctantStorage, Volumetric: Voxel> BreadthFirstIteratorMut<'a, Storage, Volumetric> {
	pub fn new(octant_storage: &'a mut Storage, voxel: Volumetric) -> Self {
		BreadthFirstIteratorMut{
			to_be_visited: VecDeque::from([(0, octant_storage.get_root_id(), voxel)]),
			storage_accessor: OctantStorageAccessorMut::<'a, Storage>::new( octant_storage)
		}
	}
}	

impl<'a, Storage: OctantStorage, Volumetric: Voxel> Iterator for BreadthFirstIteratorMut<'a, Storage, Volumetric> {
	type Item = (Depth, Storage::OctantId, &'a mut OctantStorageAccessorMut::<'a, Storage>);
	fn next(& mut self) -> Option<Self::Item>{
		
		let (depth, octant_id, voxel) = self.to_be_visited.pop_front()?;
		let child_depth = depth + 1;
		self.storage_accessor.get_existing_children(&octant_id).ok()?
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
					self.to_be_visited.push_back((child_depth, child_id, child_voxel));
				}
			);

		// when I'll understand lifetimes better I am going to do this without unsafe if possible,
		// SAFETY: this is safe because lifetime of the iterator is the same as Item
		let safe_accessor = unsafe {&mut *(&mut self.storage_accessor as *mut OctantStorageAccessorMut::<'a, Storage>)};
			
		Some((depth, octant_id, safe_accessor))
	}
}
