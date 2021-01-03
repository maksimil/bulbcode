use std::num::Wrapping;

use crate::table::Table;

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

pub fn propagate_region<T: PartialEq + Clone>(
    used: &mut Table<bool>,
    graytable: &Table<T>,
    root: &mut TNode<T>,
    p: (usize, usize),
) {
    let size = (used.width(), used.height());

    let mut pb = Vec::new();

    let mut stack = Vec::new();
    stack.push(p);

    while let Some(p) = stack.pop() {
        for n in neighbours(size, p) {
            if !used[n] {
                if graytable[n] == root.value {
                    used[n] = true;
                    stack.push(n);
                } else {
                    pb.push(n);
                }
            }
        }
    }

    for n in pb {
        if !used[n] {
            let ri = root.children.len();
            root.children.push(TNode::new(graytable[n].clone()));

            propagate_region(used, graytable, &mut root.children[ri], n);
        }
    }
}

#[derive(Debug)]
pub struct TNode<T> {
    pub value: T,
    pub children: Vec<TNode<T>>,
}

impl<T> TNode<T> {
    pub fn new(value: T) -> TNode<T> {
        TNode {
            value,
            children: Vec::new(),
        }
    }
}

pub fn topology<T: PartialEq + Clone>(graytable: &Table<T>, p: (usize, usize)) -> TNode<T> {
    let mut used = graytable.same_size(&false);

    used[(0, 0)] = true;

    let mut root = TNode::new(graytable[(0, 0)].clone());

    propagate_region(&mut used, graytable, &mut root, p);

    root
}
