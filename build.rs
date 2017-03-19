use std::fs;
use std::env;

fn main() {
    if debug_tesselator_flag_on() {
        let mut cwd = env::current_dir().unwrap();
        cwd.push("target");
        cwd.push("debug");
        cwd.push("images");

        let result = fs::create_dir_all(cwd);
        match result {
            Ok(_) => println!("/target/debug/images/ created"),
            Err(_) => panic!("/target/debug/images/ not created"),
        }
    }
}

fn debug_tesselator_flag_on() -> bool {
    match env::var("CARGO_FEATURE_DEBUG_TESSELATOR") {
        Ok(_) => true,
        Err(_) => false
    }
}
