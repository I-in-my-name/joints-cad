extern crate nalgebra as na;
use std::cmp::Ordering;

trait Translatable {
    fn translate(&self, to_translate_by: na::Vector4<i32>) -> Self;
}
//Rotate here means around the object centre and has to do with orientation and NOT position (not
//rotating around )
trait Rotatable {
    fn rotate(&self, to_rotate_by: na::Matrix3<f64>) -> Self;
}


#[derive(Clone,Copy,PartialEq,Eq,PartialOrd)]
pub struct Point{
    x: i32,
    y: i32,
    z: i32,
    d: i32,
}
//move logic to utility for calculating from another point  with default parameters 0 0 0
impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering{
        let this_value: f64 = ((i32::pow(self.x,2) + i32::pow(self.y,2) + i32::pow(self.z,2)) as f64).sqrt();
        let other_value: f64 = ((i32::pow(other.x,2) + i32::pow(other.y,2) + i32::pow(other.z,2)) as f64).sqrt();
    let value = this_value - other_value;
        if value < 0.0 {
            Ordering::Less
        } else if value > 0.0{
            Ordering::Equal
        }else{
            Ordering::Greater
        }
    }
}
#[derive(Clone)]
pub struct Surface{
    //Must change here for curved surfaces along with rendering logic, potentially an enum to
    //describe the type of curve rendering along with a function.
    //
    //Not Just an alias for future editing
    key_points: Vec<Point>,
}
pub struct PerspectiveObject{
    //An object is considered to have an orientation and is made up of sides as well as having a
    //centre point.
    orientation: na::Matrix3<f64>,
    centre: Point,
    sides: Vec<Surface>,
}
impl PerspectiveObject{
    fn new(given_sides: Vec<Surface>) -> Self{
        Self{
            orientation: na::Matrix3::<f64>::zeros(),
            centre: Self::calculate_centre(&given_sides.clone()),
            sides: given_sides,
        }
}

    fn calculate_centre(sides: &Vec<Surface>) -> Point{
        let mut points_vec: Vec<Point> = sides_to_points(sides);
        points_vec.sort();
        points_vec.dedup();

        let mut average_x: f64 = 0.0; 
        let mut average_y: f64 = 0.0; 
        let mut average_z: f64 = 0.0;

        for point in points_vec.iter(){
             average_x += point.x as f64;
             average_y += point.y as f64;
             average_z += point.z as f64;
        }
        let number_of_points: f64 = points_vec.len() as f64;
        average_x = average_x / number_of_points;
        average_y = average_y / number_of_points;
        average_z = average_z / number_of_points;

        Point{x:average_x as i32,y:average_y as i32,z:average_z as i32, d:1}
    }
}
//A quirk of rust being that there is no way to abstract over mutability, these two functions can
//be considered as having entirely different contexts and are thus coupled differently. You could
//argue that this is not in violation of DRY principles and actually leads to decoupled and
//maintainable code
pub fn sides_to_points_mut(surfaces: &mut Vec<Surface>) -> Vec<Point>{
    let mut point_vector = Vec::new();
    for surface in surfaces.iter_mut() {
        for point in surface.key_points.iter_mut(){
            point_vector.push(*point);
        }
    }
    point_vector
}


pub fn sides_to_points(surfaces: &Vec<Surface>) -> Vec<Point>{
    let mut point_vector = Vec::new();
    for surface in surfaces.iter() {
        for point in surface.key_points.iter(){
            point_vector.push(*point);
        }
    }
    point_vector 
}
