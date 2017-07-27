// This file was generated by gir (0f7cd61) from gir-files (0bcaef9)
// DO NOT EDIT

use ContextMenuItem;
use ffi;
#[cfg(feature = "v2_8")]
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct ContextMenu(Object<ffi::WebKitContextMenu>);

    match fn {
        get_type => || ffi::webkit_context_menu_get_type(),
    }
}

impl ContextMenu {
    pub fn new() -> ContextMenu {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_context_menu_new())
        }
    }

    pub fn new_with_items(items: &[ContextMenuItem]) -> ContextMenu {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_context_menu_new_with_items(items.to_glib_none().0))
        }
    }
}

pub trait ContextMenuExt {
    fn append(&self, item: &ContextMenuItem);

    fn first(&self) -> Option<ContextMenuItem>;

    fn get_item_at_position(&self, position: u32) -> Option<ContextMenuItem>;

    fn get_items(&self) -> Vec<ContextMenuItem>;

    fn get_n_items(&self) -> u32;

    #[cfg(feature = "v2_8")]
    fn get_user_data(&self) -> Option<glib::Variant>;

    fn insert(&self, item: &ContextMenuItem, position: i32);

    fn last(&self) -> Option<ContextMenuItem>;

    fn move_item(&self, item: &ContextMenuItem, position: i32);

    fn prepend(&self, item: &ContextMenuItem);

    fn remove(&self, item: &ContextMenuItem);

    fn remove_all(&self);

    #[cfg(feature = "v2_8")]
    fn set_user_data(&self, user_data: &glib::Variant);
}

impl<O: IsA<ContextMenu>> ContextMenuExt for O {
    fn append(&self, item: &ContextMenuItem) {
        unsafe {
            ffi::webkit_context_menu_append(self.to_glib_none().0, item.to_glib_none().0);
        }
    }

    fn first(&self) -> Option<ContextMenuItem> {
        unsafe {
            from_glib_none(ffi::webkit_context_menu_first(self.to_glib_none().0))
        }
    }

    fn get_item_at_position(&self, position: u32) -> Option<ContextMenuItem> {
        unsafe {
            from_glib_none(ffi::webkit_context_menu_get_item_at_position(self.to_glib_none().0, position))
        }
    }

    fn get_items(&self) -> Vec<ContextMenuItem> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::webkit_context_menu_get_items(self.to_glib_none().0))
        }
    }

    fn get_n_items(&self) -> u32 {
        unsafe {
            ffi::webkit_context_menu_get_n_items(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v2_8")]
    fn get_user_data(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(ffi::webkit_context_menu_get_user_data(self.to_glib_none().0))
        }
    }

    fn insert(&self, item: &ContextMenuItem, position: i32) {
        unsafe {
            ffi::webkit_context_menu_insert(self.to_glib_none().0, item.to_glib_none().0, position);
        }
    }

    fn last(&self) -> Option<ContextMenuItem> {
        unsafe {
            from_glib_none(ffi::webkit_context_menu_last(self.to_glib_none().0))
        }
    }

    fn move_item(&self, item: &ContextMenuItem, position: i32) {
        unsafe {
            ffi::webkit_context_menu_move_item(self.to_glib_none().0, item.to_glib_none().0, position);
        }
    }

    fn prepend(&self, item: &ContextMenuItem) {
        unsafe {
            ffi::webkit_context_menu_prepend(self.to_glib_none().0, item.to_glib_none().0);
        }
    }

    fn remove(&self, item: &ContextMenuItem) {
        unsafe {
            ffi::webkit_context_menu_remove(self.to_glib_none().0, item.to_glib_none().0);
        }
    }

    fn remove_all(&self) {
        unsafe {
            ffi::webkit_context_menu_remove_all(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_8")]
    fn set_user_data(&self, user_data: &glib::Variant) {
        unsafe {
            ffi::webkit_context_menu_set_user_data(self.to_glib_none().0, user_data.to_glib_none().0);
        }
    }
}
