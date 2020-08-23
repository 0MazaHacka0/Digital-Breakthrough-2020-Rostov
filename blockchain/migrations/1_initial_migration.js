const Vote = artifacts.require("System_Votes");

module.exports = function(deployer) {
  deployer.deploy(Vote);
};
