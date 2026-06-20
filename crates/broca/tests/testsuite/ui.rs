use snapbox::file;

#[test]
fn help() {
    crate::broca()
        .args(["--help"])
        .assert()
        .success()
        .stdout_eq(file![_: TermSvg]);
}
