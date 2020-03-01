
/// This is a very rough sketch of what a fully type-safe REST JSON API could look like in Rust,
/// where the server, client, and documentation are all generated programatically.  The only thing
/// you have to actually write is the API, and the individual type-safe handlers.


/// These are some simple types that are used below.
struct UserId(u32);
struct Name(String);
struct User {
    id: UserId,
    name: Name,
}


/// This is an API definition in Rust code.
///
/// This corresponds to an API with two routes:
///
/// -   `/user/create/<id>`
///
///     This takes a POST request body of a JSON string of type `Name`, and returns a response body
///     of a JSON `User`.
///
/// -   `/user/get?sort=true`
///
///     This takes a GET request and returns a response body of a JSON `Vec<User>`.
///
/// The main point here is that this API is able to be defined completely in Rust code, with all
/// the abstractions and normal programming mechanisms that allows.
///
/// As you can see with the `alts!` macro, it might be nice to have some simple macros for defining
/// some things, but in general you should be able to write everything without macros.
fn my_api() -> Api {
    Api::new()
        .path("user")
        .alt(
            alts![
                path("create")
                    .capture("id", "UserId")
                    .body("name", "Name")
                    .ret(POST, "User"),
                path("get")
                    .query("sort", "bool")
                    .ret(GET, "Vec<User>"),
            ]
        )
}


/// Here are handlers for our two routes above.
///
/// Like Rocket, we take in known types corresponding to url captures, query parameters, and
/// request bodies.
///
/// Unlike Rocket, our return types are also type-safe.
///
/// Notice that we don't have to serialize anything ourselves.  `generate_server!` will take care
/// of this for us.

fn handler_user_create(userId: UserId, name: Name) -> User {
    todo!();
}

fn handler_users_get(sort: bool) -> Vec<User> {
    todo!();
}


/// This is where the magic really happens.
///
/// This takes our `Api` type (`my_api`), and the handlers we have defined above
/// (`handler_user_create` and `handler_users_get`), and ties them together.
///
/// This macro creates a function that returns a type like `Vec<rocket::Route>` that we could pass
/// directly to Rocket to serve for us.
///
/// The function this macro returns is responsible for taking a `rocket::Request`, pulling out the
/// needed url captures, query parameters, and request bodies, deserializing them, and feeding them
/// into the given handler.
///
/// It then takes the response from the handler, serializes it, and gives it back to Rocket as a
/// `Response`.
///
/// The neat thing about this approach is that it is general enough to be used with multiple web
/// frameworks.  There is nothing about this that is specific to a single web framework.
generate_server!(my_api,
    server_alts![
        handler_user_create,
        handler_users_get,
    ]
);


/// Given that we have a well-defined web api, we can also easily generate a client.
///
/// The following macro would generate a client that looks like the following:
///
/// ```
/// struct Client{..};
///
/// impl Client {
///     fn user_create(id: UserId, name: Name) -> User {
///         ...
///     }
///     fn users_get(sort: bool) -> Vec<User> {
///         ...
///     }
/// }
/// ```
///
/// This can easily be used to query the API.
///
/// This is a pretty simple client for Rust, but this approach is also flexible enough to generate
/// a client that works with a different underlying HTTP client crate, for example.
///
/// Also, it would be possible to generate a client for a different programming language.  For
/// instance, it would be possible to generate a type-safe client for JavaScript.
///
/// This is similar to technologies like swagger.
generate_client!(my_api,
    client_alts![
        "user_create",
        "users_get"
    ]
);


/// Since our API is defined programatically, it should also be possible to generate documentation,
/// similar to the docs I wrote for `my_api`.  This is the type of thing you should be able to give
/// to the frontend team at your company when they want to access your web api.
///
/// This is much less error prone than writing docs by hand.
generate_docs!(my_api);
