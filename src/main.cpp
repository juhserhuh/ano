#include <gtkmm.h>
#include "ide/ide.h"

int main(int argc, char *argv[]) {
    auto app = Gtk::Application::create(argc, argv, "com.ano.ide");

    AnoIDE ide;
    return app->run(ide);
}
