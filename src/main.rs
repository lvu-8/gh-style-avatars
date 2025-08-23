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
        let hash = self.hash_input(input);
        let color = self.extract_color(&hash);
        let bin = self.get_bin_by_hash(hash.clone());
        let mut chars = bin.chars();

        let mut img = RgbImage::from_pixel(SIZE, SIZE, color);
        self.fill_avatar(&mut img, &mut chars);

        img
    }

    fn extract_color(&self, hash: &[u8]) -> Rgb<u8> {
        let length = hash.len();
        let mut color = Rgb([0, 0, 0]);

        for i in 1..4 {
            color.0[i - 1] = hash[length - i];
        }

        color
    }

    fn fill_avatar(&self, img: &mut RgbImage, chars: &mut Chars<'_>) {
        for y in 0..SIZE {
            for x in 0..CENTER {
                if chars.next() == Some('1') {
                    img.put_pixel(x, y, WHITE);
                    img.put_pixel(SIZE - x - 1, y, WHITE);
                }
            }
            if chars.next() == Some('1') {
                img.put_pixel(CENTER, y, WHITE);
            }
        }
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
    let avatar = generator.generate_avatar("a");
    avatar
        .save("test/avatar.png")
        .expect("Failed to save avatar image");
    println!("Avatar generated and saved as avatar.png");
}
