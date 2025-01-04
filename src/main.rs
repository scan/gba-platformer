#![no_std]
#![no_main]
// Required for allowing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

extern crate alloc;

mod entity;
mod gfx;
mod level;
mod num;
mod player;

mod tilemap {
    include!(concat!(env!("OUT_DIR"), "/tilemap.rs"));
}

use agb::{input::ButtonController, interrupt::VBlank};
use entity::Entity;
use level::Level;

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let (background, mut vram) = gba.display.video.tiled0();
    vram.set_background_palettes(gfx::PALETTES);

    let mut input = ButtonController::new();
    let vblank = VBlank::get();
    let object_memory = gba.display.object.get_managed();

    let mut player = Entity::new(
        object_memory.object_sprite(gfx::PLAYER_RED_STANDING.sprite(0)),
        (0, 0).into(),
        (8, 8),
    );
    player.set_visible(true);

    let level = Level::load_level(
        &mut vram,
        &background,
        &tilemap::LEVEL01_INFO,
        &gfx::GLOBAL_TILESET,
        (0, 0),
    );

    loop {
        player.set_velocity((input.x_tri() as i32, 1));
        player.update_position(&level);
        player.commit(level.scroll_pos);

        vblank.wait_for_vblank();
        object_memory.commit();
        input.update();
    }
}
