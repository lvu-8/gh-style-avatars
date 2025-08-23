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

    pub fn generate_avatar(&mut self, input: &str) -> RgbImage {
        let hash = self.hash_input(input);
        let color = self.extract_color(&hash);

        let mut img = RgbImage::from_pixel(SIZE, SIZE, color);
        self.fill_avatar(&mut img, &hash);

        img
    }

    fn extract_color(&self, hash: &[u8]) -> Rgb<u8> {
        let length = hash.len();
        let mut color = Rgb([0, 0, 0]);

        for i in 0..3 {
            color.0[i] = hash[length - i - 1];
        }

        color
    }

    fn fill_avatar(&self, img: &mut RgbImage, hash: &[u8]) {
        let mut bit_index = 0;

        for y in 0..SIZE {
            for x in 0..CENTER {
                if self.bit_at(hash, bit_index) == 1 {
                    img.put_pixel(x, y, WHITE);
                    img.put_pixel(SIZE - x - 1, y, WHITE);
                }
                bit_index += 1;
            }
            if self.bit_at(hash, bit_index) == 1 {
                img.put_pixel(CENTER, y, WHITE);
            }
            bit_index += 1;
        }
    }

    fn bit_at(&self, hash: &[u8], i: usize) -> u8 {
        let byte = hash[i / 8];
        
        let bit = 7 - (i % 8);
        (byte >> bit) & 1
    }

    fn hash_input(&self, input: &str) -> Vec<u8> {
        let mut hasher = Md5::new();
        hasher.update(input.as_bytes());
        hasher.finalize().to_vec()
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
