// This file was generated by gir (0f7cd61) from gir-files (0bcaef9)
// DO NOT EDIT

use ffi;
use glib;
#[cfg(feature = "v2_8")]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(feature = "v2_8")]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(feature = "v2_8")]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(feature = "v2_8")]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct ColorChooserRequest(Object<ffi::WebKitColorChooserRequest>);

    match fn {
        get_type => || ffi::webkit_color_chooser_request_get_type(),
    }
}

pub trait ColorChooserRequestExt {
    #[cfg(feature = "v2_8")]
    fn cancel(&self);

    #[cfg(feature = "v2_8")]
    fn finish(&self);

    //#[cfg(feature = "v2_8")]
    //fn get_element_rectangle(&self, rect: /*Ignored*/gdk::Rectangle);

    //#[cfg(feature = "v2_8")]
    //fn get_rgba(&self, rgba: /*Ignored*/gdk::RGBA);

    //#[cfg(feature = "v2_8")]
    //fn set_rgba(&self, rgba: /*Ignored*/&gdk::RGBA);

    //fn get_property_rgba(&self) -> /*Ignored*/Option<gdk::RGBA>;

    //fn set_property_rgba(&self, rgba: /*Ignored*/Option<&gdk::RGBA>);

    #[cfg(feature = "v2_8")]
    fn connect_finished<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<ColorChooserRequest> + IsA<glib::object::Object>> ColorChooserRequestExt for O {
    #[cfg(feature = "v2_8")]
    fn cancel(&self) {
        unsafe {
            ffi::webkit_color_chooser_request_cancel(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_8")]
    fn finish(&self) {
        unsafe {
            ffi::webkit_color_chooser_request_finish(self.to_glib_none().0);
        }
    }

    //#[cfg(feature = "v2_8")]
    //fn get_element_rectangle(&self, rect: /*Ignored*/gdk::Rectangle) {
    //    unsafe { TODO: call ffi::webkit_color_chooser_request_get_element_rectangle() }
    //}

    //#[cfg(feature = "v2_8")]
    //fn get_rgba(&self, rgba: /*Ignored*/gdk::RGBA) {
    //    unsafe { TODO: call ffi::webkit_color_chooser_request_get_rgba() }
    //}

    //#[cfg(feature = "v2_8")]
    //fn set_rgba(&self, rgba: /*Ignored*/&gdk::RGBA) {
    //    unsafe { TODO: call ffi::webkit_color_chooser_request_set_rgba() }
    //}

    //fn get_property_rgba(&self) -> /*Ignored*/Option<gdk::RGBA> {
    //    let mut value = Value::from(None::<&/*Ignored*/gdk::RGBA>);
    //    unsafe {
    //        gobject_ffi::g_object_get_property(self.to_glib_none().0, "rgba".to_glib_none().0, value.to_glib_none_mut().0);
    //    }
    //    value.get()
    //}

    //fn set_property_rgba(&self, rgba: /*Ignored*/Option<&gdk::RGBA>) {
    //    unsafe {
    //        gobject_ffi::g_object_set_property(self.to_glib_none().0, "rgba".to_glib_none().0, Value::from(rgba).to_glib_none().0);
    //    }
    //}

    #[cfg(feature = "v2_8")]
    fn connect_finished<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "finished",
                transmute(finished_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v2_8")]
unsafe extern "C" fn finished_trampoline<P>(this: *mut ffi::WebKitColorChooserRequest, f: glib_ffi::gpointer)
where P: IsA<ColorChooserRequest> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&ColorChooserRequest::from_glib_none(this).downcast_unchecked())
}
