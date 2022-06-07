# Rust Gopher server
A very simple implementation of a gopher server written in pure rust, using only the standard library. 

## Usage
`cargo run` will run the server, however you will probably need to run it as a superuser due to it being hosted on port 70.
All resources are in the resources directory, with the file called gophermap being the root file. Deeper directories are supported.

## Gophermap setup
A basic gophermap setup will look something like this:
```
iThis is an information line, this will be shown as plaintext.
0This is an local link that goes to a text file called test  test.txt  localhost
1This is an local link to another gophermap called test  test  localhost
0This is an external link to a text file on the floodgap server /gopher/relevance.txt	gopher.floodgap.com	70

```
Any connection to a local file will use the localhost address, the program translates it to the correct address on runtime. It does not need a port.
However, if you are linking to an external source such as a text file on another server, the port must be specified.

## TODO
Images are still not finished

