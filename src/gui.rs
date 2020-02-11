extern crate rltk;
use rltk::{Console, Rltk, RGB};
extern crate specs;
use super::{CombatStats, GameLog, Name, Player};
use specs::prelude::*;

pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
    ctx.draw_box(
        0,
        43,
        79,
        6,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );

    let combat_stats = ecs.read_storage::<CombatStats>();
    let players = ecs.read_storage::<Player>();
    let names = ecs.read_storage::<Name>();
    for (_player, stats, name) in (&players, &combat_stats, &names).join() {
        let player_name = format!(" {} ", name.name);
        ctx.print_color(
            5,
            43,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            &player_name,
        );
        let health = format!(" HP: {} / {} ", stats.hp, stats.max_hp);
        ctx.print_color(
            31,
            43,
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
            &health,
        );

        ctx.draw_bar_horizontal(
            45,
            43,
            30,
            stats.hp,
            stats.max_hp,
            RGB::named(rltk::RED),
            RGB::named(rltk::BLACK),
        );
    }
    let log = ecs.fetch::<GameLog>();

    let mut y = 44;
    for s in log.entries.iter().rev() {
        if y < 49 {
            ctx.print(2, y, s);
        }
        y += 1;
    }
}
