use snapbox::file;

#[test]
fn help() {
    crate::broca()
        .args(["--help"])
        .assert()
        .success()
        .stdout_eq(file![_: TermSvg]);
}

#[test]
fn inspect_help() {
    crate::broca()
        .args(["inspect", "--help"])
        .assert()
        .success()
        .stdout_eq(file![_: TermSvg]);
}

#[test]
fn inspect_root_help() {
    crate::broca()
        .args(["inspect", "root", "--help"])
        .assert()
        .success()
        .stdout_eq(file![_: TermSvg]);
}
