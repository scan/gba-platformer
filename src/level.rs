use agb::{
    display::{
        tile_data::TileData,
        tiled::{InfiniteScrolledMap, RegularBackgroundSize, TileFormat, Tiled0, VRamManager},
        Priority,
    },
    interrupt::VBlank,
};
use alloc::boxed::Box;

use crate::{
    gfx,
    num::{self, Rect, Vector2D},
    tilemap,
};

pub struct Level<'a> {
    info: &'a tilemap::TilemapInfo<'a>,

    background: InfiniteScrolledMap<'a>,
    foreground: InfiniteScrolledMap<'a>,

    pub scroll_pos: Vector2D,
}

impl<'a> Level<'a> {
    pub fn load_level(
        vram: &mut VRamManager,
        background_manager: &'a Tiled0<'_>,
        map_info: &'a tilemap::TilemapInfo,
        tileset_graphics: &'a TileData,
        start_pos: impl Into<Vector2D>,
    ) -> Self {
        let vblank = VBlank::get();

        let mut between_updates = || {
            // sfx.frame();
            vblank.wait_for_vblank();
        };

        let mut backdrop = InfiniteScrolledMap::new(
            background_manager.background(
                Priority::P2,
                RegularBackgroundSize::Background32x32,
                TileFormat::FourBpp,
            ),
            Box::new(|pos| {
                (
                    &tileset_graphics.tiles,
                    gfx::GLOBAL_TILESET.tile_settings[*map_info
                        .background_data
                        .get((pos.x + map_info.size.0 as i32 * pos.y) as usize)
                        .unwrap_or(&0)
                        as usize],
                )
            }),
        );

        let mut foreground = InfiniteScrolledMap::new(
            background_manager.background(
                Priority::P0,
                RegularBackgroundSize::Background32x32,
                TileFormat::FourBpp,
            ),
            Box::new(|pos| {
                (
                    &tileset_graphics.tiles,
                    gfx::GLOBAL_TILESET.tile_settings[*map_info
                        .foreground_data
                        .get((pos.x + map_info.size.0 as i32 * pos.y) as usize)
                        .unwrap_or(&0)
                        as usize],
                )
            }),
        );

        let start_pos = start_pos.into().floor();

        backdrop.init(vram, start_pos, &mut between_updates);
        foreground.init(vram, start_pos, &mut between_updates);

        backdrop.commit(vram);
        foreground.commit(vram);

        backdrop.set_visible(true);
        foreground.set_visible(true);

        Self {
            info: map_info,
            background: backdrop,
            foreground,
            scroll_pos: start_pos.into(),
        }
    }

    pub fn collides(&self, v: Vector2D) -> Option<Rect> {
        let factor = num::FixedNum::new(1) / num::FixedNum::new(8);
        let (x, y) = (v * factor).floor().get();

        if x < 0
            || y < 0
            || x >= self.info.size.0 as i32
            || y >= self.info.size.1 as i32
        {
            return Some(Rect::new((x * 8, y * 8).into(), (8, 8).into()));
        }
        let position = self.info.size.0 as usize * (y + 1) as usize + (x + 1) as usize;
        let tile_background = self.info.background_data[position];
        let tile_background_property = self.info.tile_types[tile_background as usize];

        if tile_background_property == 1 {
            Some(Rect::new((x * 8, y * 8).into(), (8, 8).into()))
        } else {
            None
        }
    }

    pub fn scroll(&mut self, vram: &mut VRamManager, offset: Vector2D) {
        self.scroll_pos += offset;
        let scroll_pos = self.scroll_pos.floor();
        self.background.set_pos(vram, scroll_pos);
        self.foreground.set_pos(vram, scroll_pos);
    }

    pub fn clear(&mut self, vram: &mut VRamManager) {
        self.background.clear(vram);
        self.foreground.clear(vram);
    }
}
