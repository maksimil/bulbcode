use std::fmt::{self, Debug, Formatter};
use std::ops::{Index, IndexMut};

pub struct Table<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Clone> Table<T> {
    pub fn new(width: usize, height: usize, fill: &T) -> Table<T> {
        Table {
            data: vec![fill.clone(); width * height],
            width,
            height,
        }
    }
}

impl<T> Index<(usize, usize)> for Table<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &T {
        if index.0 < self.width && index.1 < self.height {
            &self.data[index.0 + self.width * index.1]
        } else {
            panic!(
                "index out of bounds: ({}, {}) for size ({}, {})",
                index.0, index.1, self.width, self.height
            );
        }
    }
}

impl<T> IndexMut<(usize, usize)> for Table<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        if index.0 < self.width && index.1 < self.height {
            &mut self.data[index.0 + self.width * index.1]
        } else {
            panic!(
                "index out of bounds: ({}, {}) for size ({}, {})",
                index.0, index.1, self.width, self.height
            );
        }
    }
}

impl<T: Debug> Debug for Table<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut r = Ok(());

        for j in 0..self.height {
            for i in 0..self.width {
                r = r.and(write!(f, "{:?}, ", self[(i, j)]));
            }
            r = r.and(write!(f, "\n"));
        }
        r
    }
}
