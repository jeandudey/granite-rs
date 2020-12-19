use glib::translate::*;
use std::fmt;
use gtk::{Label, Widget};

glib::wrapper! {
    pub struct HeaderLabel(Object<ffi::GraniteHeaderLabel, ffi::GraniteHeaderLabelClass>) @extends Label, Widget;

    match fn {
        get_type => || ffi::granite_header_label_get_type(),
    }
}

impl HeaderLabel {
    #[doc(alias = "granite_header_label_new")]
    pub fn new(label: &str) -> HeaderLabel {
        unsafe {
            from_glib_none(ffi::granite_header_label_new(label.to_glib_none().0))
        }
    }
}

impl fmt::Display for HeaderLabel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("HeaderLabel")
    }
}
