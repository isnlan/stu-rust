trait Speak: Sync + Clone + Send + 'static {
    fn speak(&self);
}

#[derive(Clone)]
struct  Person {
    name: String
}


impl Speak for Person {
    fn speak(&self) {
        println!("my name {:?}", self.name);
    }
}

struct  Zone<S: Speak> {
    speaker: S,
}

impl <S:Speak>Zone<S> {
    async fn run(&self) {
        let s = self.clone();
        tokio::spawn(async move {
            loop {

            }
        }).await.unwrap();
    }
}

#[cfg(test)]
mod tests{
    use crate::{Person, Zone};
    use std::thread::sleep;
    use std::time::Duration;

    #[actix_rt::test]
    async fn is_work(){
        let p = Person{name:"s1".to_string()};
        let z = Zone{speaker: p};
        z.run().await;
        sleep(Duration::from_secs(2))
    }
}


fn main()  {
}
