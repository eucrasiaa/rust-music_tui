use std::ops::Mul;
use std::cmp::Ordering;
use std::fmt;
enum Shape {
    Circle(f64),                 
    Rect { width: f64, height: f64 },
    Point,                  
}

fn recur_sum(my_list:&[i32]) -> i32{
    match my_list {
        [] => return 0,
        [head] => {
            return *head
        },
        [head,tail @ ..] => return *head + recur_sum(&tail),
    }
}
    

#[derive(Debug, Clone, Copy, PartialEq, )]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;   // associated type: what does * produce?

    fn mul(self, rhs: f64) -> Vec2 {
        Vec2 { x: self.x * rhs, y: self.y * rhs }
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = f64;   // associated type: what does * produce?

    fn mul(self, rhs:Vec2) -> f64 {
        // let mag_one = (self.x * self.x + self.y*self.y).sqrt();
        // let mag_2 = (rhs.x * rhs.x + rhs.y*rhs.y).sqrt();
        // let dot_prod = self.x * rhs.x +  self.y * rhs.y;
        // let cosa = (dot_prod)/ (mag_one * mag_2);
        // let sintheta= (1.0 - cosa.powi(2)).sqrt();

        self.x * rhs.y - self.y * rhs.x
        // Vec2 { x: self.x * rhs, y: self.y * rhs }
    }
}
impl PartialOrd for Vec2 {
    fn partial_cmp(&self, other: &Vec2) -> Option<Ordering> {
        let mag_self  = self.x  * self.x  + self.y  * self.y;
        let mag_other = other.x * other.x + other.y * other.y;
        mag_self.partial_cmp(&mag_other)  // delegate to f64's PartialOrd
    }
}

impl fmt::Display for Vec2{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

struct Line { start: Vec2, end: Vec2 }

impl fmt::Display for Line{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "Line: {} to {}", self.start, self.end)
    }
}

fn main(){
  
    
    let my_vec = Vec2{x:10.0, y:20.0};
    let ot_vec = Vec2{x:3.0,y:7.0};
    println!("my_vec: {}",my_vec);
    println!("my_vec * 3: {}", my_vec*3.0);
    println!("cross: {}", my_vec * ot_vec);
    let line = Line{start:my_vec, end:ot_vec};
    println!("{}",line);
        // let my_int:i32 = 2;
    // let my_flo:f64 = 5.0;
    // let mut outcome:f64 = 0.0;
    //
    // outcome = (my_int as f64) * my_flo;
    // outcome = 2.0 * my_flo;
    // outcome = f64::from(my_int) * my_flo;
    //
    // let nums = [1, 2, 3, 4];
    //
    // println!("recurSum: {}", recur_sum(&nums));
    //
    // match nums.as_slice() {
    //     []               => println!("empty"),
    //     [x]              => println!("singleton: {x}"),
    //     [head, tail @ ..] => println!("head={head}, rest={tail:?}"),
    //     //  ^^^^ head  ^^^^ tail (a slice, not a list)
    // }
    // let shape=Shape::Rect { width: 10.0, height: 10.0 };
    // let area = match shape {
    //     Shape::Circle(r)               => std::f64::consts::PI * r * r,
    //     Shape::Rect { width: w, height: h } => w * h,
    //     Shape::Point                   => 0.0,
    // };
    // println!("Area: {}", area);
    //
}
