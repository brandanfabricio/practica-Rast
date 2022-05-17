use std::io::Write;
use std::io::Read;
use std::fs::OpenOptions;


fn main() {
 let mut file = std::fs::File::create("data.txt").expect("Creacion del archivo fallida");

 file.write_all("Hola mundo".as_bytes()).expect("Escritura fallida");
 file.write_all("\nTuroriales".as_bytes()).expect("Escritura fallida");
 println!("creacion y escritura de archivos"); // crear  y escribir 
 
 let mut leer = std::fs::File::open("data.txt").unwrap();
 let mut contenudo = String::new();
 leer.read_to_string(&mut contenudo).unwrap();
 println!("{}",contenudo);
 
    std::fs::remove_file("data.txt").expect("No se puede eliminar el arch");
    println!("Archivo elimunado");

    
 let mut fss = OpenOptions::new().append(true).open("data.txt").expect("cannot open file");
 fss.write_all("hola loco".as_bytes()).expect("write failed");
 fss.write_all("\n tutoloco".as_bytes()).expect("write failed");
 println!("File append success");


 let mut commando : std::env::Args = std::env::args();
 commando.next().unwrap();
    let source = commando.next().unwrap();
    let destidno = commando.next().unwrap();
    let mut file_in = std::fs::File::open(source).unwrap();
    let mut file_out = std::fs::File::create(destidno).unwrap();
    let mut buffer = [0u8;4096];
     loop {
         let nbytes = file_in.read(&mut buffer).unwrap();
         file_out.write(&buffer[..nbytes]).unwrap();
         if nbytes < buffer.len() {break;}
     }

}
