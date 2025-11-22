import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Spi } from "../target/types/spi";
import { PublicKey } from "@solana/web3.js";
import keccak256 from "keccak256";
import MerkleTree from "merkletreejs";
import { hexProofToAnchorFormat } from "./util";

describe("SPI program tests", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const users = [
    "9yq4PYKfL2TANuTmkA7jzqgSh3vVt7Sy8rS7pKXJtWfM",
    "2P7Qv8jskj3xN8X7kaHfWmZkWbySo6bbn2eEz8MPL8ha",
    "FgH7wv8nLgEtnhKyXupZk8UmD9RmA1qjQSpJyeZnXq2P",
    "4o8LB3RBJqB9UddQ4G3wVSnT1sNMF2m5y8h1g8StxDKQ",
    "7Lsx3HDhGr6yZCBf7UXuV5Vj8QzKecbTBYF3pKf9hGqN",
    "8q1gHzbLBGZwqMepHjWg8PHTQTr4LqkKxAd9VrfCuXcz",
    "EPgAqJZQ6T84P2GAsKh4x7h7zoxsGTHV3doh8vF7b6yE",
    "9nXhBph2G1LR6qVZYm3Z1t7bKJkoUXYgzokhr4BdGEum",
    "6j1Xc3o3aMB5qT3bdy7Xog1oY2myG6ZsX2rA9wLEtW8G",
    "EGqp1Wx49TmTgE7XR5CARvzq3dxLVbRXji4KcD9REdtG",
  ];

  const rootSeeds = "membership_root_spi_trial_23";
  const asaSeeds = "user_asa_spi_trial_23";

  const program = anchor.workspace.spi as Program<Spi>;
  const provider = anchor.AnchorProvider.local();

  it("Creating Prime User Merkle Tree", async () => {
    const leaves = users.map((x) => keccak256(x));
    const tree = new MerkleTree(leaves, keccak256, { sortPairs: true });
    const root = tree.getRoot().toString("hex");
    const treeRootHex = Buffer.from(root, "hex");
    const treeRoot = Array.from(treeRootHex);
    console.log("Prime User Merkle Root:", treeRoot);

    const tx = await program.methods
      .createPrimeUserMerkleRootPda(treeRoot)
      .accounts({
        admin: provider.wallet.publicKey,
      })
      .rpc();
    console.log(
      `Prime User Merkle Tree created successfully with tx: https://solscan.io/tx/${tx}?cluster=devnet`
    );
  });

  it("Updating the prime user merkle tree", async () => {
    // const leaves = users.map((address) => keccak256(Buffer.from(address)));
    const leaves = users.map((x) => keccak256(x));
    const tree = new MerkleTree(leaves, keccak256, { sortPairs: true });

    const newUser = keccak256("3gRm7Aj1x22JBu3LPRhWm1SNeEc6jHaC46uxH65Er6rg");
    tree.addLeaf(newUser);
    const root = tree.getRoot();
    const treeRoot = Array.from(root);

    console.log(root);

    const tx = await program.methods
      .updatePrimeUserMerkleTreePda(treeRoot)
      .accounts({
        admin: provider.wallet.publicKey,
      })
      .rpc();

    console.log(
      `Prime User Merkle Tree updated successfully with tx: https://solscan.io/tx/${tx}?cluster=devnet`
    );
  });

  it("creating user asa", async () => {
    const provider = anchor.AnchorProvider.local();

    const leaves = users.map((x) => keccak256(x));
    const tree = new MerkleTree(leaves, keccak256, { sortPairs: true });

    const newLeaf = keccak256("3gRm7Aj1x22JBu3LPRhWm1SNeEc6jHaC46uxH65Er6rg");
    tree.addLeaf(newLeaf);

    const root = tree.getRoot().toString("hex");
    const treeRootHex = Buffer.from(root, "hex");
    const treeRoot = Array.from(treeRootHex);

    // const treeRoot = Array.from(tree.getRoot());
    const proof = tree.getHexProof(newLeaf);
    const proofBytes = hexProofToAnchorFormat(proof);

    const validTill = Math.floor(Date.now() / 1000) + 60 * 60 * 24 * 30;

    const customerPubkey = new PublicKey(
      "3gRm7Aj1x22JBu3LPRhWm1SNeEc6jHaC46uxH65Er6rg"
    );

    const [userAsaPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(asaSeeds), customerPubkey.toBuffer()],
      program.programId
    );

    console.log("Customer:", customerPubkey.toBase58());
    console.log("Authority (payer):", provider.wallet.publicKey.toBase58());
    console.log("UserASA PDA:", userAsaPda.toBase58());

    const storedState = {
      "tree root": tree.getHexRoot(),
      "raw proof": proof,
      "encoded proof": proofBytes,
    };

    console.log("Storing state", storedState);

    const tx = await program.methods
      .updatePrimeUserMerkleTreePda(treeRoot)
      .accounts({
        admin: provider.wallet.publicKey,
      })
      .rpc();

    console.log("tx1", tx);

    const tx2 = await program.methods
      .createUserAsa("vikas", proofBytes, new anchor.BN(validTill))
      .accounts({
        authority: provider.wallet.publicKey,
        customer: customerPubkey,
        userAsa: userAsaPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("✅ ASA created successfully, tx:", tx2);
  });

  it("verify prime membership", async () => {
    const userKey = new PublicKey(
      "3gRm7Aj1x22JBu3LPRhWm1SNeEc6jHaC46uxH65Er6rg"
    );

    const [userASA] = PublicKey.findProgramAddressSync(
      [Buffer.from(asaSeeds), userKey.toBuffer()],
      program.programId
    );

    const [rootPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(rootSeeds)],
      program.programId
    );

    const userAsaAcct = await program.account.userAsa.fetch(userASA);

    const isValid = userAsaAcct.isValid;

    console.log("Valid prime member:", isValid);
  });

  it("Creating merchant token", async () => {
    const spiMint = anchor.web3.Keypair.generate();

    const tx = await program.methods
      .createToken("Kreyon", "KRYN", "https://ipfs.kreyon.in/spi-token")
      .accounts({
        payer: provider.wallet.publicKey,
        spiMint: spiMint.publicKey,
      })
      .signers([spiMint])
      .rpc();
    console.log(tx);
  });

  it("check transfer instruction", async () => {
    const merchant = new PublicKey(
      "3gRm7Aj1x22JBu3LPRhWm1SNeEc6jHaC46uxH65Er6rg"
    );
    const tokenMint = new PublicKey(
      "GdWpjJqsuDdsFa7HHkyu3o7JgfVJB7mE4JteAkNhVf3y"
    );

    const tx = await program.methods
      .mainTransfer(new anchor.BN(800000000), new anchor.BN(3000), 25)
      .accounts({
        merchant: merchant,
        sender: provider.wallet.publicKey,
        spiMint: tokenMint,
      })
      .rpc();
    console.log("transfer transaction sig", tx);
  });
});
