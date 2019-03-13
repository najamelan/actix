# Actix Wasm Example

This is quite exiting news, but yes. Actix can run on WASM!!!

Ok, lets calm down. This is a very, very, very draft Proof of Concept example. Currently actix-rt has been extended with a WASM compatible runtime, however most of the API is just shim. It just does nothing... We basically implemented just enough to make this example run. All the rest still has to be done. That means that the unit tests from actix will not pass atm.

So if you need to see it for yourself in order to believe it:

## Dependencies

- install wasm32-unkown-unknown target with rustup
- install wasm-pack

## Usage

```shell
git clone --branch wasm-runtime  https://github.com/najamelan/actix
cd actix/examples/wasm
wasm-pack build --target no-modules
```
If all goes well you should see the last line of the output as:
```
| :-) Your wasm pkg is ready to publish at ./pkg.
```

Now open the index.html file in your browser. If it says:
```
The pong value is: 15
```

than it works!
