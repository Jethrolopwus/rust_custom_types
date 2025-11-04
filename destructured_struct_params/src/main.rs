

struct Point {
    x: i32,
    y: i32,
}   

fn print_coord(Point {x, y }: Point) {
    println!("Current coordinates are: ({}, {})", x, y);
}   

fn main() {

    let p = Point{x: 0, y: 8};
    match p {
        Point{ x: 0, y } =>println!("On the y axis at {}", y),
        Point{ x, y: 0} => println!("On the x axis at {}", x),
        Point{ x, y } => println!("On neither axis: ({}, {})", x, y),
    }
let p_2 = Point{x: 5, y: 10};
print_coord(p_2);

   
}
