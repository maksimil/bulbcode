use std::num::Wrapping;

use image::Rgb;

use crate::table::{Table, ToRgb8};

#[derive(Clone)]
pub enum BlubPx {
    Unmarked,
    Outer,
    Border(usize),
    Inner(usize),
}

const COLORS: [Rgb<u8>; 7] = [
    Rgb([255, 0, 0]),
    Rgb([0, 255, 0]),
    Rgb([0, 0, 255]),
    Rgb([255, 255, 0]),
    Rgb([255, 0, 255]),
    Rgb([0, 255, 255]),
    Rgb([0, 0, 0]),
];

impl ToRgb8 for BlubPx {
    fn to_rgb8(&self) -> Rgb<u8> {
        match self {
            BlubPx::Unmarked => false.to_rgb8(),
            BlubPx::Outer => Rgb([100, 100, 100]),
            BlubPx::Border(a) => COLORS[a % COLORS.len()],
            BlubPx::Inner(_) => false.to_rgb8(),
        }
    }
}

const W1: Wrapping<usize> = Wrapping(1);

fn free_neighbours(free: &Table<bool>, (ii, ji): (usize, usize)) -> Vec<(usize, usize)> {
    let i = Wrapping(ii);
    let j = Wrapping(ji);
    [
        ((i + W1).0, ji),
        ((i - W1).0, ji),
        (ii, (j + W1).0),
        (ii, (j - W1).0),
    ]
    .iter()
    .filter_map(|p| {
        if p.0 < free.width() && p.1 < free.height() && free[*p] {
            Some(*p)
        } else {
            None
        }
    })
    .collect::<Vec<_>>()
}

pub fn relimit<T: Ord>(lims: (T, T), v: T) -> (T, T) {
    if lims.0 <= v && v <= lims.1 {
        lims
    } else if lims.1 < v {
        (lims.0, v)
    } else {
        (v, lims.1)
    }
}

pub fn detect(graytable: &Table<bool>) -> Table<BlubPx> {
    let mut data = graytable.same_size(&BlubPx::Unmarked);

    // detecting borders
    let count = {
        let mut count = 0;
        let mut free = graytable.clone();

        while let Some(p) = free.find(|v| *v) {
            let mut stack = Vec::new();

            free[p] = false;

            data[p] = BlubPx::Border(count);
            stack.push(p);

            while let Some(p) = stack.pop() {
                for n in free_neighbours(&free, p) {
                    free[n] = false;

                    data[n] = BlubPx::Border(count);
                    stack.push(n);
                }
            }

            count += 1;
        }

        count
    };

    // marking outer region
    {
        let mut free = graytable.clone();

        for i in 0..free.width() {
            for j in 0..free.height() {
                free[(i, j)] = !free[(i, j)];
            }
        }

        let mut stack = Vec::new();

        free[(0, 0)] = false;

        data[(0, 0)] = BlubPx::Outer;
        stack.push((0, 0));

        while let Some(p) = stack.pop() {
            for n in free_neighbours(&free, p) {
                free[n] = false;

                data[n] = BlubPx::Outer;
                stack.push(n);
            }
        }
    }

    data.save(r"target\data.png");

    data
}
