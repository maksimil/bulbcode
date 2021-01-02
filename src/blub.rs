use crate::table::Table;

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

pub fn detect(graytable: &Table<bool>) -> Vec<Table<bool>> {
    let mut blubs = Vec::new();

    let mut free = graytable.clone();

    while let Some(p) = free.find(|v| *v) {
        let blub = {
            let idx = blubs.len();
            blubs.push(free.same_size(&false));
            &mut blubs[idx]
        };

        let mut stack = Vec::new();

        stack.push(p);

        while let Some(p) = stack.pop() {
            for n in free_neighbours(&free, p) {
                free[n] = false;
                blub[n] = true;

                stack.push(n);
            }
        }
    }

    for (n, blub) in blubs.iter().enumerate() {
        blub.save(format!(r"target\blubs\{}.png", n));
    }

    blubs
}
