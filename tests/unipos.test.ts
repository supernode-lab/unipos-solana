import * as anchor from "@coral-xyz/anchor";
import { BN } from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { Unipos } from "../target/types/unipos";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import * as token from "@solana/spl-token";

import { assert } from "chai";
import { it } from "mocha";

const anchorProvider = anchor.AnchorProvider.env();
anchor.setProvider(anchorProvider);

const program = anchor.workspace.Unipos as Program<Unipos>;
const connection = anchor.getProvider().connection;
const MILLION = 1000000;

// Test accounts
let admin: web3.Keypair;
let provider1: web3.Keypair;
let provider2: web3.Keypair;
let user: web3.Keypair;
let beneficiary: web3.Keypair;
let stakeholder: web3.Keypair;
let stakeholder2: web3.Keypair;
let stakeholder3: web3.Keypair;
let mint: web3.Keypair;
let core: web3.PublicKey;
let coreVault: web3.PublicKey;
let userTokenAccount: token.Account;
let provider1TokenAccount: token.Account;
let provider2TokenAccount: token.Account;
let beneficiaryTokenAccount: token.Account;
let stakeholderTokenAccount: token.Account;
let stakeholderTokenAccount2: token.Account;
let stakeholderTokenAccount3: token.Account;

// Constants
// const LOCK_PERIOD = 30 * 86400;
const LOCK_PERIOD = 60;
const USER_REWARD_SHARE = 100; // 80%
const APY = 1156500;
const MIN_STAKE_AMOUNT = new anchor.BN(1000000); // 1 token
// const INSTALLMENT_NUM = 86400 * 10; // grant rewards for every 3 seconds
const INSTALLMENT_NUM = 3;
const STAKE_AMOUNT = new anchor.BN(2000_000_000_000); // 2000 tokens
const SECURITY_DEPOSIT = new anchor.BN(1000000000000); // 10000 tokens
const HALF_SECURITY_DEPOSIT = new anchor.BN(500000000000); // 5000 tokens

async function prepare() {
  admin = await createAccount();
  provider1 = await createAccount();
  provider2 = await createAccount();
  user = await createAccount();
  beneficiary = await createAccount();
  stakeholder = await createAccount();
  stakeholder2 = await createAccount();
  stakeholder3 = await createAccount();
  mint = await createMint();

  core = getCoreAddress();
  coreVault = getCoreVaultAddress();
  userTokenAccount = await createTokenAccount(user);
  provider1TokenAccount = await createTokenAccount(provider1);
  provider2TokenAccount = await createTokenAccount(provider2);
  stakeholderTokenAccount = await createTokenAccount(stakeholder);
  stakeholderTokenAccount2 = await createTokenAccount(stakeholder2);
  stakeholderTokenAccount3 = await createTokenAccount(stakeholder3);
}

describe("unipos", () => {
  before(async () => {
    // Generate keypairs
    await prepare();
  });

  it("Initializes the contract", async () => {
    await program.methods
      .initialize(
        new anchor.BN(LOCK_PERIOD),
        new anchor.BN(USER_REWARD_SHARE),
        new anchor.BN(APY),
        MIN_STAKE_AMOUNT,
        new anchor.BN(INSTALLMENT_NUM)
      )
      .accounts({
        mint: mint.publicKey,
        admin: admin.publicKey,
        provider: provider1.publicKey,
      })
      .signers([admin])
      .rpc();

    const coreAccount = await program.account.core.fetch(core);
    assert.equal(coreAccount.admin.toString(), admin.publicKey.toString());
    assert.equal(
      coreAccount.provider.toString(),
      provider1.publicKey.toString()
    );
    assert.equal(coreAccount.mint.toString(), mint.publicKey.toString());
    assert.equal(coreAccount.lockPeriodSecs.toString(), LOCK_PERIOD.toString());
    assert.equal(
      coreAccount.userRewardShare.toString(),
      USER_REWARD_SHARE.toString()
    );
    assert.equal(coreAccount.apyPercentage.toString(), APY.toString());
    assert.equal(
      coreAccount.minStakeAmount.toString(),
      MIN_STAKE_AMOUNT.toString()
    );
    assert.equal(
      coreAccount.installmentNum.toString(),
      INSTALLMENT_NUM.toString()
    );
  });

  it("Security", async () => {
    const infoBefore = await getCoreInfo();
    assert.equal(infoBefore.allowedCollateral.toString(), new BN(0).toString());
    await program.methods
      .depositSecurity(SECURITY_DEPOSIT)
      .accounts({
        providerTokenAccount: provider1TokenAccount.address,
        provider: provider1.publicKey,
      })
      .signers([provider1])
      .rpc();

    const infoAfter = await getCoreInfo();

    assert(Math.abs(infoAfter.totalSecurityDeposit.toNumber() - SECURITY_DEPOSIT.toNumber()) < 5)
    // assert.equal(
    //   infoAfter.totalSecurityDeposit.toString(),
    //   SECURITY_DEPOSIT.toString()
    // );
    assert.ok(
      infoAfter.allowedCollateral.toString() > new anchor.BN(0).toString()
    );

    let err;
    await program.methods
      .withdrawSecurity(HALF_SECURITY_DEPOSIT)
      .accounts({
        providerTokenAccount: provider2TokenAccount.address,
        provider: provider2.publicKey,
      })
      .signers([provider2])
      .rpc()
      .catch((e) => {
        err = e;
      });
    assert.ok(err.toString().includes("Error Code: ConstraintHasOne"));

    await program.methods
      .withdrawSecurity(HALF_SECURITY_DEPOSIT)
      .accounts({
        providerTokenAccount: provider1TokenAccount.address,
        provider: provider1.publicKey,
      })
      .signers([provider1])
      .rpc().catch(e=>{console.log("withdraw first time: ", e)});

    const coreAccount = await getCoreInfo();
    assert(Math.abs(coreAccount.totalSecurityDeposit.toNumber() - HALF_SECURITY_DEPOSIT.toNumber()) < 5)

    await program.methods
        .withdrawSecurity(coreAccount.totalSecurityDeposit)
        .accounts({
          providerTokenAccount: provider1TokenAccount.address,
          provider: provider1.publicKey,
        })
        .signers([provider1])
        .rpc().catch(e=>{console.log("withdraw second time: ", e)});
    const coreAccount2 = await getCoreInfo();
    assert.equal(
        coreAccount2.totalSecurityDeposit.toString(),
        "0"
    );


    await program.methods
      .transferProviderOwnership()
      .accounts({
        newProvider: provider2.publicKey,
        provider: provider1.publicKey,
      })
      .signers([provider1])
      .rpc();
    const ca = await getCoreInfo();
    assert.equal(ca.pendingProvider.toString(), provider2.publicKey.toString());
    assert.equal(ca.provider.toString(), provider1.publicKey.toString());

    await program.methods
      .acceptProviderOwnership()
      .accounts({
        pendingProvider: provider2.publicKey,
      })
      .signers([provider2])
      .rpc();
    const ca1 = await getCoreInfo();
    assert.equal(ca1.provider.toString(), provider2.publicKey.toString());

    await program.methods
        .depositSecurity(SECURITY_DEPOSIT)
        .accounts({
          providerTokenAccount: provider2TokenAccount.address,
          provider: provider2.publicKey,
        })
        .signers([provider2])
        .rpc();
  });

  it("Stakes tokens", async () => {
    await program.methods
      .stake(new anchor.BN(0), STAKE_AMOUNT)
      .accounts({
        mint: mint.publicKey,
        staker: user.publicKey,
        user: user.publicKey,
        userTokenAccount: userTokenAccount.address,
      })
      .signers([user])
      .rpc();

    const coreAccount = await getCoreInfo();
    assert.equal(
      coreAccount.totalCollateral.toString(),
      STAKE_AMOUNT.toString()
    );

    const stakerRecord = await getStakerRecord(user, 0);
    assert.equal(stakerRecord.staker.toString(), user.publicKey.toString());
    assert.equal(
      stakerRecord.lockPeriodSecs.toNumber(),
      coreAccount.lockPeriodSecs.toNumber()
    );
    assert.equal(stakerRecord.collateral.toNumber(), STAKE_AMOUNT.toNumber());
    assert.equal(stakerRecord.unstaked.toString(), new BN(0).toString());
    assert.equal(stakerRecord.claimedRewards.toNumber(), 0);

    console.log("lockedRewards: ", stakerRecord.lockedRewards.toString())
  });

  it("Initializes beneficiary", async () => {
    await program.methods
      .initBeneficiary()
      .accounts({
        admin: admin.publicKey,
        beneficiary: beneficiary.publicKey,
      })
      .signers([admin])
      .rpc();

    const coreAccount = await program.account.core.fetch(core);
    assert.equal(
      coreAccount.beneficiary.toString(),
      beneficiary.publicKey.toString()
    );
  });

  it("Add stakeholder", async () => {
    const grantedReward = new anchor.BN(22000000000);
    const grantedCollateral = new anchor.BN(1000000); // 1 token

    const stakerRecordBefore = await getStakerRecord(user, 0);
    assert.equal(stakerRecordBefore.stakeholdersCnt, 0);
    assert.equal(
      stakerRecordBefore.grantedCollateral.toString(),
      new BN(0).toString()
    );
    assert.equal(
      stakerRecordBefore.grantedReward.toString(),
      new BN(0).toString()
    );

    await program.methods
      .addStakeholder(new anchor.BN(0), grantedReward, grantedCollateral)
      .accounts({
        staker: user.publicKey,
        stakeholder: stakeholder.publicKey,
      })
      .signers([user])
      .rpc();
    let stakerRecordAfter = await getStakerRecord(user, 0);
    assert.equal(stakerRecordAfter.stakeholders.length, 1);
    assert.equal(
      stakerRecordAfter.stakeholders[0].stakeholder.toString(),
      stakeholder.publicKey.toString()
    );
    assert.equal(
      stakerRecordAfter.stakeholders[0].grantedReward.toNumber(),
      grantedReward.toNumber()
    );
    assert.equal(
      stakerRecordAfter.stakeholders[0].grantedCollateral.toNumber(),
      grantedCollateral.toNumber()
    );
    assert.equal(
      stakerRecordAfter.grantedReward.toNumber(),
      grantedReward.toNumber()
    );
    assert.equal(
      stakerRecordAfter.grantedCollateral.toString(),
      grantedCollateral.toString()
    );

    const grantedReward2 = new anchor.BN(22000000000);
    const grantedCollateral2 = new anchor.BN(2000000); // 2 token

    await program.methods
      .addStakeholder(new anchor.BN(0), grantedReward2, grantedCollateral2)
      .accounts({
        staker: user.publicKey,
        stakeholder: stakeholder2.publicKey,
      })
      .signers([user])
      .rpc();
    let stakerRecordAfter2 = await getStakerRecord(user, 0);
    assert.equal(stakerRecordAfter2.stakeholders.length, 2);
    let a = stakerRecordAfter2.stakeholders.find(
      (h) => h.stakeholder.toString() == stakeholder2.publicKey.toString()
    );
    assert.ok(a);
    assert.equal(a.grantedReward.toNumber(), grantedReward2.toNumber());
    assert.equal(a.grantedCollateral.toNumber(), grantedCollateral2.toNumber());
    assert.equal(
      stakerRecordAfter2.grantedReward.toNumber(),
      grantedReward.toNumber() + grantedReward2.toNumber()
    );
    assert.equal(
      stakerRecordAfter2.grantedCollateral.toNumber(),
      grantedCollateral.add(grantedCollateral2).toNumber()
    );
  });

  it("Claims rewards", async () => {
    // Wait for some time to accumulate rewards
    await new Promise((resolve) => setTimeout(resolve, 40000));

    await program.methods
      .claimRewards(new anchor.BN(0))
      .accounts({
        staker: user.publicKey,
        user: user.publicKey,
      })
      .signers([user])
      .rpc()
      .catch((e) => {
        console.log("err: ", e);
      });

    const coreAccount = await getCoreInfo();
    const claimedRewards = coreAccount.totalClaimedRewards;
    const beneficiaryRewards = coreAccount.beneficiaryTotalRewards;
    console.log(`total claimed rewards: ${claimedRewards}`)
    assert.isAbove(Number(claimedRewards), 0);
    assert.equal(
      Math.floor(Number(claimedRewards) / Number(beneficiaryRewards)),
      USER_REWARD_SHARE / (100 - USER_REWARD_SHARE)
    );
    const stakerVaultBalance = await getStakerVaultBalance(user);
    assert.equal(Number(stakerVaultBalance), Number(claimedRewards));

    it("Claims beneficiary rewards", async () => {
      await program.methods
        .claimBeneficiaryRewards()
        .accounts({
          beneficiary: beneficiary.publicKey,
          beneficiaryTokenAccount: beneficiaryTokenAccount.address,
        })
        .signers([beneficiary])
        .rpc();

      const bta = await token.getAccount(
        connection,
        beneficiaryTokenAccount.address
      );
      assert.equal(Number(bta.amount), Number(beneficiaryRewards));
      const coreAccount = await getCoreInfo();
      assert.equal(
        coreAccount.beneficiaryClaimedRewards.toNumber(),
        beneficiaryRewards.toNumber()
      );
      assert.equal(
        coreAccount.totalClaimedRewards.toNumber() - claimedRewards.toNumber(),
        beneficiaryRewards.toNumber()
      );
    });
  });

  it("Claims stakeholder rewards", async () => {
    const stakerRecordBefore = await getStakerRecord(user, 0);

    console.log("total rewards: ", stakerRecordBefore.lockedRewards.toNumber() + stakerRecordBefore.claimedRewards.toNumber())
    console.log("total granted rewards: ", stakerRecordBefore.grantedReward.toNumber())
    console.log("stakeholders: ", stakerRecordBefore.stakeholders)

    for (const stakeholder of stakerRecordBefore.stakeholders) {
      console.log("stakeholder: ", stakeholder.stakeholder.toString(), stakeholder.grantedReward.toNumber())
    }

    const stakerVaultBalance = await getStakerVaultBalance(user);

    console.log(`stakerRecordBefore: claimedRewards: ${stakerRecordBefore.claimedRewards}, lockedRewards: ${stakerRecordBefore.lockedRewards}`)
    const stakeholderAccount1 = await token.getAccount(
      connection,
      stakeholderTokenAccount.address
    );
    await program.methods
      .claimStakeholderReward(new anchor.BN(0))
      .accounts({
        staker: user.publicKey,
        stakeholderTokenAccount: stakeholderTokenAccount.address,
        stakeholder: stakeholder.publicKey,
      })
      .signers([stakeholder])
      .rpc();

    const stakeholderAccount2 = await token.getAccount(
      connection,
      stakeholderTokenAccount.address
    );
    let stakeholder1Earned =
      stakeholderAccount2.amount - stakeholderAccount1.amount;

    const stakeholderAccount3 = await token.getAccount(
      connection,
      stakeholderTokenAccount2.address
    );
    await program.methods
      .claimStakeholderReward(new anchor.BN(0))
      .accounts({
        staker: user.publicKey,
        stakeholderTokenAccount: stakeholderTokenAccount2.address,
        stakeholder: stakeholder2.publicKey,
      })
      .signers([stakeholder2])
      .rpc()
      .catch((e) => {
        console.log(e);
      });
    const stakeholderAccount4 = await token.getAccount(
      connection,
      stakeholderTokenAccount2.address
    );
    let stakeholder2Earned =
      stakeholderAccount4.amount - stakeholderAccount3.amount;
    console.log(`stakeholder2Earned: ${stakeholder2Earned}, stakeholder1Earned: ${stakeholder1Earned}`)
    assert.equal(stakeholder2Earned, stakeholder1Earned);

    const stakerRecordAfter = await getStakerRecord(user, 0);

    console.log(`stakerRecordAfter: claimedRewards: ${stakerRecordAfter.claimedRewards}, lockedRewards: ${stakerRecordAfter.lockedRewards}`)
  });

  it("Unstake", async () => {
    await new Promise((resolve) => setTimeout(resolve, 25000));

    const coreVaultBalance1 = await getCoreVaultBalance();
    const stakerVaultBalance1 = await getStakerVaultBalance(user);
    const coreInfo1 = await getCoreInfo();
    await program.methods
      .unstake(new anchor.BN(0))
      .accounts({
        staker: user.publicKey,
        user: user.publicKey,
      })
      .signers([user])
      .rpc();
    const stakerVaultBalance2 = await getStakerVaultBalance(user);
    const coreVaultBalance2 = await getCoreVaultBalance();
    const coreInfo2 = await getCoreInfo();

    assert.equal(
      Number(stakerVaultBalance2 - stakerVaultBalance1),
      STAKE_AMOUNT.toNumber()
    );
    assert.equal(
      Number(coreVaultBalance1 - coreVaultBalance2),
      STAKE_AMOUNT.toNumber()
    );
    assert.equal(
      coreInfo2.unstakedCollateral.toNumber() -
        coreInfo1.unstakedCollateral.toNumber(),
      STAKE_AMOUNT.toNumber()
    );

    const stakerRecord = await getStakerRecord(user, 0);
    assert.equal(stakerRecord.unstaked, 1);
    // const coreAccount = await getCoreInfo();
  });

  it("Claims stakeholder colalteral", async () => {
    const stakerRecordBefore = await getStakerRecord(user, 0);
    const stakerVaultBalance = await getStakerVaultBalance(user);

    const stakeholderAccount1 = await token.getAccount(
      connection,
      stakeholderTokenAccount.address
    );
    await program.methods
      .claimStakeholderCollateral(new anchor.BN(0))
      .accounts({
        staker: user.publicKey,
        stakeholderTokenAccount: stakeholderTokenAccount.address,
        stakeholder: stakeholder.publicKey,
      })
      .signers([stakeholder])
      .rpc();

    const stakeholderAccount2 = await token.getAccount(
      connection,
      stakeholderTokenAccount.address
    );
    let stakeholder1Earned =
      stakeholderAccount2.amount - stakeholderAccount1.amount;
    assert.equal(Number(stakeholder1Earned), 1000000);

    const stakeholderAccount3 = await token.getAccount(
      connection,
      stakeholderTokenAccount2.address
    );
    await program.methods
      .claimStakeholderCollateral(new anchor.BN(0))
      .accounts({
        staker: user.publicKey,
        stakeholderTokenAccount: stakeholderTokenAccount2.address,
        stakeholder: stakeholder2.publicKey,
      })
      .signers([stakeholder2])
      .rpc()
      .catch((e) => {
        console.log(e);
      });
    const stakeholderAccount4 = await token.getAccount(
      connection,
      stakeholderTokenAccount2.address
    );
    let stakeholder2Earned =
      stakeholderAccount4.amount - stakeholderAccount3.amount;
    assert.equal(Number(stakeholder2Earned), 2000000);
  });
  it("Collect from vault", async () => {
    const amount = 100000000;
    const coreVaultAddr = getCoreVaultAddress();
    const balance1 = await getCoreVaultBalance();
    await token.transfer(
      connection,
      user,
      userTokenAccount.address,
      coreVaultAddr,
      user,
      amount,
      [user]
    );
    const balance2 = await getCoreVaultBalance();
    assert.equal(Number(balance2 - balance1), amount);

    const provider2Balance1 = await (
      await token.getAccount(connection, provider2TokenAccount.address)
    ).amount;
    await program.methods
      .collectFromPool()
      .accounts({
        provider: provider2.publicKey,
        providerTokenAccount: provider2TokenAccount.address,
        admin: admin.publicKey,
      })
      .signers([admin])
      .rpc();
    const balance3 = await getCoreVaultBalance();
    assert.equal(balance3, balance1);

    const provider2Balance2 = await (
      await token.getAccount(connection, provider2TokenAccount.address)
    ).amount;
    assert.equal(Number(provider2Balance2 - provider2Balance1), amount);
  });
});

async function createAccount(): Promise<web3.Keypair> {
  const account = web3.Keypair.generate();
  let r1 = await connection.requestAirdrop(account.publicKey, 10000 * MILLION);
  await connection.confirmTransaction(String(r1));
  return account;
}

async function createTokenAccount(
  account: web3.Keypair
): Promise<token.Account> {
  let a = await token.getOrCreateAssociatedTokenAccount(
    connection,
    account,
    mint.publicKey,
    account.publicKey,
    null,
    null,
    null,
    token.TOKEN_PROGRAM_ID,
    token.ASSOCIATED_TOKEN_PROGRAM_ID
  );
  await token.mintTo(
    connection,
    account,
    mint.publicKey,
    a.address,
    mint,
    10000000000 * MILLION,
    [],
    null,
    token.TOKEN_PROGRAM_ID
  );
  return a;
}

async function createMint(): Promise<web3.Keypair> {
  const mint = web3.Keypair.generate();
  await token
    .createMint(
      connection,
      admin,
      mint.publicKey,
      null,
      6,
      mint,
      undefined,
      token.TOKEN_PROGRAM_ID
    )
    .catch((e) => {
      console.log("e: ", e);
    });
  return mint;
}

async function getCoreVaultBalance() {
  const addr = getCoreVaultAddress();
  return (await token.getAccount(connection, addr)).amount;
}

function getCoreAddress() {
  const [core, _] = PublicKey.findProgramAddressSync(
    [Buffer.from("core")],
    program.programId
  );
  return core;
}

function getCoreVaultAddress() {
  const [coreVault, _] = PublicKey.findProgramAddressSync(
    [Buffer.from("core_vault")],
    program.programId
  );
  return coreVault;
}

async function getCoreInfo() {
  return await program.account.core.fetch(core);
}

async function getStakerRecord(staker: web3.Keypair, num: number) {
  const [stakerRecordPda, _] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("staker_record"),
      staker.publicKey.toBuffer(),
      numberToLEBytes(num),
    ],
    program.programId
  );
  return await program.account.stakerRecord.fetch(stakerRecordPda);
}

async function getStakerVaultBalance(staker: web3.Keypair) {
  const [stakerVaultPda, _] = PublicKey.findProgramAddressSync(
    [Buffer.from("staker_vault"), staker.publicKey.toBuffer()],
    program.programId
  );
  return (await token.getAccount(connection, stakerVaultPda)).amount;
}

function numberToLEBytes(n: number): Uint8Array {
  // 转为 BigInt，确保支持大整数（假设 n 是安全整数）
  const big = BigInt(n);
  const bytes = new Uint8Array(8); // 64 位 = 8 字节

  for (let i = 0; i < 8; i++) {
    bytes[i] = Number((big >> BigInt(8 * i)) & BigInt(0xff));
  }

  return bytes;
}
