 use cpython::{PythonObject, Python, PyDict, NoArgs, PyTuple, PyString,
    ObjectProtocol, PyObject, PyResult, ToPyObject, PyInt};

use std::cmp::Ordering;
use std::ffi::{CStr, CString};
use std::str;

use callable::*;

use libc::{c_int};
use cpython::_detail::ffi;

pub fn top_frames() -> Vec<(String, String, i32)> {
    let gil = Python::acquire_gil();
    let mut v = Vec::<(String, String, i32)>::new();

    let mut interpreter = unsafe { ffi::PyInterpreterState_Head() };
    while !interpreter.is_null() {
        let mut thread = unsafe { ffi::PyInterpreterState_ThreadHead(interpreter) };
        while !thread.is_null() {
            let frame = unsafe { (*thread).frame };

            if !frame.is_null() {
                let lineno = unsafe { ffi::PyFrame_GetLineNumber(frame) } ;

                let code = unsafe { (*frame).f_code };

                if !code.is_null() {
                    let filename_obj = unsafe { ffi::PyString_AS_STRING((*code).co_filename) };

                    let filename_c = unsafe { CString::from_ptr(filename_obj) } ;
                   let filename = str::from_utf8(filename_c.to_bytes()).unwrap();

                    let name_obj = unsafe { ffi::PyString_AS_STRING((*code).co_name) };

                    let name_c = unsafe { CString::from_ptr(name_obj) };
                   let name  = str::from_utf8(name_c.to_bytes()).unwrap();

//                    v.push((String::from(filename), String::from(name), lineno));
                    v.push((String::from(""), String::from(""), lineno));
                }
            }

            thread = unsafe { ffi::PyThreadState_Next(thread) };
        }
        interpreter = unsafe { ffi::PyInterpreterState_Next(interpreter) };
    }
    v
}



pub trait ThreadProcessor {
    fn thread_id(&mut self, key: String);
    fn frame_processor(&mut self) -> &mut FrameProcessor;
}

pub trait FrameProcessor {
    fn process(&mut self, callable: &Callable, sample_type: SampleType);
}

struct StackTracePrinter;

impl ThreadProcessor for StackTracePrinter {
    fn thread_id(&mut self, key: String) {
        println!("Thread {}", key);
    }

    fn frame_processor(&mut self) -> &mut FrameProcessor {
        self
    }
}
impl FrameProcessor for StackTracePrinter {
    fn process(&mut self, callable: &Callable, sample_type: SampleType) {
        // println!("{}:{} {}", code.getattr("co_filename").unwrap(), frame.getattr("f_lineno").unwrap()

         println!("{}:{}", callable.path, callable.name)
    }
}


pub fn print_stacktrace() {
    println!("Stacktrace:");
    iterate_stacktrace(&mut StackTracePrinter);
}

pub fn iterate_stacktrace(thread_proessor: &mut ThreadProcessor) {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let sys = py.import("sys").unwrap();
    let frames_dict: PyDict = sys.call("_current_frames", NoArgs, None).unwrap().extract().unwrap();
    let frames = frames_dict.items();

    for x in frames.into_iter() {
        let tuple = unsafe {x.unchecked_cast_into::<PyTuple>()};
        let key = tuple.get_item(0);
//        let value = tuple.get_item(1).unchecked_cast_into::<PyFrame>();
        let value = tuple.get_item(1);

        let mut value:Option<PyObject> = Some(value);

        thread_proessor.thread_id(format!("{}", key));

        let frames_processor = thread_proessor.frame_processor();

        let mut top = true;

        loop {
            match value {
                Some(frame) => {
                    let code = frame.getattr("f_code").unwrap();

                    let callable = Callable::new(
                        format!("{}", code.getattr("co_filename").unwrap()),
                        format!("{}", code.getattr("co_name").unwrap()),
                        frame.getattr("f_lineno").unwrap().extract::<i32>().unwrap()
                    );

                    frames_processor.process(&callable,
                        if top {SampleType::SelfSample} else {SampleType::CumulativeSample});

                    break;
                    //
                    // match frame.getattr("f_back") {
                    //         Ok(f) => if f.compare(py.None()).unwrap() == Ordering::Equal { value = None } else {value = Some(f)},
                    //         Err(err) => {err.print(); value = None }
                    // };

                },
                None => break

                }
                top = false;
        }
    }
}

pub fn iterate_stacktrace_fast(thread_proessor: &mut ThreadProcessor) {
    let frames_processor = thread_proessor.frame_processor();

    for (filename, name, lineno) in top_frames() {
        let callable = Callable::new(filename, name, lineno);

        frames_processor.process(&callable, SampleType::SelfSample);
    }
}
