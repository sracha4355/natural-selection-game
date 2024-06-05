use life::game::*;
use life::{creature::SimpleCreature, food::SimpleFood};
use ggez::{self, event, Context, GameResult};
fn main() {
    let game_info = ggez::ContextBuilder::new("natural-selection", "Satvik Racharla")
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title("natural-selection-simulation")
        )
        .window_mode(ggez::conf::WindowMode::default()
            .resizable(true)
        )
        .build();
    let game_state: GameState<SimpleCreature, SimpleFood> = GameState::default();

    match game_info {
        Ok(data) =>{
            let (ctx, event_loop) = data;
            event::run(ctx, event_loop, game_state);
        },
        Err(_) =>{
            println!("ERROR: Could not build context");
        }
    }
}
