import {ethers} from "hardhat";
import {Mark3dCollection__factory} from "../typechain-types";
import "@nomicfoundation/hardhat-chai-matchers";

async function main() {
  let accounts = await ethers.getSigners();
  const balance = await ethers.provider.getBalance(accounts[0].address);
  console.log("account balance", balance);
  if (balance.toString() === "0") {
    throw new Error("zero balance");
  }
  const maxPriorityFee = await ethers.provider.send("eth_maxPriorityFeePerGas", []);
  console.log("max priority fee", maxPriorityFee);

  const collectionFactory = new Mark3dCollection__factory(accounts[0]);

  await collectionFactory.deploy({
    gasLimit: 1000000000,
    maxPriorityFeePerGas: maxPriorityFee,
  });
  console.log("collection deployed. check address in the explorer");
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});