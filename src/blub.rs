use crate::table::Table;

pub struct BlubData {
    pub data: Table<bool>,
    pub refpos: (usize, usize),

    pub pos: (usize, usize),
    pub dpos: (usize, usize),
}

fn free_neighbours(free: &Table<bool>, (i, j): (usize, usize)) -> Vec<(usize, usize)> {
    [
        (i + 1, j + 1),
        (i + 1, j),
        (i + 1, j - 1),
        (i, j + 1),
        (i, j - 1),
        (i - 1, j + 1),
        (i - 1, j),
        (i - 1, j - 1),
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

pub fn detect(graytable: &Table<bool>) -> Vec<BlubData> {
    let mut blubs = Vec::new();

    let mut free = graytable.clone();

    while let Some(p) = free.find(|v| *v) {
        let mut blub = BlubData {
            data: free.same_size(&false),
            refpos: (0, 0),
            pos: p,
            dpos: p,
        };

        let mut stack = Vec::new();

        stack.push(p);

        while let Some(p) = stack.pop() {
            for n in free_neighbours(&free, p) {
                free[n] = false;
                blub.data[n] = true;

                if n.0 < blub.pos.0 {
                    blub.pos.0 = n.0;
                } else if n.0 > blub.dpos.0 {
                    blub.dpos.0 = n.0;
                }

                if n.1 < blub.pos.1 {
                    blub.pos.1 = n.1;
                } else if n.1 > blub.dpos.1 {
                    blub.dpos.1 = n.1;
                }

                stack.push(n);
            }
        }

        blubs.push(blub);
    }

    for (n, blub) in blubs.iter().enumerate() {
        blub.data.save(format!(r"target\blubs\{}.png", n));
    }

    blubs
}
