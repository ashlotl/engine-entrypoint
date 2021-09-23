use engine_use::{init_statics, load_scraps, lock_scrap_data};

fn main() {
	println!("Initializing Engine Engine!");
	init_statics();
	let _ = load_scraps(false).unwrap();
	let g = lock_scrap_data();
	println!("{:#?}", g.get("voxel-renderer"));
}
