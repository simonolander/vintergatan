use vintergatan_rust::model::universe::Universe;

fn main() {
    let width = 3;
    let height = width;
    let mut universe = Universe::new(width, height);
    for n in 0..width*height*10 {
        if n % 1 == 0 {
            println!("{}score: {}\n", universe, universe.get_score());
        }
        universe.generate_step();
    }
    println!("{}", universe);
}
