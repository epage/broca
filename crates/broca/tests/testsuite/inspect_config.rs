use snapbox::prelude::*;
use snapbox::str;

#[test]
fn empty() {
    let fixture = crate::dir(crate::fn_path!())
        .with_template(&[("broca.toml", ""), ("index.md", "")])
        .unwrap();

    crate::broca()
        .args(["inspect", "config"])
        .current_dir(fixture.path().unwrap())
        .assert()
        .success()
        .stdout_eq(
            str![[r#"
[
  {}
]
"#]]
            .is_json()
            .against_jsonlines(),
        )
        .stderr_eq(str![]);

    fixture.close().unwrap();
}

#[test]
fn unused() {
    let fixture = crate::dir(crate::fn_path!())
        .with_template(&[("broca.toml", "unused = 10"), ("index.md", "")])
        .unwrap();

    crate::broca()
        .args(["inspect", "config"])
        .current_dir(fixture.path().unwrap())
        .assert()
        .success()
        .stdout_eq(
            str![[r#"
[
  {}
]
"#]]
            .is_json()
            .against_jsonlines(),
        )
        .stderr_eq(str![[r#"
warning: unused field `unused`
 --> [CARGO_TARGET_TMPDIR]/testsuite/inspect_config/unused/broca.toml

"#]]);

    fixture.close().unwrap();
}
