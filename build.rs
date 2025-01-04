use glob::glob;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
    sync::Arc,
};

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let base_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");

    let level_files = glob(&format!("{base_dir}/map/*.tmx"))
        .expect("failed to read map files")
        .map(|entry| entry.unwrap())
        .collect::<Vec<_>>();

    let output = TokenStream::from_iter(level_files.iter().map(|level_file| {
        let level_filename = level_file.to_str().unwrap();
        create_tilemap_info(level_filename)
    }));

    let output_file = File::create(format!("{out_dir}/tilemap.rs"))
        .expect("failed to open tilemap.rs file for writing");
    let mut writer = BufWriter::new(output_file);

    let output = quote! {
        #[derive(Clone, Copy)]
        pub struct TilemapInfo<'a> {
            pub background_data: &'a [u16],
            pub foreground_data: &'a [u16],
            pub size: (u32, u32),
            pub tile_types: &'a [u8],
        }

        #output
    };

    write!(&mut writer, "{output}").unwrap();
}

fn create_tilemap_info(level_filename: &str) -> TokenStream {
    println!("cargo:rerun-if-changed={level_filename}");

    let mut map_loader = tiled::Loader::new();

    let level_filename = Path::new(level_filename);
    let level_name = level_filename.file_stem().unwrap().to_str().unwrap();
    let map = map_loader.load_tmx_map(level_filename).unwrap();

    let width = map.width;
    let height = map.height;

    let tileset: Arc<tiled::Tileset> = map.tilesets()[0].clone();

    let background_layer = map
        .layers()
        .find(|layer| layer.name == "background")
        .and_then(|layer| layer.as_tile_layer())
        .unwrap();
    let background_tiles = extract_tiles(&background_layer);

    let foreground_layer = map
        .layers()
        .find(|layer| layer.name == "foreground")
        .and_then(|layer| layer.as_tile_layer())
        .unwrap();
    let foreground_tiles = extract_tiles(&foreground_layer);

    let mut tile_types = HashMap::new();
    for (id, tile) in tileset.tiles() {
        if tile.properties.contains_key("collision") {
            tile_types.insert(id, 1u8);
        }
    }
    let tile_types = (0..tileset.tilecount).map(|id| tile_types.get(&(id + 1)).unwrap_or(&0));

    let var_prefix = level_name
        .to_uppercase()
        .replace(".", "_")
        .replace("-", "_");
    let background_map_var = format_ident!("{}_BACKGROUND_MAP", var_prefix);
    let foreground_map_var = format_ident!("{}_FOREGROUND_MAP", var_prefix);
    let width_var = format_ident!("{}_WIDTH", var_prefix);
    let height_var = format_ident!("{}_HEIGHT", var_prefix);
    let tile_types_var = format_ident!("{}_TILE_TYPES", var_prefix);
    let info_var = format_ident!("{}_INFO", var_prefix);

    let output = quote! {
        pub const #background_map_var: &[u16] = &[#(#background_tiles),*];
        pub const #foreground_map_var: &[u16] = &[#(#foreground_tiles),*];
        pub const #width_var: u32 = #width;
        pub const #height_var: u32 = #height;
        pub const #tile_types_var: &[u8] = &[#(#tile_types),*];

        pub const #info_var: TilemapInfo<'static> = TilemapInfo {
            background_data: &#background_map_var,
            foreground_data: &#foreground_map_var,
            size: (#width_var, #height_var),
            tile_types: &#tile_types_var,
        };
    };

    output
}

fn extract_tiles<'a>(layer: &'a tiled::TileLayer) -> impl Iterator<Item = u16> + 'a {
    match layer {
        tiled::TileLayer::Finite(tiles) => (0..tiles.height())
            .flat_map(move |y| (0..tiles.width()).map(move |x| (y, x)))
            .map(move |(y, x)| tiles.get_tile_data(x as i32, y as i32))
            .map(|tile| tile.map(|tile| tile.id()).unwrap_or(0)),
        _ => unimplemented!("Infinite layers not supported"),
    }
    .map(get_map_id)
}

fn get_map_id(tile_id: u32) -> u16 {
    match tile_id {
        0 => 0,
        i => i as u16,
    }
}
