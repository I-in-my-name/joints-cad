extern crate nalgebra as na;
use std::cmp::Ordering;
use std::ops::Add;

trait Translatable {
    fn translate(&mut self, to_translate_by: na::Vector4<f64>);
}
//Rotate here means around the object centre and has to do with orientation and NOT position (not
//rotating around )
trait Rotatable {
    fn rotate(&mut self, to_rotate_by: na::Matrix3<f64>);
}


#[derive(Clone,Copy,PartialEq,PartialOrd)]
pub struct Point{
   point: na::Vector4<f64>,
}
impl Point{
    fn vector_to_point(vector: na::Vector4<f64>) -> Point{
        Point{
            point: vector,
        }
    }
    pub fn new(x: f64, y:f64, z:f64, d:f64) -> Self{
        Point{
            point: na::Vector4::new(
                x,
                y,
                z,
                d,
            )
        }
    }
    pub fn float_cmp(one: &Self, other: &Self) -> Ordering{
        let this_value: f64 = (one.point.x.powf(2.0) + one.point.y.powf(2.0) + one.point.z.powf(2.)).sqrt();
        let other_value: f64 = (other.point.x.powf(2.0) + other.point.y.powf(2.0) + other.point.z.powf(2.0)).sqrt();
        if this_value < other_value {
            Ordering::Less
        } else if this_value > other_value{
            Ordering::Greater
        }else{
            Ordering::Equal
        }
    }
    pub fn sort_point_vector(mut items: Vec<Point>) -> Vec<Point>{
        items.sort_by(|a, b| Point::float_cmp(a,b));
        items
    }
}
impl Add for Point{
    type Output = Self;

    fn add(self, other: Self ) -> Self {
        Self {
            point: self.point + other.point,
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
        let mut sorted_vec = Point::sort_point_vector(points_vec);
        sorted_vec.dedup();

        let mut average_x: f64 = 0.0; 
        let mut average_y: f64 = 0.0; 
        let mut average_z: f64 = 0.0;

        for outer_point in sorted_vec.iter(){
             average_x += outer_point.point.x as f64;
             average_y += outer_point.point.y as f64;
             average_z += outer_point.point.z as f64;
        }
        let number_of_points: f64 = sorted_vec.len() as f64;
        average_x = average_x / number_of_points;
        average_y = average_y / number_of_points;
        average_z = average_z / number_of_points;

        Point::new(average_x, average_y, average_z, 1.0)
    }
}

pub struct Camera {
    orientation: na::Matrix3<f64>,
    centre: Point,
    camera_matrix_superior:  na::Matrix4<f64>,
}
impl Translatable for Camera{
    fn translate(&mut self, to_translate_by: na::Vector4<f64>){
        self.centre = self.centre + Point::vector_to_point(to_translate_by);
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
