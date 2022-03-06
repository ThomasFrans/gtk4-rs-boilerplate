use adw::{glib, gio};
use adw::glib::IsA;
use adw::subclass::prelude::*;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends
            adw::ApplicationWindow,
            gtk::ApplicationWindow,
            gtk::Window,
            gtk::Widget,
        @implements
            gio::ActionGroup,
            gio::ActionMap,
            gtk::Accessible,
            gtk::Buildable,
            gtk::ConstraintTarget,
            gtk::Native,
            gtk::Root,
            gtk::ShortcutManager;
}

impl Window {
    pub fn new (application: &impl IsA<gio::Application>) -> Self {
        glib::Object::new(&[("application", application)]).unwrap()
    }
}

impl Default for Window {
    fn default() -> Self {
        glib::Object::new(&[]).unwrap()
    }
}

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct Window {}

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "Window";
        type Type = super::Window;
        type ParentType = adw::ApplicationWindow;
    }

    impl ObjectImpl for Window {}

    impl WidgetImpl for Window {}

    impl WindowImpl for Window {}

    impl ApplicationWindowImpl for Window {}

    impl AdwApplicationWindowImpl for Window {}
}
