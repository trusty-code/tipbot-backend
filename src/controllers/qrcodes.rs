
use actix_web::{Result, HttpRequest, Error };
extern crate qrcode_generator;
use actix_files::NamedFile;
use std::path::PathBuf;
use qrcode_generator::QrCodeEcc;

pub async fn get_qrcodes(request: HttpRequest) -> Result<NamedFile, Error> {

    // Get iota address from request
    let address: String = request.match_info().query("address").parse().unwrap();

    // Setup svg name String
    let mut svg_file_name: String = "data/qr/".to_owned();
    svg_file_name.push_str(&address);
    svg_file_name.push_str(".svg");
    
    // Write svg file
    qrcode_generator::to_svg_to_file(&address, QrCodeEcc::Low, 1024, None, &svg_file_name).unwrap();    
    
    // get Path and return file content
    let path = PathBuf::from(&svg_file_name);
    Ok(NamedFile::open(path)?)
}

fn is_valid_iota_address(address: String) -> bool {
    // TODO: IMPLEMENT
    true
}