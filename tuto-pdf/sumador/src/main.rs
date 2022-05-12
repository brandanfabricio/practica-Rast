fn main() {
    
    let mut rango = 0..10;
    loop {
        match rango.next(){
            Some(x) =>{
                    println!("{}",x);
            },
            None => { break }
        }
    }

        println!(" For de valores primitivos");

    let nums = vec![1,2,3];
    for i in 0..nums.len() {
        println!("{}", nums[i])
    }


        println!("otro tipo de for");

        for num in &nums {
            println!("{}",num)
        }
        
        println!("otro tipo de for");
        for num in &nums {
            println!("{}",*num)
        }


        let unoHastaCien = (1..101).collect::<Vec<i32>>();


        let mayores = (0..100)
                      .find(|x| *x > 42);
        
            match mayores {
                Some(_) => println!("Tenemos alfunos numeros!"),
                None => println!("Nose se encuentra numeros :( "),
            }

        let suma = (1..4).fold(0, |suma, x| suma + x);

        let numeros = vec![5,6,7];
        for num in numeros.iter(){
            println!("{}",num);
        }



}
