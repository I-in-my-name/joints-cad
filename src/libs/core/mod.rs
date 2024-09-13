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

    fn point_ignore_d(&mut self) -> na::Vector3<f64>{
        na::Vector3::new(
            self.point.x,
            self.point.y,
            self.point.w,
        )
    }

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
impl Rotatable for PerspectiveObject{
    fn rotate(&mut self, to_rotate_by: na::Matrix3<f64>){
     self.orientation = self.orientation * to_rotate_by;
    }
}
impl Translatable for PerspectiveObject{
    fn translate(&mut self, to_translate_by: na::Vector4<f64>){
        self.centre = self.centre + Point::vector_to_point(to_translate_by);
    }
}

pub struct Camera {
    orientation: na::Matrix3<f64>,
    centre: Point,
    calibration_matrix: na::Matrix3<f64>,
    camera_extrinsics: na::Matrix3x4<f64>,
    camera_matrix_superior:  na::Matrix3x4<f64>,
}
impl Camera{
    //The coupling here is logically necessary
    fn update_extrinsics(&mut self, new_centre: Point, new_orientation: na::Matrix3<f64>){
        self.centre = new_centre;
        self.orientation = new_orientation;
        //correct way of splicing together two distinct matrices

        self.camera_extrinsics = na::Matrix3x4::new(
            self.orientation.m11, self.orientation.m12, self.orientation.m13, self.centre.point.x, 

            self.orientation.m21, self.orientation.m22, self.orientation.m23, self.centre.point.y,

            self.orientation.m31, self.orientation.m32, self.orientation.m33, self.centre.point.z,  

            ); 
    }

    fn update_extrinsics_centre(&mut self, new_centre: Point){
        self.update_extrinsics(new_centre, self.orientation);
    }

    fn update_extrinsics_orientation(&mut self, new_orientation: na::Matrix3<f64>){
         self.update_extrinsics(self.centre, new_orientation);
    }
    //Very much subject to change, this is tracer code and needs to be fine tuned
    fn update_intrinsics(&mut self){
        self.calibration_matrix = na::Matrix3::new(
                1.0, 0.0, self.centre.point.x,
                0.0, 1.0, self.centre.point.y,
                0.0, 0.0, 1.0,
            );
    }
    //if 3x4 is fine then refactor necessary, check after tracer code is working 
    fn update_superior_matrix(&mut self){
        self.camera_matrix_superior = self.calibration_matrix * self.camera_extrinsics;
    }
}
//These are definitely subject to change as they will need to update the callibration matrix and/or
//the camera extrinsics 
impl Translatable for Camera{
    fn translate(&mut self, to_translate_by: na::Vector4<f64>){
        self.centre = self.centre + Point::vector_to_point(to_translate_by);
    }
}
impl Rotatable for Camera{
    fn rotate(&mut self, to_rotate_by: na::Matrix3<f64>){
     self.orientation = self.orientation * to_rotate_by;
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
