pub mod error;
pub mod macros;
pub mod obj;

fn main() {
    let mesh = mesh_from_obj!("../wall_with_door_gap.obj");
    println!("{:?}", mesh);
}
