extern crate spin;

fn main() {
    let mutex = spin::Mutex::new(42);
    println!("{:?}", mutex);
    {
        let x = mutex.lock();
        println!("{:?}, {:?}", mutex, *x);
    }

    let rwlock = spin::RwLock::new(42);
    println!("{:?}", rwlock);
    {
        let x = rwlock.read();
        println!("{:?}, {:?}", rwlock, *x);
    }
    {
        let x = rwlock.write();
        println!("{:?}, {:?}", rwlock, *x);
    }
}
