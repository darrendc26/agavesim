// entry: drives scheduler + bank + svm runner

mod loader;
mod types;
use loader::create_sanitized_txn;
fn main() {
    println!("Hello, world!");
    let json = r#"
{
  "jsonrpc": "2.0",
  "result": {
    "blockTime": 1761423451,
    "meta": {
      "computeUnitsConsumed": 72369,
      "costUnits": 74962,
      "err": null,
      "fee": 5000,
      "innerInstructions": [
        {
          "index": 0,
          "instructions": [
            {
              "accounts": [
                7
              ],
              "data": "84eT",
              "programIdIndex": 9,
              "stackHeight": 2
            },
            {
              "accounts": [
                0,
                1
              ],
              "data": "11119os1e9qSs2u7TsThXqkBSRVFxhmYaFKFZ1waB2X7armDmvK3p5GmLdUxYdg3h7QSrL",
              "programIdIndex": 8,
              "stackHeight": 2
            },
            {
              "accounts": [
                1
              ],
              "data": "P",
              "programIdIndex": 9,
              "stackHeight": 2
            },
            {
              "accounts": [
                1,
                7
              ],
              "data": "6UYDxH4covEfpQy9xRUbGbUoQRmVFERYYfQxu1UHtNsww",
              "programIdIndex": 9,
              "stackHeight": 2
            }
          ]
        },
        {
          "index": 3,
          "instructions": [
            {
              "accounts": [
                1,
                7,
                5,
                0
              ],
              "data": "iPgwgVjvh1EPS",
              "programIdIndex": 9,
              "stackHeight": 2
            },
            {
              "accounts": [
                4,
                12,
                3,
                11
              ],
              "data": "gvScPiLLkuvRs",
              "programIdIndex": 9,
              "stackHeight": 2
            },
            {
              "accounts": [
                13
              ],
              "data": "6nxw4wyeJdmVBNR2GLm5iSsrmSwF5GjgNgpvDWTHPRsv6TrySgm8rxeS8oTCH7KcPf9D3kBxeK3xnagKzNXuDMqE8axZdqTL7LenfZDL4FJXjAq78gvZsqAbY9skkaHx2gq26RtSKq4mtZ5Ae9izRqCzFxTQsWUxh8mLTeE4NtC6afvkX4nZjT9CFE25m",
              "programIdIndex": 10,
              "stackHeight": 2
            },
            {
              "accounts": [
                13
              ],
              "data": "EVM9wLnauu9H41GfKhTodopynPkhUSQTm7jbeh9FLdiYaWvYTJC7RgsW7RG891ghvS5xUX2tDUPuBKKv7WDyTvS9jfVpzxmFMpEF9zjqkEkjEXhKzZUjBuuWvaicB1C7PQwMNaBoL2cJrkqa1qyzbNpg6qbvR1uYL7gKrpAhRWoesfXxnqDZi5tmpEC29AQjsurYbdb94WtZWsEmPn5MzFR6Yx8wvGfbgfQd7oADevqCz2muoz7TFygJy365fT6RAdEmBnmoRoYb",
              "programIdIndex": 10,
              "stackHeight": 2
            }
          ]
        }
      ],
      "loadedAddresses": {
        "readonly": [],
        "writable": []
      },
      "logMessages": [
        "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL invoke [1]",
        "Program log: CreateIdempotent",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
        "Program log: Instruction: GetAccountDataSize",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 1569 of 797595 compute units",
        "Program return: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA pQAAAAAAAAA=",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program 11111111111111111111111111111111 invoke [2]",
        "Program 11111111111111111111111111111111 success",
        "Program log: Initialize the associated token account",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
        "Program log: Instruction: InitializeImmutableOwner",
        "Program log: Please upgrade to SPL Token 2022 for immutable owner support",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 1405 of 791008 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
        "Program log: Instruction: InitializeAccount3",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 3158 of 787126 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL consumed 19315 of 803000 compute units",
        "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL success",
        "Program 11111111111111111111111111111111 invoke [1]",
        "Program 11111111111111111111111111111111 success",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [1]",
        "Program log: Instruction: SyncNative",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 3045 of 783535 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG invoke [1]",
        "Program log: Instruction: Swap",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
        "Program log: Instruction: TransferChecked",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 6238 of 762842 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
        "Program log: Instruction: TransferChecked",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 6147 of 754152 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG invoke [2]",
        "Program cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG consumed 2098 of 741834 compute units",
        "Program cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG success",
        "Program cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG invoke [2]",
        "Program cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG consumed 2098 of 736265 compute units",
        "Program cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG success",
        "Program cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG consumed 46944 of 780490 compute units",
        "Program cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG success",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [1]",
        "Program log: Instruction: CloseAccount",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 2915 of 733546 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success"
      ],
      "postBalances": [
        997337056,
        0,
        8630400,
        2039280,
        2039280,
        66509047689,
        789146958,
        1181222631482,
        1,
        5299608127,
        1141507,
        38492372815,
        1461600,
        0
      ],
      "postTokenBalances": [
        {
          "accountIndex": 3,
          "mint": "4KCqTCbhV4qkCTGcc7adxSym5j7irot7ZW7EEVUJ2s7C",
          "owner": "8QXHLs61dPPz9V4SLKPw7WN1vsuRpgGqxVo3Wtc55jWy",
          "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "uiTokenAmount": {
            "amount": "5584589750228",
            "decimals": 6,
            "uiAmount": 5584589.750228,
            "uiAmountString": "5584589.750228"
          }
        },
        {
          "accountIndex": 4,
          "mint": "4KCqTCbhV4qkCTGcc7adxSym5j7irot7ZW7EEVUJ2s7C",
          "owner": "HLnpSz9h2S4hiLQ43rnSD9XkcUThA7B8hQMKmDaiTLcC",
          "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "uiTokenAmount": {
            "amount": "173663641805557",
            "decimals": 6,
            "uiAmount": 173663641.805557,
            "uiAmountString": "173663641.805557"
          }
        },
        {
          "accountIndex": 5,
          "mint": "So11111111111111111111111111111111111111112",
          "owner": "HLnpSz9h2S4hiLQ43rnSD9XkcUThA7B8hQMKmDaiTLcC",
          "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "uiTokenAmount": {
            "amount": "66507008409",
            "decimals": 9,
            "uiAmount": 66.507008409,
            "uiAmountString": "66.507008409"
          }
        }
      ],
      "preBalances": [
        1001195803,
        0,
        8630400,
        2039280,
        2039280,
        66505193942,
        789146958,
        1181222631482,
        1,
        5299608127,
        1141507,
        38492372815,
        1461600,
        0
      ],
      "preTokenBalances": [
        {
          "accountIndex": 3,
          "mint": "4KCqTCbhV4qkCTGcc7adxSym5j7irot7ZW7EEVUJ2s7C",
          "owner": "8QXHLs61dPPz9V4SLKPw7WN1vsuRpgGqxVo3Wtc55jWy",
          "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "uiTokenAmount": {
            "amount": "5574540307348",
            "decimals": 6,
            "uiAmount": 5574540.307348,
            "uiAmountString": "5574540.307348"
          }
        },
        {
          "accountIndex": 4,
          "mint": "4KCqTCbhV4qkCTGcc7adxSym5j7irot7ZW7EEVUJ2s7C",
          "owner": "HLnpSz9h2S4hiLQ43rnSD9XkcUThA7B8hQMKmDaiTLcC",
          "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "uiTokenAmount": {
            "amount": "173673691248437",
            "decimals": 6,
            "uiAmount": 173673691.248437,
            "uiAmountString": "173673691.248437"
          }
        },
        {
          "accountIndex": 5,
          "mint": "So11111111111111111111111111111111111111112",
          "owner": "HLnpSz9h2S4hiLQ43rnSD9XkcUThA7B8hQMKmDaiTLcC",
          "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "uiTokenAmount": {
            "amount": "66503154662",
            "decimals": 9,
            "uiAmount": 66.503154662,
            "uiAmountString": "66.503154662"
          }
        }
      ],
      "rewards": [],
      "status": {
        "Ok": null
      }
    },
    "slot": 375774267,
    "transaction": {
      "message": {
        "accountKeys": [
          "8QXHLs61dPPz9V4SLKPw7WN1vsuRpgGqxVo3Wtc55jWy",
          "HrRxugUJXCpHCYxqHvccZMafHrsFtsx5UQuBofMD2R5y",
          "yJLejZTJeMnETVDuUcbucL3FhEydMBsFLWtqnY9NABf",
          "BC8y7YcsFayqFBzezL47ZsxRViFmYg6W92QP7i5WSzBv",
          "CWZ2RjpD2Dx7CsRonFATcUu4XR1dWpjWqutMmUmMZZGf",
          "J445YBx3gqz9Xt5JYib6sZqzSsmBDDsjZvLFxXRnwH6j",
          "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
          "So11111111111111111111111111111111111111112",
          "11111111111111111111111111111111",
          "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG",
          "HLnpSz9h2S4hiLQ43rnSD9XkcUThA7B8hQMKmDaiTLcC",
          "4KCqTCbhV4qkCTGcc7adxSym5j7irot7ZW7EEVUJ2s7C",
          "3rmHSu74h1ZcmAisVcWerTCiRDQbUrBKmcwptYGjHfet"
        ],
        "addressTableLookups": [],
        "header": {
          "numReadonlySignedAccounts": 0,
          "numReadonlyUnsignedAccounts": 8,
          "numRequiredSignatures": 1
        },
        "instructions": [
          {
            "accounts": [
              0,
              1,
              0,
              7,
              8,
              9
            ],
            "data": "2",
            "programIdIndex": 6,
            "stackHeight": 1
          },
          {
            "accounts": [
              0,
              1
            ],
            "data": "3Bxs4WvxjMebNdnw",
            "programIdIndex": 8,
            "stackHeight": 1
          },
          {
            "accounts": [
              1
            ],
            "data": "J",
            "programIdIndex": 9,
            "stackHeight": 1
          },
          {
            "accounts": [
              11,
              2,
              1,
              3,
              4,
              5,
              12,
              7,
              0,
              9,
              9,
              10,
              13,
              10
            ],
            "data": "PgQWtn8ozixCzQe4mfN3v1PWXooDCK7xs",
            "programIdIndex": 10,
            "stackHeight": 1
          },
          {
            "accounts": [
              1,
              0,
              0
            ],
            "data": "A",
            "programIdIndex": 9,
            "stackHeight": 1
          }
        ],
        "recentBlockhash": "2Q77XWL3XgX5xTv15xMqjjuHmuYEp3XQ3TFr9ED14BUF"
      },
      "signatures": [
        "3QHC6GMfi2GsZaJ9pFCgRt6E3eevgvgDuULiJEPUzin86S4TUPqZoozNtptkM2iyKqiYWxd1fwFAFkTiU4QCoqfj"
      ]
    },
    "version": 0
  },
  "id": 1
}
    "#;
    let json_txn: loader::RawTxn = serde_json::from_str(json).unwrap();
    println!("{:?}", create_sanitized_txn(json_txn));
}
