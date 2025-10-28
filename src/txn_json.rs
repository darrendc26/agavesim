pub const JSON1: &str = r#"
{
  "jsonrpc": "2.0",
  "result": {
    "blockTime": 1761676473,
    "meta": {
      "computeUnitsConsumed": 109498,
      "costUnits": 113327,
      "err": null,
      "fee": 6500,
      "innerInstructions": [
        {
          "index": 4,
          "instructions": [
            {
              "accounts": [
                1,
                13,
                4,
                0
              ],
              "data": "g7RNMm7FSEK3v",
              "programIdIndex": 12,
              "stackHeight": 2
            },
            {
              "accounts": [
                5,
                19,
                3,
                2
              ],
              "data": "gwZeRh9JXPF5w",
              "programIdIndex": 12,
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
        "Program ComputeBudget111111111111111111111111111111 invoke [1]",
        "Program ComputeBudget111111111111111111111111111111 success",
        "Program ComputeBudget111111111111111111111111111111 invoke [1]",
        "Program ComputeBudget111111111111111111111111111111 success",
        "Program 11111111111111111111111111111111 invoke [1]",
        "Program 11111111111111111111111111111111 success",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [1]",
        "Program log: Instruction: InitializeAccount",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 3443 of 149550 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK invoke [1]",
        "Program log: Instruction: SwapV2",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
        "Program log: Instruction: TransferChecked",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 6238 of 64998 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
        "Program log: Instruction: TransferChecked",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 6147 of 55736 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program data: QMbN6CYIceLGSzq4mmyjYTSQT2wECCBM/iPRFQ445XjSfjB0aItbF3jDKrg8ER4OnL6dd+nBlbzg7xR9pUZXsuT4gJrJc/Dw17/BOW3XhDzc/fOJEpcjSLOCEp9RB7wSdeWzBg6ApUvivt6J7ccoleUqhxaNEUqRlWeq7Z0XJUfhiBU3EOnk8wCj4REAAAAAAAAAAAAAAABB2dASvAAAAAAAAAAAAAAAAXFCbTamv1DcMwAAAAAAAAApzr4ZgQsAAAAAAAAAAAAAfzQBAA==",
        "Program CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK consumed 102509 of 146107 compute units",
        "Program CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK success",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [1]",
        "Program log: Instruction: CloseAccount",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 3096 of 43598 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success"
      ],
      "postBalances": [
        2825202304,
        0,
        11637120,
        2039280,
        244357125235,
        2039280,
        32092560,
        13641600,
        72161280,
        72161280,
        1,
        1,
        5299613130,
        1181814706560,
        1009200,
        1844545654,
        1709413,
        18921516,
        521498896,
        1461600,
        0
      ],
      "postTokenBalances": [
        {
          "accountIndex": 3,
          "mint": "7eR1yv7pUzhaqQneKJ2fmVTRuzY61ja5KwfbBrLsq1Gu",
          "owner": "98QXkwe6DThTsTq7tukKF4g9Wc6WnRbZyTBZ1iBupY39",
          "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "uiTokenAmount": {
            "amount": "2473007728728",
            "decimals": 6,
            "uiAmount": 2473007.728728,
            "uiAmountString": "2473007.728728"
          }
        },
        {
          "accountIndex": 4,
          "mint": "So11111111111111111111111111111111111111112",
          "owner": "EM4Fvu4G3qLuSNdBeturuHzqh871hh1evxY7RLBoF6ZL",
          "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "uiTokenAmount": {
            "amount": "244355085955",
            "decimals": 9,
            "uiAmount": 244.355085955,
            "uiAmountString": "244.355085955"
          }
        },
        {
          "accountIndex": 5,
          "mint": "7eR1yv7pUzhaqQneKJ2fmVTRuzY61ja5KwfbBrLsq1Gu",
          "owner": "EM4Fvu4G3qLuSNdBeturuHzqh871hh1evxY7RLBoF6ZL",
          "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "uiTokenAmount": {
            "amount": "657651035772212",
            "decimals": 6,
            "uiAmount": 657651035.772212,
            "uiAmountString": "657651035.772212"
          }
        }
      ],
      "preBalances": [
        3125208804,
        0,
        11637120,
        2039280,
        244057125235,
        2039280,
        32092560,
        13641600,
        72161280,
        72161280,
        1,
        1,
        5299613130,
        1181814706560,
        1009200,
        1844545654,
        1709413,
        18921516,
        521498896,
        1461600,
        0
      ],
      "preTokenBalances": [
        {
          "accountIndex": 3,
          "mint": "7eR1yv7pUzhaqQneKJ2fmVTRuzY61ja5KwfbBrLsq1Gu",
          "owner": "98QXkwe6DThTsTq7tukKF4g9Wc6WnRbZyTBZ1iBupY39",
          "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "uiTokenAmount": {
            "amount": "1665238200087",
            "decimals": 6,
            "uiAmount": 1665238.200087,
            "uiAmountString": "1665238.200087"
          }
        },
        {
          "accountIndex": 4,
          "mint": "So11111111111111111111111111111111111111112",
          "owner": "EM4Fvu4G3qLuSNdBeturuHzqh871hh1evxY7RLBoF6ZL",
          "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "uiTokenAmount": {
            "amount": "244055085955",
            "decimals": 9,
            "uiAmount": 244.055085955,
            "uiAmountString": "244.055085955"
          }
        },
        {
          "accountIndex": 5,
          "mint": "7eR1yv7pUzhaqQneKJ2fmVTRuzY61ja5KwfbBrLsq1Gu",
          "owner": "EM4Fvu4G3qLuSNdBeturuHzqh871hh1evxY7RLBoF6ZL",
          "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "uiTokenAmount": {
            "amount": "658458805300853",
            "decimals": 6,
            "uiAmount": 658458805.300853,
            "uiAmountString": "658458805.300853"
          }
        }
      ],
      "rewards": [],
      "status": {
        "Ok": null
      }
    },
    "slot": 376411673,
    "transaction": {
      "message": {
        "accountKeys": [
          "98QXkwe6DThTsTq7tukKF4g9Wc6WnRbZyTBZ1iBupY39",
          "FXCF9YobD2zEzMV3rBxMds5BJKgnQoBoSx6PYvDQqDG6",
          "EM4Fvu4G3qLuSNdBeturuHzqh871hh1evxY7RLBoF6ZL",
          "GG7wqYHurrpmME4fBouwwHDkT81kdeUXqhayTybVdJkr",
          "Dsg513cScwc1u4RZUBzemzBeX1CNrxcXx44XPC1Fs13p",
          "5HMSx5AdqusgT2hwjhESQthABVytczNqiGftTCWNNfCL",
          "EcNRiNur27WLrjchyRDn3NEzbKafHKTYKhKmkpHrLA2h",
          "HxSCKEUMtEnmXWsH6H4MFRvcXpyLxzrzmeMWodaxjF9u",
          "BKtvVa4qZzdLmKA6Vg9sNbtBf6QtMM1MbHVPcGjJ7duX",
          "BhdkwMp2KeE4TiJ5MoAbcoEn78AD4fz6CY5qLwq4oFR",
          "ComputeBudget111111111111111111111111111111",
          "11111111111111111111111111111111",
          "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "So11111111111111111111111111111111111111112",
          "SysvarRent111111111111111111111111111111111",
          "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK",
          "9iFER3bpjf1PTTCQCfTRu17EJgvsxo9pVyA9QWwEuX4x",
          "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
          "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr",
          "7eR1yv7pUzhaqQneKJ2fmVTRuzY61ja5KwfbBrLsq1Gu",
          "jitodontfront111111111111111111111111138623"
        ],
        "addressTableLookups": [],
        "header": {
          "numReadonlySignedAccounts": 0,
          "numReadonlyUnsignedAccounts": 11,
          "numRequiredSignatures": 1
        },
        "instructions": [
          {
            "accounts": [],
            "data": "LEJDE7",
            "programIdIndex": 10,
            "stackHeight": 1
          },
          {
            "accounts": [],
            "data": "3GAG5eogvTjV",
            "programIdIndex": 10,
            "stackHeight": 1
          },
          {
            "accounts": [
              0,
              1
            ],
            "data": "3ipZWqEuESu3WaphD1z1ktAYHda64yXxc4DuvHAt6qrWtrCqZNUhttPtLCdHGXP1uAT2XX34XH6cm4gstW8rkNXDCfm9LmCtdJsJ3tacJWy5WjvetSGjCEdvoRR9SCLgvK7z7FXLafmTXG2PeFTnYXxYYP3e8e8WwsZypgQLQ",
            "programIdIndex": 11,
            "stackHeight": 1
          },
          {
            "accounts": [
              1,
              13,
              0,
              14
            ],
            "data": "2",
            "programIdIndex": 12,
            "stackHeight": 1
          },
          {
            "accounts": [
              0,
              16,
              2,
              1,
              3,
              4,
              5,
              6,
              12,
              17,
              18,
              13,
              19,
              7,
              8,
              9
            ],
            "data": "ASCsAbe1UnDmsbrNcohM5G8wgyTWxmdcD9LqsAZMw4TYtBQcFUnRcAq6",
            "programIdIndex": 15,
            "stackHeight": 1
          },
          {
            "accounts": [
              1,
              0,
              0,
              20
            ],
            "data": "A",
            "programIdIndex": 12,
            "stackHeight": 1
          }
        ],
        "recentBlockhash": "GMycHC9dAeNt8xAWq2Y7G7sB1wDo8mwN7hJuJZtWmm7f"
      },
      "signatures": [
        "2cd7QQSByKy6Y6eddUmQzVwZp5hfrbyhz4osnsf9gmi2EhoScvBoCs5hgJ8WpADH7zhxtQqUcCwBok49KWJ4iJPw"
      ]
    },
    "version": 0
  },
  "id": 1
}
    "#;

pub const JSON2: &str = r#"
  {
  "jsonrpc": "2.0",
  "result": {
    "blockTime": 1761609724,
    "meta": {
      "computeUnitsConsumed": 113961,
      "costUnits": 117451,
      "err": null,
      "fee": 5000,
      "innerInstructions": [
        {
          "index": 0,
          "instructions": [
            {
              "accounts": [
                10
              ],
              "data": "84eT",
              "programIdIndex": 12,
              "stackHeight": 2
            },
            {
              "accounts": [
                0,
                1
              ],
              "data": "11119os1e9qSs2u7TsThXqkBSRVFxhmYaFKFZ1waB2X7armDmvK3p5GmLdUxYdg3h7QSrL",
              "programIdIndex": 11,
              "stackHeight": 2
            },
            {
              "accounts": [
                1
              ],
              "data": "P",
              "programIdIndex": 12,
              "stackHeight": 2
            },
            {
              "accounts": [
                1,
                10
              ],
              "data": "6VDZAoLYRsB3q5F9xSDCyA2npGrfAXbcvzJpi9NFkoido",
              "programIdIndex": 12,
              "stackHeight": 2
            }
          ]
        },
        {
          "index": 1,
          "instructions": [
            {
              "accounts": [
                21,
                13
              ],
              "data": "2BfZXS1GQrCLYKfSSHGxWziZfgGAyj1VLmdEnKpRDGQDWw",
              "programIdIndex": 22,
              "stackHeight": 2
            },
            {
              "accounts": [
                3,
                10,
                1,
                14
              ],
              "data": "h6DyJ741hypeC",
              "programIdIndex": 12,
              "stackHeight": 2
            },
            {
              "accounts": [
                2,
                16,
                4,
                0
              ],
              "data": "ikyPA1XUvUnkG",
              "programIdIndex": 18,
              "stackHeight": 2
            },
            {
              "accounts": [
                2,
                16,
                5,
                0
              ],
              "data": "iUkNcWY5BumiG",
              "programIdIndex": 18,
              "stackHeight": 2
            },
            {
              "accounts": [
                19
              ],
              "data": "2R73ve6nZ42SoaP8dDaUWgVyaYi8rHLBBkT9qY1gYytpjWcbqb4p3GPqLf6iCgy93zTaofcqJbrqBzDbyBbyfe9z2V3pksaCi2BtrxDN2wi2h25FaMmvZ8PZCgfPzLw1difcEjRH5y4jL6qEeNZnCV3L6o4ftCYz8fdvH6v3sppjccKFwhaBcQEQUmFiP6yDL9eCEC3CF4cb2RfoRXLoKNvH2m5PD4jxKXccvumdjCgCVvesALgpD8J5FtBenrdxmpkmWvTuH6WPCjHhx4VB82YCyZb8szkYsXT1eTYyo1TThVcvutwV6b2VVxvbEsCtApd4wY2bJ2sYVx3Tihew2D6UTS6eCF4zX3bknojf8BryYskwVoBiSyfQLiscY4A612QYFQUZRfwmzZ7TfUG48s92hFeLrogLBWRTEtysAyDcYpPT9sLkESFxdnNKQ9xNLDqHaTtYDNAGihV4rwaN9GoZUAZHAkm9f49knK5c8GtGTCZ1XtHddnyth1ahYYf2WHrbtmpDYCoYR39e8umMjsNzZJRSF4yumcDBeAmmAUXtYAAf9daxERDxc",
              "programIdIndex": 13,
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
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 1569 of 594595 compute units",
        "Program return: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA pQAAAAAAAAA=",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program 11111111111111111111111111111111 invoke [2]",
        "Program 11111111111111111111111111111111 success",
        "Program log: Initialize the associated token account",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
        "Program log: Instruction: InitializeImmutableOwner",
        "Program log: Please upgrade to SPL Token 2022 for immutable owner support",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 1405 of 588008 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
        "Program log: Instruction: InitializeAccount3",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 3158 of 584126 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL consumed 19315 of 600000 compute units",
        "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL success",
        "Program pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA invoke [1]",
        "Program log: Instruction: Buy",
        "Program pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ invoke [2]",
        "Program log: Instruction: GetFees",
        "Program pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ consumed 4274 of 527514 compute units",
        "Program return: pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ GQAAAAAAAAAFAAAAAAAAAAAAAAAAAAAA",
        "Program pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ success",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
        "Program log: Instruction: TransferChecked",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 6238 of 519410 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb invoke [2]",
        "Program log: Instruction: TransferChecked",
        "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb consumed 2307 of 510376 compute units",
        "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb success",
        "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb invoke [2]",
        "Program log: Instruction: TransferChecked",
        "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb consumed 2307 of 505291 compute units",
        "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb success",
        "Program data: Z/RSHyz1d3f8BwBpAAAAAE2VBQwAAAAAAJTO/GCCAwAAAAAAAAAAANvnc8pFWwMAfAFNmiIAAAA/fDQycUHGBC9bzrM3qQEAGQAAAAAAAAChRqYjEAEAAAUAAAAAAAAAuqeHbTYAAADQoXTXR6oBAIpJ/ER+qgEAquQc3O69mOCwOPh3Cns9ffcFLjHgfraSbGh470/ibMF4G8ksWnKVIeTX0QEAOaISbuDw3zyHa8IPk6J8hfM4+qgNkoPOog3B8cP1WxTPfR8Wl41mswj7wiBF7exT3F8LXvPPAczdAN1MEWBqi4Nd70Vl5/oVNkZIGfM2CxTEOZT/g4OBi6j6KMPNO21ek/n6uPCXm8NyFazFskaHe6jDybxl/oIZhjGVLG0tEo+PLhQpufHnKy+le7l6vhIpjoZJAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAATZUFDAAAAAADAAAAYnV5",
        "Program pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA invoke [2]",
        "Program pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA consumed 2030 of 495714 compute units",
        "Program pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA success",
        "Program pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA consumed 91731 of 580685 compute units",
        "Program pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA success",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [1]",
        "Program log: Instruction: CloseAccount",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 2915 of 488954 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success"
      ],
      "postBalances": [
        374808413,
        0,
        2074080,
        148417972767,
        2074080,
        2074080,
        2074080,
        25068899,
        1844400,
        789146958,
        1181355288490,
        1,
        5299613127,
        109153251,
        2978880,
        4457517,
        3744480,
        1200013,
        18921513,
        1002026,
        0,
        18374410,
        1151476
      ],
      "postTokenBalances": [
        {
          "accountIndex": 2,
          "mint": "EWV6bjhKF5zkXANCdWPvDs7dofRVk4pKyVTitqgEWj8E",
          "owner": "95rVs91daKmzom4SM41dg4MRmy5M7rMEHPerenZwWaCq",
          "programId": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
          "uiTokenAmount": {
            "amount": "475845960900177",
            "decimals": 9,
            "uiAmount": 475845.960900177,
            "uiAmountString": "475845.960900177"
          }
        },
        {
          "accountIndex": 3,
          "mint": "So11111111111111111111111111111111111111112",
          "owner": "CW644FAJoQE9B9MGqmdfDseS4XfsmgS77ve7zJDJxiPS",
          "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "uiTokenAmount": {
            "amount": "148415933487",
            "decimals": 9,
            "uiAmount": 148.415933487,
            "uiAmountString": "148.415933487"
          }
        },
        {
          "accountIndex": 4,
          "mint": "EWV6bjhKF5zkXANCdWPvDs7dofRVk4pKyVTitqgEWj8E",
          "owner": "CW644FAJoQE9B9MGqmdfDseS4XfsmgS77ve7zJDJxiPS",
          "programId": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
          "uiTokenAmount": {
            "amount": "344503076480687631",
            "decimals": 9,
            "uiAmount": 344503076.4806876,
            "uiAmountString": "344503076.480687631"
          }
        },
        {
          "accountIndex": 5,
          "mint": "EWV6bjhKF5zkXANCdWPvDs7dofRVk4pKyVTitqgEWj8E",
          "owner": "JCRGumoE9Qi5BBgULTgdgTLjSgkCMSbF62ZZfGs84JeU",
          "programId": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
          "uiTokenAmount": {
            "amount": "1956444834189033",
            "decimals": 9,
            "uiAmount": 1956444.834189033,
            "uiAmountString": "1956444.834189033"
          }
        },
        {
          "accountIndex": 6,
          "mint": "EWV6bjhKF5zkXANCdWPvDs7dofRVk4pKyVTitqgEWj8E",
          "owner": "8N3GDaZ2iwN65oxVatKTLPNooAVUJTbfiVJ1ahyqwjSk",
          "programId": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
          "uiTokenAmount": {
            "amount": "0",
            "decimals": 9,
            "uiAmount": null,
            "uiAmountString": "0"
          }
        }
      ],
      "preBalances": [
        173120920,
        0,
        2074080,
        148619665260,
        2074080,
        2074080,
        2074080,
        25068899,
        1844400,
        789146958,
        1181355288490,
        1,
        5299613127,
        109153251,
        2978880,
        4457517,
        3744480,
        1200013,
        18921513,
        1002026,
        0,
        18374410,
        1151476
      ],
      "preTokenBalances": [
        {
          "accountIndex": 2,
          "mint": "EWV6bjhKF5zkXANCdWPvDs7dofRVk4pKyVTitqgEWj8E",
          "owner": "95rVs91daKmzom4SM41dg4MRmy5M7rMEHPerenZwWaCq",
          "programId": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
          "uiTokenAmount": {
            "amount": "944780237596635",
            "decimals": 9,
            "uiAmount": 944780.237596635,
            "uiAmountString": "944780.237596635"
          }
        },
        {
          "accountIndex": 3,
          "mint": "So11111111111111111111111111111111111111112",
          "owner": "CW644FAJoQE9B9MGqmdfDseS4XfsmgS77ve7zJDJxiPS",
          "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "uiTokenAmount": {
            "amount": "148617625980",
            "decimals": 9,
            "uiAmount": 148.61762598,
            "uiAmountString": "148.61762598"
          }
        },
        {
          "accountIndex": 4,
          "mint": "EWV6bjhKF5zkXANCdWPvDs7dofRVk4pKyVTitqgEWj8E",
          "owner": "CW644FAJoQE9B9MGqmdfDseS4XfsmgS77ve7zJDJxiPS",
          "programId": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
          "uiTokenAmount": {
            "amount": "344034375969831999",
            "decimals": 9,
            "uiAmount": 344034375.969832,
            "uiAmountString": "344034375.969831999"
          }
        },
        {
          "accountIndex": 5,
          "mint": "EWV6bjhKF5zkXANCdWPvDs7dofRVk4pKyVTitqgEWj8E",
          "owner": "JCRGumoE9Qi5BBgULTgdgTLjSgkCMSbF62ZZfGs84JeU",
          "programId": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
          "uiTokenAmount": {
            "amount": "1956211068348207",
            "decimals": 9,
            "uiAmount": 1956211.068348207,
            "uiAmountString": "1956211.068348207"
          }
        },
        {
          "accountIndex": 6,
          "mint": "EWV6bjhKF5zkXANCdWPvDs7dofRVk4pKyVTitqgEWj8E",
          "owner": "8N3GDaZ2iwN65oxVatKTLPNooAVUJTbfiVJ1ahyqwjSk",
          "programId": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
          "uiTokenAmount": {
            "amount": "0",
            "decimals": 9,
            "uiAmount": null,
            "uiAmountString": "0"
          }
        }
      ],
      "rewards": [],
      "status": {
        "Ok": null
      }
    },
    "slot": 376243586,
    "transaction": {
      "message": {
        "accountKeys": [
          "95rVs91daKmzom4SM41dg4MRmy5M7rMEHPerenZwWaCq",
          "CK1VokWFPPd2K4u3Jah4EPpjoADkyWXasy5gdHH7CZR4",
          "7Pex41UtrsxvNsNShfbDushiRh5FVVmKEvRPZ7KR65uu",
          "Amf6jEUXpAB1Psmhfo5t9U9cYHLRL1MfsfF3HjL5HD4U",
          "CKFjAt7uNWgV1zzj2TnGDMn2BLENS8KghDzi3PFZJkEM",
          "DgRrNv8cMY8BxD2Fbn8GkkKshVpMqY2qqoonGL6TrD9a",
          "9tV9ugnSn7mBXuZ1y5kV8whZpUiWyUYPQA8uguEWNFEq",
          "C2aFPdENg4A2HQsmrd5rTw5TaYBX5Ku887cWjbFKtZpw",
          "5raVdscuuXL61nm9k4ShxK7S4qoineKQSCjgkfinCGfe",
          "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
          "So11111111111111111111111111111111111111112",
          "11111111111111111111111111111111",
          "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA",
          "CW644FAJoQE9B9MGqmdfDseS4XfsmgS77ve7zJDJxiPS",
          "ADyA8hdefvWN2dbGGWFotbzWxrAvLW83WG6QCVXvJKqw",
          "EWV6bjhKF5zkXANCdWPvDs7dofRVk4pKyVTitqgEWj8E",
          "JCRGumoE9Qi5BBgULTgdgTLjSgkCMSbF62ZZfGs84JeU",
          "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
          "GS4CU59F31iL7aR2Q8zVS8DRrcRnXX1yjQ66TqNVQnaR",
          "8N3GDaZ2iwN65oxVatKTLPNooAVUJTbfiVJ1ahyqwjSk",
          "5PHirr8joyTMp9JMm6nW7hNDVyEYdkzDqazxPD7RaTjx",
          "pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ"
        ],
        "addressTableLookups": [],
        "header": {
          "numReadonlySignedAccounts": 0,
          "numReadonlyUnsignedAccounts": 14,
          "numRequiredSignatures": 1
        },
        "instructions": [
          {
            "accounts": [
              0,
              1,
              0,
              10,
              11,
              12
            ],
            "data": "2",
            "programIdIndex": 9,
            "stackHeight": 1
          },
          {
            "accounts": [
              14,
              0,
              15,
              10,
              16,
              1,
              2,
              3,
              4,
              17,
              5,
              12,
              18,
              11,
              9,
              19,
              13,
              6,
              20,
              7,
              8,
              21,
              22
            ],
            "data": "i43WeUBGKA6coXMv2L47E2Vwhexqi2qBXW",
            "programIdIndex": 13,
            "stackHeight": 1
          },
          {
            "accounts": [
              1,
              0,
              0
            ],
            "data": "A",
            "programIdIndex": 12,
            "stackHeight": 1
          }
        ],
        "recentBlockhash": "FHRhQi2D6473U9sePBV1oevhPrxHQhqH4GVFJEiMCoKn"
      },
      "signatures": [
        "5bvyp1nob51M6FCjCeZd68AjXFWJJEvdqkHu9ZCeRt6sMz5wpUiTi3LPBEuonUHuVg8LwzSnx5MX2RFGSv6Mb36v"
      ]
    },
    "version": 0
  },
  "id": 1
}
    "#;

pub const JSON3: &str = r#"
{
  "jsonrpc": "2.0",
  "result": {
    "blockTime": 1761670504,
    "meta": {
      "computeUnitsConsumed": 220908,
      "costUnits": 225947,
      "err": null,
      "fee": 37351,
      "innerInstructions": [
        {
          "index": 6,
          "instructions": [
            {
              "accounts": [
                24
              ],
              "data": "84eT",
              "programIdIndex": 7,
              "stackHeight": 2
            },
            {
              "accounts": [
                0,
                2
              ],
              "data": "11119os1e9qSs2u7TsThXqkBSRVFxhmYaFKFZ1waB2X7armDmvK3p5GmLdUxYdg3h7QSrL",
              "programIdIndex": 6,
              "stackHeight": 2
            },
            {
              "accounts": [
                2
              ],
              "data": "P",
              "programIdIndex": 7,
              "stackHeight": 2
            },
            {
              "accounts": [
                2,
                24
              ],
              "data": "6c9jiwWoPfpBGS9JBvY6jBdmoPGkxUSfNPtfEghq4sxH2",
              "programIdIndex": 7,
              "stackHeight": 2
            }
          ]
        },
        {
          "index": 7,
          "instructions": [
            {
              "accounts": [
                1,
                22,
                12,
                0
              ],
              "data": "hWgtv2AbuXvWL",
              "programIdIndex": 7,
              "stackHeight": 2
            },
            {
              "accounts": [
                0,
                13
              ],
              "data": "3Bxs4D9UW7fu2b5h",
              "programIdIndex": 6,
              "stackHeight": 2
            },
            {
              "accounts": [
                29,
                11,
                26,
                24,
                22,
                3,
                12,
                17,
                15,
                21,
                18,
                7,
                7,
                6,
                9,
                27,
                28,
                14,
                25,
                16,
                4,
                20,
                19
              ],
              "data": "AJTQ2h9DXrBrs6J8uNPieSNEBPHmk1JLX",
              "programIdIndex": 28,
              "stackHeight": 2
            },
            {
              "accounts": [
                20,
                28
              ],
              "data": "2BfZXS1GQrCLYwuLtn9JiM4y5eUfwmW7DQDSgxX89S9sBd",
              "programIdIndex": 19,
              "stackHeight": 3
            },
            {
              "accounts": [
                17,
                24,
                3,
                29
              ],
              "data": "hWzYSDcH1jne1",
              "programIdIndex": 7,
              "stackHeight": 3
            },
            {
              "accounts": [
                12,
                22,
                15,
                11
              ],
              "data": "hdfWFAqGhgpgY",
              "programIdIndex": 7,
              "stackHeight": 3
            },
            {
              "accounts": [
                12,
                22,
                18,
                11
              ],
              "data": "gGZnCjfvvbXCL",
              "programIdIndex": 7,
              "stackHeight": 3
            },
            {
              "accounts": [
                12,
                22,
                14,
                11
              ],
              "data": "j4cHb9hi3tUC8",
              "programIdIndex": 7,
              "stackHeight": 3
            },
            {
              "accounts": [
                27
              ],
              "data": "2R73ve6nZ42SoaP8dDaUWgUJXQ7rsSFYk2daUeNpQYsu8mFJ7XBLWC49q9xS7G5xyYG2LaJ3he3kv7Gn4oDiaaxVhNUPFJA4hGK6LteqnxDXiBM2GMqy83qt5vwCj727WNBr7Dphd32WVE5WfaYaXUnoQhjCHFXomDZu7UJQuh6xKaiv3jTJKjv8EVG6giLiRqRaALqX1ABEspQsXnLkMkSiFKAaG711UCvWtGa49mQT5wJUq1ZrxHurGVPbvfinaNwECkEj1GkrNrYUYSK1FKmcV2LZxQxrwd1LY8aT6EHhE2QdZRgU7nADZ9H5PYtoqByiy1YniVTnwBMS2HDDj3MVgXHPUwyxvF3U2sqHWoGM9k8XrcRNsoAe5XNeL1FHRuny27J6RDwUbYxDH4dDZzbHoLzB3faqpWfbY3RTdskXAfR7iT4Mrvxm1P7riDqGvky8bEwkjXQGnQBeqtmgfDKV7oy2T9LhcpmuAvR5P2J5UY14xdtnEEpqZrY2SQcCojwX38RJ7HRmB36BKvtw4mCFjaNtkpFanffMoXoNghxUNdSZqt1wz8GR6",
              "programIdIndex": 28,
              "stackHeight": 3
            },
            {
              "accounts": [
                3,
                24,
                2,
                11
              ],
              "data": "hWzYSDcH1jne1",
              "programIdIndex": 7,
              "stackHeight": 2
            }
          ]
        }
      ],
      "loadedAddresses": {
        "readonly": [
          "pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ",
          "5PHirr8joyTMp9JMm6nW7hNDVyEYdkzDqazxPD7RaTjx",
          "62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV",
          "So11111111111111111111111111111111111111112",
          "SysvarRent111111111111111111111111111111111",
          "8k1DvnFBsbuWJw2t4JqTibR7cAgqadMveqiykMr1pump",
          "FGDQFsnbmq2xdtjrzeAaugZ2psy2RKqANTAhUmBr17PN",
          "ADyA8hdefvWN2dbGGWFotbzWxrAvLW83WG6QCVXvJKqw",
          "GS4CU59F31iL7aR2Q8zVS8DRrcRnXX1yjQ66TqNVQnaR",
          "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA",
          "9S3zAF8zVffQvD6EfqaVAdRbnYjVW51kVQDH2vHjk4vq"
        ],
        "writable": [
          "HV1KXxWFaSeriyFvXyx48FqG9BoFbfinB8njCJonqP7K",
          "2rikd7tzPbmowhUJzPNVtX7fuUGcnBa8jqJnx6HbtHeE",
          "8ckLnP69xhSeoNbZCUrbJZ8aYSR86QNjRVZdpHmFfigk",
          "4pBGeyjrtZ7A28a94isV4Gj3jFKrmSAMH1cMyYC2RHQi",
          "iAK6XqgM64VHq8xbiffXTEUnu5u74ToT4tFK8PqbqVu",
          "C2aFPdENg4A2HQsmrd5rTw5TaYBX5Ku887cWjbFKtZpw",
          "4S6TTBXn5P19WBon1qn4DMSFFP7pTBRGUZpkY4NjYLJQ",
          "94qWNrtmfn42h3ZjUZwWvK1MEo9uVmmrBPd2hpNjYDjb"
        ]
      },
      "logMessages": [
        "Program ComputeBudget111111111111111111111111111111 invoke [1]",
        "Program ComputeBudget111111111111111111111111111111 success",
        "Program ComputeBudget111111111111111111111111111111 invoke [1]",
        "Program ComputeBudget111111111111111111111111111111 success",
        "Program 11111111111111111111111111111111 invoke [1]",
        "Program 11111111111111111111111111111111 success",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [1]",
        "Program log: Instruction: InitializeAccount",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 3443 of 331550 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program 11111111111111111111111111111111 invoke [1]",
        "Program 11111111111111111111111111111111 success",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [1]",
        "Program log: Instruction: SyncNative",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 3045 of 327957 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL invoke [1]",
        "Program log: CreateIdempotent",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
        "Program log: Instruction: GetAccountDataSize",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 1569 of 318007 compute units",
        "Program return: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA pQAAAAAAAAA=",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program 11111111111111111111111111111111 invoke [2]",
        "Program 11111111111111111111111111111111 success",
        "Program log: Initialize the associated token account",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
        "Program log: Instruction: InitializeImmutableOwner",
        "Program log: Please upgrade to SPL Token 2022 for immutable owner support",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 1405 of 311420 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
        "Program log: Instruction: InitializeAccount3",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4188 of 307538 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL consumed 21845 of 324912 compute units",
        "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL success",
        "Program 6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma invoke [1]",
        "Program log: Instruction: SwapV3",
        "Program log: commission_rate: 8500000, platform_fee_rate: 10000, trim_rate: 0, commission_direction: true, acc_close_flag: false",
        "Program log: order_id: 16183099615161536",
        "Program log: So11111111111111111111111111111111111111112",
        "Program log: 8k1DvnFBsbuWJw2t4JqTibR7cAgqadMveqiykMr1pump",
        "Program log: G2341KGbNxuSAfCfqNuPhfLQtPB94hPfgyVPC89Faor4",
        "Program log: G2341KGbNxuSAfCfqNuPhfLQtPB94hPfgyVPC89Faor4",
        "Program log: before_source_balance: 3651694, before_destination_balance: 0, amount_in: 3651694, expect_amount_out: 4430341653, min_return: 4279710036",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
        "Program log: Instruction: TransferChecked",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 6238 of 277671 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program 11111111111111111111111111111111 invoke [2]",
        "Program 11111111111111111111111111111111 success",
        "Program log: platform_fee_amount: 31305",
        "Program log: 8ckLnP69xhSeoNbZCUrbJZ8aYSR86QNjRVZdpHmFfigk",
        "Program log: Dex::Pumpfunamm amount_in: 3651694, offset: 0",
        "Program log: 9S3zAF8zVffQvD6EfqaVAdRbnYjVW51kVQDH2vHjk4vq",
        "Program pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA invoke [2]",
        "Program log: Instruction: Buy",
        "Program pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ invoke [3]",
        "Program log: Instruction: GetFees",
        "Program pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ consumed 4641 of 193474 compute units",
        "Program return: pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ FAAAAAAAAAAFAAAAAAAAAF8AAAAAAAAA",
        "Program pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ success",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]",
        "Program log: Instruction: TransferChecked",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 6147 of 184705 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]",
        "Program log: Instruction: TransferChecked",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 6238 of 175757 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]",
        "Program log: Instruction: TransferChecked",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 6238 of 166736 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]",
        "Program log: Instruction: TransferChecked",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 6238 of 157709 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program data: Z/RSHyz1d3do9QBpAAAAAG8huAQBAAAAbrg3AAAAAAAAAAAAAAAAADJ9VhUAAAAAli9DVYuGAAAicKhpHAAAAEcPNwAAAAAAFAAAAAAAAAAxHAAAAAAAAAUAAAAAAAAADQcAAAAAAAB4KzcAAAAAAG24NwAAAAAAfUhkMDWkZ10/Oos/H9+diwNpiBNWhY8+lAWK/l0HoLb0551iz2m4fBfUxb8gdsB4JCo3Jbz9b6Fva9qWTN/OVlKbmvxD6u0bPCFzHKnl4C667KaB0mTvhEJmuEAfcTaZG5hbGEk06WfdwETeTk2VW+QfSkuCB1qa/6jIw+AHLodKwvjQ3Vy8l+MonBl8tQYqVPPZVrnOblEV+WVnqlyz5nfZFZVfiIBzHOtKdaDMlsF0+kCVxOHZlnrPxChFrmeuEwIoup0KjjbuCJaz821Rzh1B+RFqi1JIXo0RcIyI5MtfAAAAAAAAAOiFAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAbyG4BAEAAAADAAAAYnV5",
        "Program pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA invoke [3]",
        "Program pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA consumed 2030 of 144221 compute units",
        "Program pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA success",
        "Program pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA consumed 98345 of 235813 compute units",
        "Program pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA success",
        "Program data: QMbN6CYIceJJbrg3AAAAAABvIbgEAQAAAA==",
        "Program log: SwapEvent { dex: PumpfunammBuy3, amount_in: 3651694, amount_out: 4374143343 }",
        "Program log: 2rikd7tzPbmowhUJzPNVtX7fuUGcnBa8jqJnx6HbtHeE",
        "Program log: 6ZU3sHfLYouv8oPfgBhjwhLuT9gZL24JuE4TF4tzb2Xi",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
        "Program log: Instruction: TransferChecked",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 6147 of 125661 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program log: after_source_balance: 0, after_destination_balance: 4374143343, source_token_change: 3651694, destination_token_change: 4374143343",
        "Program 6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma consumed 188910 of 303067 compute units",
        "Program 6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma success",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [1]",
        "Program log: Instruction: CloseAccount",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 2915 of 114157 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program 11111111111111111111111111111111 invoke [1]",
        "Program 11111111111111111111111111111111 success"
      ],
      "postBalances": [
        5409064,
        0,
        2039280,
        2039280,
        1844400,
        15697157473,
        1,
        5299613130,
        192559521,
        789146961,
        1,
        605296423,
        356377269,
        9682239379329,
        1214457358,
        122037385610,
        25068899,
        2039280,
        4792470379797,
        1151476,
        18374410,
        31565199287468,
        1181814706560,
        1009200,
        30839762,
        0,
        4457517,
        1002026,
        109153251,
        2978880
      ],
      "postTokenBalances": [
        {
          "accountIndex": 2,
          "mint": "8k1DvnFBsbuWJw2t4JqTibR7cAgqadMveqiykMr1pump",
          "owner": "G2341KGbNxuSAfCfqNuPhfLQtPB94hPfgyVPC89Faor4",
          "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "uiTokenAmount": {
            "amount": "4374143343",
            "decimals": 6,
            "uiAmount": 4374.143343,
            "uiAmountString": "4374.143343"
          }
        },
        {
          "accountIndex": 3,
          "mint": "8k1DvnFBsbuWJw2t4JqTibR7cAgqadMveqiykMr1pump",
          "owner": "HV1KXxWFaSeriyFvXyx48FqG9BoFbfinB8njCJonqP7K",
          "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "uiTokenAmount": {
            "amount": "0",
            "decimals": 6,
            "uiAmount": null,
            "uiAmountString": "0"
          }
        },
        {
          "accountIndex": 12,
          "mint": "So11111111111111111111111111111111111111112",
          "owner": "HV1KXxWFaSeriyFvXyx48FqG9BoFbfinB8njCJonqP7K",
          "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "uiTokenAmount": {
            "amount": "354337989",
            "decimals": 9,
            "uiAmount": 0.354337989,
            "uiAmountString": "0.354337989"
          }
        },
        {
          "accountIndex": 14,
          "mint": "So11111111111111111111111111111111111111112",
          "owner": "FGDQFsnbmq2xdtjrzeAaugZ2psy2RKqANTAhUmBr17PN",
          "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "uiTokenAmount": {
            "amount": "1212418078",
            "decimals": 9,
            "uiAmount": 1.212418078,
            "uiAmountString": "1.212418078"
          }
        },
        {
          "accountIndex": 15,
          "mint": "So11111111111111111111111111111111111111112",
          "owner": "9S3zAF8zVffQvD6EfqaVAdRbnYjVW51kVQDH2vHjk4vq",
          "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "uiTokenAmount": {
            "amount": "122035346330",
            "decimals": 9,
            "uiAmount": 122.03534633,
            "uiAmountString": "122.03534633"
          }
        },
        {
          "accountIndex": 17,
          "mint": "8k1DvnFBsbuWJw2t4JqTibR7cAgqadMveqiykMr1pump",
          "owner": "9S3zAF8zVffQvD6EfqaVAdRbnYjVW51kVQDH2vHjk4vq",
          "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "uiTokenAmount": {
            "amount": "147928614899239",
            "decimals": 6,
            "uiAmount": 147928614.899239,
            "uiAmountString": "147928614.899239"
          }
        },
        {
          "accountIndex": 18,
          "mint": "So11111111111111111111111111111111111111112",
          "owner": "62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV",
          "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "uiTokenAmount": {
            "amount": "4792468340515",
            "decimals": 9,
            "uiAmount": 4792.468340515,
            "uiAmountString": "4792.468340515"
          }
        }
      ],
      "preBalances": [
        11368694,
        0,
        0,
        2039280,
        1844400,
        15696957473,
        1,
        5299613130,
        192559521,
        789146961,
        1,
        605296423,
        356377268,
        9682239348024,
        1214423078,
        122033770002,
        25068899,
        2039280,
        4792470377992,
        1151476,
        18374410,
        31565199287468,
        1181814706560,
        1009200,
        30839762,
        0,
        4457517,
        1002026,
        109153251,
        2978880
      ],
      "preTokenBalances": [
        {
          "accountIndex": 3,
          "mint": "8k1DvnFBsbuWJw2t4JqTibR7cAgqadMveqiykMr1pump",
          "owner": "HV1KXxWFaSeriyFvXyx48FqG9BoFbfinB8njCJonqP7K",
          "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "uiTokenAmount": {
            "amount": "0",
            "decimals": 6,
            "uiAmount": null,
            "uiAmountString": "0"
          }
        },
        {
          "accountIndex": 12,
          "mint": "So11111111111111111111111111111111111111112",
          "owner": "HV1KXxWFaSeriyFvXyx48FqG9BoFbfinB8njCJonqP7K",
          "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "uiTokenAmount": {
            "amount": "354337988",
            "decimals": 9,
            "uiAmount": 0.354337988,
            "uiAmountString": "0.354337988"
          }
        },
        {
          "accountIndex": 14,
          "mint": "So11111111111111111111111111111111111111112",
          "owner": "FGDQFsnbmq2xdtjrzeAaugZ2psy2RKqANTAhUmBr17PN",
          "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "uiTokenAmount": {
            "amount": "1212383798",
            "decimals": 9,
            "uiAmount": 1.212383798,
            "uiAmountString": "1.212383798"
          }
        },
        {
          "accountIndex": 15,
          "mint": "So11111111111111111111111111111111111111112",
          "owner": "9S3zAF8zVffQvD6EfqaVAdRbnYjVW51kVQDH2vHjk4vq",
          "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "uiTokenAmount": {
            "amount": "122031730722",
            "decimals": 9,
            "uiAmount": 122.031730722,
            "uiAmountString": "122.031730722"
          }
        },
        {
          "accountIndex": 17,
          "mint": "8k1DvnFBsbuWJw2t4JqTibR7cAgqadMveqiykMr1pump",
          "owner": "9S3zAF8zVffQvD6EfqaVAdRbnYjVW51kVQDH2vHjk4vq",
          "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "uiTokenAmount": {
            "amount": "147932989042582",
            "decimals": 6,
            "uiAmount": 147932989.042582,
            "uiAmountString": "147932989.042582"
          }
        },
        {
          "accountIndex": 18,
          "mint": "So11111111111111111111111111111111111111112",
          "owner": "62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV",
          "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "uiTokenAmount": {
            "amount": "4792468338710",
            "decimals": 9,
            "uiAmount": 4792.46833871,
            "uiAmountString": "4792.46833871"
          }
        }
      ],
      "rewards": [],
      "status": {
        "Ok": null
      }
    },
    "slot": 376396640,
    "transaction": {
      "message": {
        "accountKeys": [
          "G2341KGbNxuSAfCfqNuPhfLQtPB94hPfgyVPC89Faor4",
          "AEUZ5EPwSAXrWKi2w2xQ79hhThAbTWFEdx5gq851ACxi",
          "DybuCHecpyFeWSrprmQjkREFmGZ7vKhWMRsfBCrFSHdL",
          "6ZU3sHfLYouv8oPfgBhjwhLuT9gZL24JuE4TF4tzb2Xi",
          "Dp23JZRq5bneqNVFk47WEyzq3C72FsGBGij4QZMeRBiv",
          "CyL8mfycXYbWHVoTTsfvnAfF2MvfcqeQAmmsqNQLxF7g",
          "11111111111111111111111111111111",
          "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma",
          "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
          "ComputeBudget111111111111111111111111111111"
        ],
        "addressTableLookups": [
          {
            "accountKey": "4uCLrUtySopUJdHCSZkKDQfvDj1asuahxn8xetjvmwPL",
            "readonlyIndexes": [
              88,
              89,
              91,
              60,
              109
            ],
            "writableIndexes": [
              1,
              2,
              105
            ]
          },
          {
            "accountKey": "2gU9RuCMLnq71hXkxFUiezhqvwnKmwEfTiEkyVRrMPF9",
            "readonlyIndexes": [
              64,
              67,
              39,
              41,
              42,
              62
            ],
            "writableIndexes": [
              65,
              66,
              51,
              60,
              46
            ]
          }
        ],
        "header": {
          "numReadonlySignedAccounts": 0,
          "numReadonlyUnsignedAccounts": 5,
          "numRequiredSignatures": 1
        },
        "instructions": [
          {
            "accounts": [],
            "data": "KpFGVm",
            "programIdIndex": 10,
            "stackHeight": 1
          },
          {
            "accounts": [],
            "data": "3gJTmzuZFQpo",
            "programIdIndex": 10,
            "stackHeight": 1
          },
          {
            "accounts": [
              0,
              1
            ],
            "data": "4NK93R6syNfjA4h7NEhQFPsH7KGCjQE6VTtyQ2rTK2gJCrstDEYmH3p28kHUQQ2Re4PYHRb5jPzHgY1FZ7vGkwjL5cXqeiH4uvHMsRPsRGVWa5y1vyx7LvJZG9GpTVCCXYP6Wuf1tK6CZzx",
            "programIdIndex": 6,
            "stackHeight": 1
          },
          {
            "accounts": [
              1,
              22,
              0,
              23
            ],
            "data": "2",
            "programIdIndex": 7,
            "stackHeight": 1
          },
          {
            "accounts": [
              0,
              1
            ],
            "data": "3Bxs4KNn8p2VkxXZ",
            "programIdIndex": 6,
            "stackHeight": 1
          },
          {
            "accounts": [
              1
            ],
            "data": "J",
            "programIdIndex": 7,
            "stackHeight": 1
          },
          {
            "accounts": [
              0,
              2,
              0,
              24,
              6,
              7
            ],
            "data": "2",
            "programIdIndex": 9,
            "stackHeight": 1
          },
          {
            "accounts": [
              0,
              1,
              2,
              22,
              24,
              8,
              13,
              11,
              12,
              3,
              7,
              7,
              9,
              6,
              28,
              11,
              12,
              3,
              29,
              26,
              24,
              22,
              17,
              15,
              21,
              18,
              7,
              7,
              6,
              9,
              27,
              14,
              25,
              16,
              4,
              20,
              19
            ],
            "data": "QGP9CixJcokxowxm7cxuMR1UjT78m21U7oHm8f7aqoYe8HRKFuhLpNbR7V5Cc3PNZC68goAQnhwx3jCAkw33eUteHPBv9zwn2E5bjJHD",
            "programIdIndex": 8,
            "stackHeight": 1
          },
          {
            "accounts": [
              1,
              0,
              0
            ],
            "data": "A",
            "programIdIndex": 7,
            "stackHeight": 1
          },
          {
            "accounts": [
              0,
              5
            ],
            "data": "3Bxs4Ba2u7BkmhYB",
            "programIdIndex": 6,
            "stackHeight": 1
          }
        ],
        "recentBlockhash": "6QWUauJt1LdQamwnX7RwEaVNE9HxZYhsTBqgm3Ura44p"
      },
      "signatures": [
        "2zNeSn8cJpTAW51NR8ZUHEXc5wVwerumVEk4iJFyAYdTHMjy49wumkZvWLkaeB8CD5YmjuJ3AjcbrBYBzAVKdEdD"
      ]
    },
    "version": 0
  },
  "id": 1
}
    "#;

pub const JSON4: &str = r#"
{
  "jsonrpc": "2.0",
  "result": {
    "blockTime": 1761675828,
    "meta": {
      "computeUnitsConsumed": 6385,
      "costUnits": 7738,
      "err": null,
      "fee": 5730,
      "innerInstructions": [],
      "loadedAddresses": {
        "readonly": [],
        "writable": []
      },
      "logMessages": [
        "Program 11111111111111111111111111111111 invoke [1]",
        "Program 11111111111111111111111111111111 success",
        "Program ComputeBudget111111111111111111111111111111 invoke [1]",
        "Program ComputeBudget111111111111111111111111111111 success",
        "Program ComputeBudget111111111111111111111111111111 invoke [1]",
        "Program ComputeBudget111111111111111111111111111111 success",
        "Program MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr invoke [1]",
        "Program log: Memo (len 12): \"CFP #4115716\"",
        "Program MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr consumed 5935 of 19550 compute units",
        "Program MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr success"
      ],
      "postBalances": [
        85376506062,
        435728258459,
        1,
        1,
        521498896
      ],
      "postTokenBalances": [],
      "preBalances": [
        85418511792,
        435686258459,
        1,
        1,
        521498896
      ],
      "preTokenBalances": [],
      "rewards": [],
      "status": {
        "Ok": null
      }
    },
    "slot": 376410054,
    "transaction": {
      "message": {
        "accountKeys": [
          "F5YtngCQs6QCUdy2vqT6hMtFyNkLpkJSTQF2WZKV1y8e",
          "AjWkS35VV8eptekzw59fAyXXQRXjcwanPezL4EDQfSd8",
          "11111111111111111111111111111111",
          "ComputeBudget111111111111111111111111111111",
          "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"
        ],
        "header": {
          "numReadonlySignedAccounts": 0,
          "numReadonlyUnsignedAccounts": 3,
          "numRequiredSignatures": 1
        },
        "instructions": [
          {
            "accounts": [
              0,
              1
            ],
            "data": "3Bxs4NQrPgGEkHzo",
            "programIdIndex": 2,
            "stackHeight": 1
          },
          {
            "accounts": [],
            "data": "3XnP2SXFtQQw",
            "programIdIndex": 3,
            "stackHeight": 1
          },
          {
            "accounts": [],
            "data": "EuxTsD",
            "programIdIndex": 3,
            "stackHeight": 1
          },
          {
            "accounts": [],
            "data": "2GdhgxTGuArEpr9xh",
            "programIdIndex": 4,
            "stackHeight": 1
          }
        ],
        "recentBlockhash": "7Yf63n3a3ankvbq1KjUWd6PpSLcD5gncPdWMorx6Mas9"
      },
      "signatures": [
        "2E9Ddokz6HZkn457gAhd7uaUYnWtgUuRQaGGKFb2xDtYY3Mxn81DaXPMKfLLmcWMUAEQwZG1WF6mfcXMKr2yBqZU"
      ]
    },
    "version": 0
  },
  "id": 1
}
    "#;
