use clap::{Arg, App as ClapApp, value_t};
use actix_web::{web, Error, App, HttpResponse, HttpServer};
use std::io::Write;
use actix_multipart::Multipart;
use futures_util::TryStreamExt as _;
use std::path::PathBuf;
use uuid::Uuid;
use actix_files::Files;

//Global variable, set at runtime but used by multiple threads.
struct AppState {
    directory: PathBuf,
}

fn upload() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload File</title></head>
        <body>
            <form target="/upload" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}


fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Simple File Server</title></head>
        <body>
            <h1>Simple File Server</h1>
            <ul>
            <li><a href="/upload">Upload Files</a></li>
            <li><a href="/files">Download Files</a></li>
            </ul>

            <h2>Command line tools:</h2>

            <p>Head to http://<ip address>:<given port>/files to see and download files, or use curl or wget to grab them. To upload, use the following command:</p>

            <p><code>curl -Ffile=@&ltfilename&gt http://&ltip address&gt:&ltport&gt/upload<code></p>

            <p>To download files with wget or curl (add an -O option for Powershell):</p>
            <p><code>wget http://&ltip address&gt:&ltport&gt/files/&ltfilename&gt</code></br>
            <code>curl http://&ltip address&gt:&ltport&gt/files/&ltfilename&gt<code></p>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}

async fn save_file(mut payload: Multipart, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Some(mut field) = payload.try_next().await? {
        // A multipart/form-data stream has to contain `content_disposition`
        let content_disposition = field
            .content_disposition()
            .ok_or_else(|| HttpResponse::BadRequest().finish())?;

        let filename = content_disposition.get_filename().map_or_else(
            || Uuid::new_v4().to_string(),
            |f| sanitize_filename::sanitize(f),
        ); //If a file path is profided, use the sanitized name, otherwise generate a random name.
        
        let directory = &data.directory;
        
        let filepath = (directory).join(filename);

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath)).await?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.try_next().await? {
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await?;
        }
    }

    Ok(HttpResponse::Ok().into())
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
    let temp_directory: String = clap::value_t!(matches, "directory", String).unwrap_or_else(|e| {
        println!("Failed to set directory to serve: {}",e); 
        e.exit();
    });

    println!("Starting file web server \nServing {} on port {}", temp_directory, port);
    
    HttpServer::new(move || {
        App::new()
            .data(AppState {
                directory: PathBuf::from(&temp_directory),
            })
            .service(
                web::resource("/upload")
                    .route(web::get().to(upload))
                    .route(web::post().to(save_file))
            )
            .service(Files::new("/files", &temp_directory).show_files_listing())
            .service(
                web::resource("/")
                    .route(web::get().to(index))
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
