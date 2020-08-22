const Web3 = require('web3')
const solc = require('solc')
const fs = require('fs')
const path = require('path')
const toml = require('toml')

const configs = toml.parse(fs.readFileSync(path.resolve('configurations/blockchain.toml'), 'utf-8'))

const web3 = new Web3(new Web3.providers.HttpProvider(`${configs.server.host}:${configs.server.port}`))
web3.eth.personal.unlockAccount(configs.blockchain.account, configs.blockchain.password, 1000)

const contractName = `${configs.contract.contract_name}_${configs.contract.version}`

const input = {
	language: 'Solidity',
	sources: {
		[contractName]: {
			content: fs.readFileSync(path.resolve(__dirname, 'contract.sol'), 'utf-8')
		}
	},
	settings: {
    	outputSelection: {
      		'*': {
        		'*': ['*']
      		}
    	}
  	}
}

const output = JSON.parse(solc.compile(JSON.stringify(input)))

const settingsContract = {
	from: configs.blockchain.account,
	abi: output.contracts[contractName].Storage.abi,
	bytecode: '0x' + output.contracts[contractName].Storage.evm.bytecode.object,
}

fs.writeFileSync(configs.contract.save_to, JSON.stringify(settingsContract));

const contract = new web3.eth.Contract(output.contracts[contractName].Storage.abi)

const contr = contract.deploy({
	data: '0x' + output.contracts[contractName].Storage.evm.bytecode.object,
	arguments: [0]
})
.send({
	from: settingsContract.from,
	data: settingsContract.bytecode,
	gas: '4700000'
}, (error, transaction) => {
	if (error) {
        throw new Error(error)
    }

    console.warn(transaction)

    process.exit(0)
})
