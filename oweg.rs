fn main(){
    let a = String::from("hello");
    let b = a;
    //since we are moving from a to b, to a khaali hua since only one 
    //owner can be possible accourding to the rules
    //and if we try accessing a again w get an error
    //rust performs move operation, jaise hi variable out of scope gya, ek hi memory
    //ko do baari free karne ki try - duble free error

    
    println!("the value of b is{}",b);
}