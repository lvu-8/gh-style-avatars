
mod bit_iterator;

use image::{Rgb, RgbImage};
use md5::{Digest, Md5};
use bit_iterator::BitIter;

const WHITE: Rgb<u8> = Rgb([255, 255, 255]);

const SIZE: u32 = 5;
const CENTER: u32 = SIZE / 2;

struct AvatarGenerator {}

impl AvatarGenerator {
    pub fn new() -> Self {
        AvatarGenerator {}
    }

    pub fn generate_avatar(&mut self, hash: Vec<u8>) -> RgbImage {
        let color = self.extract_color(&hash);

        let mut img = RgbImage::from_pixel(SIZE, SIZE, color);
        self.fill_avatar(&mut img, &hash);

        img
    }

    fn extract_color(&self, hash: &[u8]) -> Rgb<u8> {
        let length = hash.len();
        Rgb([hash[length - 1], hash[length - 2], hash[length - 3]])
    }

    fn fill_avatar(&self, img: &mut RgbImage, hash: &[u8]) {
        let mut iterator = BitIter::new(hash);

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

fn main() {
    let mut generator = AvatarGenerator::new();
    let keys = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

    for i in keys {
        let hash = hash_input(i);
        let avatar = generator.generate_avatar(hash);
        let filename = format!("test/avatar_{}.png", i);

        avatar.save(&filename).expect("Failed to save avatar image");

        println!("Avatar generated and saved as {}", filename);
    }
}
