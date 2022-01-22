/*
 * turingmachine.rs - Functions to simulate a turing machine.
 *
 * (C) 2021 Tim Gravert <tim.gravert@web.de>
 *
 * License: MIT OR Apache-2.0
 *
 */

use num_bigint::BigUint;
//use num_traits::Zero;
//use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub struct Conway {
    generation: BigUint,
    grid: Vec<Vec<bool>>,
}

impl Conway {
    pub fn new(gen: BigUint, gri: Vec<Vec<bool>>) -> Conway {
        Conway {
            generation: gen,
            grid: gri,
        }
    }

    pub fn next_generation(&mut self) {
        self.generation += 1 as usize;
        for x in self.grid.iter() {
            for y in x {
                println!("{}", y);
            }
        }
    }
}
