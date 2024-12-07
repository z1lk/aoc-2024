#[derive(Clone)]
pub struct Grid {
    rows: Vec<Vec<char>>,
    width: i32,
    height: i32
}

impl Grid {
    pub fn new(rows: Vec<Vec<char>>) -> Self {
        Self {
            width: rows.iter().map(|r| r.len()).max().unwrap_or(0) as i32,
            height: rows.len() as i32,
            rows,
        }
    }

    pub fn iter(&self) -> GridIter {
        GridIter::new(&self)
    }

    pub fn get(&self, x: i32, y: i32) -> Option<char> {
        // if out of bounds, return '!' which won't match
        //if x < 0 { return '!'; }
        //if y < 0 { return '!'; }
        //if x >= self.width { return '!'; }
        //if y >= self.height { return '!'; }

        match self.rows.get(y as usize) {
            Some(row) => row.get(x as usize).copied(),
            None => None
        }
        //self.rows[y as usize][x as usize]
    }

    pub fn neighbors(&self, x: i32, y: i32) -> Vec<(Option<char>, i32, i32)> {
        let offsets: [i32; 3] = [-1, 0, 1];
        let mut arr: Vec<(Option<char>, i32, i32)> = Vec::new();
        for xo in offsets {
            for yo in offsets {
                if (xo == 0 && yo == 0) { continue; }
                //if let Some(c) = self.get(x + xo, y + yo) {
                    arr.push((self.get(x + xo, y + yo), xo, yo));
                //}
            }
        }
        arr
    }
}

pub struct GridIter {
    grid: Grid,
    x: i32,
    y: i32,
}

impl GridIter {
    fn new(grid: &Grid) -> Self {
        Self {
            // not cloning here introduces a compiler error, something to do with lifetimes
            grid: grid.clone(),
            x: 0,
            y: 0
        }
    }
}

impl Iterator for GridIter {
    type Item = (Option<char>, i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let mut x = self.x + 1;
        let mut y = self.y;
        if x >= self.grid.width {
            x = 0;
            y = y + 1;
        }
        if y >= self.grid.height {
            return None;
        }
        self.x = x;
        self.y = y;
        Some( (self.grid.get(x, y), x, y) )
    }
}

