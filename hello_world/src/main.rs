fn main() {
    let x: i32 = 5;
    let _y: i32;

    assert_eq!(x,5);
    println!("Success");

    //show mutability
    let  mut m = 1;
    m = m+2;
    assert_eq!(m,3);
    println!("Success");

    //scope
    let n: i32 = 10;
    let o: i32 = 6;
    {
        let o: i32 = 5;
        println!("The value of o is {}, the value of n is {}", o, n);
    }
    println!("The value of o is {}, the value of n is {}", o, n);

    define_x();

    let  (mut a, b) = (1,2);
    a += b;
    println!("A is {}", a);

}


fn define_x(){
    let x = "hello";
    println!("{} world",x)
}