use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use std::vec;

#[derive(Debug, Clone)]
pub struct Point {
    coordinate: Vec<f64>,
    nearest_neighbors: Vec<Rc<RefCell<Point>>>,
}

impl Point {
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

/*

*/

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

#[derive(Debug)]
pub struct BallTreeBranchData {
    child_left: Box<BallTree>,
    child_right: Box<BallTree>,
}

impl BallTreeBranchData {
     pub fn construct(data: Vec<Rc<RefCell<Point>>>, leaf_size: usize) -> Box<BallTree> {
        // grab any point, find the point furthest from that point, find the point furthest from the second point
        // the second and third point are generally far from each other.
        // todo - There might be better ways to do this in the future
        // todo - in general this code is gross but it do the thing. clean it up sometime

        if data.len() < leaf_size || data.len() < 2 {
            return Box::new(BallTree::BallTreeLeaf(BallTreeLeafData { data }));
        }

        let random_pnt = Rc::clone(data[0].borrow());

        // todo - is there a more idiomatic way to do loops like this with closures?
        let mut far_pnt_1 = Rc::clone(data[1].borrow());
        let mut max_distance = Point::distance(&far_pnt_1, &random_pnt);
        for index in 1..(data.len()) {
            let pnt = Rc::clone(data[index].borrow());
            let dist = Point::distance(&pnt, &random_pnt);
            if max_distance < dist {
                max_distance = dist;
                far_pnt_1 = pnt;
            }
        }

        let mut far_pnt_2 = Rc::clone(data[0].borrow());
        let mut max_distance = Point::distance(&far_pnt_1, &far_pnt_2);
        for index in 1..(data.len()) {
            if index == 1 {
                continue;
            }
            let pnt = Rc::clone(data[index].borrow());
            let dist = Point::distance(&pnt, &random_pnt);
            if max_distance < dist {
                max_distance = dist;
                far_pnt_2 = pnt;
            }
        }

        let mut vec_left: Vec<Rc<RefCell<Point>>> = vec![];
        let mut vec_right: Vec<Rc<RefCell<Point>>> = vec![];

        for point_ref in &data {
            if Point::distance(&far_pnt_1, &point_ref) < Point::distance(&far_pnt_2, &point_ref) {
                vec_left.push(Rc::clone(point_ref));
            } else {
                vec_right.push(Rc::clone(point_ref));
            }
        }

        let child_left = BallTreeBranchData::construct(vec_left, leaf_size);
        let child_right = BallTreeBranchData::construct(vec_right, leaf_size);

        return Box::new(BallTree::BallTreeBranch(BallTreeBranchData {
            child_left,
            child_right,
        }));
    }
}


#[derive(Debug)]
pub struct BallTreeLeafData {
    data: Vec<Rc<RefCell<Point>>>,
}

#[derive(Debug)]
pub enum BallTree {
    BallTreeBranch(BallTreeBranchData),
    BallTreeLeaf(BallTreeLeafData),
}
