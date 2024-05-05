fn main() {
    println!("Hello, world!");

fn store_svg_as_png(filepath: String, svg_data: &String, width: u32, height: u32) {
    let opt = usvg::Options::default();
    let db = fontdb::Database::new();

    let tree = usvg::Tree::from_str(&svg_data, &opt, &db).unwrap();
    let transformation = tiny_skia_path::Transform::identity();
    let mut pixmap = tiny_skia::Pixmap::new(width, height).unwrap();
    resvg::render(&tree, transformation, &mut pixmap.as_mut());
    pixmap.save_png(filepath).unwrap();
}
