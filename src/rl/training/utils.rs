use std::io;

/// A helper function to create agents based on terminal input.
pub fn read_agents(n: usize) -> Vec<usize> {
    let mut agents: Vec<usize> = vec![];

    for _ in 0..n {
        let mut agent = String::new();
        println!("please pick agents.");
        io::stdin()
            .read_line(&mut agent)
            .expect("Failed to read type of agents");
        let agent: usize = agent
            .trim()
            .parse()
            .expect("please type a number (1 for dql, 2 for ql, 3 for random, 4 for human");
        agents.push(agent);
    }
    agents
}

/// Reads the amount of training- and test-games from terminal.
pub fn read_game_numbers() -> (u64, u64) {
    let mut training_games = String::new();
    let mut bench_games = String::new();

    println!("please insert the number of training games.");
    io::stdin()
        .read_line(&mut training_games)
        .expect("Failed to read number of games");

    println!("please insert the number of benchmark games.");
    io::stdin()
        .read_line(&mut bench_games)
        .expect("Failed to read number of games");

    let training_games: u64 = training_games.trim().parse().expect("please type a number");
    let bench_games: u64 = bench_games.trim().parse().expect("please type a number");
    (training_games, bench_games)
}

/// For round based games, reads an usize value from terminal.
pub fn read_rounds_per_game() -> usize {
    //set number of rounds to play per game
    let mut rounds = String::new();
    println!("please insert the number of rounds per game.");
    io::stdin()
        .read_line(&mut rounds)
        .expect("Failed to read number of rounds");

    let rounds: usize = rounds.trim().parse().expect("please type a number");
    rounds
}
