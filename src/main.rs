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

fn sink1(item: &String) {
    println!("Sink 1 got {:?}", *item)
}

fn sink2(item: &String) {
    println!("Sink 2 got {:?}", *item)
}

fn server1(item: &String) {
    let mut result = Observable::new();
    result.subscribe(sink1);
    result.subscribe(sink2);
    result.notify(item.clone());
}

fn server2(item: &String) {
    let mut result = Observable::new();
    result.subscribe(sink1);
    result.subscribe(sink2);
    result.notify(item.clone());
}

fn main() {
    let mut source1 = Observable::new();
    let mut source2 = Observable::new();

    source1.subscribe(|item: &String| {
        let mut result = Observable::new();
        result.subscribe(sink1);
        result.subscribe(sink2);
        result.notify(item.clone());
    });
    source2.subscribe(server1);
    source1.subscribe(server2);
    source2.subscribe(server2);

    source1.notify("1".to_string());
    source2.notify("2".to_string());
}
