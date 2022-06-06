/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as splToken from '@solana/spl-token'
import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'

/**
 * @category Instructions
 * @category Cancel
 * @category generated
 */
export type CancelInstructionArgs = {
  auctioneerAuthorityBump: number
  buyerPrice: beet.bignum
  tokenSize: beet.bignum
}
/**
 * @category Instructions
 * @category Cancel
 * @category generated
 */
const cancelStruct = new beet.BeetArgsStruct<
  CancelInstructionArgs & {
    instructionDiscriminator: number[] /* size: 8 */
  }
>(
  [
    ['instructionDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['auctioneerAuthorityBump', beet.u8],
    ['buyerPrice', beet.u64],
    ['tokenSize', beet.u64],
  ],
  'CancelInstructionArgs'
)
/**
 * Accounts required by the _cancel_ instruction
 *
 * @property [] auctionHouseProgram
 * @property [_writable_] wallet
 * @property [_writable_] tokenAccount
 * @property [] tokenMint
 * @property [] authority
 * @property [] auctionHouse
 * @property [_writable_] auctionHouseFeeAccount
 * @property [_writable_] tradeState
 * @property [] auctioneerAuthority
 * @property [] ahAuctioneerPda
 * @category Instructions
 * @category Cancel
 * @category generated
 */
export type CancelInstructionAccounts = {
  auctionHouseProgram: web3.PublicKey
  wallet: web3.PublicKey
  tokenAccount: web3.PublicKey
  tokenMint: web3.PublicKey
  authority: web3.PublicKey
  auctionHouse: web3.PublicKey
  auctionHouseFeeAccount: web3.PublicKey
  tradeState: web3.PublicKey
  auctioneerAuthority: web3.PublicKey
  ahAuctioneerPda: web3.PublicKey
}

const cancelInstructionDiscriminator = [232, 219, 223, 41, 219, 236, 220, 190]

/**
 * Creates a _Cancel_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category Cancel
 * @category generated
 */
export function createCancelInstruction(
  accounts: CancelInstructionAccounts,
  args: CancelInstructionArgs
) {
  const {
    auctionHouseProgram,
    wallet,
    tokenAccount,
    tokenMint,
    authority,
    auctionHouse,
    auctionHouseFeeAccount,
    tradeState,
    auctioneerAuthority,
    ahAuctioneerPda,
  } = accounts

  const [data] = cancelStruct.serialize({
    instructionDiscriminator: cancelInstructionDiscriminator,
    ...args,
  })
  const keys: web3.AccountMeta[] = [
    {
      pubkey: auctionHouseProgram,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: wallet,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: tokenAccount,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: tokenMint,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: authority,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: auctionHouse,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: auctionHouseFeeAccount,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: tradeState,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: auctioneerAuthority,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: ahAuctioneerPda,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: splToken.TOKEN_PROGRAM_ID,
      isWritable: false,
      isSigner: false,
    },
  ]

  const ix = new web3.TransactionInstruction({
    programId: new web3.PublicKey(
      '81Xv3QwiLvcWgrMXKkhPRWrYsHrdTCbBTd3N7W4rHt8H'
    ),
    keys,
    data,
  })
  return ix
}
