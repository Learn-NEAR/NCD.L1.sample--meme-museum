import { assert, near, Vector } from "near-sdk-js";
import { Meme } from "./meme/models";
import { Museum } from "./museum/models";

export enum Category {
  A,
  B,
  C,
  D,
}

export const ONE_NEAR = BigInt(10e24);
export const XCC_GAS = BigInt(2 * 10e14);
export const MIN_ACCOUNT_BALANCE = ONE_NEAR * BigInt(3);

// common keys for singlton instances and initialization
export const MEME_KEY = "state";
export const MUSEUM_KEY = "state";

// size constraints
export const PAGE_SIZE = 10;
export const MAX_COMMENT_LENGTH = 500;

export function getLast<T>(vector: Vector, n: number = PAGE_SIZE): Array<T> {
  const returnArray = new Array<T>(0);

  for (let i = vector.len() - n - 1; i < vector.len(); i++) {
    returnArray.push(vector.get(i) as T);
  }

  return returnArray;
}

export function isInitialized(key: string): boolean {
  return near.storageHasKey(key);
}

export function isCreator(): boolean {
  return near.predecessorAccountId() === Meme.get().creator;
}

export function assertIsInitialized(key: string): void {
  assert(isInitialized(key), "Contract is not initialized");
}

export function assertIsNotInitialized(key: string): void {
  assert(!isInitialized(key), "Contract is already initialized");
}

export function assertIsSignedByCreator(): void {
  assert(isCreator(), "This method can only be called by the meme creator");
}

export function assertReasonableCommentLength(text: string): void {
  assert(
    text.length < MAX_COMMENT_LENGTH,
    `Comment is too long, must be less than ${MAX_COMMENT_LENGTH}`
  );
}

export function assertIsSignerPredecessor(): void {
  assert(
    near.signerAccountId() === near.predecessorAccountId(),
    "Users must call this method directly"
  );
}

export function fullAccount(meme: string): string {
  return `${meme}.${near.currentAccountId()}`;
}
