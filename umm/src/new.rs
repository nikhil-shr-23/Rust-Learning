fn main(){
    println!("it takes time to learn syntax");
    //learning referencing, dereferencing and auto referencing

    let x:u8 = 5;
    let y = &x;
    println!("y is now a reference to the value of x and it points to the value {}",y);
}