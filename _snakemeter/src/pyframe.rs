use cpython::{PythonObject, Python, PyDict, NoArgs, PyTuple, PyString,
    PyFrame, ObjectProtocol, PyObject, PyResult, ToPyObject, PyInt};

use std::cmp::Ordering;

use callable::*;

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
