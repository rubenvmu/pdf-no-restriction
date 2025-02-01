use std::io;
use std::process::Command;
use unicode_normalization::UnicodeNormalization;
use std::path::Path;

pub fn unlock_pdf(input_pdf: &str) -> io::Result<()> {
    let input_pdf = input_pdf.trim_matches(|c| c == '\'' || c == '"');
    let input_pdf_normalized: String = input_pdf.nfc().collect();

    let input_path = Path::new(&input_pdf_normalized);
    if !input_path.is_file() {
        eprintln!(
            "Error: El archivo '{}' no existe. Por favor, verifique la ruta.",
            input_pdf_normalized
        );
        return Ok(());
    }
    let directory = input_path.parent().unwrap_or_else(|| Path::new("."));
    let file_stem = input_path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy();
    let output_name = format!("{}_unlocked.pdf", file_stem);
    let output_pdf = directory.join(output_name);

    let status = Command::new("qpdf")
        .arg("--decrypt")
        .arg(&input_pdf_normalized)
        .arg(output_pdf.to_str().unwrap())
        .status();

    match status {
        Ok(s) if s.success() => {
            println!(
                "PDF desbloqueado y guardado como: {}",
                output_pdf.display()
            );
        }
        Ok(s) => {
            eprintln!(
                "Error: No se pudo desbloquear el PDF. Es posible que esté protegido por contraseña o corrupto. Código de salida: {}",
                s.code().unwrap_or(-1)
            );
        }
        Err(e) => {
            eprintln!("Ocurrió un error inesperado: {}", e);
        }
    }

    Ok(())
}