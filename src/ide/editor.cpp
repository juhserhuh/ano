#include "ide.h"
#include <iostream>

AnoIDE::AnoIDE() {
    set_title("Ano IDE");
    set_default_size(800, 600);

    // Layout
    main_box.set_orientation(Gtk::ORIENTATION_VERTICAL);
    add(main_box);

    // Editor
    main_box.pack_start(editor);

    // Run Button
    run_button.set_label("Run");
    run_button.signal_clicked().connect(sigc::mem_fun(*this, &AnoIDE::on_run_clicked));
    main_box.pack_start(run_button, Gtk::PACK_SHRINK);

    show_all_children();
}

AnoIDE::~AnoIDE() {}

void AnoIDE::on_run_clicked() {
    auto buffer = editor.get_buffer();
    auto text = buffer->get_text();
    std::cout << "Running Anoe code:\n" << text << std::endl;
}
