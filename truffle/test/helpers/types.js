/**
 * The type of requests handled in a block.
 */
export var BlockType;
(function (BlockType) {
  BlockType[BlockType["TRANSFER"] = 0] = "TRANSFER";
  BlockType[BlockType["DEPOSIT"] = 1] = "DEPOSIT";
  BlockType[BlockType["ONCHAIN_WITHDRAWAL"] = 2] = "ONCHAIN_WITHDRAWAL";
  BlockType[BlockType["OFFCHAIN_WITHDRAWAL"] = 3] = "OFFCHAIN_WITHDRAWAL";
})(BlockType || (BlockType = {}));
/**
 * The state of the block.
 */
export var BlockState;
(function (BlockState) {
  BlockState[BlockState["COMMITTED"] = 0] = "COMMITTED";
  BlockState[BlockState["VERIFIED"] = 1] = "VERIFIED";
})(BlockState || (BlockState = {}));
