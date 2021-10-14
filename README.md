# FileShare-WebServer
A web server for sharing files.

*Warning*: This is probably super dangerous, and has not security build in at all. Don't use it ever.

## Running it

fileshare_webserver
This application starts a web server that provides a way for copy files to and from a local directory.

USAGE:
    fileshare_webserver.exe [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --directory <directory>    What directory to serve. (default, ./) [default: ./]
    -p, --port <port>              What port to serve (default 9000) [default: 9000]

## Using it

Head to http://<ip address>:<given port>/files to see and download files, or use curl or wget to grab them. To upload, use the following command:

```
curl -Ffile=@<filename> http://<ip address>:<port>
```