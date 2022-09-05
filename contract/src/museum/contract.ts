import {
  assert,
  bytes,
  call,
  near,
  NearBindgen,
  NearContract,
  UnorderedSet,
  view,
} from "near-sdk-js";
import {
  assertIsInitialized,
  Category,
  fullAccount,
  MIN_ACCOUNT_BALANCE,
  MUSEUM_KEY,
  XCC_GAS,
} from "../utils";
import { Museum } from "./models";

// TODO: factory-contract pattern still not available in JS, there is an open issue here https://github.com/near/near-sdk-js/issues/189
const CODE = "";

BigInt.prototype["toJSON"] = function () {
  return this.toString();
};

@NearBindgen
export class MuseumContract extends NearContract {
  private memes: UnorderedSet;
  private contributors: UnorderedSet;
  private owners: UnorderedSet;

  constructor({
    owners,
    name,
    defaultCall,
  }: {
    owners?: string[];
    name?: string;
    defaultCall: boolean;
  }) {
    super();

    if (defaultCall) {
      return;
    }

    this.memes = new UnorderedSet("memes");
    this.contributors = new UnorderedSet("contributors");
    this.owners = new UnorderedSet("owners");

    // contract may only be initialized once
    assertIsInitialized(MUSEUM_KEY);

    // storing meme metadata requires some storage staking (balance locked to offset cost of data storage)
    assert(
      near.attachedDeposit() > MIN_ACCOUNT_BALANCE,
      `Minimum account balance must be attached to initialize this contract (${MIN_ACCOUNT_BALANCE})`
    );

    // Must have least 1 owner account
    assert(owners.length > 0, "Must specify at least 1 owner");

    // create the museum using incoming metadata
    assert(name.length > 0, "Museum name may not be blank");
    // save the museum to storage
    Museum.set(new Museum(name));

    // capture the owners
    this.owners.extend(owners);

    near.log("museum was created");
  }

  default() {
    return new MuseumContract({ defaultCall: true });
  }

  @view
  get_museum(): Museum {
    assertIsInitialized(MUSEUM_KEY);
    return Museum.get();
  }

  @view
  get_owner_list(): string[] {
    assertIsInitialized(MUSEUM_KEY);
    return this.owners.toArray();
  }

  @view
  get_meme_list(): string[] {
    assertIsInitialized(MUSEUM_KEY);
    return this.memes.toArray();
  }

  @view
  get_meme_count(): number {
    assertIsInitialized(MUSEUM_KEY);
    return this.memes.len();
  }

  /**
   * Manage your status as a contributor
   */
  @call
  add_myself_as_contributor(): void {
    assertIsInitialized(MUSEUM_KEY);
    this.contributors.set(near.predecessorAccountId());
  }

  @call
  remove_myself_as_contributor(): void {
    assertIsInitialized(MUSEUM_KEY);
    this.contributors.remove(near.predecessorAccountId());
  }

  /**
   * Add your meme
   */
  @call
  add_meme({
    meme,
    title,
    data,
    category,
  }: {
    meme: string;
    title: string;
    data: string;
    category: Category;
  }): void {
    assertIsInitialized(MUSEUM_KEY);
    this.assertIsOwnerOrContributor();

    // storing meme metadata requires some storage staking (balance locked to offset cost of data storage)
    assert(
      near.attachedDeposit() > MIN_ACCOUNT_BALANCE,
      `Minimum account balance must be attached to initialize this contract (${MIN_ACCOUNT_BALANCE})`
    );

    const accountId = fullAccount(meme);

    // assert(
    //   env.isValidAccountID(accountId),
    //   "Meme name must be valid NEAR account name"
    // );
    assert(this.memes.contains(accountId), "Meme name already exists");

    near.log("attempting to create meme");

    const promise = near.promiseBatchCreate(accountId);
    near.promiseBatchActionCreateAccount(promise);
    near.promiseBatchActionDeployContract(promise, CODE);
    near.promiseBatchActionAddKeyWithFullAccess(
      promise,
      near.signerAccountPk(),
      0
    );
    near.promiseBatchActionFunctionCall(
      promise,
      "init",
      bytes(JSON.stringify({ title, data, category })),
      near.attachedDeposit(),
      XCC_GAS
    );

    const then = near.promiseThen(
      promise,
      near.currentAccountId(),
      "on_meme_created",
      bytes(JSON.stringify({ meme })),
      0,
      XCC_GAS
    );

    return near.promiseReturn(then);
  }

  @call
  on_meme_created({ meme }: { meme: string }): void {
    const results = near.promiseResult(0);

    // Verifying the remote contract call succeeded.
    // https://nomicon.io/RuntimeSpec/Components/BindingsSpec/PromisesAPI.html?highlight=promise#returns-3
    if (results === near.PromiseResult.Failed) {
      near.log(`Meme creation for [ ${fullAccount(meme)} ] failed`);
    } else if (results === near.PromiseResult.NotReady) {
      near.log(`Meme creation for [ ${fullAccount(meme)} ] is pending`);
    } else {
      near.log(`Meme creation for [ ${fullAccount(meme)} ] succeeded`);
    }
  }

  /*
   * Governance methods reserved for 101Labs and NEAR admins
   */
  @call
  add_contributor({ account }: { account: string }): void {
    assertIsInitialized(MUSEUM_KEY);
    this.assertIsSignedByOwner();

    this.contributors.set(account);

    near.log("contributor was added");
  }

  @call
  remove_contributor({ account }: { account: string }): void {
    assertIsInitialized(MUSEUM_KEY);
    this.assertIsSignedByOwner();

    this.contributors.remove(account);
  }

  @call
  add_owner({ account }: { account: string }): void {
    assertIsInitialized(MUSEUM_KEY);
    this.assertIsSignedByOwner();

    this.owners.set(account);
  }

  @call
  remove_owner({ account }: { account: string }): void {
    assertIsInitialized(MUSEUM_KEY);
    this.assertIsSignedByOwner();

    this.owners.remove(account);
  }

  @call
  remove_meme({ meme }: { meme: string }): void {
    assertIsInitialized(MUSEUM_KEY);
    this.assertIsSignedByOwner();

    const promise = near.promiseBatchCreate(fullAccount(meme));
    near.promiseBatchActionDeleteAccount(promise, near.currentAccountId());
    const then = near.promiseThen(
      promise,
      near.currentAccountId(),
      "on_meme_removed",
      bytes(JSON.stringify({ meme })),
      0,
      XCC_GAS
    );

    return near.promiseReturn(then);
  }

  @call
  on_meme_removed({ meme }: { meme: string }): void {
    // TODO: confirm that promise was successful
    near.log(`[ ${fullAccount(meme)} ] was removed`);
    this.memes.remove(meme);
  }

  private assertIsOwnerOrContributor(): void {
    assert(
      this.contributors.contains(near.predecessorAccountId()) ||
        this.owners.contains(near.predecessorAccountId()),
      "This method can only be called by a museum contributor or owner"
    );
  }

  private assertIsSignedByOwner(): void {
    assert(
      this.owners.contains(near.signerAccountId()),
      "This method can only be called by a museum owner"
    );
  }
}
