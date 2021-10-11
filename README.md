# FileShare-WebServer
A web server for sharing files.

*Warning*: This is probably super dangerous, and has not security build in at all. Don't use it ever.

## Using it

Head to http://<ip address>:<given port>/files to see and download files, or use curl or wget to grab them. To upload, use the following command:

```
curl -Ffile=@<filename> http://<ip address>:<port>
```