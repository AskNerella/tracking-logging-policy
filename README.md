## Tracking Logger Policy

This is a custom policy built with PDK and will be applied on Mulesoft Flex Gateway.

## Functionality
Inject a custom header for both the request and response. This header will facilitate tracking the initiation start time and completion times of each request.

## Project Setup
```bash
make setup
make build
make run
```

## Publish to Exchange
```bash
make publish # SNAPSHOT
make release # Stable Version
```