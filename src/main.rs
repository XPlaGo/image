use image::bmp;

fn main() {
    const n: usize = 1024;
    let mut file = std::fs::File::create("test.bmp").unwrap();
    let mut data: Vec<Vec<u8>> = Vec::with_capacity(n);
    for i in 0..n {
        data.push([if (i / 100) % 2 == 0 {0} else {1}; n].to_vec());
    }
    let mut image = bmp::BMP::new(data);
    image.push_color(bmp::RGBTRIPLE::new(255, 0, 0)).unwrap();
    image.write_to_file(&mut file);
}