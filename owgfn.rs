fn main(){
    let x:String = String::from("Maalik");
    yaar(x);
    println!("the value of x is {}",x);
    //shouldn't work
}


fn yaar(item:String){
    println!("the value of item is {}",item);
}


//the error is 
// println!("the value of x is {}",x);
                             

        


        //    ^ value borrowed here after move







        




    
