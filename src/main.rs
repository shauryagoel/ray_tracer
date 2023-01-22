mod tuple;
use tuple::Tuple;

fn main() {
    let data1 = Tuple {
        x: 1.0,
        y: 2.0,
        z: 3.0,
        w: 0.0,
    };

    let data2 = Tuple {
        x: 0.0,
        y: 1.0,
        z: 2.0,
        w: 0.0,
    };

    let test_data: Tuple = data1.add(&data2);

    println!(
        "Tuple points are: x={}, y={}, z={}, w={}",
        test_data.x, test_data.y, test_data.z, test_data.w
    );
}
