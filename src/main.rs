/*
 * main.rs - Console program to simulate a turing machine.
 *
 * (C) 2021 Tim Gravert <tim.gravert@web.de>
 *
 * License: MIT OR Apache-2.0
 *
 */

// use num_traits::zero;

mod conway;

fn main() {
    let mut con: conway::Conway = conway::Conway::new(
        num_traits::zero(),
        vec![
            vec![false, false, false],
            vec![false, false, false],
            vec![false, false, false],
        ],
    );
    println!("{:?}", con);
    con.next_generation();
    println!("{:?}", con);
}
