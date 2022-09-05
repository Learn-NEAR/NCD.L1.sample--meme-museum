import { near } from "near-sdk-js";
import { MUSEUM_KEY } from "../utils";

export class Museum {
  created_at: bigint = near.blockTimestamp() as bigint;

  constructor(public name: string) {}

  // ----------------------------------------------------------------------------
  // Basic functions
  // ----------------------------------------------------------------------------

  static get(): Museum {
    return JSON.parse(near.storageRead(MUSEUM_KEY));
  }

  static set(museum: Museum): void {
    near.storageWrite(MUSEUM_KEY, JSON.stringify(museum));
  }
}
