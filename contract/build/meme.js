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
function promiseBatchActionTransfer(promiseIndex, amount) {
  env.promise_batch_action_transfer(promiseIndex, amount);
}
var PromiseResult;

(function (PromiseResult) {
  PromiseResult[PromiseResult["NotReady"] = 0] = "NotReady";
  PromiseResult[PromiseResult["Successful"] = 1] = "Successful";
  PromiseResult[PromiseResult["Failed"] = 2] = "Failed";
})(PromiseResult || (PromiseResult = {}));
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

function assert(b, str) {
  if (b) {
    return;
  } else {
    throw Error("assertion failed: " + str);
  }
}

class Vote {
  created_at = blockTimestamp();

  constructor(value, voter) {
    this.value = value;
    this.voter = voter;
  }

}
class Donation {
  // by default, without a constructor, all fields are public
  // so these instance fields will be set from the context
  // and then available on the public interface
  amount = attachedDeposit();
  donor = predecessorAccountId();
  created_at = blockTimestamp();
}
class Meme {
  creator = predecessorAccountId();
  created_at = blockTimestamp();
  vote_score = 0;
  total_donations = BigInt(0);

  constructor(title, data, category) {
    this.title = title;
    this.data = data;
    this.category = category;
  } // ----------------------------------------------------------------------------
  // Basic functions
  // ----------------------------------------------------------------------------


  static create(title, data, category) {
    // data has to have identifier from valid content provider
    assert(isValidMemeData(data), "Data is not valid, must start with valid 9gag.com URL"); // save the meme to storage

    const meme = new Meme(title, data, category);
    this.set(meme);
  }

  static get() {
    return JSON.parse(storageRead(MEME_KEY));
  }

  static set(meme) {
    storageWrite(MEME_KEY, JSON.stringify(meme));
  }

}
/**
 * Handle validation and extraction of meme data
 */

function isValidMemeData(data) {
  return data.startsWith("https://9gag.com");
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

const MEME_KEY = "state";

const PAGE_SIZE = 10;
const MAX_COMMENT_LENGTH = 500;
function getLast(vector, n = PAGE_SIZE) {
  const returnArray = new Array(0);

  for (let i = vector.len() - n - 1; i < vector.len(); i++) {
    returnArray.push(vector.get(i));
  }

  return returnArray;
}
function isInitialized(key) {
  return storageHasKey(key);
}
function isCreator() {
  return predecessorAccountId() === Meme.get().creator;
}
function assertIsInitialized(key) {
  assert(isInitialized(key), "Contract is not initialized");
}
function assertIsNotInitialized(key) {
  assert(!isInitialized(key), "Contract is already initialized");
}
function assertIsSignedByCreator() {
  assert(isCreator(), "This method can only be called by the meme creator");
}
function assertReasonableCommentLength(text) {
  assert(text.length < MAX_COMMENT_LENGTH, `Comment is too long, must be less than ${MAX_COMMENT_LENGTH}`);
}
function assertIsSignerPredecessor() {
  assert(signerAccountId() === predecessorAccountId(), "Users must call this method directly");
}

var _class, _class2;

BigInt.prototype["toJSON"] = function () {
  return this.toString();
}; // The @NearBindgen decorator allows this code to compile to Base64.


let MemeContract = NearBindgen(_class = (_class2 = class MemeContract extends NearContract {
  constructor({
    defaultCall,
    title = "",
    data = "",
    category = Category.A
  }) {
    //execute the NEAR Contract's constructor
    super();

    if (defaultCall) {
      return;
    } // contract may only be initialized once


    assertIsNotInitialized(MEME_KEY); // storing meme metadata requires some storage staking (balance locked to offset cost of data storage)

    assert(attachedDeposit() >= MIN_ACCOUNT_BALANCE, "Minimum account balance must be attached to initialize this contract (3 NEAR)"); // title has to be at least 1 character

    assert(title.length > 0, "Meme title may not be blank"); // create the meme using incoming metadata

    Meme.create(title, data, category);
  }

  default() {
    return new MemeContract({
      defaultCall: true
    });
  }
  /**
   * Return the meme
   */


  get_meme() {
    assertIsInitialized(MEME_KEY);
    return Meme.get();
  } // ----------------------------------------------------------------------------
  // Voting
  // ----------------------------------------------------------------------------

  /**
   * Register a single vote, up or down, for the meme
   */


  vote({
    value
  }) {
    assertIsInitialized(MEME_KEY);
    assertIsSignerPredecessor();
    assert(value === 1 || value === -1, "Invalid vote, must be -1 or 1"); // register the vote

    this.batch_vote({
      value,
      is_batch: false
    });
  }
  /**
   * Register a batched vote where several votes are captured together
   */


  batch_vote({
    value,
    is_batch = true
  }) {
    // register the vote
    if (is_batch) {
      assert(predecessorAccountId() === currentAccountId(), "Batch votes may only be made by the Meme account");
    }

    const voter = is_batch ? "batch-" + predecessorAccountId() : predecessorAccountId(); // allow each account to vote only once

    assert(!this.voters.contains(voter), "Voter has already voted"); // fetch meme from storage

    const meme = Meme.get(); // calculate new score for meme

    meme.vote_score += value; // save it back to storage

    Meme.set(meme); // remember the voter has voted

    this.voters.set(voter); // add the new Vote

    this.votes.push(new Vote(value, voter));
  }
  /**
   * Get a list ofrecent votes
   */


  get_recent_votes() {
    assertIsInitialized(MEME_KEY);
    return getLast(this.votes);
  }
  /**
   * Get the current vote score
   */


  get_vote_score() {
    assertIsInitialized(MEME_KEY);
    return Meme.get().vote_score;
  } // ----------------------------------------------------------------------------
  // Comments
  // ----------------------------------------------------------------------------

  /**
   * Add a comment
   *
   * @param text the text of the comment, max comment length of MAX_COMMENT_LENGTH
   */


  add_comment({
    text
  }) {
    assertIsInitialized(MEME_KEY);
    assertIsSignerPredecessor();
    assertReasonableCommentLength(text);
    this.comments.push(new Comment(text));
  }
  /**
   * Get a list o recent comments
   */


  get_recent_comments() {
    assertIsInitialized(MEME_KEY);
    return getLast(this.comments);
  } // ----------------------------------------------------------------------------
  // Donations
  // ----------------------------------------------------------------------------

  /**
   * Donate tokens to the contract
   */


  donate() {
    assertIsInitialized(MEME_KEY);
    assertIsSignerPredecessor();
    assert(attachedDeposit() > BigInt(0), "Donor must attach some money"); // fetch meme from storage

    const meme = Meme.get(); // record the donation

    meme.total_donations = BigInt(meme.total_donations) + attachedDeposit(); // save it back to storage

    Meme.set(meme); // add new Donation

    this.donations.push(new Donation());
  }
  /**
   * Get a list of donations
   */


  get_donations_total() {
    assertIsInitialized(MEME_KEY);
    return Meme.get().total_donations;
  }
  /**
   * Get a list o recent comments
   */


  get_recent_donations() {
    assertIsInitialized(MEME_KEY);
    return getLast(this.donations);
  }
  /**
   * Transfer all donations to a specified account
   */


  release_donations({
    account
  }) {
    assertIsInitialized(MEME_KEY);
    assertIsSignedByCreator(); // transfer funds to provided account and call ourselves back once transfer is complete

    const promiseBatch = promiseBatchCreate(account);
    promiseBatchActionTransfer(promiseBatch, Meme.get().total_donations);
    const then = promiseThen(promiseBatch, currentAccountId(), "on_donations_released", JSON.stringify({}), BigInt(0), XCC_GAS);
    return promiseReturn(then);
  }
  /**
   * Callback method invoked once donation release is complete
   */


  on_donations_released() {
    log("Donations were released");
  }

}, (_applyDecoratedDescriptor(_class2.prototype, "get_meme", [view], Object.getOwnPropertyDescriptor(_class2.prototype, "get_meme"), _class2.prototype), _applyDecoratedDescriptor(_class2.prototype, "vote", [call], Object.getOwnPropertyDescriptor(_class2.prototype, "vote"), _class2.prototype), _applyDecoratedDescriptor(_class2.prototype, "batch_vote", [call], Object.getOwnPropertyDescriptor(_class2.prototype, "batch_vote"), _class2.prototype), _applyDecoratedDescriptor(_class2.prototype, "get_recent_votes", [view], Object.getOwnPropertyDescriptor(_class2.prototype, "get_recent_votes"), _class2.prototype), _applyDecoratedDescriptor(_class2.prototype, "get_vote_score", [view], Object.getOwnPropertyDescriptor(_class2.prototype, "get_vote_score"), _class2.prototype), _applyDecoratedDescriptor(_class2.prototype, "add_comment", [call], Object.getOwnPropertyDescriptor(_class2.prototype, "add_comment"), _class2.prototype), _applyDecoratedDescriptor(_class2.prototype, "get_recent_comments", [view], Object.getOwnPropertyDescriptor(_class2.prototype, "get_recent_comments"), _class2.prototype), _applyDecoratedDescriptor(_class2.prototype, "donate", [call], Object.getOwnPropertyDescriptor(_class2.prototype, "donate"), _class2.prototype), _applyDecoratedDescriptor(_class2.prototype, "get_donations_total", [view], Object.getOwnPropertyDescriptor(_class2.prototype, "get_donations_total"), _class2.prototype), _applyDecoratedDescriptor(_class2.prototype, "get_recent_donations", [view], Object.getOwnPropertyDescriptor(_class2.prototype, "get_recent_donations"), _class2.prototype), _applyDecoratedDescriptor(_class2.prototype, "release_donations", [call], Object.getOwnPropertyDescriptor(_class2.prototype, "release_donations"), _class2.prototype), _applyDecoratedDescriptor(_class2.prototype, "on_donations_released", [call], Object.getOwnPropertyDescriptor(_class2.prototype, "on_donations_released"), _class2.prototype)), _class2)) || _class;
function init() {
  MemeContract._init();
}
function on_donations_released() {
  let _contract = MemeContract._get();

  _contract.deserialize();

  let args = _contract.constructor.deserializeArgs();

  let ret = _contract.on_donations_released(args);

  _contract.serialize();

  if (ret !== undefined) env.value_return(_contract.constructor.serializeReturn(ret));
}
function release_donations() {
  let _contract = MemeContract._get();

  _contract.deserialize();

  let args = _contract.constructor.deserializeArgs();

  let ret = _contract.release_donations(args);

  _contract.serialize();

  if (ret !== undefined) env.value_return(_contract.constructor.serializeReturn(ret));
}
function get_recent_donations() {
  let _contract = MemeContract._get();

  _contract.deserialize();

  let args = _contract.constructor.deserializeArgs();

  let ret = _contract.get_recent_donations(args);
  if (ret !== undefined) env.value_return(_contract.constructor.serializeReturn(ret));
}
function get_donations_total() {
  let _contract = MemeContract._get();

  _contract.deserialize();

  let args = _contract.constructor.deserializeArgs();

  let ret = _contract.get_donations_total(args);
  if (ret !== undefined) env.value_return(_contract.constructor.serializeReturn(ret));
}
function donate() {
  let _contract = MemeContract._get();

  _contract.deserialize();

  let args = _contract.constructor.deserializeArgs();

  let ret = _contract.donate(args);

  _contract.serialize();

  if (ret !== undefined) env.value_return(_contract.constructor.serializeReturn(ret));
}
function get_recent_comments() {
  let _contract = MemeContract._get();

  _contract.deserialize();

  let args = _contract.constructor.deserializeArgs();

  let ret = _contract.get_recent_comments(args);
  if (ret !== undefined) env.value_return(_contract.constructor.serializeReturn(ret));
}
function add_comment() {
  let _contract = MemeContract._get();

  _contract.deserialize();

  let args = _contract.constructor.deserializeArgs();

  let ret = _contract.add_comment(args);

  _contract.serialize();

  if (ret !== undefined) env.value_return(_contract.constructor.serializeReturn(ret));
}
function get_vote_score() {
  let _contract = MemeContract._get();

  _contract.deserialize();

  let args = _contract.constructor.deserializeArgs();

  let ret = _contract.get_vote_score(args);
  if (ret !== undefined) env.value_return(_contract.constructor.serializeReturn(ret));
}
function get_recent_votes() {
  let _contract = MemeContract._get();

  _contract.deserialize();

  let args = _contract.constructor.deserializeArgs();

  let ret = _contract.get_recent_votes(args);
  if (ret !== undefined) env.value_return(_contract.constructor.serializeReturn(ret));
}
function batch_vote() {
  let _contract = MemeContract._get();

  _contract.deserialize();

  let args = _contract.constructor.deserializeArgs();

  let ret = _contract.batch_vote(args);

  _contract.serialize();

  if (ret !== undefined) env.value_return(_contract.constructor.serializeReturn(ret));
}
function vote() {
  let _contract = MemeContract._get();

  _contract.deserialize();

  let args = _contract.constructor.deserializeArgs();

  let ret = _contract.vote(args);

  _contract.serialize();

  if (ret !== undefined) env.value_return(_contract.constructor.serializeReturn(ret));
}
function get_meme() {
  let _contract = MemeContract._get();

  _contract.deserialize();

  let args = _contract.constructor.deserializeArgs();

  let ret = _contract.get_meme(args);
  if (ret !== undefined) env.value_return(_contract.constructor.serializeReturn(ret));
}

export { MemeContract, add_comment, batch_vote, donate, get_donations_total, get_meme, get_recent_comments, get_recent_donations, get_recent_votes, get_vote_score, init, on_donations_released, release_donations, vote };
//# sourceMappingURL=meme.js.map
