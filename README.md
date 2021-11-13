# rust-file-database


### Create a db file in the root dir of project
```
touch database.dawn
``` 

### Insert values
```
cargo run set name:Dogukan surname:Akkaya age:19 job:"Software Developer"
```

### Get value
```
cargo run get job
```

### Delete value
```
cargo run del job
```