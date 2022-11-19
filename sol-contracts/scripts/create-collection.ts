import * as hre from "hardhat";
import { program } from "commander";
import {Mark3dAccessToken__factory, Mark3dCollection__factory} from "../typechain-types";

const genRanHex = (size: number) =>
  [...Array(size)]
    .map(() => Math.floor(Math.random() * 16).toString(16))
    .join("");

async function main() {
  program.option("-instance, --instance <string>");
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
  const collectionFactory = new Mark3dCollection__factory(accounts[0]);
  const accessToken = accessTokenFactory.attach(args.instance);
  const salt = genRanHex(64);
  await accessToken.connect(accounts[0]).createCollection(
    "0x" + salt,
    "TEST",
    "TEST",
    "",
    "ipfs://bafkreigvvqwpop4aeucnjdw6ozjecinuwujka7cjzj7cd323pmsek7mvxu",
    "0x",
    {
      gasLimit: 1000000000,
      maxPriorityFeePerGas: maxPriorityFee,
    });
  const collectionAddress = await accessToken.predictDeterministicAddress("0x" + salt, {
    gasLimit: 1000000000,
    maxPriorityFeePerGas: maxPriorityFee,
  });
  let collectionInstance = collectionFactory.attach(collectionAddress);
  console.log("collection address: ", collectionInstance.address);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});