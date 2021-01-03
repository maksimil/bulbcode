use std::num::Wrapping;

use image::Rgb;

use crate::table::{Table, ToRgb8};

#[derive(Clone, PartialEq)]
pub enum BlubPx {
    Unmarked,
    Border(usize),
    Region(usize, usize),
}

const COLORS: [Rgb<u8>; 8] = [
    Rgb([255, 255, 255]),
    Rgb([255, 0, 0]),
    Rgb([255, 165, 0]),
    Rgb([255, 255, 0]),
    Rgb([0, 128, 0]),
    Rgb([0, 0, 255]),
    Rgb([75, 0, 130]),
    Rgb([238, 130, 238]),
];

impl ToRgb8 for BlubPx {
    fn to_rgb8(&self) -> Rgb<u8> {
        match self {
            BlubPx::Unmarked => false.to_rgb8(),
            BlubPx::Region(a, i) => {
                let mut c = COLORS[a % COLORS.len()];
                let f = COLORS[i % COLORS.len()];

                for i in 0..3 {
                    c.0[i] = (c.0[i] + (255 - c.0[i]) / 2) / 4 * 3 + f.0[i] / 4;
                }
                c
            }
            BlubPx::Border(a) => COLORS[a % COLORS.len()],
        }
    }
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
    bcount: &mut usize,
    p: (usize, usize),
) {
    let size = (data.width(), data.height());

    let bid = match data[p] {
        BlubPx::Border(bid) => bid,
        _ => panic!("Cannot propagate border without a starting point"),
    };

    let mut pr = Vec::new();

    let mut stack = Vec::new();
    stack.push(p);

    while let Some(p) = stack.pop() {
        for n in neighbours(size, p) {
            match data[n] {
                BlubPx::Unmarked => {
                    if !borders[n] {
                        pr.push(n);
                    } else {
                        data[n] = BlubPx::Border(bid);
                        stack.push(n);
                    }
                }
                BlubPx::Border(obid) => {
                    debug_assert!(bid == obid);
                }
                BlubPx::Region(_, _) => (),
            }
        }
    }

    let mut rn = 0;
    for n in pr {
        if data[n] == BlubPx::Unmarked {
            data[n] = BlubPx::Region(bid, rn);
            rn += 1;
            propagate_region(data, borders, bcount, n);
        }
    }
}

pub fn propagate_region(
    data: &mut Table<BlubPx>,
    borders: &mut Table<bool>,
    bcount: &mut usize,
    p: (usize, usize),
) {
    let size = (data.width(), data.height());

    let (rid, rn) = match data[p] {
        BlubPx::Region(rid, rn) => (rid, rn),
        _ => panic!("Cannot propagate region without a starting point"),
    };

    let mut pb = Vec::new();

    let mut stack = Vec::new();
    stack.push(p);

    while let Some(p) = stack.pop() {
        for n in neighbours(size, p) {
            match data[n] {
                BlubPx::Unmarked => {
                    if borders[n] {
                        pb.push(n);
                    } else {
                        data[n] = BlubPx::Region(rid, rn);
                        stack.push(n);
                    }
                }
                BlubPx::Region(orid, orn) => {
                    debug_assert!(rid == orid && rn == orn);
                }
                BlubPx::Border(_) => (),
            }
        }
    }

    for n in pb {
        if data[n] == BlubPx::Unmarked {
            data[n] = BlubPx::Border(*bcount);
            *bcount += 1;
            propagate_border(data, borders, bcount, n);
        }
    }
}

pub fn detect(graytable: &Table<bool>) -> Table<BlubPx> {
    let mut data = graytable.same_size(&BlubPx::Unmarked);

    data[(0, 0)] = BlubPx::Region(0, 0);
    let mut borders = graytable.clone();
    propagate_region(&mut data, &mut borders, &mut 1, (0, 0));

    data.save(r"target\data.png");

    data
}
