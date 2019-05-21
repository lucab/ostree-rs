// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Error;
use SePolicyRestoreconFlags;
use gio;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::translate::*;
use gobject_sys;
use ostree_sys;
use std::fmt;
use std::ptr;

glib_wrapper! {
    pub struct SePolicy(Object<ostree_sys::OstreeSePolicy, SePolicyClass>);

    match fn {
        get_type => || ostree_sys::ostree_sepolicy_get_type(),
    }
}

impl SePolicy {
    pub fn new<P: IsA<gio::File>, Q: IsA<gio::Cancellable>>(path: &P, cancellable: Option<&Q>) -> Result<SePolicy, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ostree_sys::ostree_sepolicy_new(path.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2017_4", feature = "dox"))]
    pub fn new_at<P: IsA<gio::Cancellable>>(rootfs_dfd: i32, cancellable: Option<&P>) -> Result<SePolicy, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ostree_sys::ostree_sepolicy_new_at(rootfs_dfd, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2016_5", feature = "dox"))]
    pub fn get_csum(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ostree_sys::ostree_sepolicy_get_csum(self.to_glib_none().0))
        }
    }

    pub fn get_label<P: IsA<gio::Cancellable>>(&self, relpath: &str, unix_mode: u32, cancellable: Option<&P>) -> Result<GString, Error> {
        unsafe {
            let mut out_label = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = ostree_sys::ostree_sepolicy_get_label(self.to_glib_none().0, relpath.to_glib_none().0, unix_mode, &mut out_label, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(out_label)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ostree_sys::ostree_sepolicy_get_name(self.to_glib_none().0))
        }
    }

    pub fn get_path(&self) -> Option<gio::File> {
        unsafe {
            from_glib_none(ostree_sys::ostree_sepolicy_get_path(self.to_glib_none().0))
        }
    }

    pub fn restorecon<P: IsA<gio::File>, Q: IsA<gio::Cancellable>>(&self, path: &str, info: Option<&gio::FileInfo>, target: &P, flags: SePolicyRestoreconFlags, cancellable: Option<&Q>) -> Result<GString, Error> {
        unsafe {
            let mut out_new_label = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = ostree_sys::ostree_sepolicy_restorecon(self.to_glib_none().0, path.to_glib_none().0, info.to_glib_none().0, target.as_ref().to_glib_none().0, flags.to_glib(), &mut out_new_label, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(out_new_label)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn setfscreatecon(&self, path: &str, mode: u32) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ostree_sys::ostree_sepolicy_setfscreatecon(self.to_glib_none().0, path.to_glib_none().0, mode, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn get_property_rootfs_dfd(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"rootfs-dfd\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    //pub fn fscreatecon_cleanup(unused: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ostree_sys:ostree_sepolicy_fscreatecon_cleanup() }
    //}
}

impl fmt::Display for SePolicy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SePolicy")
    }
}
