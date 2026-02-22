//! Build script: generates bindings and compiles the C++ helper code.
use std::path::PathBuf;

fn main() {
    // --- Manual Qt6 Configuration ---
    let qt_dir = PathBuf::from("D:\\Qt\\qt6-static");
    // ---

    // 1. Set up include paths.
    // We need the Qt include paths, and also the path to our own C++ code.
    let include_paths: Vec<PathBuf> = vec![
        qt_dir.join("include"),
        qt_dir.join("include/QtCore"),
        qt_dir.join("include/QtGui"),
        qt_dir.join("include/QtWidgets"),
        PathBuf::from("cxx"),
    ];

    // 2. Generate bindings and compile the C++ shim in one go.
    autocxx_build::Builder::new("src/main.rs", &include_paths)
        .extra_clang_args(&["-std=c++17"])
        .build()
        .expect("autocxx build failed")
        .file("cxx/qt_bridge.cpp")
        .flag_if_supported("-std=c++17")
        .flag_if_supported("/std:c++17")
        .flag_if_supported("/Zc:__cplusplus")
        .flag_if_supported("/permissive-")
        .define("QT_STATIC", None) // Important for static builds
        .compile("qt_bridge");

    // 3. Link with the required Qt libraries.
    println!("cargo:rustc-link-search=native={}", qt_dir.join("lib").to_str().unwrap());
    println!("cargo:rustc-link-search=native={}", qt_dir.join("plugins/platforms").to_str().unwrap());
    for lib in ["Qt6Widgets", "Qt6Gui", "Qt6Core"] {
        println!("cargo:rustc-link-lib=static={}", lib);
    }

    // When linking Qt statically with MSVC, we also need to link a number of
    // Windows system libraries.
    for lib in [
        "version", "winmm", "gdi32", "user32", "advapi32", "shell32", "ole32",
        "oleaut32", "imm32", "opengl32", "ws2_32",
        // Added for Qt dependencies
        "d3d11", "d3d12", "dxgi", "netapi32", "authz", "dwrite", "dxguid",
        "comdlg32", "uxtheme", "uiautomationcore", "Wtsapi32", "runtimeobject",
        "d3d9", "dwmapi", "Shcore", "Setupapi", "Shlwapi",
        // Qt-bundled libraries
        "Qt6BundledZLIB", "Qt6BundledLibpng", "Qt6BundledPcre2", "Qt6BundledHarfbuzz", "Qt6BundledFreetype",
        // Qt platform plugin
        "qwindows",
    ] {
        println!("cargo:rustc-link-lib={}", lib);
    }

    // 4. Re-run if any of the source files change.
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=cxx/qt_bridge.h");
    println!("cargo:rerun-if-changed=cxx/qt_bridge.cpp");
}
