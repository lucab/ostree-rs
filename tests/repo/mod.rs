use crate::util::*;
use ostree::gio::NONE_CANCELLABLE;
use ostree::prelude::*;
use ostree::{ObjectName, ObjectType};

#[cfg(feature = "v2016_8")]
mod checkout_at;

#[test]
fn should_commit_content_to_repo_and_list_refs_again() {
    let test_repo = TestRepo::new();

    let mtree = create_mtree(&test_repo.repo);
    let checksum = commit(&test_repo.repo, &mtree, "test");

    let repo = ostree::Repo::new_for_path(test_repo.dir.path());
    repo.open(NONE_CANCELLABLE).expect("OSTree test_repo");
    let refs = repo
        .list_refs(None, NONE_CANCELLABLE)
        .expect("failed to list refs");
    assert_eq!(1, refs.len());
    assert_eq!(checksum, refs["test"]);
}

#[test]
fn should_traverse_commit() {
    let test_repo = TestRepo::new();
    let checksum = test_repo.test_commit("test");

    let objects = test_repo
        .repo
        .traverse_commit(&checksum, -1, NONE_CANCELLABLE)
        .expect("traverse commit");

    assert_eq!(
        maplit::hashset!(
            ObjectName::new(
                "89f84ca9854a80e85b583e46a115ba4985254437027bad34f0b113219323d3f8",
                ObjectType::File
            ),
            ObjectName::new(
                "5280a884f930cae329e2e39d52f2c8e910c2ef4733216b67679db32a2b56c4db",
                ObjectType::DirTree
            ),
            ObjectName::new(
                "c81acde323d73f8639fc84f1ded17bbafc415e645f845e9f3b16a4906857c2d4",
                ObjectType::DirTree
            ),
            ObjectName::new(
                "ad49a0f4e3bc165361b6d17e8a865d479b373ee67d89ac6f0ce871f27da1be6d",
                ObjectType::DirMeta
            ),
            ObjectName::new(checksum, ObjectType::Commit)
        ),
        objects
    );
}

#[test]
fn should_checkout_tree() {
    let test_repo = TestRepo::new();
    let _ = test_repo.test_commit("test");

    let checkout_dir = tempfile::tempdir().expect("checkout dir");
    let file = test_repo
        .repo
        .read_commit("test", NONE_CANCELLABLE)
        .expect("read commit")
        .0
        .downcast::<ostree::RepoFile>()
        .expect("RepoFile");
    let info = file
        .query_info("*", gio::FileQueryInfoFlags::NONE, NONE_CANCELLABLE)
        .expect("file info");
    test_repo
        .repo
        .checkout_tree(
            ostree::RepoCheckoutMode::User,
            ostree::RepoCheckoutOverwriteMode::None,
            &gio::File::for_path(checkout_dir.path().join("test-checkout")),
            &file,
            &info,
            NONE_CANCELLABLE,
        )
        .expect("checkout tree");

    assert_test_file(checkout_dir.path());
}

#[test]
fn should_write_content_to_repo() {
    let src = TestRepo::new();
    let mtree = create_mtree(&src.repo);
    let checksum = commit(&src.repo, &mtree, "test");

    let dest = TestRepo::new();
    let objects = src
        .repo
        .traverse_commit(&checksum, -1, NONE_CANCELLABLE)
        .expect("traverse");
    for obj in objects {
        match obj.object_type() {
            ObjectType::File => copy_file(&src, &dest, &obj),
            _ => copy_metadata(&src, &dest, &obj),
        }
    }
}

fn copy_file(src: &TestRepo, dest: &TestRepo, obj: &ObjectName) {
    let (stream, len) = src
        .repo
        .load_object_stream(obj.object_type(), obj.checksum(), NONE_CANCELLABLE)
        .expect("load object stream");
    let out_csum = dest
        .repo
        .write_content(None, &stream, len, NONE_CANCELLABLE)
        .expect("write content");
    assert_eq!(out_csum.to_string(), obj.checksum());
}

fn copy_metadata(src: &TestRepo, dest: &TestRepo, obj: &ObjectName) {
    let data = src
        .repo
        .load_variant(obj.object_type(), obj.checksum())
        .expect("load variant");
    let out_csum = dest
        .repo
        .write_metadata(obj.object_type(), None, &data, NONE_CANCELLABLE)
        .expect("write metadata");
    assert_eq!(out_csum.to_string(), obj.checksum());
}
