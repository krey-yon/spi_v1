// utils/merkle.ts

/**
 * Convert hex proof array → number[][] (Anchor Vec<[u8; 32]>)
 */
export function hexProofToAnchorFormat(proofHex: string[]): number[][] {
  return proofHex.map((p) => {
    const hex = p.startsWith("0x") ? p.slice(2) : p; // remove 0x prefix
    const buf = Buffer.from(hex, "hex");

    if (buf.length !== 32) {
      throw new Error(`Invalid proof element length: ${buf.length}, expected 32`);
    }

    return Array.from(buf); // number[32]
  });
}

/**
 * Convert Anchor Vec<[u8;32]> → hex array
 */
export function anchorProofToHex(merkleProof: number[][]): string[] {
  return merkleProof.map((arr) => {
    const buf = Buffer.from(arr);
    return buf.toString("hex");
  });
}
