use std::num::Wrapping;

use crate::table::Table;

#[derive(Clone, PartialEq)]
pub enum BlubPx {
    Unmarked,
    Border,
    Region,
}

const W1: Wrapping<usize> = Wrapping(1);

fn neighbours((width, height): (usize, usize), (ii, ji): (usize, usize)) -> Vec<(usize, usize)> {
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
        if p.0 < width && p.1 < height {
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

pub fn propagate_border(
    data: &mut Table<BlubPx>,
    borders: &mut Table<bool>,
    root: &mut TNode,
    p: (usize, usize),
) {
    let size = (data.width(), data.height());

    let mut pr = Vec::new();

    let mut stack = Vec::new();
    stack.push(p);

    while let Some(p) = stack.pop() {
        for n in neighbours(size, p) {
            if data[n] == BlubPx::Unmarked {
                if !borders[n] {
                    pr.push(n);
                } else {
                    data[n] = BlubPx::Border;
                    stack.push(n);
                }
            }
        }
    }

    for n in pr {
        if data[n] == BlubPx::Unmarked {
            let ri = root.0.len();
            root.0.push(TRegion::new());

            data[n] = BlubPx::Region;

            propagate_region(data, borders, &mut root.0[ri], n);
        }
    }
}

pub fn propagate_region(
    data: &mut Table<BlubPx>,
    borders: &mut Table<bool>,
    root: &mut TRegion,
    p: (usize, usize),
) {
    let size = (data.width(), data.height());

    let mut pb = Vec::new();

    let mut stack = Vec::new();
    stack.push(p);

    while let Some(p) = stack.pop() {
        for n in neighbours(size, p) {
            if data[n] == BlubPx::Unmarked {
                if borders[n] {
                    pb.push(n);
                } else {
                    data[n] = BlubPx::Region;
                    stack.push(n);
                }
            }
        }
    }

    for n in pb {
        if data[n] == BlubPx::Unmarked {
            let ri = root.0.len();
            root.0.push(TNode::new());

            data[n] = BlubPx::Border;

            propagate_border(data, borders, &mut root.0[ri], n);
        }
    }
}

#[derive(Debug)]
pub struct TNode(Vec<TRegion>);
#[derive(Debug)]
pub struct TRegion(Vec<TNode>);

impl TNode {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl TRegion {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

pub fn detect(graytable: &Table<bool>) -> TRegion {
    let mut data = graytable.same_size(&BlubPx::Unmarked);

    data[(0, 0)] = BlubPx::Region;
    let mut borders = graytable.clone();

    let mut root = TRegion::new();

    propagate_region(&mut data, &mut borders, &mut root, (0, 0));

    root
}
