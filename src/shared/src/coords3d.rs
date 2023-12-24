#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct U3Coord {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl U3Coord {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }

    pub fn z_plus(&self) -> Option<Self> {
        if self.z == usize::MAX {
            None
        } else {
            Some(Self::new(self.x, self.y, self.z + 1))
        }
    }

    pub fn z_minus(&self) -> Option<Self> {
        if self.z == 0 {
            None
        } else {
            Some(Self::new(self.x, self.y, self.z - 1))
        }
    }

    pub fn x_plus(&self) -> Option<Self> {
        if self.x == usize::MAX {
            None
        } else {
            Some(Self::new(self.x + 1, self.y, self.z))
        }
    }

    pub fn x_minus(&self) -> Option<Self> {
        if self.x == 0 {
            None
        } else {
            Some(Self::new(self.x - 1, self.y, self.z))
        }
    }

    pub fn y_plus(&self) -> Option<Self> {
        if self.y == usize::MAX {
            None
        } else {
            Some(Self::new(self.x, self.y + 1, self.z))
        }
    }

    pub fn y_minus(&self) -> Option<Self> {
        if self.y == 0 {
            None
        } else {
            Some(Self::new(self.x, self.y - 1, self.z))
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct S3Coord {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl S3Coord {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    pub fn z_plus(&self) -> Self {
        Self::new(self.x, self.y, self.z + 1)
    }

    pub fn z_minus(&self) -> Self {
        Self::new(self.x, self.y, self.z - 1)
    }

    pub fn y_plus(&self) -> Self {
        Self::new(self.x, self.y + 1, self.z)
    }

    pub fn y_minus(&self) -> Self {
        Self::new(self.x, self.y - 1, self.z)
    }

    pub fn x_plus(&self) -> Self {
        Self::new(self.x + 1, self.y, self.z)
    }

    pub fn x_minus(&self) -> Self {
        Self::new(self.x + 1, self.y, self.z)
    }
}
