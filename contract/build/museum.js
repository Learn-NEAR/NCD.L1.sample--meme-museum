import { readFileSync } from 'fs';

function _applyDecoratedDescriptor(target, property, decorators, descriptor, context) {
  var desc = {};
  Object.keys(descriptor).forEach(function (key) {
    desc[key] = descriptor[key];
  });
  desc.enumerable = !!desc.enumerable;
  desc.configurable = !!desc.configurable;

  if ('value' in desc || desc.initializer) {
    desc.writable = true;
  }

  desc = decorators.slice().reverse().reduce(function (desc, decorator) {
    return decorator(target, property, desc) || desc;
  }, desc);

  if (context && desc.initializer !== void 0) {
    desc.value = desc.initializer ? desc.initializer.call(context) : void 0;
    desc.initializer = undefined;
  }

  if (desc.initializer === void 0) {
    Object.defineProperty(target, property, desc);
    desc = null;
  }

  return desc;
}

function call(target, key, descriptor) {}
function view(target, key, descriptor) {}
function NearBindgen(target) {
  return class extends target {
    static _init() {
      // @ts-ignore
      let args = target.deserializeArgs();
      let ret = new target(args); // @ts-ignore

      ret.init(); // @ts-ignore

      ret.serialize();
      return ret;
    }

    static _get() {
      let ret = Object.create(target.prototype);
      return ret;
    }

  };
}

const U64_MAX = 2n ** 64n - 1n;
const EVICTED_REGISTER = U64_MAX - 1n;
function log(...params) {
  env.log(`${params.map(x => x === undefined ? 'undefined' : x) // Stringify undefined
  .map(x => typeof x === 'object' ? JSON.stringify(x) : x) // Convert Objects to strings
  .join(' ')}` // Convert to string
  );
}
function signerAccountId() {
  env.signer_account_id(0);
  return env.read_register(0);
}
function signerAccountPk() {
  env.signer_account_pk(0);
  return env.read_register(0);
}
function predecessorAccountId() {
  env.predecessor_account_id(0);
  return env.read_register(0);
}
function blockTimestamp() {
  return env.block_timestamp();
}
function attachedDeposit() {
  return env.attached_deposit();
}
function panic(msg) {
  if (msg !== undefined) {
    env.panic(msg);
  } else {
    env.panic();
  }
}
function storageRead(key) {
  let ret = env.storage_read(key, 0);

  if (ret === 1n) {
    return env.read_register(0);
  } else {
    return null;
  }
}
function storageHasKey(key) {
  let ret = env.storage_has_key(key);

  if (ret === 1n) {
    return true;
  } else {
    return false;
  }
}
function storageGetEvicted() {
  return env.read_register(EVICTED_REGISTER);
}

function currentAccountId() {
  env.current_account_id(0);
  return env.read_register(0);
}
function input() {
  env.input(0);
  return env.read_register(0);
}
function promiseThen(promiseIndex, accountId, methodName, args, amount, gas) {
  return env.promise_then(promiseIndex, accountId, methodName, args, amount, gas);
}
function promiseBatchCreate(accountId) {
  return env.promise_batch_create(accountId);
}
function promiseBatchActionCreateAccount(promiseIndex) {
  env.promise_batch_action_create_account(promiseIndex);
}
function promiseBatchActionDeployContract(promiseIndex, code) {
  env.promise_batch_action_deploy_contract(promiseIndex, code);
}
function promiseBatchActionFunctionCall(promiseIndex, methodName, args, amount, gas) {
  env.promise_batch_action_function_call(promiseIndex, methodName, args, amount, gas);
}
function promiseBatchActionAddKeyWithFullAccess(promiseIndex, publicKey, nonce) {
  env.promise_batch_action_add_key_with_full_access(promiseIndex, publicKey, nonce);
}
function promiseBatchActionDeleteAccount(promiseIndex, beneficiaryId) {
  env.promise_batch_action_delete_account(promiseIndex, beneficiaryId);
}
var PromiseResult;

(function (PromiseResult) {
  PromiseResult[PromiseResult["NotReady"] = 0] = "NotReady";
  PromiseResult[PromiseResult["Successful"] = 1] = "Successful";
  PromiseResult[PromiseResult["Failed"] = 2] = "Failed";
})(PromiseResult || (PromiseResult = {}));

function promiseResult(resultIdx) {
  let status = env.promise_result(resultIdx, 0);

  if (status == PromiseResult.Successful) {
    return env.read_register(0);
  } else if (status == PromiseResult.Failed || status == PromiseResult.NotReady) {
    return status;
  } else {
    panic(`Unexpected return code: ${status}`);
  }
}
function promiseReturn(promiseIdx) {
  env.promise_return(promiseIdx);
}
function storageWrite(key, value) {
  let exist = env.storage_write(key, value, EVICTED_REGISTER);

  if (exist === 1n) {
    return true;
  }

  return false;
}
function storageRemove(key) {
  let exist = env.storage_remove(key, EVICTED_REGISTER);

  if (exist === 1n) {
    return true;
  }

  return false;
}

class NearContract {
  deserialize() {
    const rawState = storageRead("STATE");

    if (rawState) {
      const state = JSON.parse(rawState); // reconstruction of the contract class object from plain object

      let c = this.default();
      Object.assign(this, state);

      for (const item in c) {
        if (c[item].constructor?.deserialize !== undefined) {
          this[item] = c[item].constructor.deserialize(this[item]);
        }
      }
    } else {
      throw new Error("Contract state is empty");
    }
  }

  serialize() {
    storageWrite("STATE", JSON.stringify(this));
  }

  static deserializeArgs() {
    let args = input();
    return JSON.parse(args || "{}");
  }

  static serializeReturn(ret) {
    return JSON.stringify(ret);
  }

  init() {}

}

function u8ArrayToBytes(array) {
  let ret = "";

  for (let e of array) {
    ret += String.fromCharCode(e);
  }

  return ret;
} // TODO this function is a bit broken and the type can't be string
// TODO for more info: https://github.com/near/near-sdk-js/issues/78

function bytesToU8Array(bytes) {
  let ret = new Uint8Array(bytes.length);

  for (let i = 0; i < bytes.length; i++) {
    ret[i] = bytes.charCodeAt(i);
  }

  return ret;
}
function bytes(strOrU8Array) {
  if (typeof strOrU8Array == "string") {
    return checkStringIsBytes(strOrU8Array);
  } else if (strOrU8Array instanceof Uint8Array) {
    return u8ArrayToBytes(strOrU8Array);
  }

  throw new Error("bytes: expected string or Uint8Array");
}

function checkStringIsBytes(str) {
  for (let i = 0; i < str.length; i++) {
    if (str.charCodeAt(i) > 255) {
      throw new Error(`string ${str} at index ${i}: ${str[i]} is not a valid byte`);
    }
  }

  return str;
}

function assert(b, str) {
  if (b) {
    return;
  } else {
    throw Error("assertion failed: " + str);
  }
}

const ERR_INDEX_OUT_OF_BOUNDS = "Index out of bounds";
const ERR_INCONSISTENT_STATE$1 = "The collection is an inconsistent state. Did previous smart contract execution terminate unexpectedly?";

function indexToKey(prefix, index) {
  let data = new Uint32Array([index]);
  let array = new Uint8Array(data.buffer);
  let key = u8ArrayToBytes(array);
  return prefix + key;
} /// An iterable implementation of vector that stores its content on the trie.
/// Uses the following map: index -> element


class Vector {
  constructor(prefix) {
    this.length = 0;
    this.prefix = prefix;
  }

  len() {
    return this.length;
  }

  isEmpty() {
    return this.length == 0;
  }

  get(index) {
    if (index >= this.length) {
      return null;
    }

    let storageKey = indexToKey(this.prefix, index);
    return JSON.parse(storageRead(storageKey));
  } /// Removes an element from the vector and returns it in serialized form.
  /// The removed element is replaced by the last element of the vector.
  /// Does not preserve ordering, but is `O(1)`.


  swapRemove(index) {
    if (index >= this.length) {
      throw new Error(ERR_INDEX_OUT_OF_BOUNDS);
    } else if (index + 1 == this.length) {
      return this.pop();
    } else {
      let key = indexToKey(this.prefix, index);
      let last = this.pop();

      if (storageWrite(key, JSON.stringify(last))) {
        return JSON.parse(storageGetEvicted());
      } else {
        throw new Error(ERR_INCONSISTENT_STATE$1);
      }
    }
  }

  push(element) {
    let key = indexToKey(this.prefix, this.length);
    this.length += 1;
    storageWrite(key, JSON.stringify(element));
  }

  pop() {
    if (this.isEmpty()) {
      return null;
    } else {
      let lastIndex = this.length - 1;
      let lastKey = indexToKey(this.prefix, lastIndex);
      this.length -= 1;

      if (storageRemove(lastKey)) {
        return JSON.parse(storageGetEvicted());
      } else {
        throw new Error(ERR_INCONSISTENT_STATE$1);
      }
    }
  }

  replace(index, element) {
    if (index >= this.length) {
      throw new Error(ERR_INDEX_OUT_OF_BOUNDS);
    } else {
      let key = indexToKey(this.prefix, index);

      if (storageWrite(key, JSON.stringify(element))) {
        return JSON.parse(storageGetEvicted());
      } else {
        throw new Error(ERR_INCONSISTENT_STATE$1);
      }
    }
  }

  extend(elements) {
    for (let element of elements) {
      this.push(element);
    }
  }

  [Symbol.iterator]() {
    return new VectorIterator(this);
  }

  clear() {
    for (let i = 0; i < this.length; i++) {
      let key = indexToKey(this.prefix, i);
      storageRemove(key);
    }

    this.length = 0;
  }

  toArray() {
    let ret = [];

    for (let v of this) {
      ret.push(v);
    }

    return ret;
  }

  serialize() {
    return JSON.stringify(this);
  } // converting plain object to class object


  static deserialize(data) {
    let vector = new Vector(data.prefix);
    vector.length = data.length;
    return vector;
  }

}
class VectorIterator {
  constructor(vector) {
    this.current = 0;
    this.vector = vector;
  }

  next() {
    if (this.current < this.vector.len()) {
      let value = this.vector.get(this.current);
      this.current += 1;
      return {
        value,
        done: false
      };
    }

    return {
      value: null,
      done: true
    };
  }

}

const ERR_INCONSISTENT_STATE = "The collection is an inconsistent state. Did previous smart contract execution terminate unexpectedly?";
class UnorderedSet {
  constructor(prefix) {
    this.length = 0;
    this.prefix = prefix;
    this.elementIndexPrefix = prefix + "i";
    let elementsPrefix = prefix + "e";
    this.elements = new Vector(elementsPrefix);
  }

  len() {
    return this.elements.len();
  }

  isEmpty() {
    return this.elements.isEmpty();
  }

  serializeIndex(index) {
    let data = new Uint32Array([index]);
    let array = new Uint8Array(data.buffer);
    return u8ArrayToBytes(array);
  }

  deserializeIndex(rawIndex) {
    let array = bytesToU8Array(rawIndex);
    let data = new Uint32Array(array.buffer);
    return data[0];
  }

  contains(element) {
    let indexLookup = this.elementIndexPrefix + JSON.stringify(element);
    return storageHasKey(indexLookup);
  }

  set(element) {
    let indexLookup = this.elementIndexPrefix + JSON.stringify(element);

    if (storageRead(indexLookup)) {
      return false;
    } else {
      let nextIndex = this.len();
      let nextIndexRaw = this.serializeIndex(nextIndex);
      storageWrite(indexLookup, nextIndexRaw);
      this.elements.push(element);
      return true;
    }
  }

  remove(element) {
    let indexLookup = this.elementIndexPrefix + JSON.stringify(element);
    let indexRaw = storageRead(indexLookup);

    if (indexRaw) {
      if (this.len() == 1) {
        // If there is only one element then swap remove simply removes it without
        // swapping with the last element.
        storageRemove(indexLookup);
      } else {
        // If there is more than one element then swap remove swaps it with the last
        // element.
        let lastElement = this.elements.get(this.len() - 1);

        if (!lastElement) {
          throw new Error(ERR_INCONSISTENT_STATE);
        }

        storageRemove(indexLookup); // If the removed element was the last element from keys, then we don't need to
        // reinsert the lookup back.

        if (lastElement != element) {
          let lastLookupElement = this.elementIndexPrefix + JSON.stringify(lastElement);
          storageWrite(lastLookupElement, indexRaw);
        }
      }

      let index = this.deserializeIndex(indexRaw);
      this.elements.swapRemove(index);
      return true;
    }

    return false;
  }

  clear() {
    for (let element of this.elements) {
      let indexLookup = this.elementIndexPrefix + JSON.stringify(element);
      storageRemove(indexLookup);
    }

    this.elements.clear();
  }

  toArray() {
    let ret = [];

    for (let v of this) {
      ret.push(v);
    }

    return ret;
  }

  [Symbol.iterator]() {
    return this.elements[Symbol.iterator]();
  }

  extend(elements) {
    for (let element of elements) {
      this.set(element);
    }
  }

  serialize() {
    return JSON.stringify(this);
  } // converting plain object to class object


  static deserialize(data) {
    let set = new UnorderedSet(data.prefix); // reconstruct UnorderedSet

    set.length = data.length; // reconstruct Vector

    let elementsPrefix = data.prefix + "e";
    set.elements = new Vector(elementsPrefix);
    set.elements.length = data.elements.length;
    return set;
  }

}

let Category;

(function (Category) {
  Category[Category["A"] = 0] = "A";
  Category[Category["B"] = 1] = "B";
  Category[Category["C"] = 2] = "C";
  Category[Category["D"] = 3] = "D";
})(Category || (Category = {}));

const ONE_NEAR = BigInt(10e24);
const XCC_GAS = BigInt(2 * 10e14);
const MIN_ACCOUNT_BALANCE = ONE_NEAR * BigInt(3); // common keys for singlton instances and initialization
const MUSEUM_KEY = "state"; // size constraints
function isInitialized(key) {
  return storageHasKey(key);
}
function assertIsInitialized(key) {
  assert(isInitialized(key), "Contract is not initialized");
}
function fullAccount(meme) {
  return `${meme}.${currentAccountId()}`;
}

class Museum {
  created_at = blockTimestamp();

  constructor(name) {
    this.name = name;
  } // ----------------------------------------------------------------------------
  // Basic functions
  // ----------------------------------------------------------------------------


  static get() {
    return JSON.parse(storageRead(MUSEUM_KEY));
  }

  static set(museum) {
    storageWrite(MUSEUM_KEY, JSON.stringify(museum));
  }

}

var _class, _class2;
const CODE = readFileSync("../../build/meme.wasm", "utf8").toString();

BigInt.prototype["toJSON"] = function () {
  return this.toString();
};

let MuseumContract = NearBindgen(_class = (_class2 = class MuseumContract extends NearContract {
  constructor({
    owners,
    name,
    defaultCall
  }) {
    super();

    if (defaultCall) {
      return;
    }

    this.memes = new UnorderedSet("memes");
    this.contributors = new UnorderedSet("contributors");
    this.owners = new UnorderedSet("owners"); // contract may only be initialized once

    assertIsInitialized(MUSEUM_KEY); // storing meme metadata requires some storage staking (balance locked to offset cost of data storage)

    assert(attachedDeposit() > MIN_ACCOUNT_BALANCE, `Minimum account balance must be attached to initialize this contract (${MIN_ACCOUNT_BALANCE})`); // Must have least 1 owner account

    assert(owners.length > 0, "Must specify at least 1 owner"); // create the museum using incoming metadata

    assert(name.length > 0, "Museum name may not be blank"); // save the museum to storage

    Museum.set(new Museum(name)); // capture the owners

    this.owners.extend(owners);
    log("museum was created");
  }

  default() {
    return new MuseumContract({
      defaultCall: true
    });
  }

  get_museum() {
    assertIsInitialized(MUSEUM_KEY);
    return Museum.get();
  }

  get_owner_list() {
    assertIsInitialized(MUSEUM_KEY);
    return this.owners.toArray();
  }

  get_meme_list() {
    assertIsInitialized(MUSEUM_KEY);
    return this.memes.toArray();
  }

  get_meme_count() {
    assertIsInitialized(MUSEUM_KEY);
    return this.memes.len();
  }
  /**
   * Manage your status as a contributor
   */


  add_myself_as_contributor() {
    assertIsInitialized(MUSEUM_KEY);
    this.contributors.set(predecessorAccountId());
  }

  remove_myself_as_contributor() {
    assertIsInitialized(MUSEUM_KEY);
    this.contributors.remove(predecessorAccountId());
  }
  /**
   * Add your meme
   */


  add_meme({
    meme,
    title,
    data,
    category
  }) {
    assertIsInitialized(MUSEUM_KEY);
    this.assertIsOwnerOrContributor(); // storing meme metadata requires some storage staking (balance locked to offset cost of data storage)

    assert(attachedDeposit() > MIN_ACCOUNT_BALANCE, `Minimum account balance must be attached to initialize this contract (${MIN_ACCOUNT_BALANCE})`);
    const accountId = fullAccount(meme); // assert(
    //   env.isValidAccountID(accountId),
    //   "Meme name must be valid NEAR account name"
    // );

    assert(this.memes.contains(accountId), "Meme name already exists");
    log("attempting to create meme");
    const promise = promiseBatchCreate(accountId);
    promiseBatchActionCreateAccount(promise);
    promiseBatchActionDeployContract(promise, CODE);
    promiseBatchActionAddKeyWithFullAccess(promise, signerAccountPk(), 0);
    promiseBatchActionFunctionCall(promise, "init", bytes(JSON.stringify({
      title,
      data,
      category
    })), attachedDeposit(), XCC_GAS);
    const then = promiseThen(promise, currentAccountId(), "on_meme_created", bytes(JSON.stringify({
      meme
    })), 0, XCC_GAS);
    return promiseReturn(then);
  }

  on_meme_created({
    meme
  }) {
    const results = promiseResult(0); // Verifying the remote contract call succeeded.
    // https://nomicon.io/RuntimeSpec/Components/BindingsSpec/PromisesAPI.html?highlight=promise#returns-3

    if (results === PromiseResult.Failed) {
      log(`Meme creation for [ ${fullAccount(meme)} ] failed`);
    } else if (results === PromiseResult.NotReady) {
      log(`Meme creation for [ ${fullAccount(meme)} ] is pending`);
    } else {
      log(`Meme creation for [ ${fullAccount(meme)} ] succeeded`);
    }
  }
  /*
   * Governance methods reserved for 101Labs and NEAR admins
   */


  add_contributor({
    account
  }) {
    assertIsInitialized(MUSEUM_KEY);
    this.assertIsSignedByOwner();
    this.contributors.set(account);
    log("contributor was added");
  }

  remove_contributor({
    account
  }) {
    assertIsInitialized(MUSEUM_KEY);
    this.assertIsSignedByOwner();
    this.contributors.remove(account);
  }

  add_owner({
    account
  }) {
    assertIsInitialized(MUSEUM_KEY);
    this.assertIsSignedByOwner();
    this.owners.set(account);
  }

  remove_owner({
    account
  }) {
    assertIsInitialized(MUSEUM_KEY);
    this.assertIsSignedByOwner();
    this.owners.remove(account);
  }

  remove_meme({
    meme
  }) {
    assertIsInitialized(MUSEUM_KEY);
    this.assertIsSignedByOwner();
    const promise = promiseBatchCreate(fullAccount(meme));
    promiseBatchActionDeleteAccount(promise, currentAccountId());
    const then = promiseThen(promise, currentAccountId(), "on_meme_removed", bytes(JSON.stringify({
      meme
    })), 0, XCC_GAS);
    return promiseReturn(then);
  }

  on_meme_removed({
    meme
  }) {
    // TODO: confirm that promise was successful
    log(`[ ${fullAccount(meme)} ] was removed`);
    this.memes.remove(meme);
  }

  assertIsOwnerOrContributor() {
    assert(this.contributors.contains(predecessorAccountId()) || this.owners.contains(predecessorAccountId()), "This method can only be called by a museum contributor or owner");
  }

  assertIsSignedByOwner() {
    assert(this.owners.contains(signerAccountId()), "This method can only be called by a museum owner");
  }

}, (_applyDecoratedDescriptor(_class2.prototype, "get_museum", [view], Object.getOwnPropertyDescriptor(_class2.prototype, "get_museum"), _class2.prototype), _applyDecoratedDescriptor(_class2.prototype, "get_owner_list", [view], Object.getOwnPropertyDescriptor(_class2.prototype, "get_owner_list"), _class2.prototype), _applyDecoratedDescriptor(_class2.prototype, "get_meme_list", [view], Object.getOwnPropertyDescriptor(_class2.prototype, "get_meme_list"), _class2.prototype), _applyDecoratedDescriptor(_class2.prototype, "get_meme_count", [view], Object.getOwnPropertyDescriptor(_class2.prototype, "get_meme_count"), _class2.prototype), _applyDecoratedDescriptor(_class2.prototype, "add_myself_as_contributor", [call], Object.getOwnPropertyDescriptor(_class2.prototype, "add_myself_as_contributor"), _class2.prototype), _applyDecoratedDescriptor(_class2.prototype, "remove_myself_as_contributor", [call], Object.getOwnPropertyDescriptor(_class2.prototype, "remove_myself_as_contributor"), _class2.prototype), _applyDecoratedDescriptor(_class2.prototype, "add_meme", [call], Object.getOwnPropertyDescriptor(_class2.prototype, "add_meme"), _class2.prototype), _applyDecoratedDescriptor(_class2.prototype, "on_meme_created", [call], Object.getOwnPropertyDescriptor(_class2.prototype, "on_meme_created"), _class2.prototype), _applyDecoratedDescriptor(_class2.prototype, "add_contributor", [call], Object.getOwnPropertyDescriptor(_class2.prototype, "add_contributor"), _class2.prototype), _applyDecoratedDescriptor(_class2.prototype, "remove_contributor", [call], Object.getOwnPropertyDescriptor(_class2.prototype, "remove_contributor"), _class2.prototype), _applyDecoratedDescriptor(_class2.prototype, "add_owner", [call], Object.getOwnPropertyDescriptor(_class2.prototype, "add_owner"), _class2.prototype), _applyDecoratedDescriptor(_class2.prototype, "remove_owner", [call], Object.getOwnPropertyDescriptor(_class2.prototype, "remove_owner"), _class2.prototype), _applyDecoratedDescriptor(_class2.prototype, "remove_meme", [call], Object.getOwnPropertyDescriptor(_class2.prototype, "remove_meme"), _class2.prototype), _applyDecoratedDescriptor(_class2.prototype, "on_meme_removed", [call], Object.getOwnPropertyDescriptor(_class2.prototype, "on_meme_removed"), _class2.prototype)), _class2)) || _class;
function init() {
  MuseumContract._init();
}
function on_meme_removed() {
  let _contract = MuseumContract._get();

  _contract.deserialize();

  let args = _contract.constructor.deserializeArgs();

  let ret = _contract.on_meme_removed(args);

  _contract.serialize();

  if (ret !== undefined) env.value_return(_contract.constructor.serializeReturn(ret));
}
function remove_meme() {
  let _contract = MuseumContract._get();

  _contract.deserialize();

  let args = _contract.constructor.deserializeArgs();

  let ret = _contract.remove_meme(args);

  _contract.serialize();

  if (ret !== undefined) env.value_return(_contract.constructor.serializeReturn(ret));
}
function remove_owner() {
  let _contract = MuseumContract._get();

  _contract.deserialize();

  let args = _contract.constructor.deserializeArgs();

  let ret = _contract.remove_owner(args);

  _contract.serialize();

  if (ret !== undefined) env.value_return(_contract.constructor.serializeReturn(ret));
}
function add_owner() {
  let _contract = MuseumContract._get();

  _contract.deserialize();

  let args = _contract.constructor.deserializeArgs();

  let ret = _contract.add_owner(args);

  _contract.serialize();

  if (ret !== undefined) env.value_return(_contract.constructor.serializeReturn(ret));
}
function remove_contributor() {
  let _contract = MuseumContract._get();

  _contract.deserialize();

  let args = _contract.constructor.deserializeArgs();

  let ret = _contract.remove_contributor(args);

  _contract.serialize();

  if (ret !== undefined) env.value_return(_contract.constructor.serializeReturn(ret));
}
function add_contributor() {
  let _contract = MuseumContract._get();

  _contract.deserialize();

  let args = _contract.constructor.deserializeArgs();

  let ret = _contract.add_contributor(args);

  _contract.serialize();

  if (ret !== undefined) env.value_return(_contract.constructor.serializeReturn(ret));
}
function on_meme_created() {
  let _contract = MuseumContract._get();

  _contract.deserialize();

  let args = _contract.constructor.deserializeArgs();

  let ret = _contract.on_meme_created(args);

  _contract.serialize();

  if (ret !== undefined) env.value_return(_contract.constructor.serializeReturn(ret));
}
function add_meme() {
  let _contract = MuseumContract._get();

  _contract.deserialize();

  let args = _contract.constructor.deserializeArgs();

  let ret = _contract.add_meme(args);

  _contract.serialize();

  if (ret !== undefined) env.value_return(_contract.constructor.serializeReturn(ret));
}
function remove_myself_as_contributor() {
  let _contract = MuseumContract._get();

  _contract.deserialize();

  let args = _contract.constructor.deserializeArgs();

  let ret = _contract.remove_myself_as_contributor(args);

  _contract.serialize();

  if (ret !== undefined) env.value_return(_contract.constructor.serializeReturn(ret));
}
function add_myself_as_contributor() {
  let _contract = MuseumContract._get();

  _contract.deserialize();

  let args = _contract.constructor.deserializeArgs();

  let ret = _contract.add_myself_as_contributor(args);

  _contract.serialize();

  if (ret !== undefined) env.value_return(_contract.constructor.serializeReturn(ret));
}
function get_meme_count() {
  let _contract = MuseumContract._get();

  _contract.deserialize();

  let args = _contract.constructor.deserializeArgs();

  let ret = _contract.get_meme_count(args);
  if (ret !== undefined) env.value_return(_contract.constructor.serializeReturn(ret));
}
function get_meme_list() {
  let _contract = MuseumContract._get();

  _contract.deserialize();

  let args = _contract.constructor.deserializeArgs();

  let ret = _contract.get_meme_list(args);
  if (ret !== undefined) env.value_return(_contract.constructor.serializeReturn(ret));
}
function get_owner_list() {
  let _contract = MuseumContract._get();

  _contract.deserialize();

  let args = _contract.constructor.deserializeArgs();

  let ret = _contract.get_owner_list(args);
  if (ret !== undefined) env.value_return(_contract.constructor.serializeReturn(ret));
}
function get_museum() {
  let _contract = MuseumContract._get();

  _contract.deserialize();

  let args = _contract.constructor.deserializeArgs();

  let ret = _contract.get_museum(args);
  if (ret !== undefined) env.value_return(_contract.constructor.serializeReturn(ret));
}

export { MuseumContract, add_contributor, add_meme, add_myself_as_contributor, add_owner, get_meme_count, get_meme_list, get_museum, get_owner_list, init, on_meme_created, on_meme_removed, remove_contributor, remove_meme, remove_myself_as_contributor, remove_owner };
//# sourceMappingURL=museum.js.map
