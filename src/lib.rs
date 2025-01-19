// traits
pub mod octant_storage_trait;
pub mod voxel_trait;

// utils
pub mod octant_meta;
pub mod octant_storage_utils;
pub mod octant_storage_voxel_utils;

// implementations
pub mod voxels;
pub mod octree_base;
pub mod spatial_octree_base;
pub mod morton_based_storage;


// predefined and ready to use types
use morton_based_storage::hashed_octant_storage::HashedOctantStorage;

pub type SparseOctreeHashed<CustomData> = octree_base::OctreeBase<HashedOctantStorage<CustomData>>;
pub type SpatialSparseOctreeHashed<CustomData, Volumetric = voxels::voxel_cube::VolumetricCube>
	 = spatial_octree_base::SpatialOctreeBase<HashedOctantStorage<CustomData>, Volumetric>;

// default option
pub type SparseOctree<CustomData> = SparseOctreeHashed<CustomData>;
pub type SpatialSparseOctree<CustomData, Volumetric = voxels::voxel_cube::VolumetricCube>
	 = SpatialSparseOctreeHashed<CustomData, Volumetric>;