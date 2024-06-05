
use std::collections::HashSet;
use rand::seq::IteratorRandom;
use crate::utils::traits;

pub enum CellContent<Creature, Food>
{
    Creature(Creature),
    Food(Food),
    Empty   
}

pub type Map<T, U> = Vec<Vec<CellContent<T, U>>>;

pub struct GameState <
    Creature: traits::Creature, 
    Food: traits::Food
> 
where {
    map: Map<Creature, Food>,
    dimensions: (usize, usize),
    mutation_rate: f32,
    epoch_length: usize,
    creatures: usize,
    food: usize
}

impl <Creature: traits::Creature, Food: traits::Food> 
GameState<Creature, Food> {
    pub fn new(
        dimensions: &(usize, usize),
        mutation_rate: &f32, 
        epoch_length: &usize,
        creatures: &usize,
        food: &usize, 
    ) -> Option<Self> {
        let mut game_map: Map<Creature, Food> = Vec::new();
        if let Err(_err_msg) = place_creatures_and_food_randomly(&mut game_map, creatures, food) {
            return None;
        }
        Some(GameState {
            map: game_map,
            dimensions: *dimensions,    
            mutation_rate: *mutation_rate,
            epoch_length: *epoch_length,
            creatures: *creatures,
            food: *food
        })
    }

    pub fn print_map(&self) {
        let mut current_col: usize = 0;
        print!("  ");
        for column in 0..self.dimensions.0 {
            print!("{} ", column);
        }
        print!("\n\n");

        (0..self.dimensions.0).for_each(|row| {
            print!("{} ", row);
            (0..self.dimensions.1).for_each(|col| {
                match  self.map[row][col] {
                    CellContent::Empty => print!("E "),
                    CellContent::Creature(_) => print!("C "),
                    CellContent::Food(_) => print!("F ")
                }
                current_col += 1;
                if current_col == self.dimensions.1 {
                    println!("\n");
                    current_col = 0;
                } 
    
            });
        });
    }

    pub fn map(&self) -> &Map<Creature, Food> { &self.map }
    pub fn map_mut(&mut self) -> &mut Map<Creature, Food> {&mut self.map}

    pub fn dimensions(&self) -> &(usize, usize) { &self.dimensions }
    pub fn set_dimensions(&mut self, dimensions: (usize, usize)) { self.dimensions = dimensions; }

    // Getter and setter for mutation_rate
    pub fn mutation_rate(&self) -> f32 { self.mutation_rate }
    pub fn set_mutation_rate(&mut self, mutation_rate: f32) { self.mutation_rate = mutation_rate; }

    // Getter and setter for epoch_length
    pub fn epoch_length(&self) -> usize { self.epoch_length }
    pub fn set_epoch_length(&mut self, epoch_length: usize) { self.epoch_length = epoch_length; }

    // Getter and setter for creatures
    pub fn creatures(&self) -> usize { self.creatures }
    pub fn set_creatures(&mut self, creatures: usize) { self.creatures = creatures; }

    // Getter and setter for food
    pub fn food(&self) -> usize { self.food }
    pub fn set_food(&mut self, food: usize) { self.food = food; }
}

impl <Creature: traits::Creature, Food: traits::Food> Default for GameState<Creature, Food> {
    fn default() -> Self {
        let mut game_map: Map<Creature, Food> = 
        (0..10)
            .map(|_| {
                (0..10)
                    .map(|_| CellContent::Empty)
                    .collect()
            })
            .collect();
        match place_creatures_and_food_randomly(&mut game_map, &20, &50) {
            Ok(()) => GameState {
                map: game_map,
                dimensions: (10, 10),
                mutation_rate: 0.10,
                epoch_length: 20,
                creatures: 20,
                food: 50
            },
            Err(err_msg) => {
                println!("{}", err_msg);
                GameState {
                    map: game_map,
                    dimensions: (10, 10),
                    mutation_rate: 0.10,
                    epoch_length: 20,
                    creatures: 0,
                    food: 0
                }
            }
        }
    }
}

fn place_creatures_and_food_randomly <Creature: traits::Creature, Food: traits::Food>(
    game_map: &mut Map<Creature, Food>,
    creatures: &usize,
    food: &usize
) -> Result<(), String> {
    let height = game_map.len();
    let width = game_map[0].len();
    let mut available_positions: HashSet<(usize, usize)> = HashSet::new();
    let mut rand: rand::prelude::ThreadRng = rand::thread_rng();

    if creatures + food > height * width {
        return Err(
            String::from("[ERROR] Attempted to generate more food and creatures than available cells"
        ));
    }

    (0..width).for_each(|row| {
        (0..height).for_each(|col| {
            available_positions.insert((row, col));
            println!("Adding {}, {}", row, col);
        });
    });

    /* generate creatures */
    (0..*creatures).for_each(|_|{
        let random_position: Option<(usize, usize)> = available_positions.
            iter()
            .choose(&mut rand)
            .cloned();
        if let Some(pos) = random_position {
            println!("random position in-use for adding Creature: {}, {}", pos.0, pos.1);
            game_map[pos.0][pos.1] = CellContent::Creature(Creature::default());
            available_positions.remove(&pos);
        }
    });

    /* generate food */
    (0..*food).for_each(|_|{
        let random_position: Option<(usize, usize)> = available_positions.
            iter()
            .choose(&mut rand)
            .cloned();

        if let Some(pos) = random_position {
            // place creature logic here
            println!("random position in-use for adding Food: {}, {}", pos.0, pos.1);
            game_map[pos.0][pos.1] = CellContent::Food(Food::default());
            available_positions.remove(&pos);
        }
    });
    Ok(())
}




