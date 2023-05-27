use hdbscan_0::{self, Point, Clusterer};
use std::{rc::Rc};

fn main() {

    let p_1 = Point::from_as_rcc(vec![2., 2.4]);
    let p_2 = Point::from_as_rcc(vec![3., 4.3]);
    let p_3 = Point::from_as_rcc(vec![5., 4.3]);
    //let p_4 = Point::from_as_rcc(vec![2., -5.2]);

    println!("the generated points are  : {:?} , {:?}, {:?}", p_1, p_2, p_3);


    let mut clusterer = Clusterer::from_data(vec![Rc::clone(&p_1), Rc::clone(&p_2)]);

    println!("the distance between the points is : {}", Point::distance(&p_1, &p_3));

    println!("The clusterer pre-scaling is {:#?}", clusterer);
    
    clusterer.scale_data(3., 3.);
    
    println!("The clusterer post-scaling is {:#?}", clusterer);
}
