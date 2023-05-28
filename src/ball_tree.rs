use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

use crate::point;
use point::Point;

#[derive(Debug)]
pub struct BranchData {
    child_left: Box<BallTree>,
    child_right: Box<BallTree>,
}

#[derive(Debug)]
pub struct LeafData {
    point_cloud: Vec<Rc<RefCell<Point>>>,
}

pub struct BranchItrData<'a> {
    data: &'a BranchData,
    child_is_left : bool,
    child_itr: Box<BallTreeItr<'a>>,
}
pub struct LeafItrData<'a> {
    data: &'a LeafData,
    counter: usize,
}

#[derive(Debug)]
pub enum BallTree {
    Branch(BranchData),
    Leaf(LeafData),
}
pub enum BallTreeItr<'a> {
    Branch(BranchItrData<'a>),
    Leaf(LeafItrData<'a>),
}

impl<'a> BallTree {
    pub fn iter(&'a self) -> BallTreeItr<'a> {
        match self {
            BallTree::Branch(tree) => BallTreeItr::Branch(BranchItrData {
                data: tree,
                child_is_left : true,
                child_itr: Box::new(tree.child_left.iter()),
            }),
            BallTree::Leaf(tree) => BallTreeItr::Leaf(LeafItrData {
                data: tree,
                counter: 0,
            }),
        }
    }
}

impl<'a> Iterator for BallTreeItr<'a> {
    type Item = &'a Rc<RefCell<Point>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            BallTreeItr::Branch(itr) => {
                let result = itr.child_itr.next();
                match result {
                    None => {
                        if itr.child_is_left{
                            itr.child_is_left = false;
                            itr.child_itr = Box::new(itr.data.child_right.iter());
                            itr.child_itr.next()
                        } else {
                         None
                        }
                    },
                    Some(res) => Some(res)
                    
                }
            }
            BallTreeItr::Leaf(itr) => {
                let index = itr.counter;
                let pc = &itr.data.point_cloud;
                if index < pc.len() {
                    itr.counter += 1;
                    Some(&pc[index])
                } else {
                    None
                }
            }
        }
    }
}

//
// external ctor
//
pub fn construct(data: Vec<Rc<RefCell<Point>>>, leaf_size: usize) -> Box<BallTree> {
    // grab any point, find the point furthest from that point, find the point furthest from the second point
    // the second and third point are generally far from each other.
    // todo - There might be better ways to do this in the future
    // todo - in general this code is gross but it do the thing. clean it up sometime

    if data.len() < leaf_size || data.len() < 2 {
        return Box::new(BallTree::Leaf(LeafData { point_cloud: data }));
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

    let child_left = construct(vec_left, leaf_size);
    let child_right = construct(vec_right, leaf_size);

    return Box::new(BallTree::Branch(BranchData {
        child_left,
        child_right,
    }));
}
