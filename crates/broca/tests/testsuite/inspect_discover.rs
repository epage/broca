use snapbox::prelude::*;
use snapbox::str;

#[test]
fn config_dir() {
    let fixture = crate::dir(crate::fn_path!())
        .with_template(&[("broca.toml", ""), ("index.md", "")])
        .unwrap();

    crate::broca()
        .args(["inspect", "discover"])
        .current_dir(fixture.path().unwrap())
        .assert()
        .success()
        .stdout_eq(
            str![[r#"
[
  {
    "config": "[CARGO_TARGET_TMPDIR]/testsuite/inspect_discover/config_dir/broca.toml"
  }
]
"#]]
            .is_json()
            .against_jsonlines(),
        )
        .stderr_eq(str![]);

    fixture.close().unwrap();
}

#[test]
fn config_child_dir() {
    let fixture = crate::dir(crate::fn_path!())
        .with_template(&[("broca.toml", ""), ("foo/bar/index.md", "")])
        .unwrap();

    crate::broca()
        .args(["inspect", "discover"])
        .current_dir(fixture.path().unwrap().join("foo/bar"))
        .assert()
        .success()
        .stdout_eq(
            str![[r#"
[
  {
    "config": "[CARGO_TARGET_TMPDIR]/testsuite/inspect_discover/config_child_dir/broca.toml"
  }
]
"#]]
            .is_json()
            .against_jsonlines(),
        )
        .stderr_eq(str![]);

    fixture.close().unwrap();
}

#[test]
fn git_child_dir() {
    let fixture = crate::dir(crate::fn_path!())
        .with_template(&[(".git/file", ""), ("foo/bar/index.md", "")])
        .unwrap();

    crate::broca()
        .args(["inspect", "discover"])
        .current_dir(fixture.path().unwrap().join("foo/bar"))
        .assert()
        .success()
        .stdout_eq(
            str![[r#"
[
  {
    "root": "[CARGO_TARGET_TMPDIR]/testsuite/inspect_discover/git_child_dir"
  }
]
"#]]
            .is_json()
            .against_jsonlines(),
        )
        .stderr_eq(str![]);

    fixture.close().unwrap();
}
