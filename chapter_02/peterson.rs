use std::io::timer;

fn main() {
    let interested: *mut [bool, ..2] = &mut [false, ..2];
    let mut victim: int = 0;

    victim = 0;
    unsafe { (*interested)[0] = true };
    spawn(proc() { worker(interested) });
    timer::sleep(60 * 1000);
}

fn worker(interested: *mut [bool, ..2]) {
    let slice = unsafe { (*interested).as_mut_slice() };
    println!("interested: {}", slice[0]);
    slice[0] = false
}
