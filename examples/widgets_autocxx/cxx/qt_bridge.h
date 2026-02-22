#pragma once
#include <QtWidgets/QApplication>
#include <QtWidgets/QPushButton>

namespace qt_bridge {
    /// Launches a simple Qt event loop and shows a button with the given text.
    /// The call blocks until the window is closed.
    int run_qt_app_with_label(const char* label);
}
