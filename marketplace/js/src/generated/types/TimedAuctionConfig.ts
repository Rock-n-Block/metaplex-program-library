/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
export type TimedAuctionConfig = {
  startTime: beet.bignum
  endTime: beet.bignum
}

/**
 * @category userTypes
 * @category generated
 */
export const timedAuctionConfigBeet =
  new beet.BeetArgsStruct<TimedAuctionConfig>(
    [
      ['startTime', beet.u64],
      ['endTime', beet.u64],
    ],
    'TimedAuctionConfig'
  )