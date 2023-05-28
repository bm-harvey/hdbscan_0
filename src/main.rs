use std::{cell::RefCell, rc::Rc};

use rand::{distributions::Distribution, distributions::Uniform};

use hdbscan_0::{ball_tree, point::Point};

fn main() {
    let mut data: Vec<Rc<RefCell<Point>>> = vec![];

    let mut rng = rand::thread_rng();
    let distribution = Uniform::from(-1f64..1f64);

    for _ in 0..10 {
        data.push(Point::from_as_rcc(vec![
            distribution.sample(&mut rng),
            distribution.sample(&mut rng),
        ]))
    }

    let bt = ball_tree::construct(data, 2);


    // this is the behaviour i am trying to implement
    let mut counter = 0;
    for pnt in bt.iter() {
        counter += 1;
        if counter > 10{
            println!("something went wrong");
            break;
        }
        println!("{} : the point is : {:?}",counter, &pnt);
    }
    let mut counter = 0;
    for pnt in bt.iter() {
        counter += 1;
        if counter > 10{
            println!("something went wrong");
            break;
        }
        println!("{} : the point is : {:?}",counter, &pnt);
    }
}
