/*
 * main.rs - Console program to simulate Conway's Game of Life.
 *
 * (C) 2021 Tim Gravert <tim.gravert@web.de>
 *
 * License: MIT OR Apache-2.0
 *
 */

use exitcode;
use num_traits::Zero;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::{Path, PathBuf};
<<<<<<< HEAD
use std::process::exit;
=======
use std::process::ExitCode;
>>>>>>> b3c22cf435182528949ab188a7e8eca022a5a0fe
use structopt::StructOpt;

mod conway;

/// Struct for the parameters of the app.
#[derive(Debug, StructOpt)]
#[structopt(name = "rusty-conway")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    file: PathBuf,

    /// Number of generations
    #[structopt(short = "n", long = "number", default_value = "100")]
    number: usize,

    /// Print option
    #[structopt(short = "p", long = "print")]
    print: bool,
}

/// Auxiliary function to transform the input file into a Vector of single words.
/// Input file has to be formatted in such a way that every word is on a single line.
fn lines_from_file(filename: impl AsRef<Path>) -> Result<Vec<String>, (String, u8)> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => return Err((String::from("no such file"), exitcode::NOINPUT)),
    };
    let buf = BufReader::new(file);
    match buf.lines().collect() {
        Ok(res) => Ok(res),
        Err(_) => Err((String::from("file contained invalid UTF-8"), exitcode::DATAERR)),
    }
}

/// Main program logic
fn main() -> ExitCode {
    let opt = Opt::from_args();
    let file = match lines_from_file(opt.file) {
        Ok(x) => x,
        Err((err, c)) => {
            eprintln!("Error: {}", err);
            return ExitCode::from(c);
        }
    };
    let grid: Vec<Vec<bool>> = file
        .iter()
        .skip(1)
        .map(|x| x
            .chars()
            .map(|y|
            y == file[0].chars().next().unwrap()
        ).collect()
        )
        .collect();
    let mut con: conway::Conway = conway::Conway::new(Zero::zero(), grid.clone(), (grid[0].len(), grid.len()));
    println!("{}", con);
    con.run(opt.number, opt.print);
<<<<<<< HEAD
    exit(exitcode::OK)
=======
    ExitCode::SUCCESS
>>>>>>> b3c22cf435182528949ab188a7e8eca022a5a0fe
}
