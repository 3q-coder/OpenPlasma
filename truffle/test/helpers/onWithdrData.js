// ONCHAIN WITHDRAWAL -------------------------------------------------------------------

export function OnchainWithdrawal(accountId, amount) {
  this.accountId = accountId;
  this.amount = amount;
}

export const pubkey0 = [
  "0x287b5a2aa9aef9c7de0654160382d033487df1cafd431f8d2ff95f1b6b14b048",
  "0x0fe8488341e558da0b4a8dbb779deecb8159b84e4a4a461b84be825c4043b686"
];

export const pubkey1 = [
  "0x2ed3e5fd16b459910ee0d558653bd846545f815478de51ce9d4f88e8ef5e70c9",
  "0x133c5d147cf2d6ce64f503a091fe0ea37dad691f918377fd32c3555b8c42e083"
];

export var onchainWithdrawals = [
  new OnchainWithdrawal(1, 89),
  new OnchainWithdrawal(2, 101)
]

export const fakeOnWithdrHashes = [
  "0x0000000000000000000000000000000000000000000000000000000000000000",
  "0x0000000000000000000000000000000000000000000000000000000000000001",
  "0x039245dbce9cc77dc150bb394c1f8c301675fbb0fc7a8b101c477299236a4aa3"
]

export const publicInputs = [
  "0x0000000000000000000000000000000000000000000000000000000000000000",
  "0x039245dbce9cc77dc150bb394c1f8c301675fbb0fc7a8b101c477299236a4aa3",
  "0x0e3fd4890bf31bbe6725ddaf5c3c558314de2e0be7ab06f60d45565197e6df02",
  "0x0f0f91774bf1c5b3f86b3c778d96fce1c201352f2d69dd8fb12470f00b75fdc2",
  "0x0000000000000000000000000000000000000000000000000000000000000000",
  "0x0000000000000000000000000000000000000000000000000000000000000059",
  "0x0000000000000000000000000000000000000000000000000000000000000001",
  "0x0000000000000000000000000000000000000000000000000000000000000065"
]

export const proof = [
  "0x008b9d7968f1ff3fb1da419fdc07afe67bf5c26aa5e76bc36e9a3048d9a34a93",
  "0x19d05db81b2283d18b0769013abe62a41b3e803afcf5400f0c13a915f45be2e2",
  "0x2c0f772f3e7a44ab0d031c76675d69b91575b1c275e344d8a72bfbc3082c993a",
  "0x2bd0f328ec23bad55a4c0a229293573c2b16f4890c77b32b8909c981d3d81f82",
  "0x1a00e2495d62d339a73facd7454e5e90282ff141551f6b9203c9f70e022a58ca",
  "0x013367191354cce3a862738c31ea7de8f03ba21886a334e0a6509a2ee341d9d9",
  "0x0b22966694660b8cf439474df63dc6c066dd7680eca422a73ff81fd9f05061b8",
  "0x24eb5c19015659e5afb549a6c587fb32865116d3df0630ddb4dfdf32c2b537f8"
]

// меняем числа в g2 местами
export const vk = [
  "0x039ab8a73e8485bf39d809e5c7f0b906921a266ea3a6fd4ffb3d9f4ccdda913c",
  "0x088ccd6e94ff6ad3c1dd3115ede984144394beeee5b2579a52a3abc4f49396b0",
  "0x250b92dc2b9f0ce46cccd165d48bea22e7a8c80a63b512ca4696f2f78468bb09",
  "0x1bcd8e2bf14a97d368ee7921a1a8e95b28e9e9c78a877aaaf7b4cf94279a9248",
  "0x2056d11c1a6ea65bd3b837272baaeff8d59e3c4e1c4bcab89e42899893501daf",
  "0x21cba87681c13740f0b1c7bb3193d9ab46ffb83aeedeefd3c990436938e89599",
  "0x173f4bdb45eb49bf743b88d251d57ebcae8e4a8a009b4d43ec0b0e0cdb836d0c",
  "0x03ad77eed5bb6aa25754439a5d7f17a25d0e7f3a6843bd45bfc3038193301cf2",
  "0x13750d7d034e2e1350107d0a5626db2284e02586104f45a5923b77563d15b55b",
  "0x26d0cced6a7bb074838b761f77fc7c102604dcaa6920ec218d47cdaa16463191",
  "0x112a8aad1698f1af909a003da4cc7910b6c164f5d86fbbf224ec9f43e4c194ab",
  "0x1c67cc0802b52f4950632e8341c397eb005950fd8440c646545332ab1721ecb6",
  "0x10f12cab960f0da3e38b486d9bd37266f14f9406a89daacd5368d9e7ec80ccc4",
  "0x2a162b989eb99386bb655610990d4a1b0c6ae0d210c850743fbf13d3721d3fff",
  "0x0430cfebd20c21f543d2e551f0aeba8f5069f9f3d4586c01c14cb5fb6e3b58dc",
  "0x16f0cac1361d169664c32e4fbd2f8ec5f1397e922c6b2bdcd4bab4c33260a17f",
  "0x052f62ee8bde2a229a3e84798d0abf69c0a5f2a4df4524af0f256db5b833fa82",
  "0x0762f04ac121d405f41cb61226a3cab6651a6aded0f6bfdafc15ad443d071e2c",
  "0x1c8462e7f0acd044b6301b83d55e5d07417a2d5fd773946b207de19d9c3e0690",
  "0x1669946c7012a3e33aad3b9785418b100558c9a8626fdcd60c48ab3e1b8b1473",
  "0x2dd8b01c4d537375f597faee0bb724833a8a4f8349d52ea9b97d6084633c35b6",
  "0x1a0796b2800cd3bced35f60b2e6cdf7cba895fc83c375598353fafc1a63a4ebc",
  "0x13f5a1ed5ba26155245a6cd5231d76d3d6307b5fef465322c66ab7408a1d5936",
  "0x07bd8afdcade0d3ddbcbee5664df3cb897d1b3accfee277bd0cb74cb39cc9574",
  "0x17f4ad2b91608d4d7d56c132072f18fc9ddf1ded7eaeae77d4e54f4c1c0ebf17",
  "0x006236d2f9b3bf89906bb0b6c67006dd37f01c32f7e345c4d4b208dd1da64279",
  "0x1f4d70be330020e1255281905a0174703a15ebac0789e53caba752393be2aaa5",
  "0x2c666aa53452d0a5772ba4f624e6640c2d984372ea97249bf78794ee4704fcd1",
  "0x28b2087e1723b071c6e61a3dedacde28e9e1e1345a88f2a47519c026f2c6d8ea",
  "0x272220a267b27b6c905ad9b094152c8443d6f2a2ae4a7d54c9bf07a870f2dda6",
  "0x226e9020e8a8269418bb7db5a48c3b145155492a49aa0a788e527e457b479e96",
  "0x2f9678ac42179e4f90ca6f92ccb0cac47b3acfe01512fda8426ef939cfc1c402"
]