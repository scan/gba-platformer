mod sprites;

// Creates a module named `tileset`
agb::include_background_gfx!(tileset, "000000", GLOBAL_TILESET => deduplicate "gfx/source/tilemap_packed.aseprite");

pub use sprites::*;
pub use tileset::*;
