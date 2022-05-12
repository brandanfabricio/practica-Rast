use std::thread;
use std::sync::{Arc,Mutex};
use std::sync::mpsc;

fn main() {
    thread::spawn(|| {
        println!("Hola desde un hilo!")
    });

    let handle = thread::spawn(|| {
        "hola desde otro hilo!"
    });
    println!("{}", handle.join().unwrap());


    let  data = Arc::new( Mutex::new( vec![1u32,2,3]));
    for i in 0..3 {
        let data = data.clone();

         thread::spawn(move || {
             let mut data = data.lock().unwrap();

            data[i] +=1;
        } );
    }
    thread::sleep_ms(50);





    let dato = Arc::new(Mutex::new(0u32));
    let (tx,rx) = mpsc::channel();
    for _ in 0..10 {
        let (dato, tx)  = (dato.clone(), tx.clone());
        thread::spawn(move || {
            let mut dato = dato.lock().unwrap();
            *dato +=1;
            tx.send(());
        });

    }
   
    for _ in 0..10 {
        rx.recv();
    }




    let (fx,gx) = mpsc::channel();

    for _ in 0..10{
        let fx = fx.clone();
        thread::spawn(move || {
            let respuesta =42u32;
            fx.send(respuesta); 
        });
    }
    gx.recv().ok().expect("no se ha podido revibir la respuesta");


// Panico

    let resultado = thread::spawn(move || {
        panic!("ups! mi error");
    }).join();
    assert!(resultado.is_err());
}
