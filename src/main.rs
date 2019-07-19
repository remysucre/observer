pub struct Observable<T> {
    subscribers: Vec<Box<dyn Fn(&T)>>,
}

impl<T> Observable<T> {
    pub fn new() -> Observable<T> {
        Observable {
            subscribers: Vec::new(),
        }
    }

    pub fn subscribe<F>(&mut self, callback: F)
    where
        F: 'static + Fn(&T),
    {
        self.subscribers.push(Box::new(callback));
    }

    pub fn notify(&self, item: T) {
        for callback in &self.subscribers {
            callback(&item);
        }
    }
}

fn observer2(item: &usize) {
    println!("Got {:?}", *item)
}

fn observer1(item: &usize) {
    let mut observable2 = Observable::new();
    observable2.subscribe(observer2);
    observable2.notify(*item * 2);
}

fn main() {
    let mut observable1 = Observable::new();
    observable1.subscribe(observer1);
    observable1.notify(5);
}
