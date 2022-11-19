import * as hre from "hardhat";
import { program } from "commander";
import {Mark3dAccessToken__factory} from "../typechain-types";
import "@nomicfoundation/hardhat-chai-matchers";

async function main() {
  program.option("-collection, --collection <string>");
  program.option("-decider, --decider <string>")
  program.parse();
  const args = program.opts();

  let accounts = await hre.ethers.getSigners();
  const balance = await hre.ethers.provider.getBalance(accounts[0].address);
  console.log("account balance", balance);
  if (balance.toString() === "0") {
    throw new Error("zero balance");
  }
  const maxPriorityFee = await hre.ethers.provider.send("eth_maxPriorityFeePerGas", []);
  console.log("max priority fee", maxPriorityFee);

  const accessTokenFactory = new Mark3dAccessToken__factory(accounts[0]);

  await accessTokenFactory.deploy("Mark3D Access Token", "MARK3D", "",
    args.collection, true, args.decider, {
      gasLimit: 1000000000,
      maxPriorityFeePerGas: maxPriorityFee,
    });
  console.log("access token deployed. check address in the explorer");
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});