### Build
```bash
cargo build
```
### Run
```bash
./run-docker-postgress.sh
RUST_LOG=info cargo run --bin flight-service
```
### Test it
```bash
curl -s http://localhost:8080/flights | jq .
curl -s -X PUT "http://localhost:8080/flights/SC/123456"
curl -s http://localhost:8080/flights/701760f3-3e75-1ba8-bbd1-a4fdbc9aeac6 | jq .
curl -s -X DELETE "http://localhost:8080/flights/701760f3-3e75-1ba8-bbd1-a4fdbc9aeac6"
```
