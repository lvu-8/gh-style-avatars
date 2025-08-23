use image::{Rgb, RgbImage};
use md5::{Digest, Md5};
use std::str::Chars;

const WHITE: Rgb<u8> = Rgb([255, 255, 255]);

const SIZE: u32 = 5;
const CENTER: u32 = SIZE / 2;

struct AvatarGenerator {}

impl AvatarGenerator {
    pub fn new() -> Self {
        AvatarGenerator {}
    }

    pub fn generate_avatar(&mut self, input: &str) -> RgbImage {
        let hash: Vec<u8> = self.hash_input(input);
        let length: usize = hash.len();
        let color: Rgb<u8> = Rgb([hash[length - 1], hash[length - 2], hash[length - 3]]);

        let bin: String = self.get_bin_by_hash(hash.clone());
        let mut chars: Chars<'_> = bin.chars();

        let mut img = RgbImage::from_pixel(SIZE, SIZE, color);
        for y in 0..SIZE {
            for x in 0..(CENTER) {
                if chars.next() == Some('1') {
                    img.put_pixel(x, y, WHITE);
                    img.put_pixel(SIZE - x - 1, y, WHITE);
                }
            }

            if chars.next() == Some('1') {
                img.put_pixel(CENTER, y, WHITE);
            }
        }

        img
    }

    fn hash_input(&self, input: &str) -> Vec<u8> {
        let mut hasher = Md5::new();
        hasher.update(input.as_bytes());
        hasher.finalize().to_vec()
    }

    fn get_bin_by_hash(&self, hash: Vec<u8>) -> String {
        hash.iter().map(|byte| format!("{:08b}", byte)).collect()
    }
}

fn main() {
    let mut generator = AvatarGenerator::new();
    let avatar = generator.generate_avatar("key");
    avatar
        .save("test/avatar.png")
        .expect("Failed to save avatar image");
    println!("Avatar generated and saved as avatar.png");
}
