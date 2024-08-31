use super::octant_meta::OctantPlacement;

pub trait Voxel: Copy{
	fn make_sub_voxel(&self, sub_voxel_placement: OctantPlacement) -> Self;

	fn subdivide_voxel(&self) -> [Self; OctantPlacement::OCTANTS_COUNT]{
		[
			self.make_sub_voxel(OctantPlacement::UPPER_TOP_LEFT),
			self.make_sub_voxel(OctantPlacement::UPPER_TOP_RIGHT),
			self.make_sub_voxel(OctantPlacement::UPPER_BOTTOM_RIGHT),
			self.make_sub_voxel(OctantPlacement::UPPER_BOTTOM_LEFT),

			self.make_sub_voxel(OctantPlacement::LOWER_TOP_LEFT),
			self.make_sub_voxel(OctantPlacement::LOWER_TOP_RIGHT),
			self.make_sub_voxel(OctantPlacement::LOWER_BOTTOM_RIGHT),
			self.make_sub_voxel(OctantPlacement::LOWER_BOTTOM_LEFT)
		]
	}
}