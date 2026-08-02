use std::env;
use std::fs;
use std::path::Path;

#[path = "src/game/farm_structs.rs"]
mod farm_structs;
use farm_structs::*;

fn main() {
    // build script precalculates valid crop groups and includes them as PHF sets of bitmasks.

    println!("cargo:rerun-if-changed=src/game/farm_structs.rs");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("crop_group_shapes.rs");

    // melons: 2x2 square
    let mut melon_groups = vec![];
    for r in 0..2 { for c in 0..2 {
        let mut mask = BitMask(0);
        for rb in 0..2 { for cb in 0..2 {
            mask = mask.flip(from_farm_coords(r + rb, c + cb));
        }}
        melon_groups.push(mask);
    }}

    // corn: 1x2 or 1x3 vertical rectangles
    let mut corn_groups = vec![];
    for c in 0..FARM_WIDTH {
        for r in 0..2 {
            let mut mask = BitMask(0);
            for rb in 0..2 {
                mask = mask.flip(from_farm_coords(r + rb, c));
            }
            corn_groups.push(mask);
        }
        
        let mut mask = BitMask(0);
        for r in 0..FARM_HEIGHT {
            mask = mask.flip(from_farm_coords(r, c));
        }
        corn_groups.push(mask);
    }

    // blueberry: 2 non-edge-adjacent plots
    let mut blueberry_groups = vec![];
    for a in 0..NUM_FARM_PLOTS { for b in a+1..NUM_FARM_PLOTS {
        let (ar, ac) = to_farm_coords(a);
        let (br, bc) = to_farm_coords(b);

        if ar.abs_diff(br) + ac.abs_diff(bc) != 1 {
            let mut mask = BitMask(0);
            mask = mask.flip(a); mask = mask.flip(b);
            blueberry_groups.push(mask);
        }
    }}

    // eggplant: 3 plots in an L shape, rotated any-which-way
    let mut eggplant_groups = vec![];
    for &mask in &melon_groups {
        for i in mask {
            let mut mask = mask;
            mask = mask.flip(i);
            eggplant_groups.push(mask);
        }
    }

    fn phf_set(masks: Vec<BitMask>) -> String {
        let mut res = String::from("phf_set! { ");

        for mask in masks {
            res.push_str(&mask.0.to_string());
            res.push_str("u16, ");
        }

        res.push('}');
        res
    }

    let code = format!(r#"
        const MELON_GROUPS: Set<u16> = {};
        const CORN_GROUPS: Set<u16> = {};
        const BLUEBERRY_GROUPS: Set<u16> = {};
        const EGGPLANT_GROUPS: Set<u16> = {};
    "#, 
        phf_set(melon_groups), phf_set(corn_groups), phf_set(blueberry_groups), phf_set(eggplant_groups));

    fs::write(&dest_path, code).unwrap();
}