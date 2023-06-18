mod game;
mod particle;

fn main() -> Result<(), String> {
    let mut rng = rand::thread_rng();

    let game_state = game::game_init(1000, &mut rng)?;

    let _status_code = game::run_game(game_state, &mut rng)?;

    return Ok(());
}
