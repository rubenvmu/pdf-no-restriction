# pdfnorestriction

PDF No Restriction Tool is a command-line utility designed to efficiently and reliably remove restrictions from PDF documents. This application, compiled and managed using Cargo, harnesses the power of the external program **qpdf** to perform the decryption process. The project stands as a clear example of how technological innovation can empower individual freedom and grant unfettered access to information—values that resonate with a liberal outlook favoring private initiative and minimal state intervention.

---

## 🔍 Overview

The primary objective of PDF No Restriction Tool is to empower users by unlocking restricted PDF files, thereby restoring complete access to their content. The tool first normalizes file paths using Unicode Normalization Form C (NFC) and then invokes **qpdf** via a system call to execute the decryption operation. This technical solution underscores the importance of employing modern development practices to overcome bureaucratic constraints, reaffirming a commitment to autonomy and free-market principles.

---

## ✨ Features

- **Efficient Decryption**: Leverages **qpdf** to remove restrictions without compromising the document's integrity.  
- **Robust Error Handling**: Clearly notifies the user if the file does not exist or if any error occurs during the process, ensuring complete transparency at every step.  
- **Unicode Normalization**: Implements NFC to guarantee that file paths containing special characters are handled correctly.  
- **Command-Line Interface**: Provides a straightforward and user-friendly interface that facilitates integration into automated workflows.  
- **Modern and Responsive Code**: Built using contemporary software development practices, the application is continuously updated, reflecting an entrepreneurial spirit and a strong defense of free enterprise.

---

## ⚙️ Installation

To use PDF No Restriction Tool, the following prerequisites must be met:

1. **Rust and Cargo**: The project is managed with Cargo, so it is recommended that the latest stable version of Rust be installed. Installation instructions are available at [rust-lang.org](https://www.rust-lang.org/).  
2. **qpdf**: This external program is essential for the decryption operation. Installation instructions can be found on the [qpdf GitHub repository](https://github.com/qpdf/qpdf).

Once these dependencies are satisfied, the user may compile and run the tool using the following commands:

```bash
git clone https://github.com/rubenvmu/pdfnorestriction.git
cd pdfnorestriction

cargo build

./pdfnorestriction.sh
```

---

## 🚀 Usage

When the tool is executed, it prompts the user to enter the path of the PDF file to be unlocked. The operation proceeds as follows:

1. The script is initiated, displaying a welcome message similar to:  
   ```
   PDF No Restriction Tool by @https://github.com/rubenvmu
   Please enter the path of the PDF file to unlock:
   ```
2. After the path is entered, the application checks for the file’s existence, normalizes the file path, and then calls **qpdf** to remove the restrictions.  
3. If the process is successful, a confirmation message is displayed, indicating the location of the newly generated file (which carries the suffix `_unlocked.pdf`).

This procedure symbolizes both technical efficacy and a commitment to information freedom—principles that align with a respect for individual initiative and efficient resource utilization.

---

## 🛠️ Technical Details

The project’s code is comprised of several key elements:

- **I/O Operations**: It employs Rust’s standard libraries to interact with the user.  
- **Unicode Normalization**: The `unicode_normalization` crate is used to ensure the proper handling of special characters in file paths.  
- **System Command Execution**: **qpdf** is invoked through `std::process::Command`, which allows the tool to interface safely with the operating system.  
- **Error Management**: The code includes checks to verify the existence of the file and provides immediate feedback if any issues arise.

Each component is structured in a clear and maintainable manner, reflecting a professional approach and a dedication to secure, reliable software development.

---

## 🤝 Contributing

Contributions to PDF No Restriction Tool are both welcomed and highly valued. Developers interested in enhancing the project are encouraged to participate by:

- **Forking** the repository.  
- Creating feature branches for new functionalities or bug fixes.  
- Submitting pull requests with detailed descriptions of the changes.  
- Adhering to the established coding guidelines to maintain consistency throughout the codebase.

Contributing to this project is seen as an act that fosters innovation and responsible technological development, in keeping with the principles of a society that values entrepreneurial freedom and market competition.

---

## 📜 License

PDF No Restriction Tool is distributed under an open-source license that permits use, modification, and distribution, provided that the terms outlined in the LICENSE file included in the repository are followed. This legal openness reinforces the project’s commitment to transparency and collaborative development within the technological community.

