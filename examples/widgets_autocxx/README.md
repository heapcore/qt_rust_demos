# widgets_autocxx

Qt Widgets demo powered by Rust + `autocxx`.

## What It Shows

- Rust application logic
- Qt Widgets UI integration
- C++ bridge via `autocxx` and `cxx`

## Structure

- `src/main.rs` - app entry and Rust logic
- `cxx/qt_bridge.h` - bridge declarations
- `cxx/qt_bridge.cpp` - bridge implementation
- `build.rs` - binding/codegen + linker configuration

## Run

From workspace root:

```bash
cargo run -p widgets_demo
```

If your Qt install is custom, update paths in `build.rs` first.
