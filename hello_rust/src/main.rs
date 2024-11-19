fn main() 
{
    let ret:i32 = plus_one(1);
    println!("Data : {ret}");
}

fn plus_one(int_dt:i32) -> i32 {
 
    return int_dt + 1;
}