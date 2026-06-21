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
fn inspect_discover_help() {
    crate::broca()
        .args(["inspect", "discover", "--help"])
        .assert()
        .success()
        .stdout_eq(file![_: TermSvg]);
}
