// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use glib::StaticType;
use glib::Type;
use gobject_sys;
use ostree_sys;

#[cfg(any(feature = "v2017_13", feature = "dox"))]
bitflags! {
    pub struct ChecksumFlags: u32 {
        const NONE = 0;
        const IGNORE_XATTRS = 1;
    }
}

#[cfg(any(feature = "v2017_13", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for ChecksumFlags {
    type GlibType = ostree_sys::OstreeChecksumFlags;

    fn to_glib(&self) -> ostree_sys::OstreeChecksumFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v2017_13", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<ostree_sys::OstreeChecksumFlags> for ChecksumFlags {
    fn from_glib(value: ostree_sys::OstreeChecksumFlags) -> ChecksumFlags {
        ChecksumFlags::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct DiffFlags: u32 {
        const NONE = 0;
        const IGNORE_XATTRS = 1;
    }
}

#[doc(hidden)]
impl ToGlib for DiffFlags {
    type GlibType = ostree_sys::OstreeDiffFlags;

    fn to_glib(&self) -> ostree_sys::OstreeDiffFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ostree_sys::OstreeDiffFlags> for DiffFlags {
    fn from_glib(value: ostree_sys::OstreeDiffFlags) -> DiffFlags {
        DiffFlags::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct RepoCommitModifierFlags: u32 {
        const NONE = 0;
        const SKIP_XATTRS = 1;
        const GENERATE_SIZES = 2;
        const CANONICAL_PERMISSIONS = 4;
        const ERROR_ON_UNLABELED = 8;
        const CONSUME = 16;
        const DEVINO_CANONICAL = 32;
    }
}

#[doc(hidden)]
impl ToGlib for RepoCommitModifierFlags {
    type GlibType = ostree_sys::OstreeRepoCommitModifierFlags;

    fn to_glib(&self) -> ostree_sys::OstreeRepoCommitModifierFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ostree_sys::OstreeRepoCommitModifierFlags> for RepoCommitModifierFlags {
    fn from_glib(value: ostree_sys::OstreeRepoCommitModifierFlags) -> RepoCommitModifierFlags {
        RepoCommitModifierFlags::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v2015_7", feature = "dox"))]
bitflags! {
    pub struct RepoCommitState: u32 {
        const NORMAL = 0;
        const PARTIAL = 1;
        const FSCK_PARTIAL = 2;
    }
}

#[cfg(any(feature = "v2015_7", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for RepoCommitState {
    type GlibType = ostree_sys::OstreeRepoCommitState;

    fn to_glib(&self) -> ostree_sys::OstreeRepoCommitState {
        self.bits()
    }
}

#[cfg(any(feature = "v2015_7", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<ostree_sys::OstreeRepoCommitState> for RepoCommitState {
    fn from_glib(value: ostree_sys::OstreeRepoCommitState) -> RepoCommitState {
        RepoCommitState::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct RepoCommitTraverseFlags: u32 {
        const REPO_COMMIT_TRAVERSE_FLAG_NONE = 1;
    }
}

#[doc(hidden)]
impl ToGlib for RepoCommitTraverseFlags {
    type GlibType = ostree_sys::OstreeRepoCommitTraverseFlags;

    fn to_glib(&self) -> ostree_sys::OstreeRepoCommitTraverseFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ostree_sys::OstreeRepoCommitTraverseFlags> for RepoCommitTraverseFlags {
    fn from_glib(value: ostree_sys::OstreeRepoCommitTraverseFlags) -> RepoCommitTraverseFlags {
        RepoCommitTraverseFlags::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct RepoListObjectsFlags: u32 {
        const LOOSE = 1;
        const PACKED = 2;
        const ALL = 4;
        const NO_PARENTS = 8;
    }
}

#[doc(hidden)]
impl ToGlib for RepoListObjectsFlags {
    type GlibType = ostree_sys::OstreeRepoListObjectsFlags;

    fn to_glib(&self) -> ostree_sys::OstreeRepoListObjectsFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ostree_sys::OstreeRepoListObjectsFlags> for RepoListObjectsFlags {
    fn from_glib(value: ostree_sys::OstreeRepoListObjectsFlags) -> RepoListObjectsFlags {
        RepoListObjectsFlags::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct RepoListRefsExtFlags: u32 {
        const NONE = 0;
        const ALIASES = 1;
        const EXCLUDE_REMOTES = 2;
        const EXCLUDE_MIRRORS = 4;
    }
}

#[doc(hidden)]
impl ToGlib for RepoListRefsExtFlags {
    type GlibType = ostree_sys::OstreeRepoListRefsExtFlags;

    fn to_glib(&self) -> ostree_sys::OstreeRepoListRefsExtFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ostree_sys::OstreeRepoListRefsExtFlags> for RepoListRefsExtFlags {
    fn from_glib(value: ostree_sys::OstreeRepoListRefsExtFlags) -> RepoListRefsExtFlags {
        RepoListRefsExtFlags::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct RepoPullFlags: u32 {
        const NONE = 0;
        const MIRROR = 1;
        const COMMIT_ONLY = 2;
        const UNTRUSTED = 4;
        const BAREUSERONLY_FILES = 8;
        const TRUSTED_HTTP = 16;
    }
}

#[doc(hidden)]
impl ToGlib for RepoPullFlags {
    type GlibType = ostree_sys::OstreeRepoPullFlags;

    fn to_glib(&self) -> ostree_sys::OstreeRepoPullFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ostree_sys::OstreeRepoPullFlags> for RepoPullFlags {
    fn from_glib(value: ostree_sys::OstreeRepoPullFlags) -> RepoPullFlags {
        RepoPullFlags::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct RepoResolveRevExtFlags: u32 {
        const NONE = 0;
        const LOCAL_ONLY = 1;
    }
}

#[doc(hidden)]
impl ToGlib for RepoResolveRevExtFlags {
    type GlibType = ostree_sys::OstreeRepoResolveRevExtFlags;

    fn to_glib(&self) -> ostree_sys::OstreeRepoResolveRevExtFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ostree_sys::OstreeRepoResolveRevExtFlags> for RepoResolveRevExtFlags {
    fn from_glib(value: ostree_sys::OstreeRepoResolveRevExtFlags) -> RepoResolveRevExtFlags {
        RepoResolveRevExtFlags::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct SePolicyRestoreconFlags: u32 {
        const NONE = 0;
        const ALLOW_NOLABEL = 1;
        const KEEP_EXISTING = 2;
    }
}

#[doc(hidden)]
impl ToGlib for SePolicyRestoreconFlags {
    type GlibType = ostree_sys::OstreeSePolicyRestoreconFlags;

    fn to_glib(&self) -> ostree_sys::OstreeSePolicyRestoreconFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ostree_sys::OstreeSePolicyRestoreconFlags> for SePolicyRestoreconFlags {
    fn from_glib(value: ostree_sys::OstreeSePolicyRestoreconFlags) -> SePolicyRestoreconFlags {
        SePolicyRestoreconFlags::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct SysrootSimpleWriteDeploymentFlags: u32 {
        const NONE = 0;
        const RETAIN = 1;
        const NOT_DEFAULT = 2;
        const NO_CLEAN = 4;
        const RETAIN_PENDING = 8;
        const RETAIN_ROLLBACK = 16;
    }
}

#[doc(hidden)]
impl ToGlib for SysrootSimpleWriteDeploymentFlags {
    type GlibType = ostree_sys::OstreeSysrootSimpleWriteDeploymentFlags;

    fn to_glib(&self) -> ostree_sys::OstreeSysrootSimpleWriteDeploymentFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ostree_sys::OstreeSysrootSimpleWriteDeploymentFlags> for SysrootSimpleWriteDeploymentFlags {
    fn from_glib(value: ostree_sys::OstreeSysrootSimpleWriteDeploymentFlags) -> SysrootSimpleWriteDeploymentFlags {
        SysrootSimpleWriteDeploymentFlags::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct SysrootUpgraderFlags: u32 {
        const IGNORE_UNCONFIGURED = 2;
    }
}

#[doc(hidden)]
impl ToGlib for SysrootUpgraderFlags {
    type GlibType = ostree_sys::OstreeSysrootUpgraderFlags;

    fn to_glib(&self) -> ostree_sys::OstreeSysrootUpgraderFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ostree_sys::OstreeSysrootUpgraderFlags> for SysrootUpgraderFlags {
    fn from_glib(value: ostree_sys::OstreeSysrootUpgraderFlags) -> SysrootUpgraderFlags {
        SysrootUpgraderFlags::from_bits_truncate(value)
    }
}

impl StaticType for SysrootUpgraderFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ostree_sys::ostree_sysroot_upgrader_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for SysrootUpgraderFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for SysrootUpgraderFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for SysrootUpgraderFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct SysrootUpgraderPullFlags: u32 {
        const NONE = 0;
        const ALLOW_OLDER = 1;
        const SYNTHETIC = 2;
    }
}

#[doc(hidden)]
impl ToGlib for SysrootUpgraderPullFlags {
    type GlibType = ostree_sys::OstreeSysrootUpgraderPullFlags;

    fn to_glib(&self) -> ostree_sys::OstreeSysrootUpgraderPullFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ostree_sys::OstreeSysrootUpgraderPullFlags> for SysrootUpgraderPullFlags {
    fn from_glib(value: ostree_sys::OstreeSysrootUpgraderPullFlags) -> SysrootUpgraderPullFlags {
        SysrootUpgraderPullFlags::from_bits_truncate(value)
    }
}

