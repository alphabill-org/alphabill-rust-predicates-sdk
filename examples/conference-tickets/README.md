# Custom Alphabill Predicate

This project implements predicates for the 
["Transferable Conference Tickets"](https://guardtime.atlassian.net/wiki/spaces/AB/pages/3538419747/Example+Use-Case+Transferable+Conference+Tickets)
example use-case as WASM module.

To see the documentation of the crate run
```sh
cargo doc --all-features
```
and open the generated doc in a web browser.

## Use-Case

We're implementing set of custom predicates allowing to use NFT token as conference ticket.

Tickets are created (tokens minted) by the conference organizer, and there is two types of
tickets: "early-bird" and "regular".
This allows for limiting the number of tickets to match the space available and the venue.

### Business rules

- A conference ticket may be bought at an "early-bird" price P1 up to a given date D1 and at a "regular" price P2 > P1 after that.
- A conference ticket may be transferred to another person up to a deadline D2 > D1; at D2, the conference organizer prints the participant materials (including badges and other personalized items) and no changes are allowed any more.
- If the ticket was originally bought as "early-bird" and is being transferred after D1, it must first be "upgraded" to "regular" by paying the conference organizer the price difference P2-P1.


This use-case can be implemented with the following set of predicates:

### token type "conference X ticket"
- sub-type creation predicate: false;
- token creation predicate: creation order signed by the conference organizer;
- bearer predicate invariant clause:
    token data is "early-bird" and date <= D1, or
    token data is "regular" and date <= D2;
- data update predicate invariant clause: old data is "early-bird" and new data is "regular";

### token of type "conference X ticket":

- bearer predicate:
    token data is "early-bird" and receipt of payment of P1 to the conference organizer and the nonce in the payment order is the hash of the pair (1, token ID), or
    token data is "regular" and receipt of payment of P2 to the conference organizer and the nonce in the payment order is the hash of the pair (1, token ID), or
    signed by the conference organizer

- data update predicate:
    date <= D2 and receipt of payment of P2-P1 to the conference organizer and the nonce in the payment order is the hash of the pair (2, token ID), or
    signed by the conference organizer


Note that the conference organizer can limit the number of "early-bird" tickets separately from the total number of tickets either by creating only some ticket tokens before the date D1 or by creating only some of them as "early-bird" initially and the rest as "regular" from the beginning. Also note that the conference organizer will have to upgrade all unsold "early-bird" tickets to "regular" on D1, or else they can't be bought at all after D1.

The buyer must have some way of finding a ticket token to buy; presumably the conference organizers make the list available in some form.


## Steps to use this module in Alphabill:

[See also](https://guardtime.atlassian.net/wiki/spaces/AB/pages/4662788185/Tutorial+for+transferable+ticket+example+use+case)

### Create WASM binary

Compile the Rust code into WASM binary:
```sh
cargo build --release --target wasm32-unknown-unknown --all-features
```
this creates wasm binary which contains all four predicate functions needed
by the conference tickets use-case. However, creating binary per predicate
should be preferred as the size is smaller then (and that may affect the
fees paid for the predicate).

To create binary per predicate replace `--all-features` flag with specific
"feature":
- `--features type-bearer` exports entrypoint of the `type_bearer` predicate;
- `--features type-update-data` exports entrypoint of the `type_update_data` predicate;
- `--features token-update-data` exports entrypoint of the `token_update_data` predicate;
- `--features token-bearer` exports entrypoint of the `token_bearer` predicate;

The naming pattern is that `type-bearer` is a _bearer_ predicate invariant for _token type_ etc.

### Create Alphabill predicate record

The Alphabill CLI wallet has a `tool` command which can be used to create so
called "predicate record" BLOB which is then usable as custom predicate
argument with other wallet commands:

```sh
abwallet tool create-wasm-predicate --code-file=./prg/tickets.wasm --entrypoint=token_bearer --parameter-file=./prg/args.plist --output=./prg/token_bearer.cbor
```
This command has to be used four times, once for each custom predicate required by
the demo use-case.
On each call at least different `entrypoint` value has to be supplied and 
depending on whether single or multiple wasm binaries were created the `code-file`
changes too (the code file(s) were created on previous step, the valid entrypoint
values are listed there too).

And of-course each result should be saved to different `output` file.

All predicates except `type_update_data` take the same five element array as parameter
and both token type and token(s) must have all custom predicates created with the same
parameter values so the same file is shared by the `create-wasm-predicate` runs (flag
`parameter-file`).
The `type_update_data` requires no additional parameters so for that run the
`parameter-file` flag should be omitted.

In this example we use `plist` format as most convenient way to convert text
representation of required values to CBOR array expected by the predicates (the
`create-wasm-predicate` tool treats `plist` file as a special case and converts it
to CBOR).
However some other tool can be used to create the CBOR encoded array with required
values and then supplied as argument for the `parameter-file` flag.

Example content of the `args.plist` file:
```plist
(
    (
        /* D1 - timestamp, when the early-bird pricing ends */
        <*I1709683200>,
        /* D2 - timestamp, tickets can be transferred until this date */
        <*I1709783200>,
        /* P1 - early-bird price */
        <*I1000>,
        /* P2 - regular ticket price */
        <*I2500>
    )
    /* hex encoded public key hash of the organizer (receiver of payments) */
    <045559d0b5c1c260e3feb8fef6b360bd1570847e0d0c18d9b6c7a5a397873e53>
)
```
See ie https://gnustep.github.io/resources/documentation/Developer/Base/Reference/NSPropertyList.html
for plist syntax.

### Use the predicate with some Alphabill unit

First create new token type with custom bearer invariant and update data predicates
```sh
abwallet wallet token new-type non-fungible --bearer-clause=@./prg/type-bearer.cbor --data-update-clause=@./prg/type-update.cbor
```
where `type-bearer.cbor` and `type-update.cbor` files contains the predicate BLOB created
on previous step (IOW output of the `create-wasm-predicate` tool). 

Next mint a token of this type, don't forget to set custom bearer and update-data predicates.