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

    pub fn solve(&mut self, duration: &Instant, max_duration: u128) {
        self.set_initial_path();
        eprintln!(
            "Initial score: {} ({})",
            self.score,
            duration.elapsed().as_millis()
        );

        self.run_annealing(duration, 1000, true);
        eprintln!("Score after annealing: {}", self.score, duration.elapsed().as_millis());

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
    }

    fn run_annealing(&mut self, duration: &Instant, max_duration: u128) {
        let mut rng = rand::thread_rng();
        let mut temp = 10000.0; // initial temperature
        let mut alpha = 0.999; // cooling factor
        let mut best_score = self.score;
        let mut best_path = self.path;

        while (duration.elapsed().as_millis() < max_duration) && (temp > 1.0) {
            // generate a new solution by swapping two random nodes in the path
            let i = rng.gen_range(0..self.n);
            let j = rng.gen_range(0..self.n);
            
            

            // calculate the score of the new solution
            let new_score = self.calculate_score(&new_path);

            // accept or reject the new solution based on the Metropolis criterion
            let delta = new_score - self.score;
            let p = if delta < 0.0 { 1.0 } else { (-delta / temp).exp() };
            if rng.gen::<f64>() < p {
                self.path = new_path;
                self.score = new_score;
                if self.score < best_score {
                    best_score = self.score;
                    best_path = new_path;
                }
            }

            // update the temperature
            temp *= alpha;
        }

        // set the best solution found
        self.path = best_path;
        self.score = best_score;
    }


    ...
}