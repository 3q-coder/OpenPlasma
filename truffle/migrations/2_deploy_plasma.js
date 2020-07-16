const Plasma = artifacts.require("./Plasma.sol");

module.exports = function(deployer, network, accounts) {
  console.log("deploying to network: " + network);
  
  deployer
    .then(() => {
      return Promise.all([deployer.deploy(Plasma)]);
    })
    .then(() => {
      console.log(">>>>>>>> contracts deployed by deploy_plasma:");
      console.log("Plasma:", Plasma.address);
      console.log("");
    });
};
