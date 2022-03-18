// to deploy this, first cargo build --release then:
// cp pyext-myrustlib/target/release/libmyrustlib.so myrustlib.so

#[macro_use]
extern crate cpython;

use cpython::{Python, PyResult};

fn count_doubles(_py: Python, val: &str) -> PyResult<u64> {
    
    let mut total = 0u64;

    for (c1, c2) in val.chars().zip(val.chars().skip(1)) {
        if c1 == c2 {
            total += 1;
        }
    }

    Ok(total)
}

fn rust_print(_py: Python, val: &str) -> PyResult<String> {
    let ret = String::from(val);

    Ok(ret)
}

fn big_multiply(_py: Python, a: u64, b: u64) -> PyResult<u64> {
    let result = a*b;

    Ok(result)
}

py_module_initializer!(libmyrustlib, initlibmyrustlib, PyInit_myrustlib, |py, m | {
    try!(m.add(py, "__doc__", "A Python module written in rust - this module provides the function 'count_doubles' which counts the appearance of double letters without regex."));
    try!(m.add(py, "count_doubles", py_fn!(py, count_doubles(val: &str))));
    try!(m.add(py, "rust_print", py_fn!(py, rust_print(val: &str))));
    try!(m.add(py, "big_multiply", py_fn!(py, big_multiply(a: u64, b: u64))));
    Ok(())
});