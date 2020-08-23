const { iteratee } = require("lodash");

const Vote = artifacts.require("System_Votes");

contract('Vote', accounts => {
    it('deploy', async () => {
        await Vote.deployed();
    })
})