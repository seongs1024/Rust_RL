use crate::network::nn::NeuralNetwork;
use crate::rl::algorithms::utils;
use ndarray::{Array, Array1, Array2};
use ndarray_stats::QuantileExt;
use rand::{Rng, ThreadRng};

fn new(learning_rate: f32) -> NeuralNetwork {
    let mut nn = NeuralNetwork::new2d((6, 6), "bce".to_string(), "adam".to_string());
    nn.set_batch_size(4);
    nn.set_learning_rate(learning_rate);
    nn.add_convolution((3, 3), 32, 0);
    nn.add_activation("sigmoid");
    nn.add_flatten();
    nn.add_dense(100);
    nn.add_activation("sigmoid");
    nn.add_dense(36);
    nn.add_activation("sigmoid");
    nn.print_setup();
    nn
}

#[allow(dead_code)]
pub struct DQlearning {
    nn: NeuralNetwork,
    counter: usize,
    sum: usize,
    exploration: f32,
    learning_rate: f32,
    discount_factor: f32,
    last_turn: (Array2<f32>, Array1<f32>, Array1<f32>, usize, bool), // (board before last own move, allowed moves, NN output, move choosen from NN, was choosen move allowed?)
    rng: ThreadRng,
    epsilon: f32,
}

impl DQlearning {
    pub fn new(exploration: f32) -> Self {
        let learning_rate = 1e-3;
        let discount_factor = 0.95;
        DQlearning {
            sum: 0,
            counter: 0,
            nn: new(learning_rate),
            exploration,
            learning_rate,
            last_turn: (
                Array::zeros((0, 0)),
                Array::zeros(0),
                Array::zeros(0),
                42,
                false,
            ),
            discount_factor,
            rng: rand::thread_rng(),
            epsilon: 1e-8,
        }
    }

    pub fn get_exploration_rate(&self) -> f32 {
        self.exploration
    }

    pub fn set_exploration_rate(&mut self, e: f32) -> Result<(), String> {
        if e < 0. || e > 1. {
            return Err("exploration rate must be in [0,1]!".to_string());
        }
        self.exploration = e;
        Ok(())
    }
}

impl DQlearning {
    // update "table" based on last action and their result
    pub fn finish_round(&mut self, result: i32) {
        // -1 for loss, 0 for draw, 1 for win
        if self.last_move_valid() {
            self.learn_from_reward(result as f32 * 7.);
        }
    }

    fn select_move(&mut self, prediction: Array1<f32>) -> usize {
        // TODO verify using argmax
        prediction.argmax().unwrap()
    }

    fn last_move_valid(&self) -> bool {
        self.last_turn.4
    }

    fn learn_from_reward(&mut self, reward: f32) {
        let input = self.last_turn.0.clone();
        let mut target = self.last_turn.2.clone() * self.last_turn.1.clone(); // last_turn.1 (allowed moves) works as a filter to penalize illegal moves simultaneously
        let mut new_target_val = target[self.last_turn.3] + reward;
        target[self.last_turn.3] += reward; // adjust target vector to adapt to outcome of last move
        if new_target_val > 1. {
            new_target_val = 1.;
        }
        if new_target_val < 0. {
            new_target_val = 0.1;
        }
        target[self.last_turn.3] = new_target_val;
        self.nn.train2d(input, target);
    }

    pub fn get_move(
        &mut self,
        board_arr: Array2<f32>,
        action_arr: Array1<bool>,
        reward: f32,
    ) -> usize {
        let actions = action_arr.mapv(|x| (x as i32) as f32);

        if self.last_move_valid() {
            self.learn_from_reward(reward);
        }

        let predicted_moves = self.nn.predict2d(board_arr.clone());
        self.count_illegal_moves(predicted_moves.clone(), actions.clone());
        let mut next_move = self.select_move(predicted_moves.clone());
        let mut valid_move = true;

        // assert that move predicted by nn is legal
        // otherwise pick a legal move randomly
        if !action_arr[next_move] {
            // illegal move
            let target = predicted_moves.clone() * actions.clone(); // filter out all illegal moves (=give target value 0)
            self.nn.train(board_arr.clone().into_dyn(), target); // penalize nn for illegal move

            // ignore NN prediction, choose an allowed move randomly
            next_move = utils::get_random_true_entry(action_arr);
            valid_move = false; // since we didn't use NN prediction
            self.last_turn = (board_arr, actions, predicted_moves, next_move, valid_move);
            return next_move;
        }

        // shall we explore a random move and ignore prediction?
        if self.exploration > rand::thread_rng().gen() {
            next_move = utils::get_random_true_entry(action_arr);
            valid_move = false;
        }

        // bookkeeping
        self.last_turn = (board_arr, actions, predicted_moves, next_move, valid_move);

        self.last_turn.3
    }

    fn count_illegal_moves(&mut self, predicted_moves: Array1<f32>, allowed_moves: Array1<f32>) {
        let illegal_moves = allowed_moves.mapv(|x| 1. - x);
        let errors = predicted_moves * illegal_moves;
        let errors = errors
            .iter()
            .fold(0, |err, &val| if val > 0.2 { err + 1 } else { err });
        self.sum += errors;
        self.counter += 1;
        let n = 1000;
        if self.counter % n == 0 {
            println!("errors per {} moves: {}", n, self.sum);
            self.sum = 0;
            self.counter = 0;
        }
    }
}
