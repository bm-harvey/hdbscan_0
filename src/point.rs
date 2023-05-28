
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Point {
    coordinate: Vec<f64>,
    nearest_neighbors: Vec<Rc<RefCell<Point>>>,
}

impl Point {
    // type PointRef = Rc<RefCell<Point>>;
    // ctor
    pub fn from(coord: Vec<f64>) -> Self {
        Self {
            coordinate: coord,
            nearest_neighbors: vec![],
        }
    }

    pub fn from_as_rcc(coord: Vec<f64>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Point::from(coord)))
    }

    pub fn new() -> Self {
        Self {
            coordinate: vec![],
            nearest_neighbors: vec![],
        }
    }

    pub fn new_as_rcc() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Point::new()))
    }

    // access fns
    pub fn num_dimensions(&self) -> usize {
        self.coordinate.len()
    }

    pub fn get_coord(&self, index: usize) -> Option<&f64> {
        return self.coordinate.get(index);
    }

    pub fn distance(point_1: &Rc<RefCell<Point>>, point_2: &Rc<RefCell<Point>>) -> f64 {
        // if this passes, the rest of the calculation must be safe.
        let point_1 = RefCell::borrow(&point_1);
        let point_2 = RefCell::borrow(&point_2);

        assert_eq!(point_1.num_dimensions(), point_2.num_dimensions());

        let n_dim = point_1.num_dimensions();
        let mut sum: f64 = 0.0;
        for index in 0..n_dim {
            sum += (point_1.get_coord(index).unwrap() - point_2.get_coord(index).unwrap()).powf(2.);
        }
        sum.sqrt()
    }

    // mut
    pub fn add_neighbor(&mut self, other: Rc<RefCell<Point>>) {
        self.nearest_neighbors.push(Rc::clone(other.borrow()));
    }

    pub fn scale_data(&mut self, scale: f64, offset: f64) {
        for coord in &mut self.coordinate {
            *coord = (*coord) * scale + offset;
        }
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::new()
    }
}
