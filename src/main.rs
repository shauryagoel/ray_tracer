mod tuple;
use tuple::Tuple;

fn main() {
    let test_data = Tuple {
        x: 1.0,
        y: 2.0,
        z: 3.0,
        w: 0.0,
    };

    println!(
        "Tuple points are: x={}, y={}, z={}, w={}",
        test_data.x, test_data.y, test_data.z, test_data.w
    );
}
