use std::thread;

#[no_mangle]

pub extern fn procesar() {
    let handle: Vec<_> = (0..10).map(|_|{
        thread::spawn(|| {
            let mut _x = 0;
            for _ in (0..5_000_000) {
                _x += 1
            }
        })
    }).collect();
    for h in handle {
        h.join().ok().expect("Nose se pudo unir un hilo!")
    }
}