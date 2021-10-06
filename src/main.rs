// #![allow(non_upper_case_globals)]                                                                                  
// #![allow(non_camel_case_types)]                                                                                    
// #![allow(improper_ctypes)]                                                                                         
// #![allow(non_snake_case)]                                                                                          
// #![allow(unused)]
// I don't know what these are so I didn't include them.
// I'll have to ask what these are later

mod cpp_func;   // Generated bindings are here

fn main() {
    // Usage of lib. Works on my machine
    unsafe 
    {
        cpp_func::hello();
        let cpp_struct =  cpp_func::pleasework { a: 15, b: 69.228 };
        println!("cpp struct vals: a = {}, b = {}", cpp_struct.a, cpp_struct.b);
    }
    
}
