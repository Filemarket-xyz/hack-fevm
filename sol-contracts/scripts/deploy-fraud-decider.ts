import {ethers} from "hardhat";
import {FraudDeciderWeb2__factory} from "../typechain-types";
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

  const fraudDeciderFactory = new FraudDeciderWeb2__factory(accounts[0]);

  await fraudDeciderFactory.deploy({
    gasLimit: 1000000000,
    maxPriorityFeePerGas: maxPriorityFee,
  });
  console.log("fraud decider deployed. check address in the explorer");
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});