
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

use crate::point;
use point::Point;


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