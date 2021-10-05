

fn main() 
{
    println!("cargo:rerun-if-changed=src/cppcode.cpp");
    cc::Build::new()
        .cpp(true)
        .file("src/cppcode.cpp")
        .compile("cppprog");

    //println!("cargo:rustc-link-search=native={}", "home/andrii/code_playground/rust_hello_world");
    //println!("cargo:runtc-link-lib=static=libcppprog.a");
}