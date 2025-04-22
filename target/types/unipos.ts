/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/unipos.json`.
 */
export type Unipos = {
  "address": "E5BHurNhdpFtFFNJkWoFyPRCxyMvMMffZV84YGHyD8kq",
  "metadata": {
    "name": "unipos",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "acceptProviderOwnership",
      "discriminator": [
        24,
        133,
        189,
        238,
        66,
        79,
        251,
        176
      ],
      "accounts": [
        {
          "name": "core",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  114,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "pendingProvider",
          "signer": true,
          "relations": [
            "core"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "addStakeholder",
      "discriminator": [
        165,
        27,
        211,
        200,
        129,
        232,
        133,
        16
      ],
      "accounts": [
        {
          "name": "stakerRecord",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  101,
                  114,
                  95,
                  114,
                  101,
                  99,
                  111,
                  114,
                  100
                ]
              },
              {
                "kind": "account",
                "path": "staker"
              },
              {
                "kind": "arg",
                "path": "number"
              }
            ]
          }
        },
        {
          "name": "staker",
          "writable": true,
          "signer": true
        },
        {
          "name": "stakeholder"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "number",
          "type": "u64"
        },
        {
          "name": "grantedReward",
          "type": "u64"
        },
        {
          "name": "grantedCollateral",
          "type": "u64"
        }
      ]
    },
    {
      "name": "claimBeneficiaryRewards",
      "discriminator": [
        29,
        24,
        147,
        148,
        72,
        37,
        146,
        120
      ],
      "accounts": [
        {
          "name": "core",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  114,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "beneficiary",
          "signer": true
        },
        {
          "name": "beneficiaryTokenAccount",
          "writable": true
        },
        {
          "name": "coreVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  114,
                  101,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": []
    },
    {
      "name": "claimRewards",
      "discriminator": [
        4,
        144,
        132,
        71,
        116,
        23,
        151,
        80
      ],
      "accounts": [
        {
          "name": "core",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  114,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "coreVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  114,
                  101,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "staker"
        },
        {
          "name": "stakerRecord",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  101,
                  114,
                  95,
                  114,
                  101,
                  99,
                  111,
                  114,
                  100
                ]
              },
              {
                "kind": "account",
                "path": "staker"
              },
              {
                "kind": "arg",
                "path": "number"
              }
            ]
          }
        },
        {
          "name": "stakerVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  101,
                  114,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "staker"
              }
            ]
          }
        },
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "number",
          "type": "u64"
        }
      ]
    },
    {
      "name": "claimStakeholderCollateral",
      "discriminator": [
        245,
        23,
        46,
        231,
        160,
        164,
        210,
        131
      ],
      "accounts": [
        {
          "name": "core",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  114,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "stakerRecord",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  101,
                  114,
                  95,
                  114,
                  101,
                  99,
                  111,
                  114,
                  100
                ]
              },
              {
                "kind": "account",
                "path": "staker"
              },
              {
                "kind": "arg",
                "path": "number"
              }
            ]
          }
        },
        {
          "name": "stakerVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  101,
                  114,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "staker"
              }
            ]
          }
        },
        {
          "name": "staker"
        },
        {
          "name": "stakeholderTokenAccount",
          "writable": true
        },
        {
          "name": "stakeholder",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "number",
          "type": "u64"
        }
      ]
    },
    {
      "name": "claimStakeholderReward",
      "discriminator": [
        200,
        99,
        192,
        99,
        117,
        204,
        252,
        156
      ],
      "accounts": [
        {
          "name": "core",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  114,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "stakerRecord",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  101,
                  114,
                  95,
                  114,
                  101,
                  99,
                  111,
                  114,
                  100
                ]
              },
              {
                "kind": "account",
                "path": "staker"
              },
              {
                "kind": "arg",
                "path": "number"
              }
            ]
          }
        },
        {
          "name": "stakerVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  101,
                  114,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "staker"
              }
            ]
          }
        },
        {
          "name": "staker"
        },
        {
          "name": "stakeholderTokenAccount",
          "writable": true
        },
        {
          "name": "stakeholder",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "number",
          "type": "u64"
        }
      ]
    },
    {
      "name": "collect",
      "discriminator": [
        208,
        47,
        194,
        155,
        17,
        98,
        82,
        236
      ],
      "accounts": [
        {
          "name": "core",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  114,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "coreVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  114,
                  101,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "provider",
          "relations": [
            "core"
          ]
        },
        {
          "name": "providerTokenAccount",
          "writable": true
        },
        {
          "name": "admin",
          "signer": true,
          "relations": [
            "core"
          ]
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": []
    },
    {
      "name": "depositSecurity",
      "discriminator": [
        189,
        174,
        184,
        21,
        131,
        77,
        99,
        38
      ],
      "accounts": [
        {
          "name": "core",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  114,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "coreVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  114,
                  101,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "providerTokenAccount",
          "writable": true
        },
        {
          "name": "provider",
          "writable": true,
          "signer": true,
          "relations": [
            "core"
          ]
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "initBeneficiary",
      "discriminator": [
        86,
        252,
        228,
        172,
        87,
        63,
        200,
        27
      ],
      "accounts": [
        {
          "name": "core",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  114,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "admin",
          "signer": true,
          "relations": [
            "core"
          ]
        },
        {
          "name": "beneficiary"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "initialize",
      "discriminator": [
        175,
        175,
        109,
        31,
        13,
        152,
        155,
        237
      ],
      "accounts": [
        {
          "name": "core",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  114,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "coreVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  114,
                  101,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "mint"
        },
        {
          "name": "admin",
          "writable": true,
          "signer": true
        },
        {
          "name": "provider"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        }
      ],
      "args": [
        {
          "name": "lockPeriod",
          "type": "u64"
        },
        {
          "name": "userRewardShare",
          "type": "u64"
        },
        {
          "name": "apy",
          "type": "u64"
        },
        {
          "name": "minStakeAmount",
          "type": "u64"
        },
        {
          "name": "installmentNum",
          "type": "u64"
        }
      ]
    },
    {
      "name": "stake",
      "discriminator": [
        206,
        176,
        202,
        18,
        200,
        209,
        179,
        108
      ],
      "accounts": [
        {
          "name": "core",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  114,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "coreVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  114,
                  101,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "mint"
        },
        {
          "name": "stakerRecord",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  101,
                  114,
                  95,
                  114,
                  101,
                  99,
                  111,
                  114,
                  100
                ]
              },
              {
                "kind": "account",
                "path": "staker"
              },
              {
                "kind": "arg",
                "path": "number"
              }
            ]
          }
        },
        {
          "name": "staker"
        },
        {
          "name": "stakerVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  101,
                  114,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "staker"
              }
            ]
          }
        },
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "userTokenAccount",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        }
      ],
      "args": [
        {
          "name": "number",
          "type": "u64"
        },
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "transferProviderOwnership",
      "discriminator": [
        126,
        60,
        53,
        84,
        132,
        188,
        48,
        54
      ],
      "accounts": [
        {
          "name": "core",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  114,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "newProvider"
        },
        {
          "name": "provider",
          "signer": true,
          "relations": [
            "core"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "unstake",
      "discriminator": [
        90,
        95,
        107,
        42,
        205,
        124,
        50,
        225
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "core",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  114,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "coreVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  114,
                  101,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "staker"
        },
        {
          "name": "stakerRecord",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  101,
                  114,
                  95,
                  114,
                  101,
                  99,
                  111,
                  114,
                  100
                ]
              },
              {
                "kind": "account",
                "path": "staker"
              },
              {
                "kind": "arg",
                "path": "number"
              }
            ]
          }
        },
        {
          "name": "stakerVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  101,
                  114,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "staker"
              }
            ]
          }
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        }
      ],
      "args": [
        {
          "name": "number",
          "type": "u64"
        }
      ]
    },
    {
      "name": "withdrawSecurity",
      "discriminator": [
        96,
        143,
        124,
        115,
        136,
        67,
        148,
        225
      ],
      "accounts": [
        {
          "name": "core",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  114,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "coreVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  114,
                  101,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "providerTokenAccount",
          "writable": true
        },
        {
          "name": "provider",
          "writable": true,
          "signer": true,
          "relations": [
            "core"
          ]
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "core",
      "discriminator": [
        90,
        167,
        99,
        154,
        192,
        227,
        13,
        62
      ]
    },
    {
      "name": "stakerRecord",
      "discriminator": [
        32,
        233,
        136,
        62,
        39,
        209,
        227,
        86
      ]
    }
  ],
  "events": [
    {
      "name": "beneficiaryInitializedEvent",
      "discriminator": [
        152,
        156,
        79,
        221,
        78,
        116,
        224,
        175
      ]
    },
    {
      "name": "beneficiaryRewardsClaimedEvent",
      "discriminator": [
        197,
        80,
        236,
        235,
        67,
        12,
        83,
        86
      ]
    },
    {
      "name": "collectEvent",
      "discriminator": [
        138,
        16,
        76,
        55,
        167,
        75,
        242,
        47
      ]
    },
    {
      "name": "rewardsClaimedEvent",
      "discriminator": [
        22,
        1,
        42,
        183,
        250,
        8,
        157,
        146
      ]
    },
    {
      "name": "securityDepositedEvent",
      "discriminator": [
        25,
        175,
        238,
        187,
        246,
        121,
        18,
        134
      ]
    },
    {
      "name": "securityWithdrawnEvent",
      "discriminator": [
        144,
        3,
        74,
        232,
        209,
        195,
        11,
        82
      ]
    },
    {
      "name": "stakeEvent",
      "discriminator": [
        226,
        134,
        188,
        173,
        19,
        33,
        75,
        175
      ]
    },
    {
      "name": "stakeholderAddedEvent",
      "discriminator": [
        63,
        178,
        113,
        224,
        155,
        77,
        133,
        249
      ]
    },
    {
      "name": "stakeholderRewardClaimedEvent",
      "discriminator": [
        143,
        75,
        8,
        168,
        44,
        17,
        181,
        231
      ]
    },
    {
      "name": "unstakeEvent",
      "discriminator": [
        162,
        104,
        137,
        228,
        81,
        3,
        79,
        197
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "invalidStakeNumber",
      "msg": "Invalid stake number"
    },
    {
      "code": 6001,
      "name": "invalidAmount",
      "msg": "Invalid amount"
    },
    {
      "code": 6002,
      "name": "amountTooSmall",
      "msg": "Amount too small"
    },
    {
      "code": 6003,
      "name": "insufficientAllowance",
      "msg": "Insufficient allowance"
    },
    {
      "code": 6004,
      "name": "notOwner",
      "msg": "Not owner"
    },
    {
      "code": 6005,
      "name": "lockPeriodNotEnded",
      "msg": "Lock period not ended"
    },
    {
      "code": 6006,
      "name": "alreadyClaimed",
      "msg": "Already claimed"
    },
    {
      "code": 6007,
      "name": "nothingToClaim",
      "msg": "Nothing to claim"
    },
    {
      "code": 6008,
      "name": "notProvider",
      "msg": "Not provider"
    },
    {
      "code": 6009,
      "name": "insufficientSecurity",
      "msg": "Insufficient security"
    },
    {
      "code": 6010,
      "name": "notAdmin",
      "msg": "Not admin"
    },
    {
      "code": 6011,
      "name": "alreadyInitialized",
      "msg": "Already initialized"
    },
    {
      "code": 6012,
      "name": "invalidAddress",
      "msg": "Invalid address"
    },
    {
      "code": 6013,
      "name": "notBeneficiary",
      "msg": "Not beneficiary"
    },
    {
      "code": 6014,
      "name": "tooManyStakeholders",
      "msg": "Too many stakeholders"
    },
    {
      "code": 6015,
      "name": "notStakeholder",
      "msg": "Not a stakeholder"
    },
    {
      "code": 6016,
      "name": "stakeholderExists",
      "msg": "Stakeholder exists"
    },
    {
      "code": 6017,
      "name": "stakeholderNotExists",
      "msg": "Stakeholder not exists"
    },
    {
      "code": 6018,
      "name": "noLockedToken",
      "msg": "No locked token"
    },
    {
      "code": 6019,
      "name": "notUnstaked",
      "msg": "Not unstaked"
    }
  ],
  "types": [
    {
      "name": "beneficiaryInitializedEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "beneficiary",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "beneficiaryRewardsClaimedEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "beneficiary",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "collectEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "core",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "type": "pubkey"
          },
          {
            "name": "pendingProvider",
            "type": "pubkey"
          },
          {
            "name": "provider",
            "type": "pubkey"
          },
          {
            "name": "mint",
            "type": "pubkey"
          },
          {
            "name": "lockPeriod",
            "type": "u64"
          },
          {
            "name": "userRewardShare",
            "type": "u64"
          },
          {
            "name": "apy",
            "type": "u64"
          },
          {
            "name": "minStakeAmount",
            "type": "u64"
          },
          {
            "name": "installmentNum",
            "type": "u64"
          },
          {
            "name": "totalCollateral",
            "type": "u64"
          },
          {
            "name": "unstakedCollateral",
            "type": "u64"
          },
          {
            "name": "allowedCollateral",
            "type": "u64"
          },
          {
            "name": "totalClaimedRewards",
            "type": "u64"
          },
          {
            "name": "totalSecurityDeposit",
            "type": "u64"
          },
          {
            "name": "beneficiary",
            "type": "pubkey"
          },
          {
            "name": "beneficiaryTotalRewards",
            "type": "u64"
          },
          {
            "name": "beneficiaryClaimedRewards",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "rewardsClaimedEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "securityDepositedEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "totalSecurity",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "securityWithdrawnEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "remainingSecurity",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "stakeEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "startTime",
            "type": "u64"
          },
          {
            "name": "lockDays",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "stakeholderAddedEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "staker",
            "type": "pubkey"
          },
          {
            "name": "stakeholder",
            "type": "pubkey"
          },
          {
            "name": "grantedReward",
            "type": "u64"
          },
          {
            "name": "grantedCollateral",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "stakeholderInfo",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "stakeholder",
            "type": "pubkey"
          },
          {
            "name": "grantedReward",
            "type": "u64"
          },
          {
            "name": "claimedReward",
            "type": "u64"
          },
          {
            "name": "grantedCollateral",
            "type": "u64"
          },
          {
            "name": "claimedCollateral",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "stakeholderRewardClaimedEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "stakeholder",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "stakerRecord",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "staker",
            "type": "pubkey"
          },
          {
            "name": "collateral",
            "type": "u64"
          },
          {
            "name": "startTime",
            "type": "u64"
          },
          {
            "name": "lockPeriod",
            "type": "u64"
          },
          {
            "name": "lockedRewards",
            "type": "u64"
          },
          {
            "name": "claimedRewards",
            "type": "u64"
          },
          {
            "name": "unstaked",
            "type": "u8"
          },
          {
            "name": "grantedReward",
            "type": "u64"
          },
          {
            "name": "grantedCollateral",
            "type": "u64"
          },
          {
            "name": "stakeholders",
            "type": {
              "vec": {
                "defined": {
                  "name": "stakeholderInfo"
                }
              }
            }
          },
          {
            "name": "stakeholdersCnt",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "unstakeEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    }
  ]
};
