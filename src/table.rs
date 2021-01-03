use std::fmt::{self, Debug, Formatter};
use std::fs::File;
use std::ops::{Index, IndexMut};
use std::path::Path;

use image::Rgb;
use png::{BitDepth, ColorType, Compression};

#[derive(Clone)]
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

impl<T> Table<T> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn find<F>(&self, pred: F) -> Option<(usize, usize)>
    where
        F: Fn(&T) -> bool,
    {
        for i in 0..self.width {
            for j in 0..self.height {
                if pred(&self[(i, j)]) {
                    return Some((i, j));
                }
            }
        }
        None
    }

    pub fn same_size<E: Clone>(&self, fill: &E) -> Table<E> {
        Table::new(self.width(), self.height(), fill)
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

impl<T: ToRgb8> Table<T> {
    pub fn save<P: AsRef<Path>>(&self, path: P) {
        let mut data = vec![0; 3 * self.width() * self.height()];
        for i in 0..self.width() {
            for j in 0..self.height() {
                let pdata = self[(i, j)].to_rgb8();
                for k in 0..3 {
                    data[3 * (i + self.width() * j) + k] = pdata.0[k];
                }
            }
        }

        let mut encoder = png::Encoder::new(
            File::create(path).expect("Failed to open file"),
            self.width() as u32,
            self.height() as u32,
        );

        encoder.set_color(ColorType::RGB);
        encoder.set_depth(BitDepth::Eight);
        encoder.set_compression(Compression::Default);

        let mut write = encoder.write_header().expect("Failed to write header");

        write
            .write_image_data(&data)
            .expect("Failed to write image data");
    }
}

pub trait ToRgb8 {
    fn to_rgb8(&self) -> Rgb<u8>;
}

impl ToRgb8 for bool {
    fn to_rgb8(&self) -> Rgb<u8> {
        if *self {
            Rgb([0, 0, 0])
        } else {
            Rgb([255, 255, 255])
        }
    }
}
