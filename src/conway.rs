/*
 * conway.rs - Functions to simulate Conway's Game of Life.
 *
 * (C) 2021 Tim Gravert <tim.gravert@web.de>
 *
 * License: MIT OR Apache-2.0
 *
 */

#[allow(unused_imports)]
use mortal::{Color, Screen, Style, Terminal, Theme};
use num_bigint::BigUint;
use std::{fmt, thread, time};
use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

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
        let mut stdout = stdout();
        let term = Terminal::new();

        while s > 0 {
            s -= 1;
            self.next_generation();
            #[allow(unused_variables)]
            let st = self.stringify();

            if pr {
                stdout.flush().unwrap();
                sleep(Duration::from_millis(100));
                //term.as_ref().expect("REASON").write_str(&st);
                let _ = write!(term.as_ref().expect("REASON"), "\r{}", self);
            }
            thread::sleep(time::Duration::from_millis(150));
        }
    }

    pub fn stringify(&mut self) -> String {
        let vec = &self.grid;
        let mut s: String = String::from("");

        for vy in vec.iter() {
            for vx in vy.iter() {
                if *vx {
                    s += "◼";
                } else {
                    s += "◻";
                }
            }
            s += "\n";
        }

        s += "\n";

        s
    }

    //pub fn output(&mut self) ->
}

impl fmt::Display for Conway {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //write!(f, "{esc}[2J{esc}[1;1H", esc = 27 as char)?;
        let vec = &self.grid;
        write!(f, "\rGeneration: {}\n\n", &self.generation)?;
        for vy in vec.iter() {
            for vx in vy.iter() {
                write!(f, "{}", if *vx { "◼" } else { "◻" })?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}
