# Astro Contract

Astro contract is written for the astro project, the contract serves to create and manage NFTs. The set includes contracts:

* [Minter](contracts/minter): Minter Contract Customize CW721 according to the logic of the Astro project. It manages the config related to NFT
  episode in 1 collection: Base URI, NFT count, minted quantity, etc. and additional information for Royalty.
* [Factory](contracts/factory): Currently, Factory contract only has the simple task of managing the code_id of the related contracts deployed on the
  network. We can instantiate a minter contract via factory contract.

![alt text](static/contracts.PNG)

Details and implementations in the contract, please refer to each contract repository.

# Deployment

## Store cw721 contract
Astro contract uses cw721 to manage its NFTs, so the first thing you need to do is have a cw721 contract that is stored on the network.
Let's build, store and save the codeid of the contract [cw-nfts](https://github.com/CosmWasm/cw-nfts).

## Store Minter contract
The next thing we need is the store [Minter](contracts/minter) contract. Again, please save the codeid of the contract.

## Deploy Factory contract
After store [Factory](contracts/factory) contract, we need instantiate factory contract with 2 codeids obtained above.

* `InstantiateMsg` - Initialize config information for minter management.

```rust
pub struct InstantiateMsg {
  /// code id of minter contract was stored
  pub minter_code_id: u64,
  /// code id of cw721 contract was stored
  pub cw721_code_id: u64,
}
```
Now, the factory contract is ready to be used!
