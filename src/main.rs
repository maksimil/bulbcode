use image::io::Reader as ImageReader;
use image::Luma;
use table::Table;

pub mod table;

fn is_key_color(px: &Luma<u8>) -> bool {
    px.0[0] <= 128
}

fn main() {
    let image = ImageReader::open(r"target\bulb.png")
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image");
    let image = image.to_luma8();

    let width = image.width() as usize;
    let height = image.height() as usize;

    let mut graytable = Table::new(width, height, &false);

    for j in 0..height {
        println!();
        for i in 0..width {
            graytable[(i, j)] = is_key_color(image.get_pixel(i as u32, j as u32));
        }
    }
}
