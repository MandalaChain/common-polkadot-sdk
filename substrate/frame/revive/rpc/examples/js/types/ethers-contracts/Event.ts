/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */
import type {
	BaseContract,
	BigNumberish,
	BytesLike,
	FunctionFragment,
	Result,
	Interface,
	EventFragment,
	AddressLike,
	ContractRunner,
	ContractMethod,
	Listener,
} from 'ethers'
import type {
	TypedContractEvent,
	TypedDeferredTopicFilter,
	TypedEventLog,
	TypedLogDescription,
	TypedListener,
	TypedContractMethod,
} from './common'

export interface EventInterface extends Interface {
	getFunction(nameOrSignature: 'triggerEvent'): FunctionFragment

	getEvent(nameOrSignatureOrTopic: 'ExampleEvent'): EventFragment

	encodeFunctionData(functionFragment: 'triggerEvent', values?: undefined): string

	decodeFunctionResult(functionFragment: 'triggerEvent', data: BytesLike): Result
}

export namespace ExampleEventEvent {
	export type InputTuple = [sender: AddressLike, value: BigNumberish, message: string]
	export type OutputTuple = [sender: string, value: bigint, message: string]
	export interface OutputObject {
		sender: string
		value: bigint
		message: string
	}
	export type Event = TypedContractEvent<InputTuple, OutputTuple, OutputObject>
	export type Filter = TypedDeferredTopicFilter<Event>
	export type Log = TypedEventLog<Event>
	export type LogDescription = TypedLogDescription<Event>
}

export interface Event extends BaseContract {
	connect(runner?: ContractRunner | null): Event
	waitForDeployment(): Promise<this>

	interface: EventInterface

	queryFilter<TCEvent extends TypedContractEvent>(
		event: TCEvent,
		fromBlockOrBlockhash?: string | number | undefined,
		toBlock?: string | number | undefined
	): Promise<Array<TypedEventLog<TCEvent>>>
	queryFilter<TCEvent extends TypedContractEvent>(
		filter: TypedDeferredTopicFilter<TCEvent>,
		fromBlockOrBlockhash?: string | number | undefined,
		toBlock?: string | number | undefined
	): Promise<Array<TypedEventLog<TCEvent>>>

	on<TCEvent extends TypedContractEvent>(
		event: TCEvent,
		listener: TypedListener<TCEvent>
	): Promise<this>
	on<TCEvent extends TypedContractEvent>(
		filter: TypedDeferredTopicFilter<TCEvent>,
		listener: TypedListener<TCEvent>
	): Promise<this>

	once<TCEvent extends TypedContractEvent>(
		event: TCEvent,
		listener: TypedListener<TCEvent>
	): Promise<this>
	once<TCEvent extends TypedContractEvent>(
		filter: TypedDeferredTopicFilter<TCEvent>,
		listener: TypedListener<TCEvent>
	): Promise<this>

	listeners<TCEvent extends TypedContractEvent>(
		event: TCEvent
	): Promise<Array<TypedListener<TCEvent>>>
	listeners(eventName?: string): Promise<Array<Listener>>
	removeAllListeners<TCEvent extends TypedContractEvent>(event?: TCEvent): Promise<this>

	triggerEvent: TypedContractMethod<[], [void], 'nonpayable'>

	getFunction<T extends ContractMethod = ContractMethod>(key: string | FunctionFragment): T

	getFunction(nameOrSignature: 'triggerEvent'): TypedContractMethod<[], [void], 'nonpayable'>

	getEvent(
		key: 'ExampleEvent'
	): TypedContractEvent<
		ExampleEventEvent.InputTuple,
		ExampleEventEvent.OutputTuple,
		ExampleEventEvent.OutputObject
	>

	filters: {
		'ExampleEvent(address,uint256,string)': TypedContractEvent<
			ExampleEventEvent.InputTuple,
			ExampleEventEvent.OutputTuple,
			ExampleEventEvent.OutputObject
		>
		ExampleEvent: TypedContractEvent<
			ExampleEventEvent.InputTuple,
			ExampleEventEvent.OutputTuple,
			ExampleEventEvent.OutputObject
		>
	}
}
