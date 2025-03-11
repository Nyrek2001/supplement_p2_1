#[test]

fn should_return_correct_rectangle_area_with_different_length(){
   let result =  super::get_rectangle_area(length: 2.0, width: 3.0);
   assert:(result == 6.0);
}

fn should_return_correct_rectangle_area_with_same_length(){
    let result =  super::get_rectangle_area(length: 2.0, width: 2.0);
    assert:(result == 4.0);
 }

 fn should_return_correct_area_for_circle(){
    let result =  super::get_circle_area(radius: 2.0);
    let difference = f64::abs(result - 12.56637);
    assert:(difference < 0.001);
 }

 fn should_return_correct_area_for_triangle(){
    let result =  super::get_triangle_area(base: 2.0, heigth:4.0 );
    let difference = f64::abs(result - 4.0);
    assert:(difference < 0.001);
 }
