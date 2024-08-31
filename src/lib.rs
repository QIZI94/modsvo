pub mod octant_meta;
pub mod octant_storage_trait;
//pub mod octant_storage_utils;
//pub mod octant_storage_voxel_utils;
pub mod voxel_trait;
pub mod voxels;
pub mod octree_base;
pub mod spatial_octree_base;
pub mod morton_based_storage;



use morton_based_storage::hashed_octant_storage::HashedOctantStorage;

pub type Depth = u8;

pub type SparseOctreeHashed<CustomData> = octree_base::OctreeBase<HashedOctantStorage<CustomData>>;
pub type SpatialSparseOctreeHashed<CustomData> = spatial_octree_base::SpatialOctreeBase<HashedOctantStorage<CustomData>>;

// default option
pub type SparseOctree<CustomData> = SparseOctreeHashed<CustomData>;
pub type SpatialSparseOctree<CustomData> = SpatialSparseOctreeHashed<CustomData>;