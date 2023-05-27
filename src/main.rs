use std::{cell::RefCell, rc::Rc};

use rand::{distributions::Distribution, distributions::Uniform};

use hdbscan_0::{
    ball_tree,
    point::Point,
};
// use hdbscan_0::cluster::Clusterer;

fn main() {
    let mut data: Vec<Rc<RefCell<Point>>> = vec![];

    let mut rng = rand::thread_rng();
    let distribution = Uniform::from(-1f64..1f64);

    for _ in 0..1000 {
        data.push(Point::from_as_rcc(vec![
            distribution.sample(&mut rng),
            distribution.sample(&mut rng),
            distribution.sample(&mut rng),
        ]))
    }

    let ball_tree = ball_tree::construct(data, 30);
    println!("{:#?}", ball_tree);
}
