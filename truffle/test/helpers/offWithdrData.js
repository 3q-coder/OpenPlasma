// OFFCHAIN WITHDRAWAL ------------------------------------------------------------------

export function OffchainWithdrawal(accountId, amount) {
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

export var offchainWithdrawals = [
  new OffchainWithdrawal(1, 10)
]

export const publicInputs = [
  "0x00ab7879ee215ff7e6d40c7c0393f00d01de3e2bf239290f21e39fce476cfc5e",
  "0x0e3fd4890bf31bbe6725ddaf5c3c558314de2e0be7ab06f60d45565197e6df02",
  "0x0000000000000000000000000000000000000000000000000000000000000000",
  "0x000000000000000000000000000000000000000000000000000000000000000a"
]

export const proof = [
  "0x2441c31764eae5a7e546bfb02f4ccdc02c020610ff1b3ada21671eb65f38f968",
  "0x257eee1175890416fec3aec861d659a48330a1e54cd9107b100ac9ae5dac7820",
  "0x092a8d3499500ead62502ea2460c7474745618bf1d452a01976ffd46b37e4a49",
  "0x0c6f656851fa8491b4a5ce29e0943e54c49156c2ed2cd474dccb2f3555ae895e",
  "0x25a44dd750ada3a4279bab36cc4d0dd2a0cf8e595b255e1b8c45bf3cd09f85bc",
  "0x032e9046426270bbb24082ea6f9215d9fb79b84b4dcaf3de36674dbc4de29b77",
  "0x04986027b1aed4aa64bce1e2d46f9856e0f5e442b0e2c1bc684f1df85e039511",
  "0x04be03b579d6981fe55744bc5c775f5ecf2af7006cd61816822ad8d5c2c85ecf"
]

// меняем числа в g2 местами
export const vk = [
  "0x304db2266ec06cd1112281f194be336606d5007bce67bd281de8e77e6729ad88",
  "0x0b50d40f681171862f1eb4a6f20116ee7c1a8bd61bf8d9ab7e685d9e1cc82f15",

  "0x084185e07af07e3e0b43a4f5706b508d26e024076b2c78793da23adf18df2d48",
  "0x0a35fcf50f857666730689fa4377989febc85c83fb1a0d81b4b56a21eef8b664",
  "0x27f82053bbfd748fd1db430dddbc72d2aed3ca8d2986611c24d97574bb26ddd5",
  "0x2f4da9b05d88c82edf121ff11b84dfd4208575bf83c4a1847fcdde668750ec88",

  "0x0933961496b690e6fabd7eccbe256b216bfdc61fd8fb2bfb4907871aba1edf47",
  "0x1bb835e79bd4fa95b18b7c3258c90d5923d6bd2cbda67a199411855c30754e00",
  "0x08f4d47b9c70aad7b126d480b7d34854e617ee86b03810b001d14a5bf2ca078d",
  "0x1ded4b597b8a1f3b790aee8525e0b8d181b11eb1a8ba5eb7b3922bab35fb3530",

  "0x05a433050fc7e04616b89dae5877a9d08c68c49c6c103966d1aa513746676aa2",
  "0x1a9cae10eef593a0ae4a2f3394d2d183f14a578493ce07f1618e61cc81bdf8fa",
  "0x16f6f35ef9a4be486dd392b98a62c578d581f817e34e7dd9d62123dca2ca47f6",
  "0x1f5f668bd7a946204defe751ebfe7986b2b32d6556478ca9c2fbc0e62d177706",
  
  "0x047271910d0e79c9e170fdeedfb28c814a4b7e950b3c5c8e8be10dd0d67c004b",
  "0x301668757b2fc46bfc3850ee2609d7dcffd0e658359ded0a1f7c878afdde6c96",
  "0x2e447256e8aa733d8c75542655c8f21cba0da01e5232c91e56fb86d7da744283",
  "0x09528b8a6f0033510ebf82fafefc6e282b569be0a324716435cc10bdf0abec07",
  "0x23fd6f1697101ce97f1f9bef95aaadee66b0ce403e2f061564d2f8c9a8eb8cb9",
  "0x2aca7ebff87c11dd8757a7de3d85c22cb370d0e76ea40f78a70403b40228bb9f",
  "0x059327df31715f935f3276f9d524c439cfca482d7513dc51bb105fddb4d221aa",
  "0x00028b4d5d334e283b89f1dd02db09458c47af9a34dbf0acc420f2c0b3c02d37",
  "0x146bb42db58743c28a1d4d405389a53f37ccf52ac0a840813c12c6e213dc746c",
  "0x19884bf587139b1c1e3a93879e68681df32f39e4ff915ce0cd0e712167fbaa9f"
]
