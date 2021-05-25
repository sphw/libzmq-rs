use std::env;

pub fn configure() {
    println!("cargo:rerun-if-changed=build/main.rs");
    println!("cargo:rerun-if-env-changed=PROFILE");

    let wants_debug = env::var("PROFILE").unwrap() == "debug";

    let enable_curve = cfg!(feature = "curve");

    let wants_debug = env::var_os("PROFILE").unwrap() == "debug";

    let maybe_libsodium = if cfg!(feature = "libsodium") {
        let lib_dir = env::var("DEP_SODIUM_LIB")
            .expect("build metadata `DEP_SODIUM_LIB` required");
        let include_dir = env::var("DEP_SODIUM_INCLUDE")
            .expect("build metadata `DEP_SODIUM_INCLUDE` required");

        Some(zeromq_src::LibLocation::new(lib_dir, include_dir))
    } else {
        None
    };

    let artifacts = zeromq_src::Build::new()
        .link_static(true)
        .enable_draft(true)
        .enable_curve(enable_curve)
        .build_debug(wants_debug)
        .with_libsodium(maybe_libsodium)
        .build();

    artifacts.print_cargo_metadata();
}
