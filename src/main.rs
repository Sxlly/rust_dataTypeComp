// Shae Sullivan
// 20643731
// Programming Languages - Assignment
// 17/09/24


fn main() {

    //integers

    let x: i32 = 20; //32 bit signed integer
    let y: i64 = -20; //64 bit signed integer

    let a: u32 = 40; //32 bit unsigned integer
    let b: u8 = 240; //8 bit signed integer

    let x2: f32 = 4.65; //32 bit floating point number
    let y2: f64 = 4.767; //64 bit floating point number


    let x3: bool = false; //define x3 as a boolean data type with value false
    let y3: &str = "0"; //define y3 as a string data type with value "0"

    if x3 == false && y3 == false { //if x3 is false and y3 is also false do the following

        println!("x3 and y3 are false") //print to terminal "x3 and y3 are false"
    }

    else { //if above conditions are not satisfied do the following 

        println!("x3 and y3 are true") //print to terminal "x3 and y3 are true"
    }


}
