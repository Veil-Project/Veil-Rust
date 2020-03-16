// extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let mut build = cc::Build::new()
        .include("depends/secp256k1-veil/")
        .include("depends/secp256k1-veil/include")
        .include("depends/secp256k1-veil/src")
        .flag_if_supported("-Wno-unused-function") // some ecmult stuff is defined but not used upstream
        .define("SECP256K1_BUILD", Some("1"))
        // TODO these three should be changed to use libgmp, at least until secp PR 290 is merged
        .define("USE_NUM_NONE", Some("1"))
        .define("USE_FIELD_INV_BUILTIN", Some("1"))
        .define("USE_SCALAR_INV_BUILTIN", Some("1"))
        .define("ENABLE_MODULE_ECDH", Some("1"))
        .define("USE_EXTERNAL_DEFAULT_CALLBACKS", Some("1"))
        .define("ECMULT_WINDOW_SIZE", Some("15")) // This is the default in the configure file (`auto`)
        .define("USE_ENDOMORPHISM", Some("1"))
        .define("ENABLE_MODULE_RECOVERY", Some("1"))
        .define("USE_FIELD_5X52", Some("1"))
        .define("USE_SCALAR_4X64", Some("1"))
        .define("HAVE___INT128", Some("1"))
        .file("depends/secp256k1-veil/contrib/lax_der_parsing.c")
        .file("depends/secp256k1-veil/src/secp256k1.c")
        .compile("libsecp256k1-veil.a");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting ffi.
    // let bindings = bindgen::Builder::default()
    //     // The input header we would like to generate
    //     // ffi for.
    //     .header("depends/secp256k1-veil/include/secp256k1_whitelist.h")
    //     // Tell cargo to invalidate the built crate whenever any of the
    //     // included header files changed.
    //     // .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    //     // Finish the builder and generate the ffi.
    //     .generate()
    //     // Unwrap the Result and panic on failure.
    //     .expect("Unable to generate ffi");

    // Write the ffi to the $OUT_DIR/ffi.rs file.
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file(out_path.join("ffi.rs"))
    //     .expect("Couldn't write ffi!");
}
