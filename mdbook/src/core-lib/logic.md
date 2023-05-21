# Library Core Logic

In this section, we will add the logic for our library and try running it in the Rust client crate.

## API

We will use the [Open Trivia DB](https://opentdb.com/) APIs.

## Fetching Categories

Fetching the categories is the easiest one between those APIs, a get request that requires no query params or anything.

First, let us test it on our terminal.

```bash
curl 'https://opentdb.com/api_category.php'
```

Now let us send the same request from Rust, we're going to start by sending a blocking request, so our program won't continue
executing until we receive something, the result, or the error.

I like to make libraries support both async and blocking requests to give the library users the option to either throw
the concurrency to the library or manage it from the platform side, like using coroutines in Kotlin,
dispatch queue in Swift, or promises in Javascript.

To send a request, we need to add the `reqwest` crate in `quiz_core`.

```bash
cargo add -p quiz_core reqwest --features blocking json
```

> _**📄 Note:** We've added the `json` feature to parse the response as JSON. And we've added the blocking feature to use
> them to have the option of sending blocking requests._

Create a file named `blocking.rs` inside the core library and declare it as a public module in `lib.rs`.

```plantuml,format=svg
@startsalt
{
    {T
        + quiz-library
            ++ Cargo.toml
            ++ apps
                +++ rust_client
                    ++++ Cargo.toml
                    ++++ src
                        +++++ main.rs
            ++ quiz_core
                +++ Cargo.toml
                +++ src
                    ++++ <color:limegreen>blocking.rs
                    ++++ lib.rs
    }
}
@endsalt
```

```rust file=quiz_core/src/lib.rs
pub mod blocking;
```

Now lets add fetch categories request

```rust file=quiz_core/src/blocking.rs
use reqwest::blocking::Client;

pub fn fetch_categories_blocking() {
    let client = Client::new();
    let res = client
        .get("https://opentdb.com/api_category.php")
        .send()
        .unwrap()
        .text()
        .unwrap();

    println!("{:?}", res);
}
```

We will unwrap the result and worry about error handling later.

The final piece is to use this function in our rust client crate.

```rust file=apps/rust_client/src/main.rs
use quiz_core::blocking::fetch_categories_blocking;

fn main() {
    fetch_categories_blocking();
}
```

Run the app.

```bash
cargo run --bin rust_client
```

You should see the same response when we ran the curl command, but uglier.

If we look closely at the output from our code and the curl command, we will notice that the output has a structure:

```plantuml,format=svg
@startjson
{
  "trivia_categories": [
    {
      "id": 9,
      "name": "General Knowledge"
    },
    {
      "id": 17,
      "name": "Science & Nature"
    },
    {
      "id": 23,
      "name": "History"
    },
    {
      "id": 24,
      "name": "Politics"
    },
    {
      "id": 27,
      "name": "Animals"
    }
  ]
}
@endjson
```

So our response contains a field named `trivia_categories`, which has an array of objects (in our case, a structure).
Each object has an `id` and a `name`.

Create a file `dto.rs`. This will store our DTOs (data transfer objects). Add it as a public module to `lib.rs`.

```plantuml,format=svg
@startsalt
{
    {T
        + quiz-library
            ++ Cargo.toml
            ++ apps
                +++ rust_client
                    ++++ Cargo.toml
                    ++++ src
                        +++++ main.rs
            ++ quiz_core
                +++ Cargo.toml
                +++ src
                    ++++ blocking.rs
                    ++++ <color:limegreen>dto.rs
                    ++++ lib.rs
    }
}
@endsalt
```

```rust hl=[2] file=quiz_core/src/lib.rs
pub mod blocking;
pub mod dto;
```

```rust file=quiz_core/src/dto.rs
struct CategoryResponse {
    trivia_categories: Vec<Category>,
}

struct Category {
    id: u32,
    name: String,
}
```

Now to convert the response from a JSON string to our structure, we need to use a crate called serde and serde_json.

```bash
cargo add -p quiz_core serde_json serde --features serde/derive
```

Serde is a framework for efficiently and generically serializing Rust data structures.

Serializing converts a data structure to another representation that can be stored in a file and
transmitted over the network. Deserializing is the opposite action.

Other crates use `serde` to serialize and deserialize to a specific representation. We're using `serde_json` to
convert to and from JSON.

Here we also add the `derive` feature for `serde`. The `derive` feature gives us the ability to declare a structure
to be serializable or deserializable, or both by using the derive macro.

Let us declare the same structure and add `Deserialize` and `Debug` so that we can print the structure in an appealing way.

```rust hl=[1,3,9] file=quiz_core/src/dto.rs
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Category {
    id: u32,
    name: String,
}

#[derive(Deserialize, Debug)]
struct CategoryResponse {
    trivia_categories: Vec<Category>,
}
```

Now we can deserialize the request from JSON to our data structure.

```rust hl=[3,11,14] file=quiz_core/src/blocking.rs
 use reqwest::blocking::Client;

 use crate::dto::CategoryResponse;

 pub fn fetch_categories_blocking() {
     let client = Client::new();
     let res = client
         .get("https://opentdb.com/api_category.php")
         .send()
         .unwrap()
         .json::<CategoryResponse>()
         .unwrap();

     println!("{:#?}", res);
 }
```

And finally, run the rust client.

```bash
cargo run --bin rust_client
```

Our output should be prettier now.

```text
CategoryResponse {
    trivia_categories: [
        Category {
            id: 9,
            name: "General Knowledge",
        },
        Category {
            id: 10,
            name: "Entertainment: Books",
        },
...
```
