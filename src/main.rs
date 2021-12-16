use std::convert::TryInto;

use clap::{load_yaml, App};
use qrart::{calc_best_scale, draw_art_qr};
use qrcodegen::{QrCode, QrCodeEcc};
fn main() {
    let yaml = load_yaml!("app.yaml");
    let app = App::from(yaml);
    let matches = app.get_matches();
    let text = matches.value_of("text").unwrap();
    let image_path = matches.value_of("image").unwrap();
    let output_path = matches.value_of("output").unwrap();
    let image = image::open(image_path).expect("File not exists");
    let mut image = image.into_rgba8();

    let qr_code = QrCode::encode_text(text, QrCodeEcc::High).expect("Error encoding text");
    let best_scale = calc_best_scale(
        image.width(),
        image.height(),
        qr_code.size().try_into().unwrap(),
    );
    draw_art_qr(&qr_code, &mut image, best_scale, 0, 0);
    image.save(output_path).unwrap();
}
