import {ethers} from "hardhat";
import {Mark3dExchange__factory} from "../typechain-types";
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

  const exchangeFactory = new Mark3dExchange__factory(accounts[0]);
  await exchangeFactory.deploy({
    gasLimit: 1000000000,
    maxPriorityFeePerGas: maxPriorityFee,
  });
  console.log("exchange deployed. check address in the explorer");
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});