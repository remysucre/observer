pub struct Observable<T> {
    observers: Vec<Box<dyn Fn(&T)>>,
}

impl<T> Observable<T> {
    pub fn new() -> Observable<T> {
        Observable {
            observers: Vec::new(),
        }
    }

    pub fn subscribe<F>(&mut self, observer: F)
    where
        F: 'static + Fn(&T),
    {
        self.observers.push(Box::new(observer));
    }

    pub fn notify(&self, item: T) {
        for observer in &self.observers {
            observer(&item);
        }
    }
}

fn sink(item: &usize) {
    println!("Got {:?}", *item)
}

fn server(item: &usize) {
    let mut result = Observable::new();
    result.subscribe(sink);
    result.notify(*item * 2);
}

fn main() {
    let mut source = Observable::new();
    source.subscribe(server);
    source.notify(1);
}
