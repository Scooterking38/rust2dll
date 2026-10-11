# rust2dll

**One-line Tagline:** A Rust library compiled as a Windows DLL that displays a message box.

## Architecture Overview

The `rust2dll` repository is structured to create a dynamic link library (DLL) using the Rust programming language. The core of the project lies in the `src/lib.rs` file, which defines a function `hello()` that interacts with Windows APIs to display a message box. The `Cargo.toml` file manages dependencies and specifies that the crate should be built as a C-compatible dynamic library (`cdylib`).

## Tech Stack

- **Programming Language:** Rust
- **Build System:** Cargo (Rust's package manager and build system)
- **Windows API Interaction:** `windows-sys` crate, specifically using the `Win32_UI_WindowsAndMessaging::MessageBoxA` function to display a message box.

## Getting Started / Setup Guide

### Prerequisites

1. **Install Rust:**
   Ensure you have Rust installed on your system. You can download and install it from [rust-lang.org](https://www.rust-lang.org/tools/install).

2. **Windows Environment:**
   This project is specifically designed for Windows due to its reliance on the Windows API.

### Steps to Build and Use

1. **Clone the Repository:**
   ```bash
   git clone https://github.com/yourusername/rust2dll.git
   cd rust2dll
   ```

2. **Build the DLL:**
   Run the following command to build the project:
   ```bash
   cargo build --release
   ```
   This will generate a `rust2dll.dll` file in the `target/release/` directory.

3. **Using the DLL:**
   - You can load this DLL into any Windows application that supports dynamic linking.
   - The exported function `hello()` can be called from other languages or applications to display a message box with the title "Rust DLL" and the message "Hello".

### Example Usage in C

Here is an example of how you might use this DLL in a simple C program:

```c
#include <windows.h>

typedef void (*HelloFunction)();

int main() {
    HMODULE hModule = LoadLibrary("rust2dll.dll");
    if (hModule == NULL) {
        printf("Failed to load DLL\n");
        return 1;
    }

    HelloFunction hello = (HelloFunction)GetProcAddress(hModule, "hello");
    if (hello == NULL) {
        printf("Failed to get function address\n");
        FreeLibrary(hModule);
        return 1;
    }

    hello();

    FreeLibrary(hModule);
    return 0;
}
```

Compile and run this C program after building the Rust DLL. It should display a message box with the specified title and message.

## Contributing

Contributions are welcome! Please fork the repository, make your changes, and submit a pull request. Ensure that any new features or changes do not break existing functionality.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.