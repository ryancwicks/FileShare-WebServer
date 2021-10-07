use clap::{Arg, App as ClapApp, value_t};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    //Parse the input arguments.
    let matches = ClapApp::new ("fileshare_webserver")
    .about(
    "This application starts a web server that provides a way for copy files to and from a local directory." )
    .arg(Arg::with_name("directory")
                .short("d")
                .long("directory")
                .takes_value(true)
                .default_value("./")
                .help("What directory to serve. (default, ./)"))
    .arg(Arg::with_name("port")
                .short("p")
                .long("port")
                .takes_value(true)
                .default_value("9000")
                .help("What port to serve (default 9000)"))
    .get_matches();


    let port = clap::value_t!(matches, "port", u16).unwrap_or_else(|e| {
        println!("Failed to set output port: {}",e); 
        e.exit();
    });
    let directory = clap::value_t!(matches, "directory", String).unwrap_or_else(|e| {
        println!("Failed to set directory to serve: {}",e); 
        e.exit();
    });

    println!("Starting file web server \nServing {} on port {}", directory, port);
   
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
