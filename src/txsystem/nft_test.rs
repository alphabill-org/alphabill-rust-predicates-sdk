// this test is generated by Go backend, Test_generate_NFT_DecodeTests
// DO NOT EDIT!

extern crate alloc;
use super::*;
use alloc::{string::ToString, vec};

#[test]
#[allow(non_snake_case)]
fn TokenData_from_1() {
    // &tokens.NonFungibleTokenData{_:struct {}{}, Version:0x0, TypeID:types.UnitID{0x1, 0x5, 0x0}, Name:"", URI:"", Data:hex.Bytes(nil), OwnerPredicate:hex.Bytes(nil), DataUpdatePredicate:hex.Bytes(nil), Locked:0x0, Counter:0x5a}
    let data = vec![
        0x1, 0x1, 0x3, 0x0, 0x0, 0x0, 0x1, 0x5, 0x0, 0x5, 0x2, 0x5a, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x6, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ];
    assert_eq!(
        TokenData::from(data).unwrap(),
        TokenData {
            type_id: Some(vec![0x1, 0x5, 0x0]),
            name: None,
            uri: None,
            data: None,
            counter: Some(90),
            locked: Some(0),
        }
    );

    // &tokens.NonFungibleTokenData{_:struct {}{}, Version:0x0, TypeID:types.UnitID{0x1}, Name:"hot stuff", URI:"foo/bar", Data:hex.Bytes{0x9, 0x1, 0x1}, OwnerPredicate:hex.Bytes(nil), DataUpdatePredicate:hex.Bytes(nil), Locked:0x1, Counter:0x5a}
    let data = vec![
        0x1, 0x1, 0x1, 0x0, 0x0, 0x0, 0x1, 0x2, 0x4, 0x9, 0x0, 0x0, 0x0, 0x68, 0x6f, 0x74, 0x20,
        0x73, 0x74, 0x75, 0x66, 0x66, 0x3, 0x4, 0x7, 0x0, 0x0, 0x0, 0x66, 0x6f, 0x6f, 0x2f, 0x62,
        0x61, 0x72, 0x4, 0x1, 0x3, 0x0, 0x0, 0x0, 0x9, 0x1, 0x1, 0x5, 0x2, 0x5a, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x6, 0x2, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ];
    assert_eq!(
        TokenData::from(data).unwrap(),
        TokenData {
            type_id: Some(vec![0x1]),
            name: Some("hot stuff".to_string()),
            uri: Some("foo/bar".to_string()),
            data: Some(vec![0x9, 0x1, 0x1]),
            counter: Some(90),
            locked: Some(1),
        }
    );
}

#[test]
#[allow(non_snake_case)]
fn TypeData_from_1() {
    // &tokens.NonFungibleTokenTypeData{_:struct {}{}, Version:0x0, Symbol:"A", Name:"abcde", Icon:(*tokens.Icon)(nil), ParentTypeID:types.UnitID(nil), SubTypeCreationPredicate:hex.Bytes(nil), TokenMintingPredicate:hex.Bytes(nil), TokenTypeOwnerPredicate:hex.Bytes(nil), DataUpdatePredicate:hex.Bytes(nil)}
    let data = vec![
        0x2, 0x4, 0x1, 0x0, 0x0, 0x0, 0x41, 0x3, 0x4, 0x5, 0x0, 0x0, 0x0, 0x61, 0x62, 0x63, 0x64,
        0x65,
    ];
    assert_eq!(
        TypeData::from(data).unwrap(),
        TypeData {
            parent_id: None,
            symbol: Some("A".to_string()),
            name: Some("abcde".to_string()),
        }
    );

    // &tokens.NonFungibleTokenTypeData{_:struct {}{}, Version:0x0, Symbol:"oh!", Name:"qwerty", Icon:(*tokens.Icon)(nil), ParentTypeID:types.UnitID{0xff, 0x0, 0x7f, 0x80}, SubTypeCreationPredicate:hex.Bytes(nil), TokenMintingPredicate:hex.Bytes(nil), TokenTypeOwnerPredicate:hex.Bytes(nil), DataUpdatePredicate:hex.Bytes(nil)}
    let data = vec![
        0x1, 0x1, 0x4, 0x0, 0x0, 0x0, 0xff, 0x0, 0x7f, 0x80, 0x2, 0x4, 0x3, 0x0, 0x0, 0x0, 0x6f,
        0x68, 0x21, 0x3, 0x4, 0x6, 0x0, 0x0, 0x0, 0x71, 0x77, 0x65, 0x72, 0x74, 0x79,
    ];
    assert_eq!(
        TypeData::from(data).unwrap(),
        TypeData {
            parent_id: Some(vec![0xff, 0x0, 0x7f, 0x80]),
            symbol: Some("oh!".to_string()),
            name: Some("qwerty".to_string()),
        }
    );
}

#[test]
#[allow(non_snake_case)]
fn CreateType_from_1() {
    // tokens.DefineNonFungibleTokenAttributes{_:struct {}{}, Symbol:"AB", Name:"test token", Icon:(*tokens.Icon)(nil), ParentTypeID:types.UnitID(nil), SubTypeCreationPredicate:[]uint8(nil), TokenMintingPredicate:[]uint8(nil), TokenTypeOwnerPredicate:[]uint8(nil), DataUpdatePredicate:[]uint8(nil)}
    let data = vec![
        0x1, 0x4, 0x2, 0x0, 0x0, 0x0, 0x41, 0x42, 0x2, 0x4, 0xa, 0x0, 0x0, 0x0, 0x74, 0x65, 0x73,
        0x74, 0x20, 0x74, 0x6f, 0x6b, 0x65, 0x6e,
    ];
    assert_eq!(
        CreateType::from(data).unwrap(),
        CreateType {
            name: Some("test token".to_string()),
            symbol: Some("AB".to_string()),
            type_id: None,
        }
    );
    // tokens.DefineNonFungibleTokenAttributes{_:struct {}{}, Symbol:"AB-NFT", Name:"funky token", Icon:(*tokens.Icon)(nil), ParentTypeID:types.UnitID{0x1, 0x2, 0x3, 0x8, 0x9, 0x0}, SubTypeCreationPredicate:[]uint8(nil), TokenMintingPredicate:[]uint8(nil), TokenTypeOwnerPredicate:[]uint8(nil), DataUpdatePredicate:[]uint8(nil)}
    let data = vec![
        0x1, 0x4, 0x6, 0x0, 0x0, 0x0, 0x41, 0x42, 0x2d, 0x4e, 0x46, 0x54, 0x2, 0x4, 0xb, 0x0, 0x0,
        0x0, 0x66, 0x75, 0x6e, 0x6b, 0x79, 0x20, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x3, 0x1, 0x6, 0x0,
        0x0, 0x0, 0x1, 0x2, 0x3, 0x8, 0x9, 0x0,
    ];
    assert_eq!(
        CreateType::from(data).unwrap(),
        CreateType {
            name: Some("funky token".to_string()),
            symbol: Some("AB-NFT".to_string()),
            type_id: Some(vec![0x1, 0x2, 0x3, 0x8, 0x9, 0x0]),
        }
    );
}

#[test]
#[allow(non_snake_case)]
fn Mint_from_1() {
    // tokens.MintNonFungibleTokenAttributes{_:struct {}{}, TypeID:types.UnitID{0x8, 0x7, 0x6, 0x5}, Name:"test token", URI:"", Data:[]uint8(nil), OwnerPredicate:[]uint8(nil), DataUpdatePredicate:[]uint8(nil), Nonce:0x1}
    let data = vec![
        0x1, 0x4, 0xa, 0x0, 0x0, 0x0, 0x74, 0x65, 0x73, 0x74, 0x20, 0x74, 0x6f, 0x6b, 0x65, 0x6e,
        0x4, 0x2, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x5, 0x1, 0x4, 0x0, 0x0, 0x0, 0x8, 0x7,
        0x6, 0x5,
    ];
    assert_eq!(
        Mint::from(data).unwrap(),
        Mint {
            name: Some("test token".to_string()),
            uri: None,
            data: None,
            nonce: Some(1),
            type_id: Some(vec![0x8, 0x7, 0x6, 0x5]),
        }
    );
    // tokens.MintNonFungibleTokenAttributes{_:struct {}{}, TypeID:types.UnitID{0xff, 0xff, 0xff}, Name:"test token", URI:"ab://nft/token", Data:[]uint8{0x64, 0x61, 0x74, 0x61, 0x21}, OwnerPredicate:[]uint8(nil), DataUpdatePredicate:[]uint8(nil), Nonce:0x3e8}
    let data = vec![
        0x1, 0x4, 0xa, 0x0, 0x0, 0x0, 0x74, 0x65, 0x73, 0x74, 0x20, 0x74, 0x6f, 0x6b, 0x65, 0x6e,
        0x2, 0x4, 0xe, 0x0, 0x0, 0x0, 0x61, 0x62, 0x3a, 0x2f, 0x2f, 0x6e, 0x66, 0x74, 0x2f, 0x74,
        0x6f, 0x6b, 0x65, 0x6e, 0x3, 0x1, 0x5, 0x0, 0x0, 0x0, 0x64, 0x61, 0x74, 0x61, 0x21, 0x4,
        0x2, 0xe8, 0x3, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x5, 0x1, 0x3, 0x0, 0x0, 0x0, 0xff, 0xff,
        0xff,
    ];
    assert_eq!(
        Mint::from(data).unwrap(),
        Mint {
            name: Some("test token".to_string()),
            uri: Some("ab://nft/token".to_string()),
            data: Some(vec![0x64, 0x61, 0x74, 0x61, 0x21]),
            nonce: Some(1000),
            type_id: Some(vec![0xff, 0xff, 0xff]),
        }
    );
}

#[test]
#[allow(non_snake_case)]
fn Transfer_from_1() {
    // tokens.TransferNonFungibleTokenAttributes{_:struct {}{}, TypeID:types.UnitID{0x8, 0x7, 0x6, 0x5}, NewOwnerPredicate:[]uint8(nil), Counter:0x7}
    let data = vec![
        0x1, 0x1, 0x4, 0x0, 0x0, 0x0, 0x8, 0x7, 0x6, 0x5, 0x2, 0x2, 0x7, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0,
    ];
    assert_eq!(
        Transfer::from(data).unwrap(),
        Transfer {
            counter: Some(7),
            type_id: Some(vec![0x8, 0x7, 0x6, 0x5]),
        }
    );
    // tokens.TransferNonFungibleTokenAttributes{_:struct {}{}, TypeID:types.UnitID{0x8, 0x7, 0x6, 0x5}, NewOwnerPredicate:[]uint8(nil), Counter:0x7}
    let data = vec![
        0x1, 0x1, 0x4, 0x0, 0x0, 0x0, 0x8, 0x7, 0x6, 0x5, 0x2, 0x2, 0x7, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0,
    ];
    assert_eq!(
        Transfer::from(data).unwrap(),
        Transfer {
            counter: Some(7),
            type_id: Some(vec![0x8, 0x7, 0x6, 0x5]),
        }
    );
}

#[test]
#[allow(non_snake_case)]
fn Update_from_1() {
    // tokens.UpdateNonFungibleTokenAttributes{_:struct {}{}, Data:[]uint8{}, Counter:0x6}
    let data = vec![
        0x1, 0x1, 0x0, 0x0, 0x0, 0x0, 0x2, 0x2, 0x6, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ];
    assert_eq!(
        Update::from(data).unwrap(),
        Update {
            counter: Some(6),
            data: Some(Vec::new()),
        }
    );
    // tokens.UpdateNonFungibleTokenAttributes{_:struct {}{}, Data:[]uint8{0x6e, 0x65, 0x77, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x68, 0x65, 0x72, 0x65}, Counter:0x7}
    let data = vec![
        0x1, 0x1, 0xd, 0x0, 0x0, 0x0, 0x6e, 0x65, 0x77, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x68,
        0x65, 0x72, 0x65, 0x2, 0x2, 0x7, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ];
    assert_eq!(
        Update::from(data).unwrap(),
        Update {
            counter: Some(7),
            data: Some(vec![
                0x6e, 0x65, 0x77, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x68, 0x65, 0x72, 0x65
            ]),
        }
    );
}
