use std::fmt::Debug;

use super::octant_meta::OctantPlacement;
use super::Depth;
pub trait OctantStorage{
	type OctantId: Copy + PartialEq;
	/// Iterator which will go through all OctantIds of ancestors of given OctantId(exclusive)
	type ParentIdIterator: Iterator<Item = Self::OctantId>;
	/// Custom/User data.
	type Data;

	/// Get octant id which can be used to access root octant node.
	/// 
    ///```
	/// let storage = /* instantiate struct which implements octant storage */
	/// let state = storage.get_root_id();
    /// ```
	fn get_root_id(&self) -> Self::OctantId;

	/// Get octant maximum depth supported by storage.
	/// 
	/// ## Examples
    ///```
	/// let max_depth: Depth = storage.get_max_depth();
    /// ```
	fn get_max_depth(&self) -> Depth;

	/// Get depth for octant.
    ///
	/// ## Returns
	/// `Some(Depth)` when octant_id exists, otherwise `None`
    /// ## Examples
    ///
    /// ```
    ///	let depth: Depth = storage.get_octant_depth(&octant_id).unwrap();
    /// ```
	fn get_octant_depth(&self, octant_id: &Self::OctantId) -> Option<Depth>;

	/// Get data associated with octant_id.
    ///
	/// ## Returns
	/// `Some(&Data)` when octant_id exists, otherwise `None`
    /// ## Examples
    ///
    /// ```
    ///	let data = storage.get_octant(&octant_id).unwrap();
    /// ```
	fn get_octant(&self, octant_id: &Self::OctantId) -> Option<&Self::Data>;

	/// Get mutable data associated with octant_id.
    ///
	/// ## Returns
	/// `Some(&mut Data)` when octant_id exists, otherwise `None`
    /// ## Examples
    ///
    /// ```
    ///	let data = storage.get_octant_mut(&octant_id).unwrap();
    /// ```
	fn get_octant_mut(&mut self, octant_id: &Self::OctantId) -> Option<&mut Self::Data>;

	/// Get octant id of child which occupies requested placement.
	/// 
	/// ## Returns
	///  `StorageResult` which contains either child `OctantId` or `StorageError`.
	/// ## Errors
	///   * OverMaxDepth - when requested child lies below maximum depth
	///   * InvalidOctantId - when parent_id is not valid or found in storage
	///   * ChildNotFound - when child placement is not occupied by existing child
	///  
    /// ## Arguments
    ///
    /// * `parent_id` - parent id of which child will be looked-up
	/// * `child_placement` - child octant position relative to parent
    ///
    /// ## Examples
    ///
    /// ```
	/// let result_child_id = storage.get_existing_child(&parent_id, OctantPlacement::LOWER_BOTTOM_LEFT);
    /// ```
	fn get_existing_child(&self, parent_id: &Self::OctantId, child_placement: OctantPlacement) -> StorageResult<Self::OctantId>;

	/// Get all parents in form of iterator which iterates till root node id.
	/// ## Returns
	///  `Some(OctantId)` when octant id exists, otherwise `None`.
    /// ## Examples
    /// ```
	/// let parent_id: OctantId = storage.get_parent(&octant_id);
    /// ```
	fn get_parent(&self, octant_id: &Self::OctantId) -> Option<Self::OctantId>;

	/// Get all parents in form of iterator which iterates till root node id.
	/// ## Returns
	///  `Some(ParentIdIterator)` when octant id exists, otherwise `None`.
    /// ## Examples
    /// ```
	/// let parent_iter = storage.get_ancestors_for(&octant_id);
	/// for parent_id in parent_iter{
	/// 	//...//
	/// }
    /// ```
	fn get_ancestors_for(&self, octant_id: &Self::OctantId) -> Option<Self::ParentIdIterator>;

// Optional Implementations(default implementations)

	/// Get all existing children of parent id.
	/// 
	/// #### Note: octant array returned by this function must be in order set by `OctantPlacement::ORDERED`,
	/// #### otherwise some functionality expecting ordered access might not work correctly.
	/// 
	/// ## Returns
	///  `StorageResult` which contains either array of existing child ids `[Some(OctantId); OctantPlacement::OCTANTS_COUNT]` or `StorageError`,
	///  where `Some(OctantId)` when child exists in the storage, otherwise `None`
	/// ## Errors
	///   * OverMaxDepth - when requested child lies below maximum depth
	///   * InvalidOctantId - when parent_id is not valid or found in storage
	///   * ChildNotFound - when child placement is not occupied by existing child
	///  
    /// ## Examples
    ///
    /// ```
	/// let child_array = storage.get_existing_children(&parent_id).unwrap();
	/// child_array.iter()
	/// 	.flatten()
	/// 	.for_each(|existing_child: OctantId| /* do something with child id */);
    /// ```
	fn get_existing_children(&self, parent_id: &Self::OctantId) -> StorageResult<[Option<Self::OctantId>; OctantPlacement::OCTANTS_COUNT]> {
		let ordered_placement = OctantPlacement::OCTANTS_ORDERED;
		let check_error = self.get_existing_child(parent_id, ordered_placement[0]);

		let first_child = match check_error {
			Ok(first_child_id) => {
				Some(first_child_id)
			},
			Err(StorageError::ChildNotFound(_)) => {
				None
			}
			Err(err) => return Err(err)
		};
	
		Ok(
			[
				first_child,
				self.get_existing_child(parent_id, ordered_placement[1]).ok(),
				self.get_existing_child(parent_id, ordered_placement[2]).ok(),
				self.get_existing_child(parent_id, ordered_placement[3]).ok(),

				self.get_existing_child(parent_id, ordered_placement[4]).ok(),
				self.get_existing_child(parent_id, ordered_placement[5]).ok(),
				self.get_existing_child(parent_id, ordered_placement[6]).ok(),
				self.get_existing_child(parent_id, ordered_placement[7]).ok()
			]
		)
	}

	/// Get octant/child placement relative to parent using parent id and child id.
	/// 
	/// ## Returns
	///  `StorageResult` which contains either child `OctantId` or `StorageError`.
	/// ## Errors
	///   * OverMaxDepth - Never, since if both ids are valid, then they already are withing max depth
	///   * InvalidOctantId - when parent_id or child_id is not valid or found in storage
	///   * ChildNotFound - when child placement is not occupied by existing child
	///  
    /// ## Examples
    ///
    /// ```
	/// let child_placement: OctantPlacement = OctantPlacement::UPPER_BOTTOM_LEFT;
	/// let child_id = storage.get_existing_child(&parent_id).unwrap();
	/// let which_placement: OctantPlacement = storage.which_child_of(&parent_id, &child_id).unwrap();
	/// assert_eq!(child_placement, which_placement);
	/// 
	/// 
    /// ```
	fn which_child_of(&self, parent_id: &Self::OctantId, child_id: &Self::OctantId) -> StorageResult<OctantPlacement> {
		self.get_existing_children(parent_id)?.iter()
			.zip(	OctantPlacement::OCTANTS_ORDERED.iter())
			.filter_map(
				|(&maybe_octant_id, octant_placement)| {
					let octant_id  = maybe_octant_id?;
					Some((octant_id, octant_placement)) 
				}
			)
			.find(|(octant_id, _)|{
				*octant_id == *child_id
			})
			.map(
				|(_, &octant_placement)|{
					octant_placement
				}
			)
			.ok_or(StorageError::ChildNotFound(None))
	}
	
}


pub trait ModifiableOctantStorage : OctantStorage {
	/// Initializes or reinitialize root node and returns previously held data if there were any.
	/// 
	/// ## Returns
	///  `Some(Data)` when root node already held data which wil be replaced and returned, otherwise `None`.
	///  
    /// ## Arguments
    ///
    /// * `custom_data` - which will set or replace root node's data
    ///
    /// ## Examples
    ///
    /// ```
	/// let maybe_old_data = storage.insert_root(new_data);
    /// ```
	fn insert_root(&mut self, custom_data: Self::Data) -> Option<Self::Data>;

	/// Get octant id of child which occupies requested placement.
	/// 
	/// ## Returns
	///  `StorageResult` which contains either child `(OctantId, Option<Data>)` or `StorageError`,
	///  where `Option<Data>` is `Some(Data)` when requested octant previously already held `Data` otherwise `None`.
	/// ## Errors
	///   * OverMaxDepth - when created child lies below maximum depth
	///   * InvalidOctantId - when parent_id is not valid or found in storage
	///   * ChildNotFound - Never, since child is going to be created by this function
	///  
    /// ## Arguments
    ///
    /// * `parent_id` - parent id of which child will be created or replaced
	/// * `child_placement` - child octant position relative to parent which will be created or replaced
	/// * `custom_data` - data which will be used to initialize or replace exiting child's data
    ///
    /// ## Examples
    ///
    /// ```
	/// let result_created_child = storage.insert_octant(&parent_id, OctantPlacement::LOWER_BOTTOM_LEFT, Data);
    /// ```
	fn insert_octant(&mut self, parent_id: &Self::OctantId, child_octant_placement: OctantPlacement, custom_data: Self::Data) -> StorageResult<(Self::OctantId, Option<Self::Data>)>;
	
	
	/// Removes input octant id and continues down the tree till all nodes of this branch are removed.
	/// 
	/// ## Returns
	///  `Some(())` when octant id exists and removal was successful, otherwise `None`
	///  
    /// ## Arguments
    /// * `octant_id` - parent from which recursive cascade removal will start from
    /// ## Examples
    /// ```
	/// storage.remove_octant(&storage.get_root_id());
	/// assert_eq!(storage.get_octant(&storage.get_root_id(), None);
    /// ```
	fn remove_octant(&mut self, octant_id: &Self::OctantId) -> Option<()>;

	/// Removes input octant id and continues down the tree till all nodes of this branch are removed and collect octant id and associated data into vector.
	/// 
	/// ## Returns
	///  `Some(())` when octant id exists and removal was successful, otherwise `None`
	///  
    /// ## Arguments
    /// * `octant_id` - parent from which recursive cascade removal will start from
	/// * `collected_octant_data` - removed octant's data and associated octant id will be collected into this vector
    /// ## Examples
    /// ```
	/// let removed_octants: Vec<(Storage::OctantId, Storage::Data)> = Vec::default();
	/// storage.remove_octant_and_collect(&storage.get_root_id());
	/// for (octant_id, data) in removed_octants {
	/// 	println!("removed octant_id: ", octant_id);
	/// }
	/// assert_eq!(storage.get_octant(&storage.get_root_id(), None);
    /// ```
	fn remove_octant_and_fill(&mut self, octant_id: &Self::OctantId, collected_octant_data: &mut Vec<(Self::OctantId, Self::Data)>) -> Option<()>;

	fn new_with_root(root_custom_data: Self::Data) -> Self
	where Self: Default{
		let mut new_storage = Self::default();
		new_storage.insert_root(root_custom_data);
		new_storage
	}


	/// Uses subdivides into all positions of octants using function to pass Data into created/replaced child octant.
	/// 
	/// #### Note: octant array returned by this function must be in order set by `OctantPlacement::ORDERED`,
	/// #### otherwise some functionality expecting ordered access might not work correctly
	/// ## Returns
	///  `StorageResult` which contains either array of child ids `[OctantId; OctantPlacement::OCTANTS_COUNT]` or `StorageError`
	/// ## Errors -> StorageError
	///   * OverMaxDepth - when crated children lies below maximum depth
	///   * InvalidOctantId - when parent_id is not valid or found in storage
	///   * ChildNotFound - Never, since all children are going to be created by this function
	///  
    /// ## Arguments
    ///
    /// * `parent_id` - parent id of which child will be created or replaced
	/// * `create_custom_data` - functions which takes `OctantPlacement` as argument and returns `Data` (this is done to not require Copy/Clone trait implemented)
    ///
    /// ## Examples
    ///
    /// ```
	/// let result_created_child = storage.subdivide(&parent_id, |_: OctantPlacement| Data::default());
    /// ```
	fn subdivide<F>(&mut self, parent_id: &Self::OctantId, mut create_custom_data: F) -> StorageResult<[Self::OctantId; OctantPlacement::OCTANTS_COUNT]>
	where F: FnMut(OctantPlacement) -> Self::Data {
		const ORDERED_PLACEMENT: [OctantPlacement; OctantPlacement::OCTANTS_COUNT] = OctantPlacement::OCTANTS_ORDERED;
		Ok(
			[
				self.insert_octant(parent_id, ORDERED_PLACEMENT[0], create_custom_data(ORDERED_PLACEMENT[0]))?.0,
				self.insert_octant(parent_id, ORDERED_PLACEMENT[1], create_custom_data(ORDERED_PLACEMENT[1]))?.0,
				self.insert_octant(parent_id, ORDERED_PLACEMENT[2], create_custom_data(ORDERED_PLACEMENT[2]))?.0,
				self.insert_octant(parent_id, ORDERED_PLACEMENT[3], create_custom_data(ORDERED_PLACEMENT[3]))?.0,

				self.insert_octant(parent_id, ORDERED_PLACEMENT[4], create_custom_data(ORDERED_PLACEMENT[4]))?.0,
				self.insert_octant(parent_id, ORDERED_PLACEMENT[5], create_custom_data(ORDERED_PLACEMENT[5]))?.0,
				self.insert_octant(parent_id, ORDERED_PLACEMENT[6], create_custom_data(ORDERED_PLACEMENT[6]))?.0,
				self.insert_octant(parent_id, ORDERED_PLACEMENT[7], create_custom_data(ORDERED_PLACEMENT[7]))?.0
			]
		)
	}

	/// Uses `self.subdivide()` function to create/replace all children with `default` when `Default` is implemented for `Data`.
	/// 
	/// #### Note: octant array returned by this function must be in order set by `OctantPlacement::ORDERED`,
	/// #### otherwise some functionality expecting ordered access might not work correctly.
	/// ## Returns
	///  `StorageResult` which contains either array of child ids `[OctantId; OctantPlacement::OCTANTS_COUNT]` or `StorageError`
	/// ## Errors -> StorageError
	///   * OverMaxDepth - when crated children lies below maximum depth
	///   * InvalidOctantId - when parent_id is not valid or found in storage
	///   * ChildNotFound - Never, since all children are going to be created by this function
	///  
    /// ## Arguments
    ///
    /// * `parent_id` - parent id of which child will be created or replaced
    ///
    /// ## Examples
    ///
    /// ```
	/// let result_created_child = storage.subdivide(&parent_id, |_: OctantPlacement| Data::default());
    /// ```
	fn subdivide_with_default(&mut self, parent_id: &Self::OctantId) -> StorageResult<[Self::OctantId; OctantPlacement::OCTANTS_COUNT]>
	where Self::Data: Default{
		self.subdivide(parent_id, |_: OctantPlacement| Self::Data::default())
	}
	
	/// Removes input octant id and continues down the tree till all nodes of this branch are removed and collect octant id and associated data into vector and return it.
	/// 
	/// ## Returns
	///  `Some(Vec<(Self::OctantId, Self::Data)>)` when octant id exists and removal was successful, otherwise `None`
	///  
    /// ## Arguments
    /// * `octant_id` - parent from which recursive cascade removal will start from
    /// ## Examples
    /// ```
	/// let removed_octants: Vec<(Storage::OctantId, Storage::Data)> = storage.remove_octant_and_collect(&storage.get_root_id());
	/// for (octant_id, data) in removed_octants {
	/// 	println!("removed octant_id: ", octant_id);
	/// }
	/// assert_eq!(storage.get_octant(&storage.get_root_id(), None);
    /// ```
	fn remove_octant_and_collect(&mut self, octant_id: &Self::OctantId) -> Option<Vec<(Self::OctantId, Self::Data)>> {
		let mut octants_and_data: Vec<(Self::OctantId, Self::Data)> = Vec::<(Self::OctantId, Self::Data)>::default();
		self.remove_octant_and_fill(octant_id, &mut octants_and_data)?;

		Some(octants_and_data)
	}
}

pub struct OctantStorageAccessorMut<'a, Storage> {
	storage: &'a mut Storage
}

impl <'a, Storage: OctantStorage>  OctantStorageAccessorMut<'a, Storage> {
	pub fn new(storage_to_access: &'a mut Storage) -> Self {
		Self{
			storage: storage_to_access
		}
	}
}

impl <'a, Storage: OctantStorage> OctantStorage for OctantStorageAccessorMut<'a, Storage> {
	type OctantId = Storage::OctantId;
	type ParentIdIterator = Storage::ParentIdIterator;
	type Data = Storage::Data;

	fn get_root_id(&self) -> Self::OctantId {
		self.storage.get_root_id()
	}

	fn get_max_depth(&self) -> Depth{
		self.storage.get_max_depth()
	}

	fn get_octant_depth(&self, octant_id: &Self::OctantId) -> Option<Depth> {
		self.storage.get_octant_depth(octant_id)
	}

	fn get_octant(&self, octant_id: &Self::OctantId) -> Option<&Self::Data> {
		self.storage.get_octant(octant_id)
	}

	fn get_octant_mut(&mut self, octant_id: &Self::OctantId) -> Option<&mut Self::Data> {
		self.storage.get_octant_mut(octant_id)
	}

	fn get_existing_child(&self, parent_id: &Self::OctantId, child_placement: OctantPlacement) -> StorageResult<Self::OctantId> {
		self.storage.get_existing_child(parent_id, child_placement)
	}

	fn get_ancestors_for(&self, octant_id: &Self::OctantId) -> Option<Self::ParentIdIterator> {
		self.storage.get_ancestors_for(octant_id)
	}

	fn get_parent(&self, octant_id: &Self::OctantId) -> Option<Self::OctantId> {
		self.storage.get_parent(octant_id)
	}

	fn get_existing_children(&self, parent_id: &Self::OctantId) -> StorageResult<[Option<Self::OctantId>; OctantPlacement::OCTANTS_COUNT]>{
		self.storage.get_existing_children(parent_id)
	}

	fn which_child_of(&self, parent_id: &Self::OctantId, child_id: &Self::OctantId) -> StorageResult<OctantPlacement> {
		self.storage.which_child_of(parent_id, child_id)
	}
}



pub struct OctantStorageAccessor<'a, Storage> {
	storage: &'a Storage,

}

impl <'a, Storage: OctantStorage>  OctantStorageAccessor<'a, Storage> {
	pub fn new(storage_to_access: &'a Storage) -> Self {
		Self{
			storage: storage_to_access
		}
	}
}

impl <'a, Storage: OctantStorage> OctantStorage for OctantStorageAccessor<'a, Storage> {
	type OctantId = Storage::OctantId;
	type ParentIdIterator = Storage::ParentIdIterator;
	type Data = Storage::Data;

	fn get_root_id(&self) -> Self::OctantId {
		self.storage.get_root_id()
	}

	fn get_max_depth(&self) -> Depth{
		self.storage.get_max_depth()
	}

	fn get_octant_depth(&self, octant_id: &Self::OctantId) -> Option<Depth> {
		self.storage.get_octant_depth(octant_id)
	}

	fn get_octant(&self, octant_id: &Self::OctantId) -> Option<&Self::Data> {
		self.storage.get_octant(octant_id)
	}

	fn get_octant_mut(&mut self, _: &Self::OctantId) -> Option<&mut Self::Data> {
		panic!("Using mutable function on immutable octant storage accessor, use OctantStorageAccessorMut instead.")
	}

	fn get_existing_child(&self, parent_id: &Self::OctantId, child_placement: OctantPlacement) -> StorageResult<Self::OctantId> {
		self.storage.get_existing_child(parent_id, child_placement)
	}

	fn get_ancestors_for(&self, octant_id: &Self::OctantId) -> Option<Self::ParentIdIterator> {
		self.storage.get_ancestors_for(octant_id)
	}

	fn get_parent(&self, octant_id: &Self::OctantId) -> Option<Self::OctantId> {
		self.storage.get_parent(octant_id)
	}

	fn get_existing_children(&self, parent_id: &Self::OctantId) -> StorageResult<[Option<Self::OctantId>; OctantPlacement::OCTANTS_COUNT]>{
		self.storage.get_existing_children(parent_id)
	}

	fn which_child_of(&self, parent_id: &Self::OctantId, child_id: &Self::OctantId) -> StorageResult<OctantPlacement> {
		self.storage.which_child_of(parent_id, child_id)
	}	
}


/* 
impl <'a, Data, Storage: OctantStorage<Data>> From<OctantStorageAccessorMut<'a, Data, Storage>> for OctantStorageAccessor<'a, Data, Storage> {
	fn from(value: OctantStorageAccessorMut<'a, Data, Storage>) -> Self {
		OctantStorageAccessor::<'a, Data, Storage>::new(value.storage) 
	}
}
*/

pub type StorageResult<T> = Result<T, StorageError>;

pub enum StorageError{
	OverMaxDepth(Depth),
	InvalidOctantId,
	ChildNotFound(Option<OctantPlacement>)
}

impl Debug for StorageError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::OverMaxDepth(max_depth) => {
				write!(f, "Reached depth which is over maximum depth({:?})", max_depth)
			} 
			Self::InvalidOctantId => {
				write!(f, "Encountered invalid octant id")
			}
			Self::ChildNotFound(Some(missing_child_placement)) => {
				
				write!(f, "Could not find child with placement: {:?}", missing_child_placement)
			}
			Self::ChildNotFound(None) => {
				
				write!(f, "Could not find any child")
			}
		}
		
	}
}
