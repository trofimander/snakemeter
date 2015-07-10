use std::collections::HashMap;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;


pub enum SampleType {
    SelfSample, CumulativeSample
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Callable {
    pub path: String,
    pub name: String
}

impl Callable {
    pub fn new(path: String, name: String) -> Callable {
        Callable { path: path, name: name }
    }
}


pub struct CallableStats {
    pub cid: u64,
    pub cumulative_count: u64,
    pub self_count: u64
}

impl CallableStats {
    pub fn new(cid: u64)-> CallableStats {
        CallableStats {cid: cid, cumulative_count: 0, self_count: 0}
    }

    // Updates the callable statistics
    pub fn update(&mut self, sample_type: SampleType) {
        match sample_type {
            SampleType::SelfSample => self.self_count = self.self_count + 1,
            SampleType::CumulativeSample => self.cumulative_count = self.cumulative_count + 1
        }
    }
}

pub struct CallableRegistry {
    id_counter: u64,
    callable_to_id: HashMap<Callable, u64>,
    callable_stats: HashMap<u64, CallableStats>
}

impl CallableRegistry {
    pub fn new() -> CallableRegistry {
        CallableRegistry {  id_counter: 0,
                            callable_to_id: HashMap::new(),
                            callable_stats: HashMap::new()}
    }
    // Records the callable sample in registry
    pub fn record_sample(&mut self, callable: &Callable, sample_type: SampleType) {
        let cid  = self.callable_id(callable);
        let stats = self.callable_stats(cid);
        stats.update(sample_type);
    }

    fn callable_id(&mut self, callable: &Callable) -> u64 {
        if !self.callable_to_id.contains_key(&callable) {
            let cid = self.next_cid();
            self.callable_to_id.insert(callable.clone(), cid.clone());
        }
        self.callable_to_id.get(&callable).unwrap().clone()
    }

    fn callable_stats(&mut self, cid: u64) -> &mut CallableStats {
        if !self.callable_stats.contains_key(&cid)  {
            let stats = CallableStats::new(cid.clone());
            self.callable_stats.insert(cid.clone(), stats);
        }
        self.callable_stats.get_mut(&cid).unwrap()
    }

    fn next_cid(&mut self) -> u64 {
        self.id_counter= self.id_counter + 1;
        self.id_counter
    }
}
