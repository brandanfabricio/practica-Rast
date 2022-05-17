use std::fmt::Display;
use std::io::Write;

fn main() {

    let f = is_even(10);
    match f {
        Ok(f) =>{

            println!("{}",f)
        },
        Err(e)=>{
            println!("Error msg {}",e)

        }
    }


    //let resul = is_even(10).unwrap();
    
    //let arch = File::open("xxx.txt").expect("El archivo no existe");


    let t:Data<i32> = Data{values:245};
    println!("values is {}",t.values);
    let t2:Data<String> = Data{values:"Fabricio".to_string()};
    println!("values is {}",t2.values);


    let b1 = Book{
        id:1001,
        name:"Rust en accion"
    };
    b1.print();
    print_pro(10 as u8);


    print_pro(20 as u16);
    print_pro("hollo tutorial");


    let mut linea = String::new();
    println!("Entra tu nombre");
    let b1 = std::io::stdin().read_line(&mut linea).unwrap();
    println!("Hola , {}",linea);
    println!("no of bytes read, {}",b1);

    let b2 = std::io::stdout().write("Tutorial ".as_bytes()).unwrap();

    let b3 = std::io::stdout().write(String::from("Point").as_bytes()).unwrap();
        std::io::stdout().write(format!("\nbytes written {}",(b2+b3)).as_bytes()).unwrap();

        let cmd_linea = std::env::args();
        println!("No of elementos in arguments is _{}",cmd_linea.len());

        let mut sum = 0;
        let mut has_read_first_arg = false;
        for arg in cmd_linea {
            if has_read_first_arg {
                sum += arg.parse::<i32>().unwrap();

            }
            has_read_first_arg = true;
        }


        println!("Suma is {}",sum);


        // for arg in cmd_linea {
        //     println!("[{}]",arg);
            
        // }









    println!("End of main");
}











fn print_pro<T:Display>(t:T){
    println!("Inside printo_pro function:");
    println!("{}",t)
}



struct Book{
    name :&'static str,
    id:u32
}
trait Printable{
    fn print(&self);
}
impl Printable for Book {
    fn print(&self){
        println!("Printing booj whith id:{} and name:{}",self.id,self.name)
    }
}



struct Data<T>{
    values:T
}

fn is_even(no:i32)->Result<bool,String>{
    if no%2 == 0{
        return Ok(true)
    }else {
        return Err("No Es Par".to_string());
    }
}
