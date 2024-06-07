/*
 * turingmachine.rs - Functions to simulate a turing machine.
 *
 * (C) 2021 Tim Gravert <tim.gravert@web.de>
 *
 * License: MIT OR Apache-2.0
 *
 */

use num_bigint::BigUint;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Conway {
    generation: BigUint,
    grid: Vec<Vec<bool>>,
    size: (usize, usize),
}

impl Conway {
    pub fn new(gen: BigUint, gri: Vec<Vec<bool>>, siz: (usize, usize)) -> Conway {
        Conway {
            generation: gen,
            grid: gri,
            size: siz,
        }
    }

    pub fn next_generation(&mut self) {
        self.generation += 1_usize;

        let mut vec: Vec<(usize, usize)> = Vec::new();
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let mut count = 0;

                let yvec = if y > 0 && y < self.size.1 - 1 {
                    vec![y - 1, y, y + 1]
                } else if y > 0 {
                    vec![y - 1, y]
                } else if y < self.size.1 - 1 {
                    vec![y, y + 1]
                } else {
                    vec![y]
                };
                let xvec = if x > 0 && x < self.size.0 - 1 {
                    vec![x - 1, x, x + 1]
                } else if x > 0 {
                    vec![x - 1, x]
                } else if x < self.size.0 - 1 {
                    vec![x, x + 1]
                } else {
                    vec![x]
                };

                let yxvec = yvec
                    .iter()
                    .flat_map(|&vy| xvec.iter().map(move |&vx| (vy, vx)))
                    .filter(|(vy, vx)| !(*vy == y && *vx == x))
                    .collect::<Vec<(usize, usize)>>();

                for (iy, ix) in yxvec {
                    count += if self.grid[iy][ix] { 1 } else { 0 };
                }
                if !(2..=3).contains(&count) && self.grid[y][x] {
                    vec.push((y, x));
                }
                if count == 3 && !self.grid[y][x] {
                    vec.push((y, x));
                }
            }
        }
        for t in vec {
            self.grid[t.0][t.1] = !self.grid[t.0][t.1];
        }
    }

    pub fn run(&mut self, steps: usize, pr: bool) {
        let mut s = steps;
        while s > 0 {
            s -= 1;
            self.next_generation();
            if pr {
                println!("{}", self);
            }
        }
    }
}

impl fmt::Display for Conway {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.grid;
        write!(f, "Generation: {}\n\n", &self.generation)?;
        for vy in vec.iter() {
            for vx in vy.iter() {
                write!(f, "{}", if *vx { "◼" } else { "◻" })?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}
