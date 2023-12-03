use std::collections::HashSet;

use itertools::Itertools;

/// A 2D coordinate with unsigned integers
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct UCoord {
    pub row: usize,
    pub col: usize,
}

impl UCoord {
    pub fn neighbors(&self) -> HashSet<UCoord> {
        let min_row: usize = usize::max(self.row, 1usize);
        let max_row: usize = usize::min(self.row, usize::MAX - 1);
        let min_col: usize = usize::max(self.col, 1usize);
        let max_col: usize = usize::min(self.col, usize::MAX - 1);

        ((min_row - 1)..=(max_row + 1))
            .cartesian_product((min_col - 1)..=(max_col + 1))
            .filter(|(row, col)| !(self.row == *row && self.col == *col))
            .map(|(row, col)| UCoord { row: row, col: col })
            .collect::<HashSet<UCoord>>()
    }
}

#[cfg(test)]
mod tests {
    use super::UCoord;

    #[test]
    fn test_neighbors() {
        assert_eq!((UCoord { row: 1, col: 1 }).neighbors().len(), 8);
        assert_eq!((UCoord { row: 0, col: 0 }).neighbors().len(), 3);
        assert_eq!(
            (UCoord {
                row: 1,
                col: usize::MAX
            })
            .neighbors()
            .len(),
            5
        );
    }
}
