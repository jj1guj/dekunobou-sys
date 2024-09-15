use cmake;
use std::process::Command;

fn main() {
    let status = Command::new("git")
        .args(&["submodule", "update", "--init", "--recursive"])
        .status()
        .expect("Failed to updateb submodules");

    if !status.success() {
        panic!("Submodule initialization failed");
    }

    let dst = cmake::build("libdekunobou");
    println!("cargo:rustc-link-search=native={}", dst.display());

    println!("cargo:rustc-link-lib=static=dekunobou");
    println!("cargo:rustc-link-lib=gomp");

    // C++ソースコードの場合は必ずこれを追加すること
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
