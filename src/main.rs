mod cels_fahr;
use std::io;

fn string_to_num(temp:String) -> f32{
    // get a string and transform it to a float so it can be
    // used in the convertion step
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("not a valid string");

    let num_num : f32 = temp.trim().parse().expect("not a valid number");
    num_num

}

fn main() {
    loop{ 
    println!("what do you want to convert:");

    // get input and direct it to the right file
    let mut contr = String::new();
    io::stdin()
        .read_line(&mut contr)
        .expect("try to write --cnv help for more infos");

    if contr.contains("cels to fahr"){
        println!("input: C(elsius) to F(ahrenheit)");
        let z = string_to_num(contr);
        let y = cels_fahr::convertion(z);
        println!("{}", y)
    }else if contr.contains("fahr to cels"){
        println!("input: F(ahrenheit) to C(elsius)");
        let z = string_to_num(contr);
        let y = cels_fahr::sec_convertion(z);
        println!("{}", y)
    } else{
        break
    }
}
}