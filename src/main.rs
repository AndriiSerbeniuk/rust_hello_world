#![allow(non_upper_case_globals)]                                                                                  
#![allow(non_camel_case_types)]                                                                                    
#![allow(improper_ctypes)]                                                                                         
#![allow(non_snake_case)]                                                                                          
#![allow(unused)]

extern "C"
{
    pub fn hello();
}

fn main() {
    let cpp_value = 0;
    unsafe { hello() }
    println!("cpp code returned value: {}", cpp_value);
}
