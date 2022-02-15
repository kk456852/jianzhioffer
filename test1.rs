


fn is_zhengchu(n:i32, m:i32) -> bool {
    let mut x = n;
    while x>=m
    {
        x = x - m;
        println!("x is {}", x)
    }
    if x == 0{
        return true;
    }else{
        return false;
    }

}

fn main() {
    let n = 16;
    let m = 2;
    println!("被除数是 {}", n);
    println!("除数是 {}", m);
    println!("能否被整除 {}", is_zhengchu(n,m));
}