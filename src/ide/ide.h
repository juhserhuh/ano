#ifndef IDE_H
#define IDE_H

#include <gtkmm.h>

class AnoIDE : public Gtk::Window {
public:
    AnoIDE();
    virtual ~AnoIDE();

private:
    Gtk::Box main_box;
    Gtk::TextView editor;
    Gtk::Button run_button;

    void on_run_clicked();
};

#endif // IDE_H
