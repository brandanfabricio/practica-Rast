fn main(){
    /** 
     * todas las variables son inmutable 
     *              osea
     * no se pueden cambiar a lo largo
     */
    let x = 10;
    println!("{}",x);

/** 
 * colocar la palabra recerbada "mut"
 *  permiten poner mutabilidad en la variable 
 * 
 */
let  mut a = 10;


println!("{}",a);
a =20;
println!("{}",a);

/**
 * las constante se declasra con la palabra "const" y 
 * se le define el tipo de dato
 * 
 * 
 */

     const Mi_CONSTANTE: u32 = 100_000; // --> 100,000
     println!("{}",Mi_CONSTANTE);

     
}