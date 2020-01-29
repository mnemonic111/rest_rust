// src/main.rs

#[macro_use]
extern crate nickel;

//Importamos mediante use ( despues de indicar que necesitamos el crate nickel.
use nickel::{Nickel, JsonBody, HttpRouter, Request, Response, MiddlewareResult, MediaType};


fn main() {

    //Creamos una variable mutable llamada server. Mediante la funcion asociada new de Nickel
    let mut server = Nickel::new();

    //El router del servidor mediante la funcion router.
    let mut router = Nickel::router();

    //Definimos los diferentes endpoints.

    router.get("/users", hola);

    //Asociamos el servidor a nuestro router.
    server.utilize(router);

    //levantamos el servidor. cualquier IP y puerto 9000.
    server.listen("0.0.0.0:9000");

}

/**
En point para saludar
*/
fn hola<'mw>(_req: &mut Request, _res: Response<'mw>) -> MiddlewareResult<'mw> {
    _res.send("Hola mundo cruel!!") //Ojo, SIN ; ya que devolvemos el middleware como tal.
}
