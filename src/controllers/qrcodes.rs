
use actix_web::{Result, HttpRequest, Error };
extern crate qrcode_generator;
use actix_files::NamedFile;
use std::path::PathBuf;
use qrcode_generator::QrCodeEcc;

pub async fn handle_qrcode(request: HttpRequest) -> Result<NamedFile, Error> {

    // Get iota address from request
    let address: String = request.match_info().query("address").parse().unwrap();

    let mut file_name: String = "data/qr/".to_owned();
    // check if user wants a png or default svg
    if address.ends_with(".png") {
        println!("png");
        file_name.push_str(&address);

        // todo: check if file already exsit

        // Generate and write png file
        qrcode_generator::to_png_to_file(&address, QrCodeEcc::Low, 1024, &file_name).unwrap();
    } else {       
        // Setup svg name String
        file_name.push_str(&address);
        file_name.push_str(".svg");

        // todo: check if file already exsit

        // Generate and write svg file
        qrcode_generator::to_svg_to_file(&address, QrCodeEcc::Low, 1024, None, &file_name).unwrap();    
    }
    
    // get file and return content
    let path = PathBuf::from(&file_name);
    Ok(NamedFile::open(path)?)
}

fn is_valid_iota_address(address: String) -> bool {
    // TODO: IMPLEMENT
    true
}