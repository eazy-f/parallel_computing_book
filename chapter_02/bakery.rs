/* compiled on rust 0.10 on April 2014 */
use std::cmp;
use std::io::timer;

fn main() {
    let n = 200;
    let levels = &mut Vec::from_fn(n, |_| { 0 });
    let flags  = &mut Vec::from_fn(n, |_| { false });
    let srd_levels = levels as *mut Vec<int>;
    let srd_flags = flags as *mut Vec<bool>;

    for i in range(0, n - 1) {
        spawn(proc() { loop { worker(i, srd_levels, srd_flags) } } );
    }
    timer::sleep(60 * 1000);
}

fn worker(id: uint, plevels: *mut Vec<int>, pflags: *mut Vec<bool>) {
    let levels = unsafe { (*plevels).as_mut_slice() };
    let flags =  unsafe { (*pflags).as_mut_slice() };

    lock(id, levels, flags);
    println!("task: {}", id);
    timer::sleep(100);
    unlock(id, flags);
}

fn lock(id: uint, levels: &mut [int], flags: &mut [bool]) {
    let mut free = false;
    flags[id] = true;
    /* why Iterator.max_by does use Option<T> ? */
    let own_level = levels.iter().fold(0, |a, &b| cmp::max(a, b)) + 1;
    levels[id] = own_level;

    while !free {
        free = true;
        for i in range(0, levels.len() - 1) {
            if i != id && flags[i] && dominates(i, id, levels) {
                free = false;
            }
        }
        timer::sleep(10);
    }
}

fn unlock(id: uint, flags: &mut [bool]) {
    flags[id] = false;
}

fn dominates(a: uint, b: uint, levels: &[int]) -> bool {
    ( levels[a] < levels[b] ) || ( levels[a] == levels[b] && a < b)
}
