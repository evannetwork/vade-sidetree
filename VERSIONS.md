# vade-sidetree

## Next Version

### Features

### Fixes

- Add verification of "published" flag when creating a did
- Retry polling status code is other than 200 from sidetree

### Deprecations

## v0.0.4

### Features

- add support for passing keys to DID creation
- add new flag `waitForCompletion` to the creation and update process
- add typescript typings
- export sidetree-client module
- refactor code to make sidetree-client an internal module

### Fixes

- Fix the correct payload for IETF json patches
- fix in3 request list fallback payload
- update dependencies for vulnerabilities

## v0.0.3

### Features

- Add type options to did_create and did_update

## v0.0.2

### Fixes

- allow resolval only for Sidetree based DIDs

## v0.0.1

- initial implementation of sidetree based DID handling (create/update/delete) in Vade
