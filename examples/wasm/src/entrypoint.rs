use wasm_bindgen::prelude::*;
// use web_sys::console::log_1;

use actix::prelude::*;
use futures::future::Future;

/// Define `Ping` message
struct Ping(usize);

impl Message for Ping {
    type Result = usize;
}

/// Actor
struct MyActor {
    count: usize,
}

/// Declare actor and its context
impl Actor for MyActor {
    type Context = Context<Self>;
}

/// Handler for `Ping` message
impl Handler<Ping> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, _: &mut Context<Self>) -> Self::Result {
        self.count += msg.0;
        self.count
    }
}



// Called when the wasm module is instantiated
//
#[ wasm_bindgen( start ) ]
//
pub fn main() -> Result<(), JsValue>
{
	System::run( ||
	{
		// log_1( &"hi from wasm".into() );

		// start new actor
		//
		let addr = MyActor { count: 10 }.start();

		// send message and get future for result
		//
		let res = addr.send( Ping(5) );

		Arbiter::spawn
		(
			res.map(|res|
			{
				let window   = web_sys::window  ().expect( "no global `window` exists"        );
				let document = window  .document().expect( "should have a document on window" );
				let body     = document.body    ().expect( "document should have a body"      );

				// Manufacture the element we're gonna append
				//
				let val = document.create_element( "div" ).expect( "Failed to create div" );

				val.set_inner_html( &format!( "The pong value is: {}", res ) );

				body.append_child( &val ).expect( "Coundn't append child" );

			}).map_err(|_| ()),
		);

	}).expect( "Failed to run Actix System" );


/*	// Use `web_sys`'s global `window` function to get a handle on the global window object.
	//
	let window   = web_sys::window()  .expect( "no global `window` exists"        );
	let document = window  .document().expect( "should have a document on window" );
	let _body    = document.body()    .expect( "document should have a body"      );

	// Manufacture the element we're gonna append
	let val = document.create_element( "div" )?;

	val.set_class_name( "application-button" );
	val.set_inner_html( "LogViewer" );

	document.get_element_by_id( "applications" ).unwrap().append_child( &val )?;*/

	Ok(())
}

/*
#[wasm_bindgen]
//
pub fn add(a: u32, b: u32) -> u32 {
	a + b
}
*/
