# Multisignature Predicate

This project implements custom Alphabill predicate as WASM module.

To see the documentation of the crate run
```sh
cargo doc --all-features
```
and open the generated doc in a web browser.

## Use-Case

A common requirement in blockchain applications is the ability to require multiple
parties to approve a transaction before it can be executed.

### Business rules

The owner predicate is satisfied only if it successfully verifies a set of signatures
from at least n different signers from the list of authorized signers (e.g. for the
“2 of 3” option, any 2 authorized signers must sign; the last signer may also sign,
but that is not necessary).

For efficiency of verification, the signatures must be in the same order as the hashes
of their respective public keys are stored in the predicates.

## Usage

There is currently no tooling to create transaction which uses this predicate - the assumption
is that the transaction order is signed offline and once enough signatures is collected it is
sent to Alphabill.

### Configuration

aka static parameters.

Example content of the `args.plist` file:
```plist
(
    /* number of signatures required, must be smaller or equal to the length of pkh array */
    <*I2>,
    /* array of hex encoded public key hashes */
    <01020305060708090abcdef>,
    <02010305060708090abcdef>,
    <03010205060708090abcdef>
)
```
See ie https://mediawiki.gnustep.org/index.php/Property_Lists for plist syntax.

It's user's responsibility to make sure that number of PKHs is greater than or equal to the "threshold".
Up to 255 signatures is supported, it is user's responsibility not to set greater threshold.

### AuthProof

The AuthProof of the transaction order must be CBOR array of P2PKH signatures
in the same order as the PKH-s in the configuration. Missing signature must be
represented by nil (`0xf6` in CBOR) so that the length of the array equals to
the number of PKH-s in the configuration.
The signatures should be encoded as raw byte buffers (ie not typed arrays but
opaque buffers).
