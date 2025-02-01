# 📄 PDF No Restriction Tool

PDF No Restriction Tool es una herramienta de línea de comandos desarrollada para eliminar restricciones de documentos PDF de forma eficiente y confiable. Esta aplicación, compilada y gestionada mediante Cargo, utiliza el poder del programa externo **qpdf** para llevar a cabo el proceso de desencriptación. El proyecto es un claro ejemplo de cómo la innovación tecnológica puede potenciar la libertad individual y el acceso sin trabas, valores que se alinean con una perspectiva liberal en cuanto a la promoción de la iniciativa privada y la mínima intervención estatal.

---

## 🔍 Overview

El propósito de PDF No Restriction Tool es empoderar a sus usuarios al desbloquear archivos PDF que han sido restringidos, restaurando así el acceso completo a su contenido. La herramienta se encarga de normalizar las rutas de archivos utilizando la normalización Unicode (NFC) y, posteriormente, invoca a **qpdf** mediante una llamada al sistema para ejecutar la operación de desencriptación.  
Esta solución técnica destaca la importancia de aplicar prácticas de desarrollo modernas para sortear limitaciones burocráticas, reafirmando la creencia en la autonomía y el libre mercado.

---

## ✨ Features

- **Desencriptación Eficiente**: Utiliza **qpdf** para eliminar restricciones sin comprometer la integridad del documento.  
- **Manejo Robusto de Errores**: Notifica claramente si el archivo no existe o si ocurre algún error durante el proceso, garantizando transparencia en cada paso.  
- **Normalización Unicode**: Implementa NFC para asegurar que las rutas con caracteres especiales se gestionen correctamente.  
- **Interfaz de Línea de Comandos**: Ofrece una experiencia de usuario directa y amigable, facilitando su integración en flujos de trabajo automatizados.  
- **Código Moderno y Responsivo**: La aplicación se mantiene actualizada y se construye siguiendo las prácticas más vanguardistas del desarrollo de software, en sintonía con el espíritu emprendedor y la defensa del mercado libre.

---

## ⚙️ Installation

Para utilizar PDF No Restriction Tool, se deben cumplir los siguientes requisitos previos:

1. **Rust y Cargo**: El proyecto se gestiona con Cargo, por lo que se recomienda tener instalada la versión estable más reciente de Rust. Las instrucciones de instalación están disponibles en [rust-lang.org](https://www.rust-lang.org/).  
2. **qpdf**: Este programa externo es indispensable para la operación de desencriptación. Se pueden encontrar las instrucciones de instalación en el [repositorio de qpdf](https://github.com/qpdf/qpdf).

Una vez satisfechas las dependencias, el usuario puede compilar y ejecutar la herramienta con los siguientes comandos:

```bash
git clone https://github.com/rubenvmu/pdfnorestriction.git
cd pdfnorestriction

cargo build

./pdfnorestriction.sh
```

---

## 🚀 Usage

Al ejecutar la herramienta, se solicita al usuario que introduzca la ruta del archivo PDF a desbloquear. El flujo de operación es el siguiente:

1. Se inicia el script, mostrando un mensaje de bienvenida similar a:  
   ```
   PDF No Restriction Tool by @https://github.com/rubenvmu
   Introduzca la ruta del archivo PDF de entrada:
   ```
2. Una vez ingresada la ruta, la aplicación verifica la existencia del archivo, normaliza la cadena de caracteres y procede a invocar **qpdf** para eliminar las restricciones.  
3. Si el proceso es exitoso, se genera un mensaje de confirmación indicando la ubicación del nuevo archivo (con el sufijo `_unlocked.pdf`).

Este procedimiento simboliza la eficacia técnica y el compromiso con la libertad de información, aspectos que resuenan en un contexto de respeto a la iniciativa individual y la eficiencia en el uso de los recursos tecnológicos.

---

## 🛠️ Technical Details

El código del proyecto se compone de los siguientes elementos clave:

- **Operaciones de I/O**: Se utilizan las librerías estándar de Rust para interactuar con el usuario.  
- **Normalización Unicode**: Se emplea el crate `unicode_normalization` para garantizar el correcto manejo de caracteres especiales en las rutas de archivo.  
- **Ejecución de Comandos del Sistema**: Se invoca a **qpdf** mediante `std::process::Command`, permitiendo que la herramienta se integre de forma segura con el sistema operativo.  
- **Gestión de Errores**: Se implementan controles que verifican la existencia del archivo y se proporciona retroalimentación inmediata ante cualquier inconveniente.

Cada uno de estos componentes se estructura de manera clara y mantenible, lo que refleja un enfoque profesional y comprometido con el desarrollo de software seguro y confiable.

---

## 🤝 Contributing

La colaboración es bienvenida y valorada en PDF No Restriction Tool. Se invita a otros desarrolladores a participar en la mejora continua del proyecto mediante:

- **Forking** el repositorio.
- Creación de ramas específicas para nuevas funcionalidades o corrección de errores.
- Envío de pull requests con descripciones detalladas de los cambios.
- Adherencia a las directrices de codificación establecidas para mantener la coherencia en la base de código.

La contribución a este proyecto se considera una acción de fomento de la innovación y el desarrollo tecnológico responsable, en línea con los principios de una sociedad que promueve el libre emprendimiento y la competencia en el mercado.

---

## 📜 License

PDF No Restriction Tool se distribuye bajo una licencia de código abierto que permite su uso, modificación y distribución, siempre y cuando se cumplan los términos estipulados en el archivo LICENSE incluido en el repositorio.  
Esta apertura legal refuerza el compromiso con la transparencia y la colaboración en la comunidad tecnológica.

---

## 🏁 Final Remarks

PDF No Restriction Tool representa una síntesis de excelencia técnica y principios de libertad individual. La herramienta demuestra cómo la modernidad en el desarrollo de software puede combinarse con una filosofía de libre mercado y responsabilidad personal, eliminando barreras innecesarias para el acceso a la información.  
Con un enfoque en la eficiencia y la autonomía, este proyecto se posiciona como una propuesta de vanguardia para quienes valoran la iniciativa privada y la innovación sin trabas burocráticas.  
Para más detalles, discusión o soporte, se invita a visitar el repositorio en [GitHub](https://github.com/rubenvmu/pdfnorestriction) y a participar activamente en la comunidad.
