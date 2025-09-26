use image::{Rgba, RgbaImage};
use std::vec;

const ASCII_BRIGHTNESS: &str =
    " `^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

fn get_ascii_from_brightness(brightness: u8) -> char {
    let percentage = (brightness as f32) / 255.;
    let index = ((ASCII_BRIGHTNESS.len() as f32) * percentage) as usize;
    ASCII_BRIGHTNESS
        .chars()
        .nth(index.min(ASCII_BRIGHTNESS.len() - 1))
        .unwrap()
}

fn rgba_to_brightness(rgba: &Rgba<u8>) -> u8 {
    let rgba = rgba.0;
    let alpha = rgba[3] as f32 / 255.;
    ((0.21 * rgba[0] as f32) + (0.72 * rgba[1] as f32) + (0.07 * rgba[2] as f32) * (1. / alpha))
        as u8
}

pub fn image_to_ascii(image: &RgbaImage) -> Vec<Vec<(char, [u8; 4])>> {
    let size = (image.width() as usize, image.height() as usize);
    let mut pixels = vec![vec![(' ', [0, 0, 0, 0]); size.0]; size.1];
    for (y, x, pixel) in image.enumerate_pixels() {
        let c = get_ascii_from_brightness(rgba_to_brightness(pixel));
        pixels[x as usize][y as usize] = (c, pixel.0);
    }
    pixels
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_picture() {
        let have = image_to_ascii(&RgbaImage::new(100, 100));
        let want = vec![vec![(' ', [0, 0, 0, 0]); 100]; 100];
        assert_eq!(have, want);
    }

    mod ascii_to_brightness {
        use super::*;
        #[test]
        fn are_all_brightnesses_valid() {
            for i in u8::MIN..=u8::MAX {
                _ = get_ascii_from_brightness(i);
            }
        }
        #[test]
        fn test_brightness() {
            assert_eq!(' ', get_ascii_from_brightness(0));
            assert_eq!('$', get_ascii_from_brightness(255));
        }
    }
}
