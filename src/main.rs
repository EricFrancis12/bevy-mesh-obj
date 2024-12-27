pub mod error;
pub mod macros;
pub mod obj;

#[cfg(test)]
pub mod tests;

fn main() {
    let mesh = mesh_from_obj!("../assets/wall_with_door_gap.obj");
    println!("{:?}", mesh);
}
