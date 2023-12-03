use std::collections::HashSet;

/// A 2D coordinate with unsigned integers
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct UCoord {
    pub row: usize,
    pub col: usize,
}

// impl UCoord {
//     pub fn neighbors(&self) -> HashSet<UCoord> {

//     }
// }
