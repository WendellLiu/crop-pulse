## crop-pulse

### bins

```
# fetch and save daily transactions
cargo run --bin fetch_crop_transactions -- -s 113.01.01

# aggregate the daily crop transactions
cargo run --bin aggregate_daily_crop_transactions -- -s 112.12.28 -e 113.01.01

# calculate the summary data
cargo run --bin daily_crop_summary
```
