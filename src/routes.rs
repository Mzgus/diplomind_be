use poem::{
    Route, get
};


#[poem::handler]
fn test() -> &'static str {
    "hello"
}

pub fn routes() -> Route {
	Route::new()
		.at("/", get(test))

}