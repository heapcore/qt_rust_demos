# qml_cxxqt

Qt QML demo powered by Rust + `cxx-qt`.

## What It Shows

- Rust-to-QML object bridge via `#[cxx_qt::bridge]`
- QML UI loaded from resource file (`qml.qrc`)
- Qt module linking through `cxx-qt-build`

## Structure

- `src/main.rs` - app entry
- `src/cxxqt_object.rs` - Rust/QML bridge object
- `qml/main.qml` - QML UI
- `qml/qml.qrc` - Qt resource file
- `build.rs` - `cxx-qt` build integration

## Run

From workspace root:

```bash
cargo run -p qml_demo
```
