use std::thread::sleep;
use std::time::Duration;
use engine::model::galaxy::Galaxy;
use engine::model::universe::Universe;

fn main() {
    loop {
        let width = 10;
        let height = width;
        let universe = Universe::generate(width, height);
        println!("{}", universe);
        println!("Score: {}", universe.get_score());
        println!();
        sleep(Duration::from_millis(1000));
    }
    // let largest_galaxy = universe.get_galaxies().iter().max_by_key(|g| g.size()).unwrap().clone();
    // let mut u2 = Universe::new(width, height);
    // u2.add_galaxy(&largest_galaxy);
    // println!("{}", u2);
    // let mut u3 = Universe::new(width, height);
    // let rects = largest_galaxy.rectangles();
    // for rect in &rects {
    //     u3.add_galaxy(&Galaxy::from_rect(rect))
    // };
    // println!("{}", u3);
    // println!("{}", rects.len());
    // println!("{:?}", rects);
    // println!("{}", largest_galaxy.rectangles().len());
}
