use callable::*;

use mio::{EventLoop, Handler, EventLoopConfig};
use clock_ticks::precise_time_ns as time_ns;

use pyframe::{print_stacktrace, iterate_stacktrace, ThreadProcessor, FrameProcessor};

use std::thread;
use std::sync::{Arc, Mutex};

pub struct Sampler {
    callable_registry: CallableRegistry,
    timestamp: u64,
    elapsed_time: u64,
    samples_count: u64,
    run: bool
}

struct SamplingTimerHandler {
    sampler: Arc<Mutex<Sampler>>
}

impl Handler for SamplingTimerHandler {
    type Timeout = u64;
    type Message = ();

    fn timeout(&mut self, event_loop: &mut EventLoop<SamplingTimerHandler>, timeout: u64) {
        let now = time_ns();

        let mut lock = self.sampler.lock().unwrap();
        lock.sample(now);
        if lock.run {
            // setup next timer
            event_loop.timeout_ms(timeout, timeout);
        }
    }
}

impl Sampler {
    fn new() -> Sampler {
        Sampler {
            callable_registry: CallableRegistry::new(),
            timestamp: time_ns(),
            elapsed_time:0,
            samples_count:0,
            run: true
        }
    }

    pub fn init(rate: u64) -> Arc<Mutex<Sampler>> {
        let timeout_ms = if 1000/rate >0 {1000/rate} else {1} ;

        let mut event_loop = EventLoop::configured(EventLoopConfig {
            io_poll_timeout_ms: 1_000,
            notify_capacity: 4_096,
            messages_per_tick: 256,
            timer_tick_ms: timeout_ms,
            timer_wheel_size: 1_024,
            timer_capacity: 65_536,
        }).unwrap();

        let timeout = event_loop.timeout_ms(timeout_ms, timeout_ms).unwrap();

        let sampler = Arc::new(Mutex::new(Sampler::new()));

        let mut handler = SamplingTimerHandler {
            sampler : sampler.clone()
        };

        thread::spawn(move || {
            let _ = event_loop.run(&mut handler);
        });

        sampler
    }

    pub fn sample(&mut self, now: u64) {    
          self.elapsed_time = self.elapsed_time + (now - self.timestamp);
          self.samples_count = self.samples_count + 1;

          iterate_stacktrace(self);
//        print_stacktrace();

          self.timestamp = time_ns();
    }

    pub fn stop(&mut self) {
        self.run = false;
    }

    pub fn stats(&mut self) -> Stats {
        Stats {
            callable_stats: self.callable_registry.as_tuples_list(),
            total_time: self.elapsed_time,
            samples_count: self.samples_count
        }
    }
}

#[derive(Debug, Clone)]
pub struct Stats {
    pub callable_stats: Vec<(String, String, u64, u64)>,
    pub total_time: u64,
    pub samples_count: u64
}

impl ThreadProcessor for Sampler {
    fn thread_id(&mut self, key: String) {
    }

    fn frame_processor(&mut self) -> &mut FrameProcessor {
        self
    }
}
impl FrameProcessor for Sampler {
    fn process(&mut self, callable: &Callable, sample_type: SampleType) {
        self.callable_registry.record_sample(callable, sample_type);
    }
}
