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

    pub fn from_lines(lines: Vec<String>) -> Self {
        let rows: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
        Self::new(rows)
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

    // NOTE: started to allow setting outside bounds,
    // but vecs don't appear to have negative indices,
    // so we'll have to shift everything up in that case
    pub fn set(&mut self, c: char, x: i32, y: i32) -> () {
        //self.rows.resize((y + 1) as usize, Vec::<char>::new());
        //let row: &mut Vec<char> = self.rows.get_mut(y as usize).unwrap();
        //row.resize((x + 1) as usize, '.');
        //row[x as usize] = c;
        self.rows[y as usize][x as usize] = c;
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

    pub fn draw(&self) {
        std::process::Command::new("clear").status().unwrap();
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

