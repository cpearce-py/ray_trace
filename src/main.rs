use ray_trace::vector;

fn main() {
    let vec = vector::Vec3{x: 1.1, y: 1.2, z: 1.3};
    println!("{}", vec.length());
}
