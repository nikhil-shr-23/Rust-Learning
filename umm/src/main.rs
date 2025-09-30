fn main() {
    //reference passing 
    let mut s1:String = String::from("hello now the owner is s1");
    appendss(&s1);
    


    
    

}
// dont forget to create references 

fn appendss(s3:&String){
    s3.push_str("this is extensions");
    
}