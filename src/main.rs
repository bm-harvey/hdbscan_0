use std::{cell::RefCell, rc::Rc};

use rand::{distributions::Distribution, distributions::Uniform};

use hdbscan_0::{ball_tree, point::Point};

use std::time::Instant;

fn main() {
    let mut data: Vec<Rc<RefCell<Point>>> = vec![];

    let mut rng = rand::thread_rng();
    let distribution = Uniform::from(-1f64..1f64);

    let num = 10000000;

    for _ in 0..num {
        data.push(Point::from_as_rcc(vec![
            distribution.sample(&mut rng),
            distribution.sample(&mut rng),
        ]))
    }
    let start = Instant::now();
    let leaf_size = (num as f64).log2() as usize; // a reasonable rule of thumb for a good leaf size
    let bt = ball_tree::construct(data, leaf_size);
    let time = start.elapsed();

    println!("ball tree took {:?}", time);

    println!("there are : {} elements", bt.count_elements());
    // this is the behaviour i am trying to implement
}
