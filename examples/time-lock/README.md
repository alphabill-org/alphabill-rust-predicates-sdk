# Custom Alphabill Predicate

This project implements predicate for the 
["Time Lock"](https://guardtime.atlassian.net/wiki/spaces/AB/pages/4690575396/Example+Use-Case+Time+Lock)
example use-case as WASM module.

To see the documentation of the crate run
```sh
cargo doc --all-features
```
and open the generated doc in a web browser.

## Use-Case

A common requirement in blockchain applications is the ability to lock assets until a
specific time in the future.

Here we implement predicate which can be used as bearer predicate of an unit, ie current
owner transfer the unit to this predicate with new owner's public key hash and unlock date
as arguments. The new owner can operate with the unit only after the unlock date.


### Business rules

The predicate stores the following data:
- Owner: hash of an ECDSA public key for the designated token owner (lke for the P2PKH predicate);
- Unlock time: a timestamp during which the predicate becomes satisfiable with the owner’s signature (in clock time as defined by the shard (initially root) consensus);

The owner predicate is satisfied only if the shard clock time equals or is later than the unlock time specified in the predicate and a signature is provided (in the owner proof) from the owner public key. The signature itself is the same as if they were regular single signatures on the transaction order.


## Steps to use this module in Alphabill:

### Create WASM binary

Compile the Rust code into WASM binary:
```sh
cargo build --release --target wasm32-unknown-unknown --all-features
```
this creates wasm binary which contains the predicate function `time_lock`.

### Create Alphabill predicate record

The Alphabill CLI wallet has a `tool` command which can be used to create so
called "predicate record" BLOB which is then usable as custom predicate
argument with other wallet commands:

```sh
abwallet tool create-wasm-predicate --code-file=./prg/time_lock.wasm --entrypoint=time_lock --parameter-file=./prg/args.plist --output=./prg/time_lock_bearer.cbor
```
The `parameter-file` flag refers to the file which contains the unlock date and new
owner arguments - this makes the wasm code reusable, just this command has to be
used to create new predicate for each concrete use.

In this example we use `plist` format as most convenient way to convert text
representation of required values to CBOR array expected by the predicate (the
`create-wasm-predicate` tool treats `plist` file as a special case and converts it
to CBOR).
However some other tool can be used to create the CBOR encoded array with required
values and then supplied as argument for the `parameter-file` flag.

Example content of the `args.plist` file:
```plist
(
    /* timestamp, when the unit will be unlocked */
    <*I1709683200>,
    /* hex encoded public key hash of the (new) owner */
    <01020305060708090abcdef>
)
```
See ie https://gnustep.github.io/resources/documentation/Developer/Base/Reference/NSPropertyList.html
for plist syntax.

### Use the predicate with some Alphabill unit

To send an "time locked" NFT:
```sh
abwallet wallet token send non-fungible --bearer-clause=@./prg/time_lock_bearer.cbor --token-identifier <TOKEN-ID>
```
where `time_lock_bearer.cbor` file contains the predicate BLOB created
on previous step (IOW output of the `create-wasm-predicate` tool). 

**NB!** The wallet currently actually does not support the `bearer-clause` flag with `send` command!
