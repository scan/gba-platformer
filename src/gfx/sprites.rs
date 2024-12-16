use agb::{display::object::Graphics, include_aseprite};

const SPRITES: &Graphics = include_aseprite!("gfx/sprites/units.aseprite");

/// @see https://github.com/agbrs/agb/blob/master/examples/the-dungeon-puzzlers-lament/src/resources.rs
macro_rules! named_tag {
    (
        $sprites:ident, [
            $($name:tt),+ $(,)?
        ] $(,)?
    ) => {
        $(
            pub const $name: &agb::display::object::Tag = $sprites.tags().get(stringify!($name));
        )+
    };
}

named_tag!(
    SPRITES,
    [
        PLAYER_RED_STANDING,
        PLAYER_RED_WALKING, // 2 Frames
        PLAYER_RED_JUMPING,
        PLAYER_RED_DUCKING, // 2 Frames
        PLAYER_BLUE_STANDING,
        PLAYER_BLUE_WALKING, // 2 Frames
        PLAYER_BLUE_JUMPING,
        PLAYER_BLUE_DUCKING, // 2 Frames
        POP_RED,             // 3 Frames
        POP_BLUE,            // 3 Frames
        POP_BLACK,           // 2 Frames
        ENEMY_WALKING,       // 2 Frames
        ENEMY_FLYING,        // 2 Frames
        ENEMY_INVADER,
        ENEMY_SKULL,
    ],
);
