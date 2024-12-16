#![no_std]
#![no_main]
// Required for allowing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

extern crate alloc;

mod gfx;
mod player;

use agb::{input::ButtonController, interrupt::VBlank};

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let mut input = ButtonController::new();
    let vblank = VBlank::get();
    let object = gba.display.object.get_managed();

    let mut player = object.object_sprite(gfx::PLAYER_RED_STANDING.sprite(0));
    player.show();

    loop {
        vblank.wait_for_vblank();
        object.commit();
        input.update();
    }
}
