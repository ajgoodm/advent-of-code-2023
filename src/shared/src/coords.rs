use std::collections::HashSet;

use itertools::Itertools;

use super::direction::Direction;

/// A 2D coordinate with unsigned integers
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct UCoord {
    pub row: usize,
    pub col: usize,
}

impl UCoord {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row: row, col: col }
    }

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

    pub fn cardinal_neighbors(&self) -> HashSet<UCoord> {
        let mut result: HashSet<UCoord> = HashSet::new();
        if self.row > 0 {
            result.insert(UCoord {
                row: self.row - 1,
                col: self.col,
            });
        }
        if self.row < usize::MAX {
            result.insert(UCoord {
                row: self.row + 1,
                col: self.col,
            });
        }
        if self.col > 0 {
            result.insert(UCoord {
                row: self.row,
                col: self.col - 1,
            });
        }
        if self.col < usize::MAX {
            result.insert(UCoord {
                row: self.row,
                col: self.col + 1,
            });
        }
        result
    }

    pub fn north(&self) -> Option<UCoord> {
        if self.row > 0 {
            Some(UCoord {
                row: self.row - 1,
                col: self.col,
            })
        } else {
            None
        }
    }

    pub fn north_east(&self) -> Option<UCoord> {
        if self.row > 0 && self.col < usize::MAX {
            Some(UCoord {
                row: self.row - 1,
                col: self.col + 1,
            })
        } else {
            None
        }
    }

    pub fn east(&self) -> Option<UCoord> {
        if self.col < usize::MAX {
            Some(UCoord {
                row: self.row,
                col: self.col + 1,
            })
        } else {
            None
        }
    }

    pub fn south_east(&self) -> Option<UCoord> {
        if self.row < usize::MAX && self.col < usize::MAX {
            Some(UCoord {
                row: self.row + 1,
                col: self.col + 1,
            })
        } else {
            None
        }
    }

    pub fn south(&self) -> Option<UCoord> {
        if self.row < usize::MAX {
            Some(UCoord {
                row: self.row + 1,
                col: self.col,
            })
        } else {
            None
        }
    }

    pub fn south_west(&self) -> Option<UCoord> {
        if self.row < usize::MAX && self.col > 0 {
            Some(UCoord {
                row: self.row + 1,
                col: self.col - 1,
            })
        } else {
            None
        }
    }

    pub fn west(&self) -> Option<UCoord> {
        if self.col > 0 {
            Some(UCoord {
                row: self.row,
                col: self.col - 1,
            })
        } else {
            None
        }
    }

    pub fn north_west(&self) -> Option<UCoord> {
        if self.row > 0 && self.col > 0 {
            Some(UCoord {
                row: self.row - 1,
                col: self.col - 1,
            })
        } else {
            None
        }
    }

    pub fn neighbor_by_dir(&self, direction: &Direction) -> Option<UCoord> {
        match direction {
            Direction::North => self.north(),
            Direction::NorthEast => self.north_east(),
            Direction::East => self.east(),
            Direction::SouthEast => self.south_east(),
            Direction::South => self.south(),
            Direction::SouthWest => self.south_west(),
            Direction::West => self.west(),
            Direction::NorthWest => self.north_west(),
        }
    }

    pub fn manhattan_distance(&self, other: &UCoord) -> usize {
        let max_y = self.row.max(other.row);
        let min_y = self.row.min(other.row);
        let max_x = self.col.max(other.col);
        let min_x = self.col.min(other.col);

        (max_y - min_y) + (max_x - min_x)
    }
}

impl SCoord {
    pub fn new(row: isize, col: isize) -> Self {
        Self { row: row, col: col }
    }

    pub fn neighbors(&self) -> HashSet<Self> {
        ((self.row - 1)..=(self.row + 1))
            .cartesian_product((self.col - 1)..=(self.col + 1))
            .filter(|(row, col)| !(self.row == *row && self.col == *col))
            .map(|(row, col)| SCoord { row: row, col: col })
            .collect::<HashSet<SCoord>>()
    }

    pub fn cardinal_neighbors(&self) -> HashSet<Self> {
        HashSet::from([
            SCoord::new(self.row - 1, self.col),
            SCoord::new(self.row + 1, self.col),
            SCoord::new(self.row, self.col - 1),
            SCoord::new(self.row, self.col + 1),
        ])
    }

    pub fn north(&self) -> Self {
        Self::new(self.row - 1, self.col)
    }

    pub fn north_east(&self) -> Self {
        Self::new(self.row - 1, self.col + 1)
    }

    pub fn east(&self) -> Self {
        Self::new(self.row, self.col + 1)
    }

    pub fn south_east(&self) -> Self {
        Self::new(self.row + 1, self.col + 1)
    }

    pub fn south(&self) -> Self {
        Self::new(self.row + 1, self.col)
    }

    pub fn south_west(&self) -> Self {
        Self::new(self.row + 1, self.col - 1)
    }

    pub fn west(&self) -> Self {
        Self::new(self.row, self.col - 1)
    }

    pub fn north_west(&self) -> Self {
        Self::new(self.row - 1, self.col - 1)
    }

    pub fn neighbor_by_dir(&self, direction: &Direction) -> Self {
        match direction {
            Direction::North => self.north(),
            Direction::NorthEast => self.north_east(),
            Direction::East => self.east(),
            Direction::SouthEast => self.south_east(),
            Direction::South => self.south(),
            Direction::SouthWest => self.south_west(),
            Direction::West => self.west(),
            Direction::NorthWest => self.north_west(),
        }
    }

    pub fn manhattan_distance(&self, other: &SCoord) -> isize {
        let max_y = self.row.max(other.row);
        let min_y = self.row.min(other.row);
        let max_x = self.col.max(other.col);
        let min_x = self.col.min(other.col);

        (max_y - min_y) + (max_x - min_x)
    }
}

/// A 2D coordinate with signed integers
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct SCoord {
    pub row: isize,
    pub col: isize,
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
