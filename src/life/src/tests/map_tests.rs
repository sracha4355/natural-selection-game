use std::default;
use crate::game::{CellContent, GameState};
use crate::creature::SimpleCreature;
use crate::food::SimpleFood;

#[test]
fn check_default_map() {
    let game_map:GameState<SimpleCreature, SimpleFood> = GameState::default();
    game_map.print_map();
    let mut count: (usize, usize) = (0,0);
    for row in game_map.map().iter(){
        for cell in row.iter(){
            match cell {
                CellContent::Creature(_) => count.1 += 1,
                CellContent::Food(_) => count.0 += 1,
                _ => {}
            }
        }
    }
    assert_eq!(70, count.1 + count.0);
}