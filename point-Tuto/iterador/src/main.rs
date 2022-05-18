use std::ops::Deref;
use std::thread;
use std::time::Duration;



fn main() {
    
      //declare an array
//    let a = [10,20,30];

//    let mut iter = a.iter(); 
   // fetch an iterator object for the array
//    println!("{:?}",iter);

//    //fetch individual values from the iterator object
//    println!("{:?}",iter.next());
//    println!("{:?}",iter.next());
//    println!("{:?}",iter.next());
//    println!("{:?}",iter.next());

//    for data in iter {
//        println!("{}\t",data);
//    }

// let name = vec!["Kanna","kobayashi","toru"];
// for name in name.into_iter(){
//     match name {
//     "kanna" => println!("La amo"),
//     _=> println!("No esta mi amor kobayashi {}",name)
// }
// }

// //println!("{:?}",name)




let is_even = |x|{
    x%2==0 
};
    let no = 13;
    println!("{} is even? {}",no,is_even(no));



    let val = 10;
    let clouse = |x| {
        x + val 
    };
    println!("{}",clouse(5));





 let var_i32  = 5;
    let b = Box::new(var_i32);

    println!("b = {}",b);



let x = 5;
let y = MyBox::new(x);
println!("{}",5==x);
println!("{}",5==*y);


println!("5==x is {}",5==x);
println!("5==*y is {}",5==*y); 
// dereferencing y
println!("x==*y is {}",x==*y);
//dereferencing y




thread::spawn(||{
    for i in 1..10 {
        println!("numero dentro de spawne {}", i );
        thread::sleep(Duration::from_millis(1))
    }
});






for i in 1..5 {
    println!("numeros {} desde fuera",i);
    thread::sleep(Duration::from_millis(1))
}




let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("numero {} desde hilo",i);
            thread::sleep(Duration::from_millis(1));
        }
});

for i in 1..5{
    println!("numero desd fuera {}",i);
    thread::sleep(Duration::from_millis(1));
} 
handle.join().unwrap();









}




struct MyBox<T>(T);


impl<T> MyBox<T> {
    fn new(x:T) -> MyBox<T>{
        MyBox(x)
    }
}
impl<T>Deref for MyBox<T>{
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }

}