use hdbscan_0::{self, Point, Clusterer};
use std::{rc::Rc, cell::RefCell};
use rand::{Rng, distributions::Uniform, distributions::Distribution};

fn main() {

    let mut data : Vec<Rc<RefCell<Point>>> = vec![];



    let mut rng = rand::thread_rng();
    let distribution = Uniform::from(-1f64..1f64);


    for _ in 0..10000000{
        data.push(Point::from_as_rcc(vec![
            distribution.sample(&mut rng),
            distribution.sample(&mut rng),
            distribution.sample(&mut rng),
        ]))
    }

//    println!("data is {:#?}", data);

    let ball_tree = hdbscan_0::BallTreeBranchData::construct(data, 30);
    println!("{:#?}", ball_tree);

        //BallTreeBranch::construct(data, 30);
    



}
