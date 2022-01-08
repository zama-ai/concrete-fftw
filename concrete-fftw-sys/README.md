# Concrete FFTW sys

This library contains an unsafe wrapper of the FFTW library.

To compile with fftw, you need to have libclang installed on your computer.

## Fork

This crate is a fork of the [rust-math/fftw](https://github.com/rust-math/fftw) wrapper of the 
FFTW library.

## Building

The `build.rs` script to compile this library needs the `cmake` and `ninja` build tools. Instructions on how to install these tools for different platforms:

### Linux

#### Debian/Ubuntu

```
sudo apt install cmake ninja-build
```

#### Fedora/Red Hat

```
sudo yum install cmake ninja-build
```

### MacOS

Assuming Homebrew is installed:

```
brew install cmake ninja
```

### Windows

First, install [Visual Studio Community](https://visualstudio.microsoft.com/vs/community/) from the Microsoft website. In the installer, get "Desktop Development for C++" under the Workloads tab, and "C++ CMake tools for Windows" under the Individual Components tab.

When compiling a project that uses this library, you must do so from a command prompt that has access to `cmake` and `ninja`. Assuming you installed Visual Studio 2022 to its default location, you can open a terminal with these tools by running:

```
cmd.exe /k "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat"
```

You can then run `cargo build` from within this terminal.
