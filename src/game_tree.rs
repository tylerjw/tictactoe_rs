use itertools::Itertools;
use std::fmt;

use crate::game::{next_games, Game, Winner};

pub struct GameTree {
    game: Game,
    edges: Vec<Edge>,
}

impl fmt::Display for GameTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Current State:\n{}\n\n{}",
            self.game,
            self.edges
                .iter()
                .map(|e| {
                    format!(
                        "{}\nO wins: {}\nX wins: {}\nTies: {}",
                        e.child.game, e.x_wins, e.o_wins, e.ties
                    )
                })
                .join("\n\n")
        )
    }
}

struct Edge {
    child: GameTree,
    x_wins: f32,
    o_wins: f32,
    ties: f32,
}

impl GameTree {
    pub fn from(game: Game) -> Self {
        if game.is_finished() {
            return Self {
                game,
                edges: vec![],
            };
        }

        let edges = next_games(&game)
            .iter()
            .map(|game| {
                let child = GameTree::from(game.clone());
                let x_wins = child.x_wins();
                let o_wins = child.o_wins();
                let ties = child.ties();
                Edge {
                    child,
                    x_wins,
                    o_wins,
                    ties,
                }
            })
            .collect();

        Self { game, edges }
    }

    pub fn o_wins(&self) -> f32 {
        if self.game.is_finished() {
            return match self.game.winner {
                Some(Winner::O) => 1.0,
                _ => 0.0,
            };
        }
        assert_ne!(self.edges.len(), 0);
        self.edges.iter().map(|e| e.child.o_wins()).sum::<f32>() / self.edges.len() as f32
    }

    // percentage of the times that x wins
    pub fn x_wins(&self) -> f32 {
        if self.game.is_finished() {
            return match self.game.winner {
                Some(Winner::X) => 1.0,
                _ => 0.0,
            };
        }
        assert_ne!(self.edges.len(), 0);
        self.edges.iter().map(|e| e.child.x_wins()).sum::<f32>() / self.edges.len() as f32
    }

    // percentage of the times that game ends in tie
    pub fn ties(&self) -> f32 {
        if self.game.is_finished() {
            return match self.game.winner {
                Some(Winner::Tie) => 1.0,
                _ => 0.0,
            };
        }
        assert_ne!(self.edges.len(), 0);
        self.edges.iter().map(|e| e.child.ties()).sum::<f32>() / self.edges.len() as f32
    }
}
