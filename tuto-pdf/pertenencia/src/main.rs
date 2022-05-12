fn main() {
    let v = 2;
    let v2 = v;
    println!("v es: {}",v);
    println!("v2 es: {}",v2);
    
    
    let v3 = vec![1,2,3];
    let v4 = v3;
    println!("v4 es: {}",v4[0]);
    
    let x = true;
     let _y = doblar(v);
    let _x = camvio(x);
     println!("{}",v);

     println!("{}",x);


     let mut origen = Punto{x:5 , y:2};

     println!("El origen esta en ({},{})",origen.x,origen.y);
        
        origen.y = 5;

     println!("El origen esta en ({},{})",origen.x,origen.y);
}


fn doblar(x:i32)-> i32 {
    x * 2
}

fn camvio(x:bool) -> bool {
    !x
}

struct Punto{
    x:i32,
    y:i32,
}