// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.15;

import "../../../solidity_utils/lib.sol";

contract main {
    // solution: a = 1
    function process(uint256 x) public {
        require(msg.sender == address(0x0A101a8A56121470c49B5f477CE7eE7376a92c0a));
        require(x * 2 + 12 == 12903821381231231234132132);
        typed_bug("0x3322");
    }
}