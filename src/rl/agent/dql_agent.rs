use crate::rl::algorithms::DQlearning;

use crate::rl::agent::Agent;
use crate::rl::env::Environment;

/// An agent using Deep-Q-Learning, based on a small neural network.
pub struct DQLAgent {
    dqlearning: DQlearning,
}

// based on Q-learning using a HashMap as table
//
impl DQLAgent {
    /// A constructor including an initial exploration rate.
    pub fn new(exploration: f32) -> Self {
        DQLAgent {
            dqlearning: DQlearning::new(exploration),
        }
    }
}

impl Agent for DQLAgent {
    fn get_id(&self) -> String {
        "dqlearning agent".to_string()
    }

    fn finish_round(&mut self, result: i32) {
        // -1 for loss, 0 for draw, 1 for win
        self.dqlearning.finish_round(result);
    }

    fn get_move(&mut self, board: &Box<dyn Environment>) -> usize {
        self.dqlearning.get_move(board)
    }

    fn get_exploration_rate(&self) -> f32 {
        self.dqlearning.get_exploration_rate()
    }

    fn set_exploration_rate(&mut self, e: f32) -> Result<(), String> {
        if e < 0. || e > 1. {
            return Err("exploration rate must be in [0,1]!".to_string());
        }
        self.dqlearning.set_exploration_rate(e)?;
        Ok(())
    }
}