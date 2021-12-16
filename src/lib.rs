use image::{Rgba, RgbaImage};
use qrcodegen::QrCode;
use std::collections::HashSet;
use std::convert::TryInto;

pub fn get_imp_modules(qr_code: &QrCode) -> HashSet<(i32, i32)> {
    let mut imp = HashSet::new();
    let size = qr_code.size();
    let ver = qr_code.version().value();
    // Finder Pattern
    let mut get_finder_pattern = |x, y| {
        for dy in -4..=4 {
            for dx in -4..=4 {
                let xx: i32 = x + dx;
                let yy: i32 = y + dy;
                if 0 <= xx && xx < size && 0 <= yy && yy < size {
                    imp.insert((xx, yy));
                }
            }
        }
    };
    get_finder_pattern(3, 3);
    get_finder_pattern(size - 4, 3);
    get_finder_pattern(3, size - 4);
    // Timing Pattern
    for i in 0..size {
        imp.insert((6, i));
        imp.insert((i, 6));
    }
    // // Alignment Pattern
    let alignpatpos: Vec<i32> = if ver == 1 {
        vec![]
    } else {
        let numalign = i32::from(ver) / 7 + 2;
        let step: i32 = if ver == 32 {
            26
        } else {
            (i32::from(ver) * 4 + numalign * 2 + 1) / (numalign * 2 - 2) * 2
        };
        let mut result: Vec<i32> = (0..numalign - 1)
            .map(|i| qr_code.size() - 7 - i * step)
            .collect();
        result.push(6);
        result.reverse();
        result
    };
    let numalign = alignpatpos.len();
    for i in 0..numalign {
        for j in 0..numalign {
            // Don't draw on the three finder corners
            if !(i == 0 && j == 0 || i == 0 && j == numalign - 1 || i == numalign - 1 && j == 0) {
                for dy in -2..=2 {
                    for dx in -2..=2 {
                        imp.insert((alignpatpos[i] + dx, alignpatpos[j] + dy));
                    }
                }
            }
        }
    }
    imp
}

pub fn draw_art_qr(qr_code: &QrCode, image: &mut RgbaImage, scale: u32, x: u32, y: u32) {
    let imp = get_imp_modules(qr_code);
    let foreground = Rgba([0, 0, 0, 255]);
    let background = Rgba([255, 255, 255, 255]);
    let draw_rect = |img: &mut RgbaImage, x: u32, y: u32, w: u32, h: u32, pixel| {
        for xx in x..x + w {
            for yy in y..y + h {
                img.put_pixel(xx, yy, pixel)
            }
        }
    };
    let size = qr_code.size() as u32;
    for i in 0..size {
        for j in 0..size {
            if qr_code.get_module(i.try_into().unwrap(), j.try_into().unwrap()) {
                if imp.contains(&(i.try_into().unwrap(), j.try_into().unwrap())) {
                    draw_rect(
                        image,
                        (x + i) * scale,
                        (y + j) * scale,
                        scale,
                        scale,
                        foreground,
                    )
                } else {
                    draw_rect(
                        image,
                        (x + i) * scale + scale * 5 / 12,
                        (y + j) * scale + scale * 5 / 12,
                        scale / 6,
                        scale / 6,
                        foreground,
                    )
                }
            } else {
                if imp.contains(&(i.try_into().unwrap(), j.try_into().unwrap())) {
                    draw_rect(
                        image,
                        (x + i) * scale,
                        (y + j) * scale,
                        scale,
                        scale,
                        background,
                    )
                } else {
                    draw_rect(
                        image,
                        (x + i) * scale + scale * 5 / 12,
                        (y + j) * scale + scale * 5 / 12,
                        scale / 6,
                        scale / 6,
                        background,
                    )
                }
            }
        }
    }
}
pub fn calc_best_scale(img_w: u32, img_h: u32, qr_size: u32) -> u32 {
    match img_w.cmp(&img_h) {
        std::cmp::Ordering::Greater => img_h / qr_size,
        _ => img_w / qr_size,
    }
}

#[test]
fn get_imp_modules_correct() {
    let chars: Vec<char> = "hello".chars().collect();
    let segment = qrcodegen::QrSegment::make_segments(&chars);
    let qr_code = QrCode::encode_segments_advanced(
        &segment,
        qrcodegen::QrCodeEcc::High,
        qrcodegen::Version::new(1),
        qrcodegen::Version::new(1),
        None,
        false,
    )
    .unwrap();
    let result = get_imp_modules(&qr_code);
    assert!(result.len() == 202);
}
