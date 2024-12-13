#[derive(Clone)]
pub struct Grid {
    rows: Vec<Vec<char>>,
    width: i32,
    height: i32,
    default: char
}

impl Grid {
    pub fn get_width(&self) -> i32 { self.width }
    pub fn get_height(&self) -> i32 { self.height }

    pub fn from_lines(lines: Vec<String>) -> Self {
        let rows: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
        Self::from_rows(rows)
    }

    pub fn from_rows(rows: Vec<Vec<char>>) -> Self {
        let mut g = Self {
            width: 0,
            height: 0,
            rows,
            default: '.'
        };
        g.update_dims();
        g
    }

    pub fn fresh(default: char, width: i32, height: i32) -> Self {
        let mut g = Self {
            width,
            height,
            rows: Vec::new(),
            default,
        };
        for (_, x, y) in g.iter() {
            g.set(default, x, y);
        }
        g
    }

    fn update_dims(&mut self) {
        self.width = self.rows.iter().map(|r| r.len()).max().unwrap_or(0) as i32;
        self.height = self.rows.len() as i32;
    }

    pub fn iter(&self) -> GridIter {
        GridIter::new(&self)
    }

    pub fn find(&self, d: char) -> Option<(i32, i32)> {
        for (oc, x, y) in self.iter() {
            if let Some(c) = oc {
                if c == d { return Some((x, y)); }
            }
        }
        return None;
    }

    pub fn count(&self, d: char) -> i32 {
        let mut count = 0;
        for (oc, x, y) in self.iter() {
            if let Some(c) = oc {
                if c == d { count += 1 }
            }
        }
        count
    }

    pub fn get(&self, x: i32, y: i32) -> Option<char> {
        match self.rows.get(y as usize) {
            Some(row) => row.get(x as usize).copied(),
            None => None
        }
    }

    // NOTE: started to allow setting negatives,
    // but vecs don't appear to have negative indices,
    // so we'll have to shift everything up in that case.
    // Could track origin and then offset all returned x/y,
    // e.g. if origin is (-2,-2) then the rows[1][1] is (-1,-1), origin + (x,y)
    pub fn set(&mut self, c: char, x: i32, y: i32) -> () {
        let y_len = (y + 1) as usize;
        let x_len = (x + 1) as usize;
        while self.rows.len() < y_len {
            self.rows.push(Vec::<char>::new());
        }
        // TODO: fix other rows to match?
        while self.rows[y as usize].len() < x_len {
            self.rows[y as usize].push(self.default);
        }
        self.rows[y as usize][x as usize] = c;
    }

    pub fn neighbors(&self, x: i32, y: i32, diagonals: bool) -> Vec<(Option<char>, i32, i32)> {
        let offsets: [i32; 3] = [-1, 0, 1];
        let mut arr: Vec<(Option<char>, i32, i32)> = Vec::new();
        for dx in offsets {
            for dy in offsets {
                if (dx == 0 && dy == 0) { continue; }
                // for diagonals, dx & dy will both be nonzero
                if (!diagonals && dx != 0 && dy != 0) { continue } 
                //if let Some(c) = self.get(x + dx, y + dy) {
                    // push it even if there is no char there. caller can decide how to handle
                    arr.push((self.get(x + dx, y + dy), dx, dy));
                //}
            }
        }
        arr
    }

    pub fn draw(&self, clear: bool) {
        if (clear) {
            std::process::Command::new("clear").status().unwrap();
        }
        for row in &self.rows {
            for c in row {
                print!("{}", c);
            }
            print!("\n");
        }
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
            x: -1, // first next() will inc x to 0
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

