/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/unipos.json`.
 */
export type Unipos = {
  "address": "4FFs789SLFzoYK46z4eShQ1ACZJ4xuEJrKRY3Jpa5Fz7",
  "metadata": {
    "name": "unipos",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
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
          "name": "claimTokenAccount",
          "writable": true
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
      "name": "core",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "type": "pubkey"
          },
          {
            "name": "mint",
            "type": "pubkey"
          },
          {
            "name": "lockPeriodSecs",
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
            "name": "totalClaimedRewards",
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
      "name": "stakerRecord",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "staker",
            "type": "pubkey"
          },
          {
            "name": "startTime",
            "type": "u64"
          },
          {
            "name": "lockPeriodSecs",
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
            "name": "number",
            "type": "u64"
          },
          {
            "name": "claimTokenAccount",
            "type": "pubkey"
          }
        ]
      }
    }
  ]
};
