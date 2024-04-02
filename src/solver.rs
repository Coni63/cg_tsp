use std::time::Instant;

use rand::{seq::SliceRandom, Rng};

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

    pub fn solve(&mut self, duration: &Instant, max_duration: u128) {
        self.set_initial_path();
        eprintln!(
            "Initial score: {} ({})",
            self.score,
            duration.elapsed().as_millis()
        );

        self.run_annealing(duration, 2000);
        eprintln!("Score after first annealing: {}", self.score);

        self.run_two_opt();
        eprintln!(
            "Score after 2-opt: {} ({})",
            self.score,
            duration.elapsed().as_millis()
        );
        self.run_three_opt();
        eprintln!(
            "Score after 3-opt: {} ({})",
            self.score,
            duration.elapsed().as_millis()
        );

        self.run_annealing(duration, max_duration);
        eprintln!("Score after annealing: {}", self.score);
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
        visited[0] = true;
        let mut current_node: usize = 0;

        for i in 1..self.n {
            let mut closest_node: usize = 0;
            let mut closest_distance: f32 = 10000000.0;

            for j in 1..self.n {
                if !visited[j] && self.distance[current_node][j] < closest_distance {
                    closest_node = j;
                    closest_distance = self.distance[current_node][j];
                }
            }

            current_node = closest_node;
            visited[current_node] = true;

            self.path[i] = closest_node;
        }

        self.score = self.get_distance(&self.path[..self.n + 1]);
    }

    fn get_distance(&self, path: &[usize]) -> f32 {
        // Calculate the total distance of a path
        path.iter()
            .zip(path.iter().skip(1))
            .map(|(&a, &b)| self.distance[a][b])
            .sum()
    }

    fn run_annealing(&mut self, duration: &Instant, max_duration: u128) {
        if (self.n <= 10) {
            return;
        }

        let mut best_candidate = vec![0; self.n + 1];
        best_candidate.copy_from_slice(&self.path[..self.n + 1]);

        let mut annealing_candidate = self.tweak_candidate(&best_candidate);
        let mut annealing_score = self.get_distance(&annealing_candidate);

        let mut rng = rand::thread_rng();
        loop {
            let portion_elapsed = (duration.elapsed().as_millis() as f64 / max_duration as f64);

            if portion_elapsed >= 1.0 {
                break;
            }

            let next_candidate = self.tweak_candidate(&annealing_candidate);
            let next_score = self.get_distance(&next_candidate);

            let next_is_better = next_score < annealing_score;
            let replacement_threshold = 0.5 * 1.0f64.exp().powf(-10.0 * portion_elapsed.powf(3.0));

            if next_is_better || (rng.gen_range(0.0..1.0) < replacement_threshold) {
                annealing_candidate = next_candidate;
                annealing_score = next_score;
            }

            if annealing_score < self.score {
                eprintln!("New best score: {}", annealing_score);
                self.path[..self.n + 1].copy_from_slice(&annealing_candidate);
                self.score = annealing_score;
            }
        }
    }

    fn tweak_candidate(&self, candidate: &[usize]) -> Vec<usize> {
        let mut rng = rand::thread_rng();

        loop {
            let width = rng.gen_range(3..10);
            let start = rng.gen_range(1..self.n - width);
            let end = start + width;

            if start == end {
                continue;
            }

            let (start, end) = if start < end {
                (start, end)
            } else {
                (end, start)
            };
            let mut ans = vec![0; self.n + 1];
            ans.copy_from_slice(candidate);
            ans[start..end].shuffle(&mut rng);

            return ans;
        }
    }

    fn run_two_opt(&mut self) {
        let mut improved = true;
        while improved {
            improved = false;
            for i in 1..self.n - 1 {
                for j in (i + 2)..self.n + 1 {
                    let cost_change = self.distance[self.path[i - 1]][self.path[j - 1]]
                        + self.distance[self.path[i]][self.path[j]]
                        - self.distance[self.path[i - 1]][self.path[i]]
                        - self.distance[self.path[j - 1]][self.path[j]];

                    if cost_change < -0.01 {
                        self.score += cost_change;
                        self.path[i..j].reverse();
                        improved = true;
                    }
                }
            }
        }
    }

    fn run_three_opt(&mut self) {
        let mut temp = vec![0; MAX_NODES + 1];
        for i in 1..self.n - 3 {
            for j in i + 2..self.n - 1 {
                for k in j + 2..self.n {
                    let (a, b, c, d, e, f) = (
                        self.path[i - 1],
                        self.path[i],
                        self.path[j - 1],
                        self.path[j],
                        self.path[k - 1],
                        self.path[k],
                    );

                    let (d0, d1, d2, d3, d4) = (
                        self.distance[a][b] + self.distance[c][d] + self.distance[e][f],
                        self.distance[a][c] + self.distance[b][d] + self.distance[e][f],
                        self.distance[a][b] + self.distance[c][e] + self.distance[d][f],
                        self.distance[a][d] + self.distance[e][b] + self.distance[c][f],
                        self.distance[f][b] + self.distance[c][d] + self.distance[e][a],
                    );

                    if d0 > d1 {
                        self.path[i..j].reverse();
                        self.score += d1 - d0;
                    } else if d0 > d2 {
                        self.path[j..k].reverse();
                        self.score += d2 - d0;
                    } else if d0 > d4 {
                        self.path[i..k].reverse();
                    } else if d0 > d3 {
                        temp.copy_from_slice(&self.path);
                        self.path[i..i + k - j].copy_from_slice(&temp[j..k]);
                        self.path[i + k - j..k].copy_from_slice(&temp[i..j]);
                        self.score += d3 - d0;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug() {
        let mut a = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        a[3..6].reverse();
        assert_eq!(a, [0, 1, 2, 5, 4, 3, 6, 7, 8, 9]);
    }

    #[test]
    fn test_array_swap() {
        let mut rng = rand::thread_rng();
        let mut a = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let start_idx = 2;
        let width = 3;
        let n = 8;

        let mut temp = vec![0; n];

        temp.copy_from_slice(&a[..n]);
        temp[start_idx..start_idx + width].shuffle(&mut rng);
        println!("{:?}", a);

        a[0..n].copy_from_slice(&temp);
        println!("{:?}", a);
    }

    #[test]
    fn test_tweak_candidate() {
        let solver = Solver::new([[0; 2]; 250], 5);
        let candidate = vec![0, 1, 2, 3, 4, 0];
        let tweaked = solver.tweak_candidate(&candidate);
        println!("{:?}", tweaked);
    }
}
