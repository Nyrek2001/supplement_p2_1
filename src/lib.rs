/// # Description
/// This function takes a length and width as a paremeter and returns the area of a rectangle
/// # Paremeters
/// - length the length of the rectangle
/// - width the width of the rectangle
/// the area of the rectangle with the specified dimensions.
pub fn get_rectangle_area(length: f64, width: f64) -> f64{
   
    if length == width{
       return length*length 
    }
    length*width   }
/// # Description
/// This function takes a radius as a paremeter and returns the area of a circle
/// # Paremeters
/// - radius the radius of the circle
/// the area of the circle with the specified dimensions.

pub fn get_circle_area(radius: f64) -> f64{
    std::f64::consts::PI * f64::powi(radius, 2)
}
/// # Description
/// This function takes a base and height as a paremeter and returns the area of a triangle
/// # Paremeters
/// - base the base of the triangle
/// - height the height of the triangle
/// the area of the triangle with the specified dimensions.
pub fn get_triangle_area(base: f64, heigth: f64) -> f64{
    base * height * 0.5
}
 mod test:
