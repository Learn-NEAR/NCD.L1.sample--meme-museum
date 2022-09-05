import {
  NearBindgen,
  NearContract,
  near,
  call,
  view,
  Vector,
  LookupSet,
  assert,
} from "near-sdk-js";
import {
  assertIsInitialized,
  assertIsNotInitialized,
  assertIsSignedByCreator,
  assertIsSignerPredecessor,
  assertReasonableCommentLength,
  Category,
  getLast,
  MEME_KEY,
  MIN_ACCOUNT_BALANCE,
  XCC_GAS,
} from "../utils";
import { Donation, Meme, Vote } from "./models";

BigInt.prototype["toJSON"] = function () {
  return this.toString();
};

// The @NearBindgen decorator allows this code to compile to Base64.
@NearBindgen
export class MemeContract extends NearContract {
  private comments: Vector;
  private votes: Vector;
  private voters: LookupSet;
  private donations: Vector;

  constructor({
    defaultCall,
    title = "",
    data = "",
    category = Category.A,
  }: {
    title?: string;
    data?: string;
    category?: Category;
    defaultCall: boolean;
  }) {
    //execute the NEAR Contract's constructor
    super();

    if (defaultCall) {
      return;
    }

    // contract may only be initialized once
    assertIsNotInitialized(MEME_KEY);

    // storing meme metadata requires some storage staking (balance locked to offset cost of data storage)
    assert(
      near.attachedDeposit() >= MIN_ACCOUNT_BALANCE,
      "Minimum account balance must be attached to initialize this contract (3 NEAR)"
    );

    // title has to be at least 1 character
    assert(title.length > 0, "Meme title may not be blank");

    // create the meme using incoming metadata
    Meme.create(title, data, category);
  }

  default() {
    return new MemeContract({ defaultCall: true });
  }

  /**
   * Return the meme
   */
  @view
  get_meme(): Meme {
    assertIsInitialized(MEME_KEY);
    return Meme.get();
  }

  // ----------------------------------------------------------------------------
  // Voting
  // ----------------------------------------------------------------------------

  /**
   * Register a single vote, up or down, for the meme
   */
  @call
  vote({ value }: { value: number }): void {
    assertIsInitialized(MEME_KEY);
    assertIsSignerPredecessor();
    assert(value === 1 || value === -1, "Invalid vote, must be -1 or 1");

    // register the vote
    this.batch_vote({ value, is_batch: false });
  }

  /**
   * Register a batched vote where several votes are captured together
   */
  @call
  batch_vote({
    value,
    is_batch = true,
  }: {
    value: number;
    is_batch: boolean;
  }): void {
    // register the vote
    if (is_batch) {
      assert(
        near.predecessorAccountId() === near.currentAccountId(),
        "Batch votes may only be made by the Meme account"
      );
    }

    const voter = is_batch
      ? "batch-" + near.predecessorAccountId()
      : near.predecessorAccountId();

    // allow each account to vote only once
    assert(!this.voters.contains(voter), "Voter has already voted");
    // fetch meme from storage
    const meme = Meme.get();
    // calculate new score for meme
    meme.vote_score += value;
    // save it back to storage
    Meme.set(meme);
    // remember the voter has voted
    this.voters.set(voter);
    // add the new Vote
    this.votes.push(new Vote(value, voter));
  }

  /**
   * Get a list ofrecent votes
   */
  @view
  get_recent_votes(): Array<Vote> {
    assertIsInitialized(MEME_KEY);
    return getLast(this.votes);
  }

  /**
   * Get the current vote score
   */
  @view
  get_vote_score(): number {
    assertIsInitialized(MEME_KEY);
    return Meme.get().vote_score;
  }

  // ----------------------------------------------------------------------------
  // Comments
  // ----------------------------------------------------------------------------

  /**
   * Add a comment
   *
   * @param text the text of the comment, max comment length of MAX_COMMENT_LENGTH
   */
  @call
  add_comment({ text }: { text: string }): void {
    assertIsInitialized(MEME_KEY);
    assertIsSignerPredecessor();
    assertReasonableCommentLength(text);

    this.comments.push(new Comment(text));
  }

  /**
   * Get a list o recent comments
   */
  @view
  get_recent_comments(): Array<Comment> {
    assertIsInitialized(MEME_KEY);

    return getLast(this.comments);
  }

  // ----------------------------------------------------------------------------
  // Donations
  // ----------------------------------------------------------------------------

  /**
   * Donate tokens to the contract
   */
  @call
  donate(): void {
    assertIsInitialized(MEME_KEY);
    assertIsSignerPredecessor();
    assert(near.attachedDeposit() > BigInt(0), "Donor must attach some money");

    // fetch meme from storage
    const meme = Meme.get();
    // record the donation
    meme.total_donations =
      BigInt(meme.total_donations) + (near.attachedDeposit() as bigint);
    // save it back to storage
    Meme.set(meme);
    // add new Donation
    this.donations.push(new Donation());
  }

  /**
   * Get a list of donations
   */
  @view
  get_donations_total(): bigint {
    assertIsInitialized(MEME_KEY);
    return Meme.get().total_donations;
  }

  /**
   * Get a list o recent comments
   */
  @view
  get_recent_donations(): Array<Donation> {
    assertIsInitialized(MEME_KEY);
    return getLast(this.donations);
  }

  /**
   * Transfer all donations to a specified account
   */
  @call
  release_donations({ account }: { account: string }): void {
    assertIsInitialized(MEME_KEY);
    assertIsSignedByCreator();

    // transfer funds to provided account and call ourselves back once transfer is complete
    const promiseBatch = near.promiseBatchCreate(account);
    near.promiseBatchActionTransfer(promiseBatch, Meme.get().total_donations);
    const then = near.promiseThen(
      promiseBatch,
      near.currentAccountId(),
      "on_donations_released",
      JSON.stringify({}),
      BigInt(0),
      XCC_GAS
    );
    return near.promiseReturn(then);
  }

  /**
   * Callback method invoked once donation release is complete
   */
  @call
  on_donations_released(): void {
    near.log("Donations were released");
  }
}
