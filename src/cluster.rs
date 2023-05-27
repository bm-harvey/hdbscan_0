use crate::point;
use point::Point;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Clusterer {
    data: Vec<Rc<RefCell<Point>>>,
}

impl Clusterer {
    // ctor-like
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn from_data(d: Vec<Rc<RefCell<Point>>>) -> Self {
        Self { data: d }
    }

    // mut
    pub fn scale_data(&mut self, scale: f64, offset: f64) {
        for point in &mut self.data {
            let mut mut_point = RefCell::borrow_mut(point);
            mut_point.scale_data(scale, offset);
            // i need to use the Rc<RefCell<Point>> pattern
        }
    }
}

impl Default for Clusterer {
    fn default() -> Self {
        Self::new()
    }
}