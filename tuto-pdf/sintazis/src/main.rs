

fn main() {
  inprimir_numero(5,4);

  let sumar = suma(2);
  println!("el resultado se la fn suma es: {}",sumar);

  //apuntadores a funciom 

  let f :fn(i32)->i32 = mas_uno;
    
let f = mas_uno;
let seis = f(5);
 println!("numero :{}",seis);


        //tipos de valores primitivos
    let a:bool = true;  //booleanos  true false 

    let chars = 'x'; //los char se crean con commilla simpler
    
    //los valores que estan definido  ej: i32 son valores
    //que contiene signos   
    //los valores guardados con ej: u32 no tienen sognos;
    
    //los  flotantes se definesn com f32 , f64
    let float  : f32 = 19.9;

    let aa = [0,1,2,3,4];
    let middle = &aa[1..4]; // 1 a 3
    let complete = &aa[..]; //0 a 4

    // println!("middle {}",middle);
    // println!("complete {}",complete);
                //tupla

    let tu:(i32,&str) = (1,"hola");
   // println!("tupla {}",tu);

    let (z,x,c) = (1,2,3);
    println!("x es {}",x);

            //condicional %if

            let of = 5;
            if of == 5 {
                println!("of es cinco!")
            } else {
                println!("of no es cinco :(")
            }

            let yf = if of == 6 {10}else{15};
                println!("resultado de if {}",yf);


                //ciclos 
                //Rust actualmente provee tres enfoques para realizar actividad iterativa. loop , while y
                //for . Cada uno de dichos enfoques tiene sus propios usos.

                //Loop
               // loop{
                   // println!("esto iterara siempre");
                //}

                //while

                let mut bucle = 5;

                let mut completo = false;

                while !completo {
                    bucle += bucle - 3;
                    println!("{}",bucle);
                    if bucle % 5 == 0 {
                        completo = true;
                    }

                }
        
                
                //for
                for (i,j) in (5..10).enumerate(){
                    println!("i = {} y j = {} ",i,j);
                }

                let lineas = "hola\nmundo".lines();

                for(numero_linea, linea) in lineas.enumerate(){
                    println!("{}: {}",numero_linea,linea);
                };

                //iteracion de manera temprana

                loop{
                    bucle += bucle - 3;
                    println!("{}",bucle);
                    if bucle % 5 == 0 {break;}
                }

                for x in 0..10 {
                    if x % 2 == 0 {continue;}
                    println!("{}",x)
                }

                'exterior:for x in 0..10 {
                    'interior: for y in 0..10 {
                        if x % 2 == 0 {continue 'exterior;}
                        if y % 2 == 0 {continue 'interior}
                        println!("x: {}, y: {}",x,y);
                    }
                }
}









fn inprimir_numero(x:i32, y:i32){
    println!("la suma es: {} ", x+y);

}

fn suma(x:i32) -> i32 {
    x +1
}

//funcion divergente  estas funcoones no retorna nungun valor
fn divergente()-> !{
        panic!("Esta funcion nunca retorna!");
}

fn mas_uno(i:i32)-> i32{
    i + 1
}