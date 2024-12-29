use rltk::{RGB, RRandomNumberGenerator};
use specs::prelude::*;
use super::{CombatStats, Player, Renderable, Name, Position, Vieshed, Monster, BlocksTile};

pub fn player(ecs: &mut World, player_x: i32, player_y: i32) -> Entity {
    ecs
        .create_entity()
        .with(Position { x: playe_rx, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .with(Viewshed{visible_tiles: Vec::new(), range: 8, dirty: true })
        .with(Name{name: "Player".to_string()})
        .with(CombatStats{max_hp: 30, hp: 30, defense: 2, power: 5})
        .build()
}