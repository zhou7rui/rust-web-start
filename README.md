# rust web service start
**rust** **actix-web** **sqlx** **postgres**

## install
```shell
cargo install sqlx-cli
```



## before

```shell
mv .env.example .env
```

## migration
```shell
sqlx migrate run
```

## migration revert
```
sqlx migrate revert
```

## start
```shell
cargo run
```
