use std::io;
use std::time::Instant;

mod solver;

use solver::{Solver, MAX_NODES};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

pub fn read_input() -> (usize, [[i32; 2]; MAX_NODES]) {
    let mut nodes = [[0; 2]; MAX_NODES];
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, usize); // This variables stores how many nodes are given
    for i in 0..n {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(' ').collect::<Vec<_>>();
        nodes[i][0] = parse_input!(inputs[0], i32); // The x coordinate of the given node
        nodes[i][1] = parse_input!(inputs[1], i32); // The y coordinate of the given node
    }

    (n, nodes)
}

fn main() {
    let (n, nodes) = read_input();

    let start_time = Instant::now();

    let mut solver = Solver::new(nodes, n);
    solver.solve(&start_time, 4950_u128);

    solver.show_solution();
}
