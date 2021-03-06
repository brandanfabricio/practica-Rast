use std::thread;
use std::sync::{Mutex,Arc};


struct Mesa {
    tenedores : Vec<Mutex<()>>,
}



struct Filosofo {
    nombre : String,
    izquierda : usize,
    derecha : usize,
}

impl Filosofo{
    fn new(nombre: &str, izquierda:usize, derecha: usize) -> Filosofo {
            Filosofo{
                nombre: nombre.to_string(),
                izquierda: izquierda,
                derecha: derecha,
            }
    }

    fn comer(&self, mesa: &Mesa){
        let _izquierda = mesa.tenedores[self.izquierda].lock().unwrap();
        let _derecha = mesa.tenedores[self.derecha].lock().unwrap();

        println!("{} Esta comiendo. ",self.nombre);

        thread::sleep_ms(1000);
        println!("{} has finalizado de comer. ",self.nombre);
    }
}



fn main() {

    let mesa = Arc::new(Mesa {tenedores:vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let filosofo = vec![
        Filosofo::new("Judith butler",0,1)
        ,Filosofo::new("Guilles Deleuze",1,2)
        ,Filosofo::new("Karl Marx",2,3)
        ,Filosofo::new("Emma Goldman",3,4)
        ,Filosofo::new("Miche Foucault",0,4),
    ];
   let handles: Vec<_> = filosofo.into_iter().map(|f| {
       let mesa = mesa.clone();
       thread::spawn(move || {
           f.comer(&mesa);
       })
   }).collect();

   for h in handles{
       h.join().unwrap();
   }
}
