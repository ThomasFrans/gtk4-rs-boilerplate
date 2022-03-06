use adw::{
    glib,
    gio
};
use adw::subclass::prelude::*;
use gtk::subclass::prelude::*;
use gtk::prelude::*;
use std::cell::RefCell;
use crate::window::Window;
use Default;

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends
            adw::Application,
            gtk::Application,
            gio::Application,
        @implements
            gio::ActionGroup,
            gio::ActionMap;
}

impl Application {
    pub fn new(id: &str) -> Self {
        glib::Object::new(&[("application-id", &id)]).unwrap()
    }
}

impl Default for Application {
    fn default() -> Self {
        glib::Object::new(&[]).unwrap()
    }
}

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct Application {
        pub win: RefCell<Option<Window>>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Application {
        const NAME: &'static str = "Application";
        type Type = super::Application;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for Application {}

    impl ApplicationImpl for Application {
        fn activate(&self, application: &Self::Type) {
            self.parent_activate(application);

            *self.win.borrow_mut() = Some(Window::new(application));

            (*self.win.borrow()).as_ref().unwrap().present();
        }
    }

    impl GtkApplicationImpl for Application {}

    impl AdwApplicationImpl for Application {}
}