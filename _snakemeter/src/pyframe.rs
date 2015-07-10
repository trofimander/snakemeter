use cpython::{PythonObject, Python, PyDict, NoArgs, PyTuple, PyString,
    PyFrame, ObjectProtocol, PyObject, PyResult, ToPyObject, PyInt};

use std::cmp::Ordering;

pub fn print_stacktrace() {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let sys = py.import("sys").unwrap();
    let frames_dict: PyDict = sys.call("_current_frames", NoArgs, None).unwrap().extract().unwrap();
    let frames = frames_dict.items();


    println!("Stacktrace:");
    for x in frames.into_iter() {
        let tuple = unsafe {x.unchecked_cast_into::<PyTuple>()};
        let key = tuple.get_item(0);
//        let value = tuple.get_item(1).unchecked_cast_into::<PyFrame>();
        let value = tuple.get_item(1);

        let mut value:Option<PyObject> = Some(value);

        println!("Thread {}", key);
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
}
