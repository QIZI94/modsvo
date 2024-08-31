#[cfg(test)]
mod tests{			
	#[derive(Debug, Default)]
	struct NoData;
	use std::fmt::Debug;




	use modsvo::morton_based_storage::hashed_octant_storage::HashedOctantStorage;

	use super::super::test_modifiable_octant_storage;

	#[test]
	fn test_hash_storage_interface_functions(){
		let mut octant_storage: HashedOctantStorage<u32> = HashedOctantStorage::<u32>::default();

		test_modifiable_octant_storage(&mut octant_storage,3);
	}
}