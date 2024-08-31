
use std::collections::HashMap;
use super::super::octant_storage_trait::{OctantStorage, ModifiableOctantStorage, StorageError, StorageResult};
use super::morton_octant_id::{MortonOctantId, MortonParentIdIterator};
use super::super::Depth;
use super::super::octant_meta::OctantPlacement;



type HashedMortonMap<Data> = HashMap<MortonOctantId, Data>;

pub struct HashedOctantStorage<Data>{
	octants: HashedMortonMap<Data>
}

impl<Data> HashedOctantStorage<Data> {
	pub fn iter(&self) -> impl Iterator<Item = (&MortonOctantId, &Data)> {
		self.octants.iter()
	}

	pub fn iter_mut(&mut self) -> impl Iterator<Item = (&MortonOctantId, &mut Data)> {
		self.octants.iter_mut()
	}

	pub fn storage(&self) ->  &HashedMortonMap<Data> {
		&self.octants
	}
}

impl<Data> OctantStorage for HashedOctantStorage<Data> {
	type OctantId = MortonOctantId;
	type ParentIdIterator = MortonParentIdIterator;
	type Data = Data;

	fn get_max_depth(&self) -> Depth {
		MortonOctantId::MAX_DEPTH
	}
	//
	fn get_root_id(&self) -> Self::OctantId {
		MortonOctantId::ROOT_OCTANT_ID
	}

	fn get_octant_depth(&self, octant_id: &Self::OctantId) -> Option<Depth> {
		let _ = self.get_octant(octant_id)?;
		Some(octant_id.compute_depth())
	}

	fn get_octant(&self, octant_id: &Self::OctantId) -> Option<&Self::Data> {
		self.octants.get(octant_id)
	}	

	fn get_octant_mut(&mut self, octant_id: &Self::OctantId) -> Option<&mut Self::Data> {
		self.octants.get_mut(octant_id)
	}
	
	fn get_existing_child(&self, parent_id: &Self::OctantId, child_placement: OctantPlacement) -> StorageResult<Self::OctantId> {
		if parent_id.compute_depth() >= MortonOctantId::MAX_DEPTH{
			return Err(StorageError::OverMaxDepth(MortonOctantId::MAX_DEPTH));
		}

		let child_id: MortonOctantId = parent_id.children_ids()[child_placement as usize];
		if self.get_octant(&child_id).is_some(){
			Ok(child_id)
		}
		else if self.get_octant(&parent_id).is_some(){
				Err(StorageError::ChildNotFound(Some(child_placement)))
		}
		else {
			Err(StorageError::InvalidOctantId)
		}
	}

	fn get_ancestors_for(&self, octant_id: &Self::OctantId) -> Option<Self::ParentIdIterator> {
		let _ = self.get_octant(octant_id)?;
		Some(octant_id.parent_id_iter())
	}

	fn get_parent(&self, octant_id: &Self::OctantId) -> Option<Self::OctantId> {
		if *octant_id == self.get_root_id() {
			None
		}
		else {
			let _ = self.get_octant(octant_id)?;
			Some(octant_id.parent_id())
		}
	}
	
	fn which_child_of(&self, parent_id: &Self::OctantId, child_id: &Self::OctantId) -> StorageResult<OctantPlacement> {
		let _ = self.get_octant(child_id).ok_or(StorageError::InvalidOctantId)?;
		parent_id.has_child(child_id).ok_or(StorageError::ChildNotFound(None))
	}
}

impl<Data> ModifiableOctantStorage for HashedOctantStorage<Data> {

	fn new_with_root(root_custom_data: Self::Data) -> Self
	where Self: Default {
		HashedOctantStorage{
			octants: HashMap::from([(MortonOctantId::ROOT_OCTANT_ID, root_custom_data)])
		}
	}

	fn insert_root(&mut self, root_custom_data: Self::Data) -> Option<Self::Data>{
		self.octants.insert(self.get_root_id(), root_custom_data)
	}


	fn insert_octant(&mut self, parent_id: &Self::OctantId, child_octant_placement: OctantPlacement, custom_data: Self::Data) -> StorageResult<(Self::OctantId, Option<Self::Data>)> {
		if parent_id.compute_depth() >= MortonOctantId::MAX_DEPTH {
			Err(StorageError::OverMaxDepth(MortonOctantId::MAX_DEPTH))
		}
		else {
			let _ = self.get_octant(parent_id).ok_or(StorageError::InvalidOctantId)?;
			let child_id: MortonOctantId = parent_id.child_id_by_placement(child_octant_placement);
			let old_data: Option<Data> = self.octants.insert(child_id, custom_data);

			Ok((child_id, old_data))
		}		
	}

	fn remove_octant(&mut self, octant_id: &Self::OctantId) -> Option<()> {
		let _ = self.get_octant(octant_id)?;

		if *octant_id == self.get_root_id() {
			self.octants.clear();
		}
		else {
			self.octants.remove(octant_id)?;
			for child_id in octant_id.children_ids(){
				self.remove_octant(&child_id);
			}
		}

		Some(())
	}

	fn remove_octant_and_fill(&mut self, octant_id: &Self::OctantId, collected_octant_data: &mut Vec<(Self::OctantId, Self::Data)>) -> Option<()> {
		let _ = self.get_octant(octant_id)?;
		if *octant_id == self.get_root_id() {
			let capacity_left_for_filling: usize = collected_octant_data.capacity() - collected_octant_data.len();
			if capacity_left_for_filling < self.octants.len() {
				collected_octant_data.reserve(self.octants.len() - capacity_left_for_filling);
			}
			collected_octant_data.extend(self.octants.drain());
		}
		else {
			let custom_data = self.octants.remove(octant_id)?;

			collected_octant_data.push((*octant_id, custom_data));

			for child_id in octant_id.children_ids(){
				self.remove_octant_and_fill(&child_id, collected_octant_data);
			}
		}

		Some(())
	}
}

impl<Data: Default> Default for HashedOctantStorage<Data> {
	fn default() -> Self {
		HashedOctantStorage {
			octants: HashMap::from([(MortonOctantId::ROOT_OCTANT_ID, Data::default())])
		}
	}
}


