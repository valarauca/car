extern crate gcc;
extern crate filetime;

use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{PathBuf, Path};
use std::process::Command;

use filetime::FileTime;

macro_rules! t {
    ($e:expr) => (match $e {
        Ok(t) => t,
        Err(e) => panic!("{} return the error {}", stringify!($e), e),
    })
}

fn main() {
    let target = env::var("TARGET").unwrap();
    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let src = t!(env::current_dir());

    println!("cargo:rustc-link-search={}/lib", dst.display());
    println!("cargo:root={}", dst.display());
    println!("cargo:include={}/include", dst.display());
    println!("cargo:rerun-if-changed=xz-5.2.2/configure");

    if target.contains("msvc") {
        println!("cargo:rustc-link-lib=static=liblzma");
        let mut msbuild = gcc::windows_registry::find(&target, "msbuild")
                              .expect("needs msbuild installed");
        let build = dst.join("build");
        let _ = fs::remove_dir_all(&build);
        let _ = fs::remove_dir_all(dst.join("lib"));
        let _ = fs::remove_dir_all(dst.join("include"));
        cp_r(Path::new("xz-5.2.2"), &build);

        run(msbuild.current_dir(build.join("windows"))
                   .arg("liblzma.vcxproj")
                   .arg("/p:Configuration=Release"));
        t!(fs::create_dir(dst.join("lib")));
        t!(fs::create_dir(dst.join("include")));
        let platform = if target.contains("x86_64") {"X64"} else {"Win32"};
        t!(fs::copy(build.join("windows/Release")
                         .join(platform)
                         .join("liblzma/liblzma.lib"),
                    dst.join("lib/liblzma.lib")));
        t!(fs::copy(build.join("src/liblzma/api/lzma.h"),
                    dst.join("include/lzma.h")));
        cp_r(&build.join("src/liblzma/api/lzma"), &dst.join("include/lzma"));
    } else {
        // Looks like xz-5.2.2's build system is super sensitive to mtimes, so
        // if we just blindly use what's on the filesystem it's likely to try to
        // run tons of automake junk or modify files in the build directory,
        // neither of which we want.
        //
        // Work around this by just touching every file to the same time.
        let meta = t!(Path::new("xz-5.2.2/configure").metadata());
        let now = FileTime::from_last_modification_time(&meta);
        set_all_mtime(Path::new("xz-5.2.2"), &now);

        println!("cargo:rustc-link-lib=static=lzma");
        let cfg = gcc::Config::new();
        let compiler = cfg.get_compiler();

        let _ = fs::create_dir(&dst.join("build"));

        let mut cmd = Command::new("sh");
        let mut cflags = OsString::new();
        for arg in compiler.args() {
            cflags.push(arg);
            cflags.push(" ");
        }
        cmd.env("CC", compiler.path())
           .env("CFLAGS", cflags)
           .current_dir(&dst.join("build"))
           .arg(src.join("xz-5.2.2/configure").to_str().unwrap()
                   .replace("C:\\", "/c/")
                   .replace("\\", "/"));
        cmd.arg(format!("--prefix={}", sanitize_sh(&dst)));
        cmd.arg("--disable-doc");
        cmd.arg("--disable-lzma-links");
        cmd.arg("--disable-lzmainfo");
        cmd.arg("--disable-lzmadec");
        cmd.arg("--disable-xz");
        cmd.arg("--disable-xzdec");
        cmd.arg("--disable-scripts");
        cmd.arg("--disable-shared");
        cmd.arg("--disable-nls");
        cmd.arg("--disable-rpath");

        if target.contains("windows") {
            cmd.arg("--enable-threads=win95");
        } else {
            cmd.arg("--enable-threads=yes");
        }

        run(&mut cmd);
        run(Command::new("make")
                    .arg(&format!("-j{}", env::var("NUM_JOBS").unwrap()))
                    .current_dir(&dst.join("build")));
        run(Command::new("make")
                    .arg("install")
                    .current_dir(&dst.join("build/src/liblzma")));
    }
}

fn run(cmd: &mut Command) {
    println!("running: {:?}", cmd);
    assert!(t!(cmd.status()).success());
}

fn cp_r(src: &Path, dst: &Path) {
    t!(fs::create_dir(dst));
    for e in t!(src.read_dir()).map(|e| t!(e)) {
        let src = e.path();
        let dst = dst.join(e.file_name());
        if t!(e.file_type()).is_dir() {
            cp_r(&src, &dst);
        } else {
            t!(fs::copy(&src, &dst));
        }
    }
}

fn set_all_mtime(path: &Path, mtime: &FileTime) {
    for e in t!(path.read_dir()).map(|e| t!(e)) {
        let path = e.path();
        if t!(e.file_type()).is_dir() {
            set_all_mtime(&path, mtime);
        } else {
            t!(filetime::set_file_times(&path, *mtime, *mtime));
        }
    }
}

fn sanitize_sh(path: &Path) -> String {
    let path = path.to_str().unwrap().replace("\\", "/");
    return change_drive(&path).unwrap_or(path);

    fn change_drive(s: &str) -> Option<String> {
        let mut ch = s.chars();
        let drive = ch.next().unwrap_or('C');
        if ch.next() != Some(':') {
            return None
        }
        if ch.next() != Some('/') {
            return None
        }
        Some(format!("/{}/{}", drive, &s[drive.len_utf8() + 2..]))
    }
}
