import  * as anchor from "@coral-xyz/anchor";
import {
  Connection,
  PublicKey,
  clusterApiUrl,
  SystemProgram
} from "https://esm.sh/@solana/web3.js";
import idl from "./idl/unipos.json" with { type: "json" };

import { Buffer as BufferPolyfill } from "https://esm.sh/buffer@6.0.3";
window.Buffer = BufferPolyfill;

import { getAssociatedTokenAddress, TOKEN_PROGRAM_ID, getOrCreateAssociatedTokenAccount } from "https://esm.sh/@solana/spl-token";

// ✅ 替换为实际 token mint 与合约 programId 映射
// const TOKEN_PROGRAM_MAP = {
//   TOKEN_MINT_A: {
//     mint: new PublicKey("MintPubkeyForTokenA"),
//     programId: new PublicKey("9GZrD2fo3xR54YUjrNiWSxXz8t71HqHE4BBf6R8kqfgz")
//   },
//   TOKEN_MINT_B: {
//     mint: new PublicKey("MintPubkeyForTokenB"),
//     programId: new PublicKey("5p6gFeVmwcD4vYUW7DfDVt1qk3JUbwZfvCRw5QdQEQ7q")
//   }
// };
const PROGRAM_ID = new PublicKey("FbgntXBDCycTQZqGW39rx7FN1DrfgGqmbKJ5s2ABTZiG");
const NEW_PROGRAM_ID = new PublicKey("5p6gFeVmwcD4vYUW7DfDVt1qk3JUbwZfvCRw5QdQEQ7q");
const NETWORK = clusterApiUrl("devnet");
const MINT_ADDRESS = new PublicKey("14Zt6tAhQ7HenvyusiCjKzkK2eQLArMM48niXgD6p2eh");
const PROVIDER_PUBKEY = new PublicKey("DnYmtDs6RMjU5Ta8etcimCEp93H6EH2tPQcATZzHP4kY");
const ASSOCIATED_PROGRAM_ID = new PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
const CORE_SEED = "core";
const CORE_VAULT_SEED = "core_vault";
const STAKE_RECORD_SEED = "staker_record";
const STAKE_VAULT_SEED = "staker_vault";
let provider, wallet;

const log = (...args) => {
  document.getElementById("log").textContent += args.join(" ") + "\n";
};

async function connectWallet() {
  if (!window.solana?.isPhantom) {
    alert("请安装 Phantom 钱包");
    return;
  }

  await window.solana.connect();
  wallet = window.solana;
  log("✅ 钱包连接成功:", wallet.publicKey.toBase58());

  const connection = new Connection(NETWORK, "confirmed");

  const lamports = await connection.getBalance(wallet.publicKey);
  const sol = lamports / 1e9;
  log("钱包余额:", sol.toFixed(2), "SOL");
}

function getSelectedProgram() {
  const selected = document.getElementById("tokenSelect").value;
  // const config = TOKEN_PROGRAM_MAP[selected];

  // if (!config) throw new Error("未知 Token");

  const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
  provider = new anchor.AnchorProvider(connection, wallet, { commitment: "confirmed" });
  console.log("old dl.address:", idl.address);
  idl.address = NEW_PROGRAM_ID.toBase58();
  console.log("new idl.address:", idl.address);
  const program = new anchor.Program(idl, provider);
  console.log("Program ID:", program.programId.toBase58());
  console.log("methods: ", program.methods);
  return program;
}

async function findPda(seed) {
  const [pda, bump] = await PublicKey.findProgramAddress([Buffer.from(seed)], NEW_PROGRAM_ID);
  log(`${seed} Address:`, pda.toBase58());
  return pda;
}

async function init() {
  if (!wallet) {
    log("请先连接钱包");
    return;
  }

  const program = getSelectedProgram();
  const staker = wallet.publicKey;

  const corePDA = await findPda(CORE_SEED);
  const coreVaultPDA = await findPda(CORE_VAULT_SEED);
  log("wallet: ", staker.toBase58());
  log("corePDA: ", corePDA);
  log("coreVaultPDA: ", coreVaultPDA);  
  

  const lockPeriod = 86400 * 30; // 30天锁仓期
  const userRewardShare = 80;
  const apy = 20;
  const minStakeAmount = 1000000; // 最小质押量
  const installmentNum = 3;

  
  try{
    const a = await program.methods
      .initialize(
        new anchor.BN(lockPeriod),
        new anchor.BN(userRewardShare),
        new anchor.BN(apy),
        new anchor.BN(minStakeAmount),
        new anchor.BN(installmentNum)
      )
      .accounts({
        core: corePDA,
        coreVault: coreVaultPDA,
        admin: staker,
        provider: staker, // 需要你填真实的 provider pubkey
        mint: MINT_ADDRESS,         // 需要你填真实的 mint pubkey
        systemProgram: SystemProgram.programId, // ✅ 正确 system program
        tokenProgram: TOKEN_PROGRAM_ID,
      })
    .rpc();
    log("已初始化核心账户和核心金库");
    log("请继续进行存款操作");

    const coreAccount = await program.account.core.all();
    log("核心账户信息:", coreAccount);
    console.log("核心账户信息:", coreAccount);
  } catch (e) {
    console.error(e);
    log("❌ 初始化失败:", e.message || e.toString());
  }
  
}

async function stake() {
  if (!wallet) {
    log("请先连接钱包");
    return;
  }

  const program = getSelectedProgram();
  const staker = wallet.publicKey;

  const corePDA = await findPda(CORE_SEED);
  const coreVaultPDA = await findPda(CORE_VAULT_SEED);
  log("wallet: ", staker.toBase58());
  log("corePDA: ", corePDA);
  log("coreVaultPDA: ", coreVaultPDA);  
  const stakerRecordPDA = await findPda(STAKE_RECORD_SEED);
  const stakerVaultPDA = await findPda(STAKE_VAULT_SEED);
  log("stakerRecordPDA: ", stakerRecordPDA);
  log("stakerVaultPDA: ", stakerVaultPDA);  

  const list = await program.account.stakerRecord.all();
  log("已存在的 Staker Records 数量:", list.length);

  const number = list.length + 1;
  const amount = 1000000;
  const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

  const userTokenAccount = await getOrCreateAssociatedTokenAccount(connection, staker, MINT_ADDRESS, staker);    
  console.log("userTokenAccount:", userTokenAccount.address.toBase58());
  try{
    const a = await program.methods
      .stake(
        new anchor.BN(number),
        new anchor.BN(amount)
      )
      .accounts({
        core: corePDA,
        coreVault: coreVaultPDA,
        mint: MINT_ADDRESS,         // 需要你填真实的 mint pubkey
        systemProgram: SystemProgram.programId, // ✅ 正确 system program
        tokenProgram: TOKEN_PROGRAM_ID,
        stakerRecordPDA: stakerRecordPDA,
        stakerVaultPDA: stakerVaultPDA,
        user: staker,
        userTokenAccount: userTokenAccount.address,
        staker: staker, // 需要你填真实的 staker pubkey
        associatedTokenProgram: ASSOCIATED_PROGRAM_ID, // ✅ 正确的 Associated Token Program
      })
    .rpc();
    log("stake 成功:", a);
  
  } catch (e) {
    console.error(e);
    log("❌ 初始化失败:", e.message || e.toString());
  }
  
}


async function deposit() {
  if (!wallet) {
    log("请先连接钱包");
    return;
  }

  const program = getSelectedProgram();
  const staker = wallet.publicKey;

  const corePDA = await findPda(CORE_SEED);
  const coreVaultPDA = await findPda(CORE_VAULT_SEED);
  log("corePDA: ", corePDA);
  log("coreVaultPDA: ", coreVaultPDA);  

  const amount = 1000000;

  console.log("staker:", staker.toBase58());


  const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
  const ataInfo = await getOrCreateAssociatedTokenAccount( connection, PROVIDER_PUBKEY, MINT_ADDRESS, PROVIDER_PUBKEY );
  console.log("ataInfo:", ataInfo.address.toBase58());
  try{
    const a = await program.methods
      .depositSecurity(
        new anchor.BN(amount)
      )
      .accounts({
        core: corePDA,
        coreVault: coreVaultPDA,
        tokenProgram: TOKEN_PROGRAM_ID,
        provider: staker,
        mint: MINT_ADDRESS,         // 需要你填真实的 mint pubkey
        providerTokenAccount: ataInfo.address,
      })
    .rpc();
    log("deposit 成功:", a);
  
  } catch (e) {
    console.error(e);
    log("❌ deposit失败:", e.message || e.toString());
  }
  
}

document.getElementById("connectBtn").addEventListener("click", connectWallet);
document.getElementById("initBtn").addEventListener("click", init);
document.getElementById("depositBtn").addEventListener("click", deposit);
document.getElementById("stakeBtn").addEventListener("click", stake);