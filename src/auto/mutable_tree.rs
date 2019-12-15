// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Error;
#[cfg(any(feature = "v2018_7", feature = "dox"))]
use Repo;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use ostree_sys;
use std::fmt;
use std::ptr;

glib_wrapper! {
    pub struct MutableTree(Object<ostree_sys::OstreeMutableTree, ostree_sys::OstreeMutableTreeClass, MutableTreeClass>);

    match fn {
        get_type => || ostree_sys::ostree_mutable_tree_get_type(),
    }
}

impl MutableTree {
    pub fn new() -> MutableTree {
        unsafe {
            from_glib_full(ostree_sys::ostree_mutable_tree_new())
        }
    }

    #[cfg(any(feature = "v2018_7", feature = "dox"))]
    pub fn new_from_checksum(repo: &Repo, contents_checksum: &str, metadata_checksum: &str) -> MutableTree {
        unsafe {
            from_glib_full(ostree_sys::ostree_mutable_tree_new_from_checksum(repo.to_glib_none().0, contents_checksum.to_glib_none().0, metadata_checksum.to_glib_none().0))
        }
    }
}

impl Default for MutableTree {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_MUTABLE_TREE: Option<&MutableTree> = None;

pub trait MutableTreeExt: 'static {
    #[cfg(any(feature = "v2018_7", feature = "dox"))]
    fn check_error(&self) -> Result<(), Error>;

    fn ensure_dir(&self, name: &str) -> Result<MutableTree, Error>;

    //fn ensure_parent_dirs(&self, split_path: /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 0, id: 28 }, metadata_checksum: &str) -> Result<MutableTree, Error>;

    #[cfg(any(feature = "v2018_7", feature = "dox"))]
    fn fill_empty_from_dirtree(&self, repo: &Repo, contents_checksum: &str, metadata_checksum: &str) -> bool;

    fn get_contents_checksum(&self) -> Option<GString>;

    //fn get_files(&self) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 };

    fn get_metadata_checksum(&self) -> Option<GString>;

    //fn get_subdirs(&self) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 1, id: 40 };

    fn lookup(&self, name: &str) -> Result<(GString, MutableTree), Error>;

    #[cfg(any(feature = "v2018_9", feature = "dox"))]
    fn remove(&self, name: &str, allow_noent: bool) -> Result<(), Error>;

    fn replace_file(&self, name: &str, checksum: &str) -> Result<(), Error>;

    fn set_contents_checksum(&self, checksum: &str);

    fn set_metadata_checksum(&self, checksum: &str);

    //fn walk(&self, split_path: /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 0, id: 28 }, start: u32) -> Result<MutableTree, Error>;
}

impl<O: IsA<MutableTree>> MutableTreeExt for O {
    #[cfg(any(feature = "v2018_7", feature = "dox"))]
    fn check_error(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ostree_sys::ostree_mutable_tree_check_error(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn ensure_dir(&self, name: &str) -> Result<MutableTree, Error> {
        unsafe {
            let mut out_subdir = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = ostree_sys::ostree_mutable_tree_ensure_dir(self.as_ref().to_glib_none().0, name.to_glib_none().0, &mut out_subdir, &mut error);
            if error.is_null() { Ok(from_glib_full(out_subdir)) } else { Err(from_glib_full(error)) }
        }
    }

    //fn ensure_parent_dirs(&self, split_path: /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 0, id: 28 }, metadata_checksum: &str) -> Result<MutableTree, Error> {
    //    unsafe { TODO: call ostree_sys:ostree_mutable_tree_ensure_parent_dirs() }
    //}

    #[cfg(any(feature = "v2018_7", feature = "dox"))]
    fn fill_empty_from_dirtree(&self, repo: &Repo, contents_checksum: &str, metadata_checksum: &str) -> bool {
        unsafe {
            from_glib(ostree_sys::ostree_mutable_tree_fill_empty_from_dirtree(self.as_ref().to_glib_none().0, repo.to_glib_none().0, contents_checksum.to_glib_none().0, metadata_checksum.to_glib_none().0))
        }
    }

    fn get_contents_checksum(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ostree_sys::ostree_mutable_tree_get_contents_checksum(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_files(&self) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 } {
    //    unsafe { TODO: call ostree_sys:ostree_mutable_tree_get_files() }
    //}

    fn get_metadata_checksum(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ostree_sys::ostree_mutable_tree_get_metadata_checksum(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_subdirs(&self) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 1, id: 40 } {
    //    unsafe { TODO: call ostree_sys:ostree_mutable_tree_get_subdirs() }
    //}

    fn lookup(&self, name: &str) -> Result<(GString, MutableTree), Error> {
        unsafe {
            let mut out_file_checksum = ptr::null_mut();
            let mut out_subdir = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = ostree_sys::ostree_mutable_tree_lookup(self.as_ref().to_glib_none().0, name.to_glib_none().0, &mut out_file_checksum, &mut out_subdir, &mut error);
            if error.is_null() { Ok((from_glib_full(out_file_checksum), from_glib_full(out_subdir))) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2018_9", feature = "dox"))]
    fn remove(&self, name: &str, allow_noent: bool) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ostree_sys::ostree_mutable_tree_remove(self.as_ref().to_glib_none().0, name.to_glib_none().0, allow_noent.to_glib(), &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn replace_file(&self, name: &str, checksum: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ostree_sys::ostree_mutable_tree_replace_file(self.as_ref().to_glib_none().0, name.to_glib_none().0, checksum.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_contents_checksum(&self, checksum: &str) {
        unsafe {
            ostree_sys::ostree_mutable_tree_set_contents_checksum(self.as_ref().to_glib_none().0, checksum.to_glib_none().0);
        }
    }

    fn set_metadata_checksum(&self, checksum: &str) {
        unsafe {
            ostree_sys::ostree_mutable_tree_set_metadata_checksum(self.as_ref().to_glib_none().0, checksum.to_glib_none().0);
        }
    }

    //fn walk(&self, split_path: /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 0, id: 28 }, start: u32) -> Result<MutableTree, Error> {
    //    unsafe { TODO: call ostree_sys:ostree_mutable_tree_walk() }
    //}
}

impl fmt::Display for MutableTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MutableTree")
    }
}
