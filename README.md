# qt_rust_demos

> **WARNING:** This repository may be unstable or non-functional. Use at your own risk.

Unified Qt + Rust demos in one workspace.

This repository intentionally uses two different integration approaches:

- `autocxx` for the Qt Widgets example.
- `cxx-qt` for the Qt QML example.

Using one tool for both is possible, but this split is more practical:

- `autocxx` works well for direct C++/Widgets interop.
- `cxx-qt` is better aligned with QML patterns and Qt object bridging.

## Workspace Layout

- `examples/widgets_autocxx` - Qt Widgets + Rust via `autocxx`
- `examples/qml_cxxqt` - Qt QML + Rust via `cxx-qt`

## Quick Start

From `qt_rust_demos`:

```bash
cargo metadata
```

Run the widgets demo:

```bash
cargo run -p widgets_demo
```

Run the QML demo:

```bash
cargo run -p qml_demo
```

## Notes

- Build prerequisites differ between the two examples.
- See each example's `README.md` for environment setup details.

## License

See `LICENSE`.
