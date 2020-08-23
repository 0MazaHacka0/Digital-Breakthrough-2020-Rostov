const yargs = require('yargs').argv
const Web3 = require('web3')
const fs = require('fs')
const path = require('path')
const toml = require('toml')

const voteId = yargs.vote_id

if (voteId == undefined) {
    throw new Error('getting id is undefined!')
}

const settings = toml.parse(fs.readFileSync(path.resolve('configurations/blockchain.toml'), 'utf-8'));

const web3 = new Web3(new Web3.providers.HttpProvider(`${settings.server.host}:${settings.server.port}`))

web3.eth.personal.unlockAccount(settings.blockchain.account, settings.blockchain.password, 1000)

const contractInfo = JSON.parse((fs.readFileSync(path.resolve(`blockchain/build/contracts/${settings.votes.name}.json`), 'utf-8')))

var contract = new web3.eth.Contract(contractInfo.abi, {
	from: settings.blockchain.account,
	data: contractInfo.bytecode
})

contract.options.address = settings.votes.address

contract.methods.getVote(+voteId).call()
    .then(console.log)
    .catch(console.error)
