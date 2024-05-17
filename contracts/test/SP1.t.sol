// // SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Test, console} from "lib/forge-std/src/Test.sol";
import {stdJson} from "lib/forge-std/src/StdJson.sol";
import {SP1Verifier} from "../src/SP1Verifier.sol";

struct SP1ProofFixtureJson {
    bytes proof;
    bytes publicValues;
    bytes32 vkey;
}

contract SP1VerifierTest is Test {
    using stdJson for string;

    SP1Verifier public sp1_verifier;

    function loadFixture() public view returns (SP1ProofFixtureJson memory) {
        string memory root = vm.projectRoot();
        string memory path = string.concat(
            root,
            "/src/fixtures/fixture.json"
        );
        string memory json = vm.readFile(path);
        bytes memory jsonBytes = json.parseRaw(".");
        return abi.decode(jsonBytes, (SP1ProofFixtureJson));
    }

    function setUp() public {
        sp1_verifier = new SP1Verifier();
    }

    function test_ValidGroth16Proof() public view {
        SP1ProofFixtureJson memory fixture = loadFixture();
        sp1_verifier.verifyProof(
            fixture.vkey,
            fixture.publicValues,
            fixture.proof
        );
    }
}
