
fn main() {
    for_ex();
}

fn for_ex() {
    let big_birds = ["ostrich", "peacock", "stork"];
    for bird in big_birds.iter() {
        println!("The {} is a big bird.", bird);
    }

    for number in 0..5 {
        println!("{}", number*2);
    }
}

fn while_ex() {
    let mut counter = 0;
    while counter < 5 {
        println!("We loop a while...");
        counter = counter + 1;
    } 
}

fn loop_ex() {
    let mut counter: f64 = 1.0;
    let mut n = 0;
    let stop_loop = loop {
        n += 1;
        counter *= 0.9999;
        if counter < 0.000001 {
            break (n, counter);
        }
    };

    println!("Break the loop at counter = {}, iter_num = {}.", stop_loop.1, stop_loop.0);
}
