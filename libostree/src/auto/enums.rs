// This file was generated by gir (https://github.com/gtk-rs/gir @ c385982)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use ffi;
use glib::translate::*;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum GpgSignatureFormatFlags {
    GpgSignatureFormatDefault,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for GpgSignatureFormatFlags {
    type GlibType = ffi::OstreeGpgSignatureFormatFlags;

    fn to_glib(&self) -> ffi::OstreeGpgSignatureFormatFlags {
        match *self {
            GpgSignatureFormatFlags::GpgSignatureFormatDefault => ffi::OSTREE_GPG_SIGNATURE_FORMAT_DEFAULT,
            GpgSignatureFormatFlags::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::OstreeGpgSignatureFormatFlags> for GpgSignatureFormatFlags {
    fn from_glib(value: ffi::OstreeGpgSignatureFormatFlags) -> Self {
        match value {
            0 => GpgSignatureFormatFlags::GpgSignatureFormatDefault,
            value => GpgSignatureFormatFlags::__Unknown(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum ObjectType {
    File,
    DirTree,
    DirMeta,
    Commit,
    TombstoneCommit,
    CommitMeta,
    PayloadLink,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for ObjectType {
    type GlibType = ffi::OstreeObjectType;

    fn to_glib(&self) -> ffi::OstreeObjectType {
        match *self {
            ObjectType::File => ffi::OSTREE_OBJECT_TYPE_FILE,
            ObjectType::DirTree => ffi::OSTREE_OBJECT_TYPE_DIR_TREE,
            ObjectType::DirMeta => ffi::OSTREE_OBJECT_TYPE_DIR_META,
            ObjectType::Commit => ffi::OSTREE_OBJECT_TYPE_COMMIT,
            ObjectType::TombstoneCommit => ffi::OSTREE_OBJECT_TYPE_TOMBSTONE_COMMIT,
            ObjectType::CommitMeta => ffi::OSTREE_OBJECT_TYPE_COMMIT_META,
            ObjectType::PayloadLink => ffi::OSTREE_OBJECT_TYPE_PAYLOAD_LINK,
            ObjectType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::OstreeObjectType> for ObjectType {
    fn from_glib(value: ffi::OstreeObjectType) -> Self {
        match value {
            1 => ObjectType::File,
            2 => ObjectType::DirTree,
            3 => ObjectType::DirMeta,
            4 => ObjectType::Commit,
            5 => ObjectType::TombstoneCommit,
            6 => ObjectType::CommitMeta,
            7 => ObjectType::PayloadLink,
            value => ObjectType::__Unknown(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum RepoCheckoutMode {
    None,
    User,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for RepoCheckoutMode {
    type GlibType = ffi::OstreeRepoCheckoutMode;

    fn to_glib(&self) -> ffi::OstreeRepoCheckoutMode {
        match *self {
            RepoCheckoutMode::None => ffi::OSTREE_REPO_CHECKOUT_MODE_NONE,
            RepoCheckoutMode::User => ffi::OSTREE_REPO_CHECKOUT_MODE_USER,
            RepoCheckoutMode::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::OstreeRepoCheckoutMode> for RepoCheckoutMode {
    fn from_glib(value: ffi::OstreeRepoCheckoutMode) -> Self {
        match value {
            0 => RepoCheckoutMode::None,
            1 => RepoCheckoutMode::User,
            value => RepoCheckoutMode::__Unknown(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum RepoCheckoutOverwriteMode {
    None,
    UnionFiles,
    AddFiles,
    UnionIdentical,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for RepoCheckoutOverwriteMode {
    type GlibType = ffi::OstreeRepoCheckoutOverwriteMode;

    fn to_glib(&self) -> ffi::OstreeRepoCheckoutOverwriteMode {
        match *self {
            RepoCheckoutOverwriteMode::None => ffi::OSTREE_REPO_CHECKOUT_OVERWRITE_NONE,
            RepoCheckoutOverwriteMode::UnionFiles => ffi::OSTREE_REPO_CHECKOUT_OVERWRITE_UNION_FILES,
            RepoCheckoutOverwriteMode::AddFiles => ffi::OSTREE_REPO_CHECKOUT_OVERWRITE_ADD_FILES,
            RepoCheckoutOverwriteMode::UnionIdentical => ffi::OSTREE_REPO_CHECKOUT_OVERWRITE_UNION_IDENTICAL,
            RepoCheckoutOverwriteMode::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::OstreeRepoCheckoutOverwriteMode> for RepoCheckoutOverwriteMode {
    fn from_glib(value: ffi::OstreeRepoCheckoutOverwriteMode) -> Self {
        match value {
            0 => RepoCheckoutOverwriteMode::None,
            1 => RepoCheckoutOverwriteMode::UnionFiles,
            2 => RepoCheckoutOverwriteMode::AddFiles,
            3 => RepoCheckoutOverwriteMode::UnionIdentical,
            value => RepoCheckoutOverwriteMode::__Unknown(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum RepoMode {
    Bare,
    Archive,
    BareUser,
    BareUserOnly,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for RepoMode {
    type GlibType = ffi::OstreeRepoMode;

    fn to_glib(&self) -> ffi::OstreeRepoMode {
        match *self {
            RepoMode::Bare => ffi::OSTREE_REPO_MODE_BARE,
            RepoMode::Archive => ffi::OSTREE_REPO_MODE_ARCHIVE,
            RepoMode::BareUser => ffi::OSTREE_REPO_MODE_BARE_USER,
            RepoMode::BareUserOnly => ffi::OSTREE_REPO_MODE_BARE_USER_ONLY,
            RepoMode::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::OstreeRepoMode> for RepoMode {
    fn from_glib(value: ffi::OstreeRepoMode) -> Self {
        match value {
            0 => RepoMode::Bare,
            1 => RepoMode::Archive,
            2 => RepoMode::BareUser,
            3 => RepoMode::BareUserOnly,
            value => RepoMode::__Unknown(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum RepoPruneFlags {
    None,
    NoPrune,
    RefsOnly,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for RepoPruneFlags {
    type GlibType = ffi::OstreeRepoPruneFlags;

    fn to_glib(&self) -> ffi::OstreeRepoPruneFlags {
        match *self {
            RepoPruneFlags::None => ffi::OSTREE_REPO_PRUNE_FLAGS_NONE,
            RepoPruneFlags::NoPrune => ffi::OSTREE_REPO_PRUNE_FLAGS_NO_PRUNE,
            RepoPruneFlags::RefsOnly => ffi::OSTREE_REPO_PRUNE_FLAGS_REFS_ONLY,
            RepoPruneFlags::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::OstreeRepoPruneFlags> for RepoPruneFlags {
    fn from_glib(value: ffi::OstreeRepoPruneFlags) -> Self {
        match value {
            0 => RepoPruneFlags::None,
            1 => RepoPruneFlags::NoPrune,
            2 => RepoPruneFlags::RefsOnly,
            value => RepoPruneFlags::__Unknown(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum RepoRemoteChange {
    Add,
    AddIfNotExists,
    Delete,
    DeleteIfExists,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for RepoRemoteChange {
    type GlibType = ffi::OstreeRepoRemoteChange;

    fn to_glib(&self) -> ffi::OstreeRepoRemoteChange {
        match *self {
            RepoRemoteChange::Add => ffi::OSTREE_REPO_REMOTE_CHANGE_ADD,
            RepoRemoteChange::AddIfNotExists => ffi::OSTREE_REPO_REMOTE_CHANGE_ADD_IF_NOT_EXISTS,
            RepoRemoteChange::Delete => ffi::OSTREE_REPO_REMOTE_CHANGE_DELETE,
            RepoRemoteChange::DeleteIfExists => ffi::OSTREE_REPO_REMOTE_CHANGE_DELETE_IF_EXISTS,
            RepoRemoteChange::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::OstreeRepoRemoteChange> for RepoRemoteChange {
    fn from_glib(value: ffi::OstreeRepoRemoteChange) -> Self {
        match value {
            0 => RepoRemoteChange::Add,
            1 => RepoRemoteChange::AddIfNotExists,
            2 => RepoRemoteChange::Delete,
            3 => RepoRemoteChange::DeleteIfExists,
            value => RepoRemoteChange::__Unknown(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum RepoResolveRevExtFlags {
    RepoResolveRevExtNone,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for RepoResolveRevExtFlags {
    type GlibType = ffi::OstreeRepoResolveRevExtFlags;

    fn to_glib(&self) -> ffi::OstreeRepoResolveRevExtFlags {
        match *self {
            RepoResolveRevExtFlags::RepoResolveRevExtNone => ffi::OSTREE_REPO_RESOLVE_REV_EXT_NONE,
            RepoResolveRevExtFlags::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::OstreeRepoResolveRevExtFlags> for RepoResolveRevExtFlags {
    fn from_glib(value: ffi::OstreeRepoResolveRevExtFlags) -> Self {
        match value {
            0 => RepoResolveRevExtFlags::RepoResolveRevExtNone,
            value => RepoResolveRevExtFlags::__Unknown(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum StaticDeltaGenerateOpt {
    Lowlatency,
    Major,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for StaticDeltaGenerateOpt {
    type GlibType = ffi::OstreeStaticDeltaGenerateOpt;

    fn to_glib(&self) -> ffi::OstreeStaticDeltaGenerateOpt {
        match *self {
            StaticDeltaGenerateOpt::Lowlatency => ffi::OSTREE_STATIC_DELTA_GENERATE_OPT_LOWLATENCY,
            StaticDeltaGenerateOpt::Major => ffi::OSTREE_STATIC_DELTA_GENERATE_OPT_MAJOR,
            StaticDeltaGenerateOpt::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::OstreeStaticDeltaGenerateOpt> for StaticDeltaGenerateOpt {
    fn from_glib(value: ffi::OstreeStaticDeltaGenerateOpt) -> Self {
        match value {
            0 => StaticDeltaGenerateOpt::Lowlatency,
            1 => StaticDeltaGenerateOpt::Major,
            value => StaticDeltaGenerateOpt::__Unknown(value),
        }
    }
}

