use std::io::{self, Write};
use pdfnorestriction::unlock_pdf;

fn main() {
    println!("PDF No Restriction Tool by @https://github.com/rubenvmu");

    print!("Introduzca la ruta del archivo PDF de entrada: ");
    io::stdout().flush().unwrap();

    let mut input_pdf = String::new();
    io::stdin()
        .read_line(&mut input_pdf)
        .expect("Error al leer la entrada");

    if let Err(e) = unlock_pdf(&input_pdf.trim()) {
        eprintln!("Error: {}", e);
    }
}