#[derive(Clone, Debug)]
pub struct Range<T: Ord + PartialOrd + Copy> {
    pub start: T,
    pub end: T,
}

impl<T> Range<T>
where
    T: Ord + PartialOrd + Copy,
{
    pub fn contains(&self, val: T) -> bool {
        val >= self.start && val < self.end
    }

    pub fn intersection(&self, other: &Range<T>) -> Option<Range<T>> {
        if other.end <= self.start || other.start >= self.end {
            return None;
        }
        Some(Range {
            start: T::max(self.start, other.start),
            end: T::min(self.end, other.end),
        })
    }

    pub fn difference(&self, other: &Range<T>) -> Vec<Range<T>> {
        if other.start <= self.start && other.end >= self.end {
            return Vec::new();
        }

        match self.intersection(other) {
            None => return vec![self.clone()],
            _ => (),
        }

        if other.contains(self.start) && !other.contains(self.end) {
            vec![Range {
                start: other.end,
                end: self.end,
            }]
        } else if other.contains(self.end) && !other.contains(self.start) {
            vec![Range {
                start: self.start,
                end: other.start,
            }]
        } else {
            vec![
                Range {
                    start: self.start,
                    end: other.start,
                },
                Range {
                    start: other.end,
                    end: self.end,
                },
            ]
        }
    }
}
