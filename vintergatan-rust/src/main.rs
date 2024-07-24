use vintergatan_rust::model::universe::Universe;

fn main() {
    let width = 10;
    let height = width;
    let universe = Universe::generate(width, height);
    println!("{}", universe);
    println!("{}", universe.get_score())
}
