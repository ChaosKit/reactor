extern crate byteorder;
extern crate image;

use std::io::{self, Read, BufReader};
use std::ops;
use std::path::Path;
use std::collections::VecDeque;
use std::convert::From;
use image::Rgba;
use byteorder::{ByteOrder, BigEndian};

struct Point {
    x: f64,
    y: f64,
    color: f64
}

struct ProjectedPoint<'a> {
    x: f64,
    y: f64,
    color: f64,
    extent: &'a Extent
}

type Pixel = usize;

#[derive(Clone, Copy)]
struct Color {
    r: f64,
    g: f64,
    b: f64,
    a: f64
}

impl Point {
    fn from_bytes(bytes: &[u8]) -> Point {
        Point {
            x: BigEndian::read_f64(&bytes[8*0..8*1]),
            y: BigEndian::read_f64(&bytes[8*1..8*2]),
            color: BigEndian::read_f64(&bytes[8*2..8*3])
        }
    }

    fn project<'a>(&'a self, extent: &'a Extent) -> ProjectedPoint {
        ProjectedPoint {
            x: (self.x + 2.0) / 4.0 * (extent.x as f64),
            y: (self.y + 2.0) / 4.0 * (extent.y as f64),
            color: self.color,
            extent: extent
        }
    }
}

impl<'a> ProjectedPoint<'a> {
    fn does_fit(&self) -> bool {
        self.x >= 0.0 && self.y >= 0.0 && self.x < (self.extent.x as f64) && self.y < (self.extent.y as f64)
    }

    fn to_pixel(&self) -> Pixel {
        // Pixel { x: self.x as usize, y: self.y as usize }
        (self.y.trunc() as usize) * self.extent.x + (self.x.trunc() as usize)
    }
}

impl Color {
    fn new() -> Color {
        Color {r: 0.0, g: 0.0, b: 0.0, a: 0.0}
    }

    fn map(&self, exposure: f64) -> Vec<u8> {
        if self.a == 0.0 {
            return vec![0u8; 3];
        }

        let scale = self.a.log2() / self.a;

        vec![
            (tone_map(self.r * scale, exposure) * 255.0).trunc() as u8,
            (tone_map(self.g * scale, exposure) * 255.0).trunc() as u8,
            (tone_map(self.b * scale, exposure) * 255.0).trunc() as u8
        ]
    }
}

impl<'a> ops::Add for &'a Color {
    type Output = Color;

    fn add(self, rhs: &Color) -> Color {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a
        }
    }
}

impl<'a> From<&'a Rgba<u8>> for Color {
    fn from(rgba: &'a Rgba<u8>) -> Color {
        Color {
            r: (rgba[0] as f64) / 255.0,
            g: (rgba[1] as f64) / 255.0,
            b: (rgba[2] as f64) / 255.0,
            a: (rgba[3] as f64) / 255.0
        }
    }
}

struct Extent {
    x: usize,
    y: usize
}

trait Palette {
    fn color_at(&self, point: f64) -> Color;
}

struct BlackWhite;

impl Palette for BlackWhite {
    fn color_at(&self, point: f64) -> Color {
        Color { r: point, g: point, b: point, a: 1.0 }
    }
}

struct ImagePalette {
    colors: Vec<Color>
}

impl ImagePalette {
    fn from_file(path: &Path) -> Result<ImagePalette, image::ImageError> {
        let img = try!(image::open(path)).to_rgba();
        let (width, _) = img.dimensions();
        let mut colors = Vec::with_capacity(width as usize);

        for x in 0..width {
            let pixel = img.get_pixel(x, 0);
            colors.push(Color::from(pixel));
        }

        Ok(ImagePalette { colors: colors })
    }
}

impl Palette for ImagePalette {
    fn color_at(&self, point: f64) -> Color {
        let index = (point * self.colors.len() as f64).round() as usize;

        self.colors[index]
    }
}

fn filmic_map(x: f64) -> f64 {
    const SHOULDER_STRENGTH: f64 = 0.15;
    const LINEAR_STRENGTH: f64 = 0.7;
    const LINEAR_ANGLE: f64 = 0.1;
    const TOE_STRENGTH: f64 = 6.0;
    const TOE_NUMERATOR: f64 = 0.02;
    const TOE_DENOMINATOR: f64 = 0.3;

    ((x * (SHOULDER_STRENGTH * x + LINEAR_ANGLE * LINEAR_STRENGTH) + TOE_STRENGTH * TOE_NUMERATOR) / (x * (SHOULDER_STRENGTH * x + LINEAR_STRENGTH) + TOE_STRENGTH * TOE_DENOMINATOR)) - TOE_NUMERATOR / TOE_DENOMINATOR
}

fn tone_map(subpixel: f64, exposure: f64) -> f64 {
    filmic_map(subpixel * exposure)
}

fn make_image(colors: Vec<Color>) -> Vec<u8> {
    let average_brightness = colors.iter().map(|ref color| color.a).fold(0.0, |current, alpha| current + alpha) / colors.len() as f64;

    let exposure = 8.0 / average_brightness.log2();

    colors.iter().flat_map(move|ref color| color.map(exposure).into_iter()).collect()
}

const IMAGE_SIZE: Extent = Extent { x: 1024, y: 1024 };
const PIXEL_COUNT: usize = IMAGE_SIZE.x * IMAGE_SIZE.y;

fn main() {
    let mut float_buffer: Vec<Color> = vec![Color::new(); PIXEL_COUNT];
    let mut point_bytes: [u8; 24] = [0; 24];

    let mut reader = BufReader::new(io::stdin());
    // let mut file = File::open("testdata.bin").ok().unwrap();
    // let mut reader = BufReader::new(file);

    let palette = ImagePalette::from_file(&Path::new("palette.png")).unwrap();

    println!("Capturing points…");

    let mut fit_count: i32 = 0;
    let mut total_count: i32 = 0;
    let mut point_buffer = VecDeque::with_capacity(24);

    loop {
        match reader.read(&mut point_bytes) {
            Ok(count) => {
                for i in 0..count {
                    point_buffer.push_back(point_bytes[i]);
                }

                if point_buffer.is_empty() || (count == 0 && point_buffer.len() < 24) { break; }
                else if point_buffer.len() < 24 { continue; }

                let mut bytes = Vec::with_capacity(24);
                for _ in 0..24 {
                    bytes.push(point_buffer.pop_front().unwrap());
                }

                total_count += 1;

                let point = Point::from_bytes(&bytes);
                let image_size = &IMAGE_SIZE;
                let projected_point = point.project(image_size);

                if projected_point.does_fit() {
                    fit_count += 1;

                    let pixel = projected_point.to_pixel();
                    float_buffer[pixel] = &float_buffer[pixel] + &palette.color_at(projected_point.color);
                }
            }
            Err(err) => panic!("{}", err)
        }
    }

    println!("{} points captured, {} fit", total_count, fit_count);
    println!("Creating image…");

    let byte_buffer = make_image(float_buffer);

    image::save_buffer(&Path::new("output.png"), &byte_buffer[..], IMAGE_SIZE.x as u32, IMAGE_SIZE.y as u32, image::RGB(8)).unwrap()
}
