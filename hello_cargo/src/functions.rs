fn main(){
  cheeku();
  //so this is how you call a funtion in rust
  //now to add two numbers
  let num1:u8=200;
  let num2:u8=10;
  let mut result:u8 = addum(num1,num2);
  println!("the result is {}",result);
}


fn cheeku(){
    println!("hello nikhil this is cheeku");
}

fn addum(item1:u8,item2:u8)->u8{
    //tum kya return kara rhe ho? define by arrow
    return item1 + item2;
}