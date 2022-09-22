use tictactoe::{game::Game, game_tree::GameTree};

fn main() {
    let game_tree = GameTree::from(Game::new());
    println!("{}", game_tree);
    println!(
        "\n\nX: {}\nO: {}\nTie: {}\nsum: {}",
        game_tree.x_wins(),
        game_tree.o_wins(),
        game_tree.ties(),
        game_tree.x_wins() + game_tree.o_wins() + game_tree.ties()
    );
}
