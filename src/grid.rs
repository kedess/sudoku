use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const BLOCKS: [[(usize, usize); 9]; 9] = [
    [
        (0, 0),
        (0, 1),
        (0, 2),
        (1, 0),
        (1, 1),
        (1, 2),
        (2, 0),
        (2, 1),
        (2, 2),
    ],
    [
        (0, 3),
        (0, 4),
        (0, 5),
        (1, 3),
        (1, 4),
        (1, 5),
        (2, 3),
        (2, 4),
        (2, 5),
    ],
    [
        (0, 6),
        (0, 7),
        (0, 8),
        (1, 6),
        (1, 7),
        (1, 8),
        (2, 6),
        (2, 7),
        (2, 8),
    ],
    [
        (3, 0),
        (3, 1),
        (3, 2),
        (4, 0),
        (4, 1),
        (4, 2),
        (5, 0),
        (5, 1),
        (5, 2),
    ],
    [
        (3, 3),
        (3, 4),
        (3, 5),
        (4, 3),
        (4, 4),
        (4, 5),
        (5, 3),
        (5, 4),
        (5, 5),
    ],
    [
        (3, 6),
        (3, 7),
        (3, 8),
        (4, 6),
        (4, 7),
        (4, 8),
        (5, 6),
        (5, 7),
        (5, 8),
    ],
    [
        (6, 0),
        (6, 1),
        (6, 2),
        (7, 0),
        (7, 1),
        (7, 2),
        (8, 0),
        (8, 1),
        (8, 2),
    ],
    [
        (6, 3),
        (6, 4),
        (6, 5),
        (7, 3),
        (7, 4),
        (7, 5),
        (8, 3),
        (8, 4),
        (8, 5),
    ],
    [
        (6, 6),
        (6, 7),
        (6, 8),
        (7, 6),
        (7, 7),
        (7, 8),
        (8, 6),
        (8, 7),
        (8, 8),
    ],
];

#[derive(Debug)]
pub struct Grid {
    pub data: [[Option<u8>; 9]; 9],
    cells: Vec<(usize, usize)>,
    is_solve: bool,
}

impl Grid {
    pub fn from_file(path: &str) -> anyhow::Result<Grid> {
        let mut grid = [[None; 9]; 9];
        let file = File::open(path)?;
        let buf_reader = BufReader::new(file);
        for (i, line) in buf_reader.lines().enumerate() {
            let line = line?;
            let lines: Vec<&str> = line.split('|').collect();
            for (j, item) in lines.iter().enumerate() {
                if item.len() != 3 {
                    anyhow::bail!("File invalid format: '{}'", path);
                }
                for (k, ch) in item.as_bytes().iter().enumerate() {
                    match *ch {
                        b'*' => {}
                        value @ b'1'..=b'9' => grid[i][j * 3 + k] = Some(value),
                        value => anyhow::bail!("Unexpected symbol from file : '{}'", value),
                    }
                }
            }
        }
        Ok(Grid {
            data: grid,
            cells: vec![],
            is_solve: false,
        })
    }
    pub fn calculate(&mut self) -> anyhow::Result<[[char; 9]; 9]> {
        for i in 0..9 {
            for j in 0..9 {
                if self.data[i][j].is_none() {
                    self.cells.push((i, j));
                }
            }
        }
        self.solve(0);
        if self.is_solve {
            let mut ans: [[char; 9]; 9] = [['0' as char; 9]; 9];
            for i in 0..9 {
                for j in 0..9 {
                    ans[i][j] = self.data[i][j].unwrap() as char;
                }
            }
            return Ok(ans);
        }
        anyhow::bail!("Solve not found")
    }
    pub fn solve(&mut self, idx: usize) {
        if idx == self.cells.len() && !self.is_solve {
            self.is_solve = true;
        }
        if idx < self.cells.len() && !self.is_solve {
            let (i, j) = self.cells[idx];
            for ch in b'1'..=b'9' {
                if !self.is_solve {
                    self.data[i][j] = Some(ch);
                    if self.check() {
                        self.solve(idx + 1);
                    }
                }
            }
            if !self.is_solve {
                self.data[i][j] = None;
            }
        }
    }
    fn check(&self) -> bool {
        for i in 0..9 {
            let mut arr: [u8; 9] = [0; 9];
            for j in 0..9 {
                if let Some(value) = self.data[i][j] {
                    arr[(value - b'1') as usize] += 1;
                }
            }
            for j in 0..9 {
                if arr[j] > 1 {
                    return false;
                }
            }
        }
        for j in 0..9 {
            let mut arr: [u8; 9] = [0; 9];
            for i in 0..9 {
                if let Some(value) = self.data[i][j] {
                    arr[(value - b'1') as usize] += 1;
                }
            }
            for i in 0..9 {
                if arr[i] > 1 {
                    return false;
                }
            }
        }
        for block in BLOCKS {
            let mut arr: [u8; 9] = [0; 9];
            for (i, j) in block {
                if let Some(value) = self.data[i][j] {
                    arr[(value - b'1') as usize] += 1;
                }
            }
            for i in 0..9 {
                if arr[i] > 1 {
                    return false;
                }
            }
        }
        true
    }
}
