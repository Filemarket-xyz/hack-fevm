# Mark3d Hack-FEVM submission
## FVM developers important notes
For better developer experience with FVM such tips might be useful:
1. Working version of FVM exists at commit 1a06485a8c0463ea6a7a0604d0ad421bdcc2c953 in experimental/fvm-m2 branch of lotus repository
2. While using local network with default parameters there is huge delay between status Active and deal creation. This can be solved with proper miner config - [reference config](./miner-config.toml)
3. [Great entrypoint for native FVM actors which can be used to kickstart development](https://coinsbench.com/fvm-fvm-create-miner-with-smart-contract-native-actor-and-part-1-9a5d03b41c31)
## Native actor examples
Actor code is placed in [simple-actor](./simple-actor) directory

To build it nightly mode is required:
```shell
rustup override set nightly
```

Build:
```shell
cargo build
```

File to deploy will be located at path - `simple-actor/target/debug/wbuild/simple_actor/simple_actor.compact.wasm`

Deploy can be done with following instructions:
```shell
lotus chain install-actor ${path}
lotus chain create-actor ${cid obtained from previous step} ${constructor params}
```

Constructor or invoke params are base64 encoded raw bytes(fvm encoding). They can obtained with [serialization helper](./serialization-helper) package.
## Deploy contracts
### Requirements
First, you need to place your private key in file sol-contracts/.wallaby-key in hex format without 0x prefix.

Factory deploy functions are now returning same address for every contract, so contract addresses
should be obtained from explorer. It can be done with following instructions:
1. Go to deployer account/address page
2. Open deploy transaction page
3. Tap "Click to see more"
4. Scroll down to "Return" section
5. Decode eth address from base64 to hex (for example, with this tool - https://base64.guru/converter/decode/hex)

### Steps
All steps commands must be executed in sol-contracts directory
1. Deploy collection instance for future cloning
```shell
yarn hardhat --network wallaby run scripts/deploy-collection.ts
```
2. Deploy fraud decider
```shell
yarn hardhat --network wallaby run scripts/deploy-fraud-decider.ts
```
3. Deploy access token
```shell
HARDHAT_NETWORK=wallaby ts-node scripts/deploy-access-token.ts --collection {address from first step} --decider {address from second step}
```
4. Deploy exchange
```shell
yarn hardhat --network wallaby run scripts/deploy-exchange.ts
```
5. Create(clone) collection
```shell
HARDHAT_NETWORK=wallaby ts-node scripts/create-collection.ts --instance {address from third step}
```

### Example instances in Wallaby
* Collection instance for cloning - [0x173A9B63CA6BBD8Dbc6241e32Bd20a0cc7b34a9A](https://explorer.glif.io/address/0x173a9b63ca6bbd8dbc6241e32bd20a0cc7b34a9a/?network=wallabynet). Deploy cid - [bafy2bzacecnblpjymmhonthp4odo5t3dgnurnhoppomwiqbvjj2izvx7qlel6](https://explorer.glif.io/tx/bafy2bzacecnblpjymmhonthp4odo5t3dgnurnhoppomwiqbvjj2izvx7qlel6/?network=wallabynet)
* Fraud decider instance - [0x5e23161443792d15728d82146dc86ec34ec803a2](https://explorer.glif.io/address/0x5e23161443792d15728d82146dc86ec34ec803a2/?network=wallabynet). Deploy cid - [bafy2bzacecawnrecqwsl7uzwyv6nha57ubwblvmij26un2jlw56zpgixlxnc6](https://explorer.glif.io/tx/bafy2bzacecawnrecqwsl7uzwyv6nha57ubwblvmij26un2jlw56zpgixlxnc6/?network=wallabynet)
* Access token instance - [0xa784559FEB900D932BCcB3230565aCe5B2511503](https://explorer.glif.io/address/0xa784559feb900d932bccb3230565ace5b2511503/?network=wallabynet). Deploy cid - [bafy2bzacedlpkeezo4ojqxb4f4q3xqtbn5lx7mw4c4boddiqui3cbgswj6jow](https://explorer.glif.io/tx/bafy2bzacedlpkeezo4ojqxb4f4q3xqtbn5lx7mw4c4boddiqui3cbgswj6jow/?network=wallabynet)
* Exchange instance - [0x232eaA6673E9cF2639Bc7aE85040B33E03064808](https://explorer.glif.io/address/0x232eaa6673e9cf2639bc7ae85040b33e03064808/?network=wallabynet). Deploy cid - [bafy2bzaceauqfkkw6x7r5kyd6tv24nb43y7wsboyzkacsneyehbcrnl4rkzk6](https://explorer.glif.io/tx/bafy2bzaceauqfkkw6x7r5kyd6tv24nb43y7wsboyzkacsneyehbcrnl4rkzk6/?network=wallabynet)
* Created collection - [0x12D74ad3a2cFeB6E5ab47aEd2cAff0192337c6c9](https://explorer.glif.io/address/0x12D74ad3a2cFeB6E5ab47aEd2cAff0192337c6c9/?network=wallabynet). Clone cid - []()