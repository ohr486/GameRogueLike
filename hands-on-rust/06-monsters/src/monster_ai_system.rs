use specs::prelude::*;
use super::{Viewshed, Monster, Name};
use rltk::{Point, console};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = ( ReadExpect<'a, Point>,
                        ReadStorage<'a, Viewshed>,
                        ReadStorage<'a, Monster>,
                        ReadStorage<'a, Name>);

    fn run(&mut self, data : Self::SystemData) {
        let (player_pos, viewsheds, monster, names) = data;

        for (viewshed,  _monster, name) in (&viewsheds, &monster, &names).join() {
            if viewshed.visible_tiles.contains(&*player_pos) {
                console::log(format!("{} shouts insults", name.name));
            }
        }
    }
}