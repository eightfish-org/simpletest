
### bootstrap

```
docker compose -f devmode-1node-05s-tmpl.yml up
```

### deploy wasm file 

install ef-cli

```
cargo install --git https://github.com/eightfish-org/ef-cli
```

deploy test wasm file

```
RUST_LOG=info ef-cli --file ./simpletest.wasm --proto simpletest --version 1
```

wait for about 20 seconds.

### tests

make sure you installed `hurl`, is not, use: `cargo install hurl` to install it.

and then

```
# create a new article
hurl new_article.hurl

# query this article
hurl get_one_article.hurl --variable id=5wzxHoJnQd5QhbGcdKkesGiEwtUkynPY4JFrUrm9Us5q  # change this id str

# check the version of this example
hurl version.hurl
```

