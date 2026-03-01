use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-env-changed=LCDHAT_BACKEND");
    println!("cargo:rerun-if-env-changed=LCDHAT_SKIP_BINDGEN");

    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR missing"));
    let workspace_root = manifest_dir
        .parent()
        .and_then(Path::parent)
        .expect("Failed to resolve workspace root");

    let lcdhat_root = workspace_root.join("vendor/lcdhat");
    let header = lcdhat_root.join("include/lcdhat/lcdhat.h");
    let cmakelists = lcdhat_root.join("CMakeLists.txt");

    println!("cargo:rerun-if-changed={}", header.display());
    println!("cargo:rerun-if-changed={}", cmakelists.display());
    println!(
        "cargo:rerun-if-changed={}",
        lcdhat_root.join("src").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        lcdhat_root.join("lib").display()
    );

    if !header.exists() {
        panic!("lcdhat header not found: {}", header.display());
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR missing"));
    let generated = out_dir.join("lcdhat_bindings.rs");

    copy_pregenerated(&manifest_dir, &generated);

    if cfg!(target_os = "linux") {
        build_and_link_lcdhat(&lcdhat_root, &out_dir);
    }
}

fn copy_pregenerated(manifest_dir: &Path, generated: &Path) {
    let source = manifest_dir.join("src/bindings_pregenerated.rs");
    fs::copy(&source, generated).unwrap_or_else(|err| {
        panic!(
            "Failed to copy pregenerated bindings from {} to {}: {err}",
            source.display(),
            generated.display(),
        )
    });
}

fn build_and_link_lcdhat(lcdhat_root: &Path, out_dir: &Path) {
    let backend = env::var("LCDHAT_BACKEND").unwrap_or_else(|_| "DEV".to_string());
    if !matches!(backend.as_str(), "DEV" | "WIRINGPI" | "BCM2835") {
        panic!("Unsupported LCDHAT_BACKEND={backend}. Expected DEV|WIRINGPI|BCM2835");
    }

    let cmake_build_dir = out_dir.join("cmake-build");
    let cmake_install_dir = out_dir.join("cmake-install");

    run(
        Command::new("cmake")
            .arg("-S")
            .arg(lcdhat_root)
            .arg("-B")
            .arg(&cmake_build_dir)
            .arg("-DCMAKE_BUILD_TYPE=Release")
            .arg(format!(
                "-DCMAKE_INSTALL_PREFIX={}",
                cmake_install_dir.display()
            ))
            .arg(format!("-DLCDHAT_BACKEND={backend}")),
        "configure lcdhat cmake",
    );

    run(
        Command::new("cmake")
            .arg("--build")
            .arg(&cmake_build_dir)
            .arg("--target")
            .arg("install")
            .arg("--config")
            .arg("Release"),
        "build/install lcdhat",
    );

    let mut search_dirs = vec![
        cmake_install_dir.clone(),
        cmake_install_dir.join("lib"),
        cmake_install_dir.join("lib64"),
        cmake_build_dir.clone(),
    ];
    search_dirs.retain(|p| p.exists());

    for dir in &search_dirs {
        println!("cargo:rustc-link-search=native={}", dir.display());
    }

    println!("cargo:rustc-link-lib=dylib=lcdhat");

    if let Some(runtime_dir) = search_dirs
        .iter()
        .find(|p| p.join("liblcdhat.so").exists())
        .or_else(|| {
            search_dirs
                .iter()
                .find(|p| p.join("liblcdhat.dylib").exists())
        })
    {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", runtime_dir.display());
    }
}

fn run(cmd: &mut Command, label: &str) {
    let output = cmd.output().unwrap_or_else(|err| {
        panic!("Failed to execute command ({label}): {err}");
    });

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!(
            "Command failed ({label}) with status {}.\nstdout:\n{}\nstderr:\n{}",
            output.status, stdout, stderr
        );
    }
}
