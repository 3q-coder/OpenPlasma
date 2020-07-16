export default class Artifacts {
  constructor(artifacts) {
    this.Plasma = artifacts.require("Plasma");
    this.blockVerifier = artifacts.require("BlockVerifier");
  }
}
