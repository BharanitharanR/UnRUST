fn main() {
    let mut s1 = String::from("function");
    //s.push_str(" world");
    println!("Owner : {s1}");

    let s2=s1.clone();
    println!("Owner : - {s2}");
    s1.push_str(" appended by s1");
    println!("Owner : {s1} - {s2}");

    let  string_data:String=String::from("Owned");
    transfer_ownership(string_data);
    // string_data lost ownership when it was passed as an arg to function
    //
    // let err_string = string_data;

    // Int data are stored in stack
    //  Not Moved - and only deep copy 
    // Int alignments are similar to clone() and no drop is invoked
    let int_data:i32=5;
    transfer_int_ownership(int_data);
    let mov_int=int_data;
    println!("int data: {int_data}:{mov_int}");

    let mut s1 = String::from("function");
    s1=transfer_ownership_args(s1);
    println!("String data: {s1}");
}


fn transfer_ownership(owned_string:String) {
    println!("The string:{owned_string}");
}

fn transfer_int_ownership(owned_int:i32) {
    println!("The i32:{owned_int}");
}

fn transfer_ownership_args(mut owned_string:String) -> String {
    owned_string.push_str(" , transfer_ownership_args");
    return owned_string;
}