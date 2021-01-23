use actix_web::{get, post, web, App, HttpServer, Responder, HttpRequest,  HttpResponse};
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use serde::{Deserialize, Serialize};
use actix::{Actor, SyncContext, Addr, SyncArbiter, Message, Handler};
use futures::TryFutureExt;
use std::io::Error;

#[macro_use]
extern crate actix_web;

macro_rules! server_err {
    ($msg:expr) => {
        Err(actix_web::error::ErrorInternalServerError($msg))
    };
}

#[derive(Clone)]
struct DB {
    pub inner: Arc<Mutex<HashMap<String, String>>>,
}

impl DB {
    fn new() -> Self {
        Self{inner:Arc::new(Mutex::new(HashMap::new()))}
    }
}

impl Actor for DB {
    type Context = SyncContext<Self>;
}

impl Handler<UserRegisterRequest> for DB {
    type Result = Result<String, Error>;

    fn handle(&mut self, UserRegisterRequest{username, password}: UserRegisterRequest, ctx: &mut Self::Context) -> Self::Result {
        let mut db = self.inner.lock().unwrap();
        db.insert(username.clone(), password);
        Ok(username)
    }
}

#[derive(Clone)]
struct State {
    pub inner :Addr<DB>
}

impl State {
    fn new() -> Self {
        let db = DB::new();
        let state = SyncArbiter::start(1, move || {
            db.clone()
        });
        Self{inner: state}
    }

    fn get(&self) -> &Addr<DB> {
        &self.inner
    }
}

#[get("/ping")]
async fn pong(req: HttpRequest) -> impl Responder {
    "pong"
}

#[derive(Deserialize, Serialize)]
struct UserRegisterRequest {
    pub username: String,
    pub password: String,
}

impl Message for UserRegisterRequest {
    type Result = Result<String, Error>;
}

#[post("/api/user/register")]
async fn register_user(req: web::Json<UserRegisterRequest>, state: web::Data<State>) -> impl Responder {
    let state = state.get();
    let res = state.send(req.into_inner()).await.unwrap().unwrap();

    res
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = State::new();
    HttpServer::new(move|| App::new()
        .data(state.clone())
        .service(pong)
        .service(register_user)
    )
        .bind("0.0.0.0:9080")?
        .run()
        .await
}
