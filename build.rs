#![deny(warnings)]

use std::process::{Command, Stdio};
use std::fs::{File};
use std::io::{Write};
use std::path::{PathBuf};
use std::env::{self};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let c_file = out_dir.join("iconv_types.c");
    let bin_file = out_dir.join("iconv_types");
    let rs_file = out_dir.join("iconv_types.rs");

    {
        let c_file_display = c_file.display();
        let mut c_file = File::create(&c_file).unwrap_or_else(|_| panic!("cannot create {}", c_file_display));
        c_file.write_all(br##"
#include <stdio.h>
#include <stdalign.h>
#include <iconv.h>
#include <limits.h>
int main() {
    printf("type IconvTRaw = i%zd;\n", sizeof(iconv_t) * CHAR_BIT);
    return 0;
}
"##).unwrap_or_else(|_| panic!("cannot write {}", c_file_display));
    }

    let build = cc::Build::new();
    let mut compiler = build.try_get_compiler().unwrap().to_command();
    compiler.arg("-o").arg(&bin_file).arg(&c_file);
    let compiler_status = compiler.stdin(Stdio::null()).status()
        .unwrap_or_else(|_| panic!("cannot compile {}", c_file.display()));
    if !compiler_status.success() {
        panic!("{} compilation failed with non-zero {}", c_file.display(), compiler_status);
    }
    let rs_file = File::create(&rs_file)
        .unwrap_or_else(|_| panic!("cannot create {}", rs_file.display()));
    Command::new(&bin_file).stdin(Stdio::null()).stdout(rs_file).status()
        .unwrap_or_else(|_| panic!("{} failed", bin_file.display()));
}

