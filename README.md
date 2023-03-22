# rust-blog-server
This is a server made with rust and rocket

# Why Rust? 
I wanted to expriment with Rust and its library Rocket to develop a simple server, I also wanted to use something other then Javascript and React. 

# How to run
Install the packages from Cargo.toml by running either cargo run or cargo install --path [path to Cargo.toml].

For now I'm useing sqlite for a database, you can create a file named [data.db] in the root directory. You will also need to create a [.env] file with [DATABASE_URL=sqlite://data.db] but feel free to name the databse what ever you want!

The last thing you have to do is set up the migrations with disel,
first install the disel CLI with sqllite using this command -> [cargo install diesel_cli --no-default-features --features sqlite],
to setup the migrations run -> [diesel setup].

Now all we need to do is generate the migrations with -> [diesel migration generate create_tasks]. In the migrasion folder you can make a [up.sql] and a [down.sql] to write your sql. 

To actually run the migratons run -> [diesel migration run].
To revert the sql run -> [diesel migration revert]

