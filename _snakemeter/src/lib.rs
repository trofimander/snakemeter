#![feature(libc)]
#![feature(plugin)]
#![plugin(interpolate_idents)]

#[macro_use] extern crate cpython;
extern crate mio;
extern crate clock_ticks;


mod callable;
mod sampler;
mod pyframe;


use cpython::{PythonObject, Python, PyDict, NoArgs, PyTuple, PyString,
    PyFrame, ObjectProtocol, PyObject, PyResult, ToPyObject, PyInt, PyRustTypeBuilder, PyRustObject};

extern crate libc;

use libc::c_char;
use std::ffi::CStr;
use std::str;
use std::cmp::Ordering;
use std::sync::{Arc, Mutex};

use sampler::Sampler;

py_module_initializer!(_snakemeter, |_py, m| {
    try!(m.add("__doc__", "Module documentation string"));
    try!(m.add("print_version", py_fn!(print_version)));
    try!(m.add("current_frames_count", py_fn!(current_frames_count)));
    try!(m.add("start_sampling", py_fn!(start_sampling)));
    try!(m.add("stop_sampling", py_fn!(stop_sampling)));
    try!(m.add("get_sampling_stats", py_fn!(get_sampling_stats)));
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
    let sampler_object = args.get_item(0);
    let rate = unsafe {args.get_item(1).unchecked_cast_into::<PyInt>().value()};
    let sampler = Sampler::init(rate as u64);

    let t = PyRustTypeBuilder::<Arc<Mutex<Sampler>>>::new(py, "Sampler").finish().unwrap();

    let inst = t.create_instance(sampler, ());

    sampler_object.setattr("_sampler", inst);

    Ok(py.None())
}

pub fn stop_sampling<'p>(py: Python<'p>, args: &PyTuple<'p>) -> PyResult<'p, PyObject<'p>> {
    let sampler_object = args.get_item(0);

    let sampler = get_sampler(sampler_object);

    let mut lock = sampler.lock().unwrap();

    lock.stop();

    Ok(py.None())
}

pub fn get_sampling_stats<'p>(py: Python<'p>, args: &PyTuple<'p>) -> PyResult<'p, PyObject<'p>> {
    let sampler_object = args.get_item(0);

    let sampler = get_sampler(sampler_object);

    let mut lock = sampler.lock().unwrap();

    let list = lock.statistics();

    let mut boxed_slice = list.into_boxed_slice();

    Ok(ToPyObject::to_py_object(&mut *boxed_slice, py).into_object())
}

fn get_sampler(obj: PyObject) -> Arc<Mutex<Sampler>> {
    let pyobj = obj.getattr("_sampler").unwrap();
    let r: PyRustObject<Arc<Mutex<Sampler>>, PyObject> = unsafe {PyRustObject::unchecked_downcast_from(pyobj) };
    r.get().clone()
}
