use vintergatan::model::universe::Universe;

fn main() {
    loop {
        let width = 10;
        let height = width;
        let universe = Universe::generate(width, height);
        println!("{}", universe);
        println!("Score: {}", universe.get_score());
        println!();
    }
}
