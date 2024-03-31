use std::time::Instant;

use rand::Rng;

pub const MAX_NODES: usize = 250;

pub struct Solver {
    n: usize,
    nodes: [[i32; 2]; MAX_NODES],
    path: [usize; MAX_NODES + 1],
    distance: [[f32; MAX_NODES]; MAX_NODES],
    score: f32,
}

impl Solver {
    pub fn new(nodes: [[i32; 2]; MAX_NODES], n: usize) -> Solver {
        let mut solver = Solver {
            n,
            nodes,
            path: [0; MAX_NODES + 1],
            distance: [[0.0; MAX_NODES]; MAX_NODES],
            score: 0.0,
        };
        solver.build_pairwise_matrix();
        solver
    }

    #[allow(dead_code)]
    pub fn describe(&self) {
        eprintln!("n: {}", self.n);
        for i in 0..self.n {
            eprintln!("{} {}", self.nodes[i][0], self.nodes[i][1]);
        }
    }

    pub fn show_solution(&self) {
        // Print the solution
        let space_separated_string = self
            .path
            .iter()
            .take(self.n + 1)
            .map(|&x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", space_separated_string);
        eprintln!("Score: {}", self.score)
    }

    pub fn solve(&mut self, duration: Instant, max_duration: u128) {
        self.set_initial_path();

        let mut rng = rand::thread_rng();
        while duration.elapsed().as_millis() < max_duration {
            let i = rng.gen_range(1..self.n - 1);
            let j = rng.gen_range(i + 1..self.n);

            let mut new_score = self.score;
            new_score -= self.distance[self.path[i - 1]][self.path[i]];
            new_score -= self.distance[self.path[j]][self.path[j + 1]];
            new_score += self.distance[self.path[i - 1]][self.path[j]];
            new_score += self.distance[self.path[i]][self.path[j + 1]];

            if new_score < self.score {
                self.score = new_score;
                self.path[i..=j].reverse();
                eprintln!("New score: {}", self.score);
            }
        }
    }

    fn build_pairwise_matrix(&mut self) {
        // Build a matrix of pairwise distances between nodes
        for i in 0..self.n - 1 {
            for j in (i + 1)..self.n {
                let a = (self.nodes[i][0] - self.nodes[j][0]).pow(2);
                let b = (self.nodes[i][1] - self.nodes[j][1]).pow(2);
                let d: f32 = ((a + b) as f32).sqrt();

                self.distance[i][j] = d;
                self.distance[j][i] = d;
            }
        }
    }

    fn set_initial_path(&mut self) {
        // Set the initial path to be the order of the nodes

        let mut visited: [bool; MAX_NODES] = [false; MAX_NODES];
        let mut current_node: usize = 0;
        visited[0] = true;

        for i in 1..self.n {
            let mut closest_node: usize = 0;
            let mut closest_distance: f32 = 10000000.0;

            for j in 0..self.n {
                if !visited[j] && self.distance[current_node][j] < closest_distance {
                    closest_node = j;
                    closest_distance = self.distance[current_node][j];
                }
            }

            current_node = closest_node;
            visited[current_node] = true;

            self.path[i] = closest_node;
            self.score += closest_distance;
        }

        self.score += self.distance[current_node][0];
    }
}
