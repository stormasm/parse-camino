
```rust
cre getnlines ~/j/tmp17/dataset/json/shakespeare_6.0.json 10
```

So the above will create a json file...

You will then take a random sample of the **text_entries** from
the generated json file above,
and combine it with each one of the dates below to create your test
moravec file...

Then combined with this

```rust
seq date -b '2021-01-01' -e '2021-12-31'
```

[shakespeare.md](./shakespeare.md)

[shakespeare test csv file](https://github.com/stormasm/nudata/blob/main/csv/shakespeare.csv)

[shakespeare_nu6.csv](https://github.com/stormasm/dataset/tree/main/csv)
