mod macros;

mod inspect_config;
mod inspect_discover;
mod ui;

fn broca() -> snapbox::cmd::Command {
    snapbox::cmd::Command::cargo_bin("broca").with_assert(assert())
}

fn assert() -> snapbox::Assert {
    let mut redactions = snapbox::Redactions::new();
    redactions
        .insert("[EXE]", std::env::consts::EXE_SUFFIX)
        .unwrap();
    redactions
        .insert(
            "[CARGO_TARGET_TMPDIR]",
            env!("CARGO_TARGET_TMPDIR").replace(std::path::MAIN_SEPARATOR_STR, "/"),
        )
        .unwrap();
    redactions
        .insert(
            "[CARGO_MANIFEST_DIR]",
            std::env::current_dir()
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap()
                .replace(std::path::MAIN_SEPARATOR_STR, "/"),
        )
        .unwrap();

    snapbox::Assert::new()
        .action_env(snapbox::assert::DEFAULT_ACTION_ENV)
        .redact_with(redactions)
}

fn dir(fn_path: &str) -> snapbox::dir::DirRoot {
    let name = fn_path.replace("::", std::path::MAIN_SEPARATOR_STR);
    let path = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = std::fs::remove_dir_all(&path);
    std::fs::create_dir_all(&path).unwrap();
    snapbox::dir::DirRoot::mutable_at(&path).unwrap()
}
