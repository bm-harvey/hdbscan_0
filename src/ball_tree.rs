use std::borrow::Borrow;
use std::cell::{Ref, RefCell};
// use std::collections::btree_map::Iter;
use std::rc::Rc;

use crate::point;
use point::Point;

#[derive(Debug)]
pub struct BallTreeBranchData {
    child_left: Box<BallTree>,
    child_right: Box<BallTree>,
    // child_left_done_iterating: bool,
    // child_riht_done_iterating: bool,
}

impl BallTreeBranchData {}

#[derive(Debug)]
pub struct BallTreeLeafData {
    data: Vec<Rc<RefCell<Point>>>,
    itr_counter: usize,
}

#[derive(Debug)]
pub enum BallTree {
    BallTreeBranch(BallTreeBranchData),
    BallTreeLeaf(BallTreeLeafData),
}

struct BallTreeBranchIterator<'a> {
    internal_data: &'a BallTreeBranchData,
    state: usize,
}

impl<'a> Iterator for BallTreeBranchIterator<'a> {
    type Item = &'a Rc<RefCell<Point>>;

    fn next(&mut self) -> Option<Self::Item> {
        
        let result_left = self.internal_data.child_left.next();
        match result_left {
            Some(itr) => Some(itr),
            None => self.internal_data.child_right.next(),
        }
    }
}

// impl Iterator for BallTree {
//     type Item = Rc<RefCell<Point>>;
//     fn next(&mut self) -> Option<Self::Item> {
//         match self {
//             BallTree::BallTreeLeaf(tree) => {
//                 if tree.itr_counter < tree.data.len() {
//                     let index = tree.itr_counter;
//                     tree.itr_counter += 1;
//                     return Some(Rc::clone(tree.data.get(index).unwrap()));
//                 }
//                 None
//             }
//             BallTree::BallTreeBranch(tree) => {
//                 let result_left = tree.child_left.next();

//                 match result_left {
//                     Some(itr) => Some(itr),
//                     None => tree.child_right.next(),
//                 }
//             }
//         }
//     }
// }

// impl Iterator for BallTreeBranchData {
//     type Item = Rc<RefCell<Point>>;

//     fn next(&mut self) -> Option<Self::Item> {
//         let result_left = self.child_left.next();
//         None
//     }
// }

// impl Iterator for BallTreeLeafData{
//     type Item = Rc<RefCell<Point>>;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.itr_counter < self.data.len(){
//             self.itr_counter += 1;
//             return Some(self.data[self.itr_counter - 1].to_owned());
//         }

//         None
//     }
// }
/*
impl IntoIterator for BallTreeLeafData{
    type Item = Rc<RefCell<Point>>;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
*/

pub fn construct(data: Vec<Rc<RefCell<Point>>>, leaf_size: usize) -> Box<BallTree> {
    // grab any point, find the point furthest from that point, find the point furthest from the second point
    // the second and third point are generally far from each other.
    // todo - There might be better ways to do this in the future
    // todo - in general this code is gross but it do the thing. clean it up sometime

    if data.len() < leaf_size || data.len() < 2 {
        return Box::new(BallTree::BallTreeLeaf(BallTreeLeafData {
            data,
            itr_counter: 0,
        }));
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

    return Box::new(BallTree::BallTreeBranch(BallTreeBranchData {
        child_left,
        child_right,
    }));
}
