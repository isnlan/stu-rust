use std::sync::Arc;

trait Print {
    fn print(&self);
}

#[derive(Clone)]
struct Stub {
    svr: Arc<Option<Box<dyn Print>>>,
}

impl Stub {
    pub fn print(&self) {
        self.svr.as_deref().map(|p| p.print());
    }
}

#[derive(Clone)]
struct Server {
    stub: Stub,
}

impl Print for Server {
    fn print(&self) {
        println!("server");
    }
}

fn main() {
    let mut svr = Server {
        stub: Stub {
            svr: Arc::new(None),
        },
    };

    svr.stub.svr = Arc::new(Some(Box::new(svr.clone())))
}
