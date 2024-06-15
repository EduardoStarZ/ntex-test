use core::fmt;
use ntex::web;
use serde::Deserialize;
use askama::Template;

#[derive(Deserialize)]
#[derive(Debug)]
struct Info {
    username : Option<String>,
    value : Option<String>
}

#[derive(Template)]
#[template(path="form.html")]
struct FormTemplate {}

impl fmt::Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.username.clone().unwrap(), self.value.clone().unwrap())
    }
}

#[web::get("/")]
async fn index(web::types::Query(info) : web::types::Query<Info>) -> web::HttpResponse {
    web::HttpResponse::Ok().body(format!("hiiiii there {}! your value is {}", info.username.unwrap(), info.value.unwrap()))
}

#[web::post("/test")]
async fn index_post(web::types::Form(form) : web::types::Form<Info>) -> web::HttpResponse {                                                
    web::HttpResponse::Ok().body(format!("You just did something i guess\n The values of the submited form: name - {} / value - {} ", form.username.unwrap(), form.value.unwrap()))
}

#[web::get("/form")]
async fn post_form() -> web::HttpResponse {
    web::HttpResponse::Ok().body(FormTemplate{}.render().unwrap())
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let adress : &str = "127.0.0.1";
    let port : u16 = 8000;
    
    println!("Starting server at adress {adress}:{port}");

    web::HttpServer::new(|| {
        web::App::new()
            .service(index)
            .service(index_post)
            .service(post_form)
    })
    .bind((adress, port))?
    .run()
    .await
}
