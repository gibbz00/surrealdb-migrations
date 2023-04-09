use assert_cmd::Command;
use serial_test::serial;

use crate::helpers;

#[test]
#[serial]
fn create_schema_file() {
    helpers::clear_files_dir();

    {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

        cmd.arg("scaffold").arg("empty");

        cmd.assert().success();
    }

    {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

        cmd.arg("create")
            .arg("schema")
            .arg("post")
            .arg("-f")
            .arg("name,title,published_at");

        cmd.assert().success();
    }

    let post_file = std::fs::read_to_string("tests-files/schemas/post.surql").unwrap();

    assert_eq!(
        post_file,
        "DEFINE TABLE post SCHEMALESS;

DEFINE FIELD name ON post;
DEFINE FIELD title ON post;
DEFINE FIELD published_at ON post;"
    );
}

#[test]
#[serial]
fn create_event_file() {
    helpers::clear_files_dir();

    {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

        cmd.arg("scaffold").arg("empty");

        cmd.assert().success();
    }

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    {
        cmd.arg("create")
            .arg("event")
            .arg("publish_post")
            .arg("-f")
            .arg("post_id,created_at");

        cmd.assert().success();
    }

    let publish_post_file =
        std::fs::read_to_string("tests-files/events/publish_post.surql").unwrap();

    assert_eq!(
        publish_post_file,
        "DEFINE TABLE publish_post SCHEMALESS;

DEFINE FIELD post_id ON publish_post;
DEFINE FIELD created_at ON publish_post;

DEFINE EVENT publish_post ON TABLE publish_post WHEN $before == NONE THEN (
    # TODO
);",
    );
}

#[test]
#[serial]
fn create_migration_file() {
    helpers::clear_files_dir();

    {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

        cmd.arg("scaffold").arg("empty");

        cmd.assert().success();
    }

    {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

        cmd.arg("create").arg("migration").arg("AddPost");

        cmd.assert().success();
    }

    let migrations_folder = std::fs::read_dir("tests-files/migrations").unwrap();

    assert_eq!(migrations_folder.count(), 1);
}

#[test]
#[serial]
fn create_schema_file_dry_run() {
    helpers::clear_files_dir();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    cmd.arg("create")
        .arg("schema")
        .arg("post")
        .arg("-f")
        .arg("name,title,published_at")
        .arg("--dry-run");

    cmd.assert().success().stdout(
        "DEFINE TABLE post SCHEMALESS;

DEFINE FIELD name ON post;
DEFINE FIELD title ON post;
DEFINE FIELD published_at ON post;\n",
    );
}

#[test]
#[serial]
fn create_event_file_dry_run() {
    helpers::clear_files_dir();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    cmd.arg("create")
        .arg("event")
        .arg("publish_post")
        .arg("-f")
        .arg("post_id,created_at")
        .arg("--dry-run");

    cmd.assert().success().stdout(
        "DEFINE TABLE publish_post SCHEMALESS;

DEFINE FIELD post_id ON publish_post;
DEFINE FIELD created_at ON publish_post;

DEFINE EVENT publish_post ON TABLE publish_post WHEN $before == NONE THEN (
    # TODO
);\n",
    );
}
