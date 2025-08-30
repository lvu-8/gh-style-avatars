mod bit_iterator;

use bit_iterator::BitIter;
use image::{Rgb, RgbImage};
use md5::{Digest, Md5};

const WHITE: Rgb<u8> = Rgb([255, 255, 255]);

const SIZE: u32 = 5;
const CENTER: u32 = SIZE / 2;

struct AvatarGenerator {}

impl AvatarGenerator {
    pub fn new() -> Self {
        AvatarGenerator {}
    }

    pub fn generate_avatar(&self, hash: Vec<u8>) -> RgbImage {
        let (color_bytes, rest) = hash.split_at(3);
        let color = Rgb(color_bytes[0..3].try_into().unwrap());
        let mut iterator = BitIter::new(rest);

        let mut img = RgbImage::from_pixel(SIZE, SIZE, color);
        self.fill_avatar(&mut img, &mut iterator);

        img
    }

    fn fill_avatar(&self, img: &mut RgbImage, iterator: &mut BitIter<'_>) {
        for y in 0..SIZE {
            for x in 0..CENTER {
                if iterator.next().unwrap() == 1 {
                    img.put_pixel(x, y, WHITE);
                    img.put_pixel(SIZE - x - 1, y, WHITE);
                }
            }

            if iterator.next().unwrap() == 1 {
                img.put_pixel(CENTER, y, WHITE);
            }
        }
    }
}

fn hash_input(input: &str) -> Vec<u8> {
    let mut hasher = Md5::new();

    hasher.update(input.as_bytes());
    hasher.finalize().to_vec()
}

use std::env;
use std::fs;
use std::path::Path;

fn sanitize_filename(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);

    let key = match args.next() {
        Some(k) => k,
        None => {
            eprintln!("Require an input string as an argument.");
            std::process::exit(1);
        }
    };

    let generator = AvatarGenerator::new();
    let hash = hash_input(&key);
    let avatar = generator.generate_avatar(hash);

    let safe = sanitize_filename(&key);
    let filename = format!("test/avatar_{}.png", safe);

    if let Some(parent) = Path::new(&filename).parent() {
        fs::create_dir_all(parent)?;
    }

    avatar.save(&filename)?;
    println!("Avatar saved({})", filename);

    Ok(())
}
