use std::fmt::Debug;
//use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]

// top is back
// bottom is front
pub enum OctantPlacement {
	/// [0, 0, 0]
	LOWER_BOTTOM_LEFT	= 0,
	/// [0, 0, 1]
	LOWER_TOP_LEFT 		= 1,
	/// [0, 1, 0]
	UPPER_BOTTOM_LEFT 	= 2,
	/// [0, 1, 1]
	UPPER_TOP_LEFT 		= 3,
	
	/// [1, 0, 0]
	LOWER_BOTTOM_RIGHT 	= 4,
	/// [1, 0, 1]
	LOWER_TOP_RIGHT 	= 5,
	/// [1, 1, 0]
	UPPER_BOTTOM_RIGHT 	= 6,
	/// [1, 1, 1]
	UPPER_TOP_RIGHT 	= 7,
}

impl OctantPlacement {
	pub const OCTANTS_COUNT: usize = 8;
	pub const OCTANT_FACING_NEIGHBORS_COUNT: usize = 3;
	pub const OCTANT_DIAGONAL_NEIGHBORS_COUNT: usize = 4;
	pub const OCTANT_ALL_NEIGHBORS_COUNT: usize = Self::OCTANT_FACING_NEIGHBORS_COUNT + Self::OCTANT_DIAGONAL_NEIGHBORS_COUNT;
	
	pub const OCTANTS_ORDERED: [OctantPlacement; OctantPlacement::OCTANTS_COUNT] = [
		Self::LOWER_BOTTOM_LEFT,
		Self::LOWER_TOP_LEFT,	
		Self::UPPER_BOTTOM_LEFT,
		Self::UPPER_TOP_LEFT,
		
		Self::LOWER_BOTTOM_RIGHT,
		Self::LOWER_TOP_RIGHT,	
		Self::UPPER_BOTTOM_RIGHT,
		Self::UPPER_TOP_RIGHT
	];

	pub const OCTANT_FACING_NEIGHBORS: [[(OctantNeighborDirection, OctantPlacement); Self::OCTANT_FACING_NEIGHBORS_COUNT] ; Self::OCTANTS_COUNT] = [
		[// LOWER_BOTTOM_LEFT
			// [*  ]
			// [* *]
			(OctantNeighborDirection::E, OctantPlacement::LOWER_BOTTOM_RIGHT),
			(OctantNeighborDirection::N, OctantPlacement::LOWER_TOP_LEFT),
			(OctantNeighborDirection::U, OctantPlacement::UPPER_BOTTOM_LEFT)
		],

		[// LOWER_TOP_LEFT
			// [* *]
			// [*  ]
			(OctantNeighborDirection::E, OctantPlacement::LOWER_TOP_RIGHT),
			(OctantNeighborDirection::S, OctantPlacement::LOWER_BOTTOM_LEFT),
			(OctantNeighborDirection::U, OctantPlacement::UPPER_TOP_LEFT)
		],
		
		[// LOWER_BOTTOM_RIGHT
			// [  *]
			// [* *]
			(OctantNeighborDirection::W, OctantPlacement::LOWER_BOTTOM_LEFT),
			(OctantNeighborDirection::N, OctantPlacement::LOWER_TOP_RIGHT), 
			(OctantNeighborDirection::U, OctantPlacement::UPPER_BOTTOM_RIGHT)
		],

		[// LOWER_TOP_RIGHT
			// [* *]
			// [  *]
			(OctantNeighborDirection::W, OctantPlacement::LOWER_TOP_LEFT),
			(OctantNeighborDirection::S, OctantPlacement::LOWER_BOTTOM_RIGHT),
			(OctantNeighborDirection::U, OctantPlacement::UPPER_TOP_RIGHT)
		],
		

		[// UPPER_BOTTOM_LEFT
			// [*  ]
			// [* *]
			(OctantNeighborDirection::E, OctantPlacement::UPPER_BOTTOM_RIGHT),
			(OctantNeighborDirection::N, OctantPlacement::UPPER_TOP_LEFT),
			(OctantNeighborDirection::D, OctantPlacement::LOWER_BOTTOM_LEFT)
		],

		[// UPPER_TOP_LEFT
			// [* *]
			// [*  ]
			(OctantNeighborDirection::E, OctantPlacement::UPPER_TOP_RIGHT), 
			(OctantNeighborDirection::S, OctantPlacement::UPPER_BOTTOM_LEFT),
			(OctantNeighborDirection::D, OctantPlacement::LOWER_TOP_LEFT)
		],

		[// UPPER_BOTTOM_RIGHT
			// [  *]
			// [* *]
			(OctantNeighborDirection::W, OctantPlacement::UPPER_BOTTOM_LEFT),
			(OctantNeighborDirection::N, OctantPlacement::UPPER_TOP_RIGHT), 
			(OctantNeighborDirection::D, OctantPlacement::LOWER_BOTTOM_RIGHT)
		],

		[// UPPER_TOP_RIGHT
			// [* *]
			// [  *]
			(OctantNeighborDirection::W, OctantPlacement::UPPER_TOP_LEFT),
			(OctantNeighborDirection::S, OctantPlacement::UPPER_BOTTOM_RIGHT),
			(OctantNeighborDirection::D, OctantPlacement::LOWER_TOP_RIGHT)
		],	
	];

	pub const OCTANT_DIAGONAL_NEIGHBORS: [[(OctantNeighborDirection, OctantPlacement); Self::OCTANT_DIAGONAL_NEIGHBORS_COUNT] ; Self::OCTANTS_COUNT] = [
		[// LOWER_BOTTOM_LEFT
			// [* *]
			// [  *]
			(OctantNeighborDirection::UN, OctantPlacement::UPPER_TOP_LEFT),
			(OctantNeighborDirection::UE, OctantPlacement::UPPER_BOTTOM_RIGHT),
			(OctantNeighborDirection::UNE, OctantPlacement::UPPER_TOP_RIGHT),
			(OctantNeighborDirection::NE, OctantPlacement::LOWER_TOP_RIGHT)
		],
		[// LOWER_TOP_LEFT
			// [  *]
			// [* *]
			(OctantNeighborDirection::UE, OctantPlacement::UPPER_TOP_RIGHT),
			(OctantNeighborDirection::US, OctantPlacement::UPPER_BOTTOM_LEFT),
			(OctantNeighborDirection::USE, OctantPlacement::UPPER_BOTTOM_RIGHT),
			(OctantNeighborDirection::SE, OctantPlacement::LOWER_BOTTOM_RIGHT)
		],
		[// LOWER_BOTTOM_RIGHT
			// [* *]
			// [*  ]
			(OctantNeighborDirection::UN, OctantPlacement::UPPER_TOP_RIGHT),
			(OctantNeighborDirection::UW, OctantPlacement::UPPER_BOTTOM_LEFT),
			(OctantNeighborDirection::UNW, OctantPlacement::UPPER_TOP_LEFT),
			(OctantNeighborDirection::NW, OctantPlacement::LOWER_TOP_LEFT)
			
		],
		[// LOWER_TOP_RIGHT
			// [*  ]
			// [* *]
			(OctantNeighborDirection::UW, OctantPlacement::UPPER_TOP_LEFT),
			(OctantNeighborDirection::US, OctantPlacement::UPPER_BOTTOM_RIGHT),
			(OctantNeighborDirection::USW, OctantPlacement::UPPER_BOTTOM_LEFT),
			(OctantNeighborDirection::SW, OctantPlacement::LOWER_BOTTOM_LEFT)
		],

		[// UPPER_BOTTOM_LEFT
			// [* *]
			// [  *]
			(OctantNeighborDirection::DN, OctantPlacement::LOWER_TOP_LEFT),
			(OctantNeighborDirection::DE, OctantPlacement::LOWER_BOTTOM_RIGHT),
			(OctantNeighborDirection::DNE, OctantPlacement::LOWER_TOP_RIGHT),
			(OctantNeighborDirection::NE, OctantPlacement::UPPER_TOP_RIGHT)
		],
		[// UPPER_TOP_LEFT
			// [  *]
			// [* *]
			(OctantNeighborDirection::DE, OctantPlacement::LOWER_TOP_RIGHT),
			(OctantNeighborDirection::DS, OctantPlacement::LOWER_BOTTOM_LEFT),
			(OctantNeighborDirection::DSE, OctantPlacement::LOWER_BOTTOM_RIGHT),
			(OctantNeighborDirection::SE, OctantPlacement::UPPER_BOTTOM_RIGHT)
		],
		[// UPPER_BOTTOM_RIGHT
			// [* *]
			// [*  ]
			(OctantNeighborDirection::DN, OctantPlacement::LOWER_TOP_RIGHT),
			(OctantNeighborDirection::DW, OctantPlacement::LOWER_BOTTOM_LEFT), 
			(OctantNeighborDirection::DNW, OctantPlacement::LOWER_TOP_LEFT),
			(OctantNeighborDirection::NW, OctantPlacement::UPPER_TOP_LEFT)
		],
		[// UPPER_TOP_RIGHT
			// [*  ]
			// [* *]
			(OctantNeighborDirection::DW, OctantPlacement::LOWER_TOP_LEFT),
			(OctantNeighborDirection::DS, OctantPlacement::LOWER_BOTTOM_RIGHT),
			(OctantNeighborDirection::DSW, OctantPlacement::LOWER_BOTTOM_LEFT),
			(OctantNeighborDirection::SW, OctantPlacement::UPPER_BOTTOM_LEFT)
		],	
	];


	pub const fn facing_neighbors_for(octant_placement: OctantPlacement) -> [(OctantNeighborDirection, OctantPlacement); Self::OCTANT_FACING_NEIGHBORS_COUNT] {
		Self::OCTANT_FACING_NEIGHBORS[octant_placement as usize]
	}

	pub const fn diagonal_neighbors_for(octant_placement: OctantPlacement) -> [(OctantNeighborDirection, OctantPlacement); Self::OCTANT_DIAGONAL_NEIGHBORS_COUNT] {
		Self::OCTANT_DIAGONAL_NEIGHBORS[octant_placement as usize]
	}

	pub const fn all_neighbors_for(octant_placement: OctantPlacement) -> [(OctantNeighborDirection, OctantPlacement); Self::OCTANT_ALL_NEIGHBORS_COUNT] {
		let facing_neighbors = Self::facing_neighbors_for(octant_placement);
		let diagonal_neighbors = Self::diagonal_neighbors_for(octant_placement);

		[
			facing_neighbors[0],
			facing_neighbors[1],
			facing_neighbors[2],

			diagonal_neighbors[0],
			diagonal_neighbors[1],
			diagonal_neighbors[2],
			diagonal_neighbors[3]
		]
	}
}

impl TryFrom<usize> for OctantPlacement {
	type Error = ();
	fn try_from(index: usize) -> Result<Self, Self::Error> {
		OctantPlacement::OCTANTS_ORDERED.get(index).ok_or(()).copied()
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OctantNeighborDirection{
	// Facing directions
	/// UP
	U = 0,
	/// DOWN
	D = 1,
	/// NORTH
	N = 2,
	/// SOUTH
	S = 3,
	/// EAST
	E = 4,
	/// WEST
	W = 5,

	// Diagonal directions
	
	/// NORTH-EAST
	NE = 6,
	/// NORTH-WEST
	NW = 7,
	/// SOUTH-EAST
	SE = 8,
	/// SOUTH-WEST
	SW = 9,
	
	
	/// UP-NORTH
	UN = 10,
	/// UP-SOUTH
	US = 11,
	/// UP-EAST
	UE = 12,
	/// UP-WEST
	UW = 13,
	
	/// DOWN-NORTH
	DN = 14,
	/// DOWN-SOUTH
	DS = 15,
	/// DOWN-EAST
	DE = 16,
	/// DOWN-WEST
	DW = 17,
	
	/// UP-NORTH-EAST
	UNE = 18,
	/// UP-NORTH-WEST
	UNW = 19,
	/// UP-SOUTH-EAST
	USE = 20,
	/// UP-SOUTH-WEST
	USW = 21,

	/// DOWN-NORTH-EAST
	DNE = 22,
	/// DOWN-NORTH-WEST
	DNW = 23,
	/// DOWN-SOUTH-EAST
	DSE = 24,
	/// DOWN-SOUTH-WEST
	DSW = 25
	
}
impl OctantNeighborDirection {
	pub const NEIGHBOR_DIRECTIONS_COUNT: usize = 26;
	pub const FACING_NEIGHBOR_DIRECTIONS_COUNT: usize = 6;
	pub const DIAGONAL_NEIGHBOR_DIRECTIONS_COUNT: usize = Self::NEIGHBOR_DIRECTIONS_COUNT - Self::FACING_NEIGHBOR_DIRECTIONS_COUNT;

	pub const ALL_DIRECTIONS: [Self; Self::NEIGHBOR_DIRECTIONS_COUNT] = [
		Self::U,
		Self::D,
		Self::N,
		Self::S,
		Self::E,
		Self::W,
		
		Self::NE,
		Self::NW,
		Self::SE,
		Self::SW,
		
		Self::UN,
		Self::US,
		Self::UE,
		Self::UW,

		Self::DN,
		Self::DS,
		Self::DE,
		Self::DW,
		
		Self::UNE,
		Self::UNW,
		Self::USE,
		Self::USW,

		Self::DNE,
		Self::DNW,
		Self::DSE,
		Self::DSW
	];
	

	pub const FACING_DIRECTIONS: [Self; Self::FACING_NEIGHBOR_DIRECTIONS_COUNT]	= [
		Self::U,
		Self::D,
		Self::N,
		Self::S,
		Self::E,
		Self::W,
	];
	

	pub const DIAGONAL_DIRECTIONS: [Self; Self::DIAGONAL_NEIGHBOR_DIRECTIONS_COUNT] = [
		Self::NE,
		Self::NW,
		Self::SE,
		Self::SW,
		
		Self::UN,
		Self::US,
		Self::UE,
		Self::UW,

		Self::DN,
		Self::DS,
		Self::DE,
		Self::DW,
		
		Self::UNE,
		Self::UNW,
		Self::USE,
		Self::USW,

		Self::DNE,
		Self::DNW,
		Self::DSE,
		Self::DSW
	];
	
}
