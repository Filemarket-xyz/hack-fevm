import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import fs from "fs";

const wallabyAccounts: string[] = [];

if (fs.existsSync(".wallaby-key")) {
  wallabyAccounts.push(fs.readFileSync(".wallaby-key").toString().trim());
}

const config: HardhatUserConfig = {
  solidity: {
    version: "0.8.17",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200,
      },
    },
  },
  networks: {
    wallaby: {
      url: "https://wallaby.node.glif.io/rpc/v0",
      chainId: 31415,
      accounts: wallabyAccounts,
    }
  }
};

export default config;
