#![no_std]
#![no_main]
// Required for allowing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

extern crate alloc;

mod gfx;
mod player;
mod num;

use agb::{input::ButtonController, interrupt::VBlank};
use player::Player;

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    use agb::fixnum::num;

    let mut input = ButtonController::new();
    let vblank = VBlank::get();
    let object_memory = gba.display.object.get_managed();

    let mut player = Player::new(object_memory.object_sprite(gfx::PLAYER_RED_STANDING.sprite(0)));

    loop {
        player.translate(num::FixedNum::from(input.x_tri() as i32) * num!(0.8));
        player.update();

        vblank.wait_for_vblank();
        object_memory.commit();
        input.update();
    }
}
