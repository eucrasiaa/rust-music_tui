enum Shape {
    Circle(f64),                 
    Rect { width: f64, height: f64 },
    Point,                  
}
fn main(){
    
    let shape=Shape::Rect { width: 10.0, height: 10.0 };
    let area = match shape {
        Shape::Circle(r)               => std::f64::consts::PI * r * r,
        Shape::Rect { width: w, height: h } => w * h,
        Shape::Point                   => 0.0,
    };
    println!("Area: {}", area);
    
}
