mod tuple;
mod utils;
use tuple::Tuple;

fn main() {
    let data1 = Tuple::new(1.0, 2.0, 3.0, 0.0);
    let data2 = Tuple::new(0.0, 1.0, 2.0, 0.0);

    let test_data: Tuple = data1.add(&data2);

    println!(
        "Tuple points are: x={}, y={}, z={}, w={}",
        test_data.x, test_data.y, test_data.z, test_data.w
    );
}
