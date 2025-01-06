use gtk4::{
    gio::ApplicationFlags,
    prelude::{ApplicationExt, ApplicationExtManual, GtkWindowExt, WidgetExt},
};
use gtk4_layer_shell::LayerShell;

fn main() {
    let app = gtk4::Application::new(Some("qws.qxb-widget-system"), ApplicationFlags::default());

    app.connect_activate(|app| {
        let bar_window = gtk4::ApplicationWindow::new(app);

        bar_window.init_layer_shell();
        bar_window.set_layer(gtk4_layer_shell::Layer::Top);
        bar_window.auto_exclusive_zone_enable();

        bar_window.set_anchor(gtk4_layer_shell::Edge::Left, true);
        bar_window.set_anchor(gtk4_layer_shell::Edge::Right, true);
        bar_window.set_anchor(gtk4_layer_shell::Edge::Top, true);

        let label = gtk4::Label::new(Some("hello"));

        bar_window.set_child(Some(&label));
        bar_window.show();
    });

    app.run();
}
