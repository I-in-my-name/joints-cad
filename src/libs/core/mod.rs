use nalgebra::{SMatrix};
use std::cmp::Ordering;
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
        let thisValue: f64 = ((i32::pow(self.x,2) + i32::pow(self.y,2) + i32::pow(self.z,2)) as f64).sqrt();
        let otherValue: f64 = ((i32::pow(other.x,2) + i32::pow(other.y,2) + i32::pow(other.z,2)) as f64).sqrt();
    let value = thisValue - otherValue;
        if value < 0.0 {
            Ordering::Less
        } else if value > 0.0{
            Ordering::Equal
        }else{
            Ordering::Greater
        }
    }
}
pub struct Surface{
    //Must change here for curved surfaces along with rendering logic, potentially an enum to
    //describe the type of curve rendering along with a function.
    //
    //Not Just an alias for future editing
    keyPoints: Vec<Point>,
}
pub struct PerspectiveObject{
    //An object is considered to have an orientation and is made up of sides as well as having a
    //centre point.
    orientation: SMatrix<f32, 2, 3>,
    sides: Vec<Surface>,
    centre: Point,
}
impl PerspectiveObject{
    //fn calculateCentre(&self) -> Point{
        
    //}
}
//A quirk of rust being that there is no way to abstract over mutability, these two functions can
//be considered as having entirely different contexts and are thus coupled differently. You could
//argue that this is not in violation of DRY principles and actually leads to decoupled and
//maintainable code
pub fn sidesToPoints_mut(surfaces: &mut Vec<Surface>) -> Vec<Point>{
    let mut pointVector = Vec::new();
    for surface in surfaces.iter_mut() {
        for point in surface.keyPoints.iter_mut(){
            pointVector.push(*point);
        }
    }
    pointVector
}


pub fn sidesToPoints(surfaces: Vec<Surface>) -> Vec<Point>{
    let mut pointVector = Vec::new();
    for surface in surfaces.iter() {
        for point in surface.keyPoints.iter(){
            pointVector.push(*point);
        }
    }
    pointVector 
}
