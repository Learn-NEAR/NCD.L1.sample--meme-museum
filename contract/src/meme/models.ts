import { assert, near } from "near-sdk-js";
import { storageRead, storageWrite } from "near-sdk-js/lib/api";
import { Category, MEME_KEY } from "../utils";

export class Comment {
  created_at: bigint = near.blockTimestamp() as bigint;
  author: string = near.signerAccountId();

  constructor(public text: string) {}
}

export class Vote {
  created_at: bigint = near.blockTimestamp() as bigint;

  constructor(public value: number, public voter: string) {}
}

export class Donation {
  // by default, without a constructor, all fields are public
  // so these instance fields will be set from the context
  // and then available on the public interface
  amount: bigint = near.attachedDeposit() as bigint;
  donor: string = near.predecessorAccountId();
  created_at: bigint = near.blockTimestamp() as bigint;
}

export class Meme {
  creator: string = near.predecessorAccountId();
  created_at: bigint = near.blockTimestamp() as bigint;
  vote_score: number = 0;
  total_donations: bigint = BigInt(0);

  constructor(
    public title: string,
    public data: string,
    public category: Category
  ) {}

  // ----------------------------------------------------------------------------
  // Basic functions
  // ----------------------------------------------------------------------------

  static create(title: string, data: string, category: Category): void {
    // data has to have identifier from valid content provider
    assert(
      isValidMemeData(data),
      "Data is not valid, must start with valid 9gag.com URL"
    );

    // save the meme to storage
    const meme = new Meme(title, data, category);
    this.set(meme);
  }

  static get(): Meme {
    return JSON.parse(storageRead(MEME_KEY));
  }

  static set(meme: Meme): void {
    storageWrite(MEME_KEY, JSON.stringify(meme));
  }
}

/**
 * Handle validation and extraction of meme data
 */
function isValidMemeData(data: string): boolean {
  return data.startsWith("https://9gag.com");
}
