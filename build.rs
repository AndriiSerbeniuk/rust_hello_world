extern crate bindgen;
// i also installed clang, as per tutorial's instructions
// my clang version is 10.0.0-4ubuntu1

fn main() 
{
    println!("cargo:rerun-if-changed=src/wrapper.hpp");
    
    cc::Build::new()
        .cpp(true)
        .file("src/cppcode.cpp")
        .compile("cppprog");

    // cc crate compiles the library and links it to the progect automatically
    // so we can just move straight to binding to it
    // the code below was straight copy-pasted from official tutorial (except the paths, of course)
    let bindings = bindgen::Builder::default()
        .header("src/wrapper.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    
    bindings.write_to_file("src/cpp_func.rs")
        .expect("Couldn't write the bindings");
    
    // didn't need these
    //println!("cargo:rustc-link-search=native={}", "home/andrii/code_playground/rust_hello_world");
    //println!("cargo:runtc-link-lib=static=libcppprog.a");
}