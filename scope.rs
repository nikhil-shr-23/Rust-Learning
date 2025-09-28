fn main(){
    let outside_variabel:u8 = 12;

    {
        let inside_variavle:u8 = 10;
        println!("the value of inside variable is {}",inside_variavle);
    }

    println!("the value of outside variable is {}",outside_variabel);
}