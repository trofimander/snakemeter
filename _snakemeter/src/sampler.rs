use callable::*;

use mio::{EventLoop, Handler};
use clock_ticks::precise_time_ns as time_ns;

use pyframe::print_stacktrace;

use std::thread;

struct MyHandler;

impl Handler for MyHandler {
    type Timeout = u64;
    type Message = ();

    fn timeout(&mut self, event_loop: &mut EventLoop<MyHandler>, timeout: u64) {
        print_stacktrace();
        event_loop.timeout_ms(timeout, timeout);
    }
}


pub struct Sampler {
    callable_registry: CallableRegistry,
    timestampt: u64,
    elapsed_time: u64,
    samples_count: u64
    // event_loop: EventLoop<MyHandler>
}

impl Sampler {
    pub fn new(rate: u64) -> Sampler {
        let mut event_loop = EventLoop::new().unwrap();
        let timeout_ms = 1000/rate;
        let timeout = event_loop.timeout_ms(timeout_ms, timeout_ms).unwrap();

        thread::spawn(move || {
            let _ = event_loop.run(&mut MyHandler);
        });


        Sampler { callable_registry: CallableRegistry::new(),
            timestampt: time_ns(),
            elapsed_time:0,
            samples_count:0 }
            // event_loop: event_loop }
    }
}
