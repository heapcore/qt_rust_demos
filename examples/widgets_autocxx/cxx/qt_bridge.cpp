#include "qt_bridge.h"
#include <QtPlugin>
#include <QDateTime>
#include <QDebug>

Q_IMPORT_PLUGIN(QWindowsIntegrationPlugin)

int qt_bridge::run_qt_app_with_label(const char* label) {
    int argc = 0;
    char** argv = nullptr;
    QApplication app(argc, argv);

    QPushButton button(label);
    button.resize(300, 60);

    QObject::connect(&button, &QPushButton::clicked, []() {
        qDebug() << "Hello from" << QDateTime::currentDateTime().toString();
    });

    button.show();

    return app.exec();
}
