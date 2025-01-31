import pikepdf
import os
import unicodedata

def desbloquear_pdf(input_pdf):
    input_pdf = input_pdf.strip("'").strip('"')
    input_pdf_normalizado = unicodedata.normalize('NFC', input_pdf)
    
    if not os.path.isfile(input_pdf_normalizado):
        print(f"Error: El archivo '{input_pdf_normalizado}' no existe. Verifique la ruta.")
        return
    
    directorio = os.path.dirname(input_pdf_normalizado)
    nombre_salida = os.path.splitext(os.path.basename(input_pdf_normalizado))[0] + "_desbloqueado.pdf"
    output_pdf = os.path.join(directorio, nombre_salida)

    try:
        with pikepdf.open(input_pdf_normalizado) as pdf:
            if hasattr(pdf, 'permissions') and pdf.permissions:
                for permiso, estado in pdf.permissions.items():
                    print(f"  - {permiso}: {'Permitido' if estado else 'Restringido'}")
            pdf.save(output_pdf)
        
        print(f"PDF desbloqueado y guardado como: {output_pdf}")

    except pikepdf.PasswordError:
        print("Error: El PDF está protegido con contraseña. No se puede desbloquear.")
    except pikepdf.PdfError:
        print("Error: Archivo PDF corrupto o no válido.")
    except Exception as e:
        print(f"Ocurrió un error inesperado: {e}")

if __name__ == "__main__":
    print("""
    _____  ____  ____     __  _  ____    _____  ____   ____  _____ _____  _  ____  _____  _  ____  __  _ 
    | ()_)| _) \| ===|   |  \| |/ () \   | () )| ===| (_ (_`|_   _|| () )| |/ (__`|_   _|| |/ () \|  \| |
    |_|   |____/|__|     |_|\__|\____/   |_|\_\|____|.__)__)  |_|  |_|\_\|_|\____)  |_|  |_|\____/|_|\__|
    por @rubvenvmu
    """)
    
    input_pdf = input("Ingrese la ruta del archivo PDF de entrada: ").strip()
    desbloquear_pdf(input_pdf)
