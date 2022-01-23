/*
 * main.rs - Console program to simulate a turing machine.
 *
 * (C) 2021 Tim Gravert <tim.gravert@web.de>
 *
 * License: MIT OR Apache-2.0
 *
 */

use num_traits::Zero;

mod conway;

fn main() {
    let mut con: conway::Conway = conway::Conway::new(
        Zero::zero(),
        vec![
            vec![false, false, false, false, false, false],
            vec![false, true, true, false, false, false],
            vec![false, true, false, false, false, false],
            vec![false, false, false, false, true, false],
            vec![false, false, false, true, true, false],
            vec![false, false, false, false, false, false],
        ],
        6,
    );
    println!("{}", con);
    con.run(10);
    println!("{}", con);
}
