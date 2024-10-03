extern crate nalgebra as na;
use std::cmp::Ordering;
use std::ops::Add;

#[derive(Clone,Debug)]
pub enum coordinate_object{
    Camera_object(Camera),
    Point_object(Point),
    Perspective_object(PerspectiveObject),
    Line_object(Line),

}
impl Point_Construct for coordinate_object{
    fn get_points(&self) -> Vec<Point>{
        match self{
            Self::Camera_object(camera) => camera.get_points(),
            Self::Point_object(point) => point.get_points(),
            Self::Perspective_object(perspective_object) => perspective_object.get_points(),
            Self::Line_object(line) => line.get_points(),
        }
    }
}

pub trait Translatable {
    fn translate(&mut self, to_translate_by: na::Vector4<f64>);
}
//Rotate here means around the object centre and has to do with orientation and NOT position (not
//rotating around )
pub trait Rotatable {
    fn rotate(&mut self, to_rotate_by: na::Matrix3<f64>);
}
pub trait Point_Construct{
    fn get_points(&self) -> Vec<Point>;
}


#[derive(Clone,Copy,Debug,PartialEq,PartialOrd)]
pub struct Point{
   point: na::Vector4<f64>,
}
impl Point{

    //Note!! not equalised!!!
    fn point_ignore_w(&mut self) -> na::Vector3<f64>{
        na::Vector3::new(
            self.point.x,
            self.point.y,
            self.point.z,
        )
    }

    pub fn vector_to_point(vector: na::Vector4<f64>) -> Point{
        Point{
            point: vector,
        }
    }

    pub fn point_to_vector(&self) -> na::Vector4<f64>{
         na::Vector4::new(
                self.point.x,
                self.point.y,
                self.point.z,
                self.point.w,
            )
    }

    pub fn new(x: f64, y:f64, z:f64, w:f64) -> Self{
        Point{
            point: na::Vector4::new(
                x,
                y,
                z,
                w,
            )}
    }
    pub fn get_depth(&self) -> f64{
        self.point.z
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

impl Point_Construct for Point{
    fn get_points(&self) -> Vec<Point>{
        vec![*self]
    }

}

#[derive(Clone,Copy,Debug)]
pub struct Line{
    point_a: Point,
    point_b: Point,
}
impl Line{
    pub fn new(point_one: Point, point_two: Point) -> Self{
        Line{
            point_a: point_one,
            point_b: point_two,
        }
    }
    pub fn get_start(&self) -> Point{
        self.point_a
    }
    pub fn get_end(&self) -> Point{
        self.point_b
    }
}
impl Point_Construct for Line{
    fn get_points(&self) -> Vec<Point>{
        vec![self.point_a,self.point_b]
    }
}



#[derive(Clone,Debug)]
pub struct Surface{
    //Must change here for curved surfaces along with rendering logic, potentially an enum to
    //describe the type of curve rendering along with a function.
    //
    //Not Just an alias for future editing
    key_points: Vec<Point>,
}

#[derive(Clone,Debug)]
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
impl Point_Construct for PerspectiveObject{
    fn get_points(&self) -> Vec<Point>{
        let mut points_vec: Vec<Point> = sides_to_points(&self.sides);
        let mut sorted_vec = Point::sort_point_vector(points_vec);
        sorted_vec.dedup();

        sorted_vec
    }
}

#[derive(Clone,Debug)]
pub struct Camera {
    orientation: na::Matrix3<f64>,
    centre: Point,
    calibration_matrix: na::Matrix4<f64>,
    camera_extrinsics: na::Matrix4<f64>,
    extrinsics_inverse: na::Matrix4<f64>,
    camera_matrix_superior:  na::Matrix4<f64>,
    basis_change_matrix: na::Matrix3<f64>,
    fov_y: f64,
    fov_x: f64,
    screen_x: u8,
    screen_y: u8,
    min_depth_difference: f64,
    max_depth_difference: f64,
}
impl Camera{
    pub fn new() -> Self{
        let mut new_camera = Camera {
            orientation: na::Matrix3::<f64>::identity(),
            centre: Point::new(0.0,0.0,0.0,0.0),
            calibration_matrix: na::Matrix4::<f64>::zeros(),
            camera_extrinsics: na::Matrix4::<f64>::zeros(),
            extrinsics_inverse: na::Matrix4::<f64>::zeros(),
            camera_matrix_superior: na::Matrix4::<f64>::zeros(),
            basis_change_matrix: na::Matrix3::<f64>::zeros(),
            fov_y: 90.0,
            fov_x: 90.0,
            screen_x: 128,
            screen_y:128,
            min_depth_difference: 2.0,
            max_depth_difference: 120.0,
        };
        new_camera.update_extrinsics_centre(Point::new(0.0,0.0,1.0,1.0));
        new_camera.update_intrinsics();
        new_camera.update_superior_matrix();
        new_camera

    }
    //The coupling here is logically necessary
    pub fn update_extrinsics(&mut self, new_centre: Point, new_orientation: na::Matrix3<f64>){
        self.centre = new_centre;
        self.orientation = new_orientation;
        //correct way of splicing together two distinct matrices

        self.camera_extrinsics = na::Matrix4::new(
            self.orientation.m11, self.orientation.m12, self.orientation.m13, self.centre.point.x, 

            self.orientation.m21, self.orientation.m22, self.orientation.m23, self.centre.point.y,

            self.orientation.m31, self.orientation.m32, self.orientation.m33, self.centre.point.z,

            0.0,                  0.0,                  0.0,                  1.0,
            );
        self.extrinsics_inverse = self.camera_extrinsics.clone();
        match self.extrinsics_inverse.try_inverse_mut(){
            true => (), 
            _ => self.extrinsics_inverse = 
                na::Matrix4::new( 
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
                ), 
        };
        
    }

    pub fn update_extrinsics_centre(&mut self, new_centre: Point){
        self.update_extrinsics(new_centre, self.orientation);
    }

    pub fn update_extrinsics_orientation(&mut self, new_orientation: na::Matrix3<f64>){
         self.update_extrinsics(self.centre, new_orientation);
    }
    //Very much subject to change, this is tracer code and needs to be fine tuned
    pub fn update_intrinsics(&mut self){
        self.calibration_matrix = na::Matrix4::new(
                1.0, 0.0, self.centre.point.x, 0.0,
                0.0, 1.0, self.centre.point.y, 0.0,
                0.0, 0.0, 1.0,                 0.0,
                0.0, 0.0, 0.0,                 1.0
            );
    }
    //if 3x4 is fine then refactor necessary, check after tracer code is working 
    pub fn update_superior_matrix(&mut self){
        self.camera_matrix_superior = self.calibration_matrix * self.camera_extrinsics;
    }
    pub fn to_local_coords(&self, point: Point) -> Point{
        Point::vector_to_point(self.extrinsics_inverse * point.point_to_vector()) 
    }
    pub fn update_basis_change_matrix(&mut self){
        //creating new unit vectors for the local coordinates of the camera that are facing
        //the way the camera faces
                
        let unit_vector_x = self.orientation * Point::new(1.0,0.0,0.0,1.0).point_ignore_w(); 
        let unit_vector_y = self.orientation * Point::new(0.0,1.0,0.0,1.0).point_ignore_w();
        let unit_vector_z = self.orientation * Point::new(0.0,0.0,1.0,1.0).point_ignore_w(); 
                
        //manually making a matrix where each column is one of the unit vectors.
        self.basis_change_matrix = na::Matrix3::new(
        unit_vector_x.x, unit_vector_y.x, unit_vector_z.x, 
        unit_vector_x.y, unit_vector_y.y, unit_vector_z.y,
        unit_vector_x.z, unit_vector_y.z, unit_vector_z.z 
                );

    }
    //output from this functiom is not mutable for borrowing and logical, purposes
    pub fn return_visible_objects<'a>(& 'a self, objects: & 'a Vec<coordinate_object>) -> Vec<&coordinate_object>{
        //predeclare before for loop
        let mut visible_objects: Vec<& coordinate_object> = vec![];
        let mut local_point: na::Vector3<f64>;
        let mut depth_difference: f64;
        for object in objects.iter(){
            for point in object.get_points().iter(){
                //because of the way we represent lines, we need to be able to register a line in
                //front of us with a point really far away, this is why the max_depth_difference is
                //used to render objects and not here (it would invalidate lines longer than the
                //max difference)
                
                //apply change of basis to get truly camera oriented coords
                let local_point_world_coords: na::Vector4::<f64>= self.extrinsics_inverse * point.clone().point_to_vector();
                local_point = self.basis_change_matrix * na::Vector3::new(
                    local_point_world_coords.x,
                    local_point_world_coords.y,
                    local_point_world_coords.z
                );
                depth_difference = local_point.z;
                
                print!("point: {:?}\n",depth_difference);
                print!("the moved point in local coords:{:?}\n", local_point);
                print!("depth_difference: {:?}\n",depth_difference);

                if depth_difference >= self.min_depth_difference{
                    visible_objects.push(&object);
                    break;
                }
            }
        }
        visible_objects
    }
    pub fn get_screen_values(&mut self, objects: &Vec<coordinate_object>) -> Vec<u8>{
        let mut return_values: Vec<u8> = vec![0; (self.screen_x * self.screen_y) as usize];
        let mut visible_objects: Vec<&coordinate_object> = self.return_visible_objects(objects);
        let mut temp_vec: na::Vector4<f64>;

        for vis_obj in visible_objects.iter(){
            match vis_obj{
                coordinate_object::Point_object(point) => {
                    },
                _ => (),
            }
        }
        return_values
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
impl Point_Construct for Camera{
    fn get_points(&self) -> Vec<Point>{
        vec![self.centre]
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
