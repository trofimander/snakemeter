#![feature(libc)]
#![feature(plugin)]
#![plugin(interpolate_idents)]

#[macro_use] extern crate cpython;
extern crate mio;
extern crate clock_ticks;


mod callable;
mod sampler;


use cpython::{PythonObject, Python, PyDict, NoArgs, PyTuple, PyString,
    PyFrame, ObjectProtocol, PyObject, PyResult, ToPyObject, PyInt};

extern crate libc;

use libc::c_char;
use std::ffi::CStr;
use std::str;
use std::cmp::Ordering;

use sampler::Sampler;

py_module_initializer!(_snakemeter, |_py, m| {
    try!(m.add("__doc__", "Module documentation string"));
    try!(m.add("print_version", py_fn!(print_version)));
    try!(m.add("current_frames_count", py_fn!(current_frames_count)));
    try!(m.add("print_stacktrace", py_fn!(print_stacktrace)));
    try!(m.add("start_sampling", py_fn!(start_sampling)));
    Ok(())
});

pub fn print_version<'p>(py: Python<'p>, args: &PyTuple<'p>) -> PyResult<'p, PyObject<'p>> {
    let sys = py.import("sys").unwrap();
    let version: String = sys.get("version").unwrap().extract().unwrap();
    println!("Hello Python {}", version);
    Ok(py.None())
}

pub fn current_frames_count<'p>(py: Python<'p>, args: &PyTuple<'p>) -> PyResult<'p, PyObject<'p>> {
    let sys = py.import("sys").unwrap();
    let frames_dict: PyDict = sys.call("_current_frames", NoArgs, None).unwrap().extract().unwrap();
    let frames_count = frames_dict.items().len();
    Ok(frames_count.to_py_object(py))
}

pub fn start_sampling<'p>(py: Python<'p>, args: &PyTuple<'p>) -> PyResult<'p, PyObject<'p>> {
    let rate = unsafe {args.get_item(0).unchecked_cast_into::<PyInt>().value()};
    let sampler = Sampler::new(rate as u64);
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
                    let code = frame.getattr("f_code").unwrap();
                    println!("{}:{} {}", code.getattr("co_filename").unwrap(), frame.getattr("f_lineno").unwrap(),
                    code.getattr("co_name").unwrap());

                    match frame.getattr("f_back") {
                            Ok(f) => if f.compare(py.None()).unwrap() == Ordering::Equal { value = None } else {value = Some(f)},
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
