const { iteratee } = require("lodash");

const Vote = artifacts.require("Vote");

contract('Vote', accounts => {
    it('deploy', async () => {
        await MetaCoin.deployed();
    })
})