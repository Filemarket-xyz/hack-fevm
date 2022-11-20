# Mark3d Hack-FEVM submission
## FVM developers important notes
For better developer experience with FVM such tips might be useful:
1. Working version of FVM exists at commit 1a06485a8c0463ea6a7a0604d0ad421bdcc2c953 in experimental/fvm-m2 branch of lotus repository
2. While using local network with default parameters there is huge delay between status Active and deal creation. This can be solved with proper miner config - [reference config](./miner-config.toml).
However, this config should be used after deal creation, because if it is used from start, deal creation will fail(
3. To reduce deal publish time lotus command `lotus-miner storage-deals pending-publish --publish-now` can be used
4. [Great entrypoint for native FVM actors which can be used to kickstart development](https://coinsbench.com/fvm-fvm-create-miner-with-smart-contract-native-actor-and-part-1-9a5d03b41c31)
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
* Collection instance for cloning - [0xf2f1500e39c25b8fa8c6663a6d8510b315f40c21](). Deploy cid - [bafy2bzacebjxsrrb22viny7b3vjkfsto2lpculjbuz7vutveu7jjkw5obxog2]()
* Fraud decider instance - [0x86c9c461e00fcbee97066a12e565bd69abe4c4cf](). Deploy cid - [bafy2bzacecuurqqwbmzzl76uoppllt5c5dc2gxjyatsmz57j3rfgkfga3tacw]()
* Access token instance - [0xa02f7bb288500b7d900dd0ed2317ec125555f6fc](). Deploy cid - [bafy2bzaceciluvjev6d23yzqcy6fkmhp3j22qmcti64ipifxok3x6igq2pgu4]()
* Exchange instance - [0x39ce94751e608e8785cf07ab6c044b5acecda4cb](). Deploy cid - [bafy2bzaceai4bx2dvnjd4xa5fmy262nifvutggdgvp5hebnym7z6lcvi23se4]()
* Created collection - [](). Clone cid - [bafy2bzacedjkgmwqzjmynypz7qhjc4i26n7e5v3rgujxjvvmiywluxql4xyf4]()