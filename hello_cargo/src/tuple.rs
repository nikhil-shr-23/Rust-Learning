fn main(){
    println!("this is demonstration of tuples");
    let emp_info:(&str,u8)=("Nikhil",21);
    let emp_name= emp_info.0;
    let emp_age = emp_info.1;

    println!("emp name is {}, and emp age is {}",emp_name,emp_age);

    //demo of tuples 
}