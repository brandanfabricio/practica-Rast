use std::collections::{HashMap,HashSet};

use movies::play;
use Pelicula::english::comedy::reproducir;


fn main() {
    
hola("fabricio");
 let resiltado :i32 = suma(10 ,5);
 println!("Resultado de la suema = {}",resiltado);    
 
 tupla();
    let x:(i32,bool,f64) = (2,true,1.5);

destruturacion(x);

    matris();

    let v = vec![10,20,30];
    print_vector(&v);
    println!("{}",v[0]);

    slice();

    let empelado = Empleado{
        compania:String::from("origen"),
        name:String::from("Fabricio"),
        año:24
    };

    println!("Nombre :{} compania :{} edad:{}",empelado.name,empelado.compania,empelado.año);


    let pl = Point::getInstance(10, 30);
    pl.display();
    
    let mujer = GeneroCaregoria::Femenino;
    let hombre = GeneroCaregoria::Masculino;

    println!("{:?}",mujer);
    println!("{:?}",hombre);

    print_size(CarType::Hatch);
    print_size(CarType::Sedan);
    print_size(CarType::SUV);


    match is_even(5){
         Some(data) =>{
             if data==true {
                 println!("Event no");
             }
         }
         None => {
             println!("not event")
         }
    };


    let p1 = usuario::Name(String::from("Fabricio"));
    let p2 =  usuario::Usr_ID(100);
    println!("{:?}",p1);
    println!("{:?}",p2);
    match p1 {
        usuario::Name(val)=>{
            println!("{}",val)
        }
        usuario::Usr_ID(val)=>{
            println!("{}",val)
        }
    };


    play("Hola desde otro play".to_string());

        reproducir("morta kombar".to_string());
        reproducir("la momia".to_string());
 Pelicula::english::comedy::reproducir("batman".to_string());

            //vectores
    let mut vec = Vec::new();

    vec.push(20);
    vec.push(30);
    vec.push(40);
    vec.push(50);

    println!("Tamaño del vector : {}",vec.len());
    println!("{:?}",vec);

    //HashMap
    let mut stateCodes = HashMap::new();

    stateCodes.insert("FB","fabricio");
    stateCodes.insert("TB","tamara");

    println!("{:?}",stateCodes);

    match stateCodes.get(&"FB"){
        Some(value)=>{
            println!("Llave encontrada {}",value)
        }
        None =>{
            println!("Llave No encontrada")
        }
    }

    for (key,val) in stateCodes.iter(){
        println!("Key: {} value: {} ",key,val)
    };
    stateCodes.remove(&"TB");
    for (key,val) in stateCodes.iter(){
        println!("Key: {} value: {} ",key,val)
    };
    

    //HASHSET
        println!("Hashset");
    let mut names = HashSet::new();

    names.insert("Fabricioooo");
    names.insert("Tamaraaa");
    names.insert("Negro");

    println!("{:?}",names);
    
    for name in names.iter() {
        println!("nombre : {}",name)
    };

    match names.get(&"Negro"){
        Some(value)=>{
            println!("Si se encuentra : {}",value);
        }
        None => {
            println!("No se encuentra en el hashset");
        }
    }

}   










pub mod Pelicula {
    pub mod english {
        pub mod comedy {
            pub fn reproducir(name:String){
                println!("Reproducionde pelicula de nombre {}",name)
            }
        }
    }
}

pub mod movies {
    pub fn play(name:String){
        println!("Play movie {}",name);
    }
}





#[derive(Debug)]
enum usuario {
    Name(String),Usr_ID(i32)
}


fn is_even(no:i32)-> Option<bool>{
        if no%2 == 0 {
            Some(true)
        }else{
            None
        }
}


enum CarType{
    Hatch,
    Sedan,
    SUV
}

fn print_size(car:CarType) {
    match car {
        CarType::Hatch =>{
            println!("Small size car");
        },
        CarType::Sedan =>{
            println!("medium size car");
        }
        CarType::SUV => {
            print!("Large sise sport Utility car")
        }

    }
}




#[derive(Debug)]
enum GeneroCaregoria{
    Masculino,Femenino
}


fn suma(x:i32,y:i32) -> i32 {
        x + y
}
fn hola(x:&str){
        println!("Hola {}",x)
}

fn tupla(){
    let tuplaa:(i32,f64,u8) = (-23,4.6,22);
    println!("valor de la tupla {:?}",tuplaa);
}


fn destruturacion(x:(i32,bool,f64)){
     let (ano,gato,plata) = x;
     println!("Tienes gato {}, caul es su edad {} año, peso {} ",gato,ano,plata);
}

fn matris(){
    let arr:[i32;4] = [10,20,30,12];
    println!("Arr {:?}",arr);

    for index in 0..4 {
        println!("proxicion = {}  valor = {}",index,arr[index]);
    }

    for x in arr.iter() {
        println!("{}",x);
    }

}


fn print_vector(x:&Vec<i32>){
    println!("Inside print_vector function {:?}",x);
 }

 fn slice(){
     let n1 = "Tutorial".to_string();
     let c1 = &n1[4..8];
     println!("{}",c1);
 }

 //structuras
 struct Empleado {
     name:String,
     compania:String,
      año: u32
 }

 struct Point{
     x:i32,
     y:i32
 }
 impl Point {
     fn getInstance(x:i32,y:i32) -> Point {
         Point{x:x , y:y}
     }
     fn display (&self){
         println!("x = {} y = {} ",self.x,self.y)
     }
 }