static MATRIX_WIDTH: u8 = 8;
static MATRIX_HEIGHT: u8 = 8;

#[derive(Default)]
struct Pixel {
    on: bool,
    position: (u8, u8)
}

struct RGBPixel {
    red: Pixel,
    green: Pixel,
    blue: Pixel
}

impl Default for RGBPixel {
    fn default() -> Self {
        Self {
            red: Pixel::default(),
            green: Pixel::default(),
            blue: Pixel::default()
        }
    }
}

#[derive(Default)]
struct RGBMatrix {
    pixels: Vec<Vec<RGBPixel>>
}

fn main() {
    let mut matrix = RGBMatrix::default();
    
    for i in 0..MATRIX_HEIGHT {
        let mut row = Vec::new();
        for j in 0..MATRIX_WIDTH {
            row.push(RGBPixel::default())
        }
        matrix.pixels.push(row);
    }
}

