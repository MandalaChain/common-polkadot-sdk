/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */

import { Contract, Interface, type ContractRunner } from 'ethers'
import type { PiggyBank, PiggyBankInterface } from '../PiggyBank'

const _abi = [
	{
		inputs: [],
		stateMutability: 'nonpayable',
		type: 'constructor',
	},
	{
		inputs: [],
		name: 'deposit',
		outputs: [
			{
				internalType: 'uint256',
				name: '',
				type: 'uint256',
			},
		],
		stateMutability: 'payable',
		type: 'function',
	},
	{
		inputs: [],
		name: 'getDeposit',
		outputs: [
			{
				internalType: 'uint256',
				name: '',
				type: 'uint256',
			},
		],
		stateMutability: 'view',
		type: 'function',
	},
	{
		inputs: [],
		name: 'owner',
		outputs: [
			{
				internalType: 'address',
				name: '',
				type: 'address',
			},
		],
		stateMutability: 'view',
		type: 'function',
	},
	{
		inputs: [
			{
				internalType: 'uint256',
				name: 'withdrawAmount',
				type: 'uint256',
			},
		],
		name: 'withdraw',
		outputs: [
			{
				internalType: 'uint256',
				name: 'remainingBal',
				type: 'uint256',
			},
		],
		stateMutability: 'nonpayable',
		type: 'function',
	},
] as const

export class PiggyBank__factory {
	static readonly abi = _abi
	static createInterface(): PiggyBankInterface {
		return new Interface(_abi) as PiggyBankInterface
	}
	static connect(address: string, runner?: ContractRunner | null): PiggyBank {
		return new Contract(address, _abi, runner) as unknown as PiggyBank
	}
}