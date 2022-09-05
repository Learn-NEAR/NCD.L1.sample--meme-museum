# The Meme Museum

This repository includes contracts for NEAR Academy

## ⚠️ Warning

Any content produced by NEAR, or developer resources that NEAR provides, are for educational and inspiration purposes only. NEAR does not encourage, induce or sanction the deployment of any such applications in violation of applicable laws or regulations.

## Usage

### Getting started

1. clone this repo to a local folder
2. run `npm`
3. run `npm test`

### Top-level `npm` commands

- run `npm test` to run all tests
- run `npm run build` to quickly verify build status

### Other documentation

- **Meme** and **Museum** contract documentation
  - see `contract/README.md` for Meme interface
- integration tests
  - see `/integration-tests/README.md` for integration testing

## The file system

Please note that boilerplate project configuration files have been ommitted from the following lists for simplicity.

### Contracts and Unit Tests

```txt
contract/src
├── meme                          <-- Meme contract
│   ├── contract.ts
│   └── models.ts
├── museum                        <-- Museum contract
│   ├── contract.ts
│   ├── models.ts
└── utils.ts                      <-- shared contract code
```

### Simulation Tests

```txt
integration-tests                 <-- simulation tests
└── src
    ├── main.ava.ts
```
