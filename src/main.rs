use crate::game::Game;

mod assets;
mod game;
mod game_logic;
mod graphics;
mod map;
mod mesh;

fn main() -> anyhow::Result<()> {
    Game::new()?.run()?;

    Ok(())
}
