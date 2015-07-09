#![feature(libc)]
#![feature(plugin)]
#![plugin(interpolate_idents)]

#[macro_use] extern crate cpython;


use cpython::{PythonObject, Python, PyDict, NoArgs, PyTuple, PyString, PyFrame, ObjectProtocol, PyObject, PyResult};

extern crate libc;

use libc::c_char;
use std::ffi::CStr;
use std::str;


py_module_initializer!(_snakemeter, |_py, m| {
    try!(m.add("__doc__", "Module documentation string"));
    try!(m.add("print_version", py_fn!(print_version)));
    try!(m.add("print_stacktrace", py_fn!(print_stacktrace)));
    Ok(())
});

pub fn print_version<'p>(py: Python<'p>, args: &PyTuple<'p>) -> PyResult<'p, PyObject<'p>> {
    let sys = py.import("sys").unwrap();
    let version: String = sys.get("version").unwrap().extract().unwrap();
    println!("Hello Python {}", version);
    Ok(py.None())
}


pub fn print_stacktrace<'p>(py: Python<'p>, args: &PyTuple<'p>) -> PyResult<'p, PyObject<'p>> {
    let sys = py.import("sys").unwrap();
    let frames_dict: PyDict = sys.call("_current_frames", NoArgs, None).unwrap().extract().unwrap();
    let frames = frames_dict.items();


    for x in frames.into_iter() {
        let tuple = unsafe {x.unchecked_cast_into::<PyTuple>()};
        let key = tuple.get_item(0);
//        let value = tuple.get_item(1).unchecked_cast_into::<PyFrame>();
        let value = tuple.get_item(1);

        println!("key = {}", key);
        println!("value = {}", value);
        println!("lineno = {}", value.getattr("f_lineno").unwrap());
        let mut value:Option<PyObject> = Some(value);

        loop {

            match value {
                Some(frame) => {
                    println!("{}", frame);

                    let code = frame.getattr("f_code").unwrap();
                    println!("{}:{} {}", code.getattr("co_filename").unwrap(), frame.getattr("f_lineno").unwrap(),
                    code.getattr("co_name").unwrap());

                    match frame.getattr("f_back") {
                            Ok(f) => if f ==  { value = None } else {value = Some(f)},
                            Err(err) => {err.print(); value = None }
                        };

                },
                None => break

                }
        }



    }

    println!("Dict size = {}", frames_dict.len());

    Ok(py.None())
}
