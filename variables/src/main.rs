fn main() {
    /* MUTABILIDADE */
    let x= 5;
    println!("The value of x is: {x}");
    //x = 6; //cannot mutate immutable variable `x`
    println!("The value of x is: {x}");

    let mut y= 5;
    println!("The value of y is: {y}");
    y = 6;
    println!("The value of y is: {y}");


    /* CONSTANTES */
    const PI_NUMBER: f32 = 3.141516;
    println!("Pi number is: {PI_NUMBER}");

    /* SHADOWING */
    let z = 5;
    let z = 6;
    println!("Value of z: {z}");
    
}
