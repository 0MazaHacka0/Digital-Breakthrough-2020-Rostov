const yargs = require('yargs').argv
const Web3 = require('web3')
const fs = require('fs')
const path = require('path')
const toml = require('toml')

const voteId = yargs.vote_id
const userId = yargs.user_id
const state = yargs.state

if ([userId, voteId, state].includes(undefined)) {
    throw new Error('saving id is undefined!')
}

if (![0, 1, 2].includes(state)) {
    throw new Error('state is not valid!')
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

contract.methods.addVoice(+voteId, +userId, +state).send()
    .then(console.log)
    .catch(console.error)
