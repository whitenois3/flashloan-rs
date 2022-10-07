// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import "contracts/interfaces/IERC20.sol";
import "contracts/interfaces/IERC3156FlashBorrower.sol";
import "contracts/interfaces/IERC3156FlashLender.sol";

/// @title FlashBorrower
/// @author asnared <https://github.com/abigger87>
/// @notice A Minimal, Multicallable Flashloan Receiver
contract FlashBorrower is IERC3156FlashBorrower {
    /// @notice The flashloan lender
    IERC3156FlashLender lender;

    /// @notice The contract owner
    address immutable OWNER;

    /// @notice The amount we need to pay back to the lender
    uint256 payback;

    /// @notice Errors if the caller is not the msg.sender
    error Unauthorized();

    /// @notice Errors if the caller is not the flashloan lender
    error UntrustedLender();

    /// @notice Errors if the flashloan initiator is not this contract
    error UntrustedInitiator();

    /// @notice Only this contract can call
    modifier onlySelf() {
        if (msg.sender != address(this)) revert Unauthorized();
        _;
    }

    /// @notice Only the receiver owner can call
    modifier onlyOwner() {
        if (msg.sender != OWNER) revert Unauthorized();
        _;
    }

    ///  /‾‾\__/‾‾\__/‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾\__/‾‾\__/‾‾\
    ///                      CORE LOGIC
    ///  \__/‾‾\__/‾‾\________________________/‾‾\__/‾‾\__/

    /// @notice Receiver Construction
    constructor(IERC3156FlashLender lender_, address owner_) {
        lender = lender_;
        OWNER = owner_;
    }

    /// @notice Executes the flashloan and middle calls
    /// @notice This is the only contract entrypoint
    function flashBorrow(address token, uint256 amount, Call3[] calldata calls) public onlyOwner {
        // Approve the lender to pull the repayment tokens
        // NOTE: Can make the below viewable calls offchain and pass as calldata to save gas
        uint256 allowance = IERC20(token).allowance(address(this), address(lender));
        uint256 fee = lender.flashFee(token, amount);
        uint256 repayment = amount + fee;
        IERC20(token).approve(address(lender), allowance + repayment);

        // Execute the flashloan with encoded calls
        bytes memory data = abi.encode(calls);
        lender.flashLoan(this, token, amount, data);
    }

    ///  /‾‾\__/‾‾\__/‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾\__/‾‾\__/‾‾\
    ///                    ERC-3156 LOGIC
    ///  \__/‾‾\__/‾‾\________________________/‾‾\__/‾‾\__/

    /// @dev ERC-3156 Flash loan callback
    function onFlashLoan(
        address initiator,
        address, // token,
        uint256, // amount
        uint256, // fee
        bytes calldata data
    ) external override returns (bytes32) {
        // Can only be called by the flashloan lender
        if (msg.sender != address(lender)) revert UntrustedLender();

        // The flashloan initiater must be this contract
        if (initiator != address(this)) revert UntrustedInitiator();

        // Execute multicall
        // Place for strategy execution (e.g. https://github.com/makerdao/dss-flash#usage)
        // NOTE: Can use a weiroll virtual machine here to construct contextual calls
        (Call3[] memory calls) = abi.decode(data, (Call3[]));
        aggregate3(calls);

        // Return the ERC-3156 success value
        // The Flashloan lender will pull the approved token (amount + fee) from this contract
        return keccak256("ERC3156FlashBorrower.onFlashLoan");
    }

    ///  /‾‾\__/‾‾\__/‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾\__/‾‾\__/‾‾\
    ///                   MULTICALL LOGIC
    ///  \__/‾‾\__/‾‾\________________________/‾‾\__/‾‾\__/

    struct Call3 {
        address target;
        bool allowFailure;
        uint256 value;
        bytes callData;
    }

    struct Call3Result {
        bool success;
        bytes returnData;
    }

    /// @notice Aggregate calls, ensuring each returns success if required
    /// @notice Only this contract can call from the onFlashLoan callback
    /// @param calls An array of Call3 objects
    /// @return returnData An array of Call3Result objects
    function aggregate3(Call3[] memory calls) internal returns (Call3Result[] memory returnData) {
        uint256 length = calls.length;
        returnData = new Call3Result[](length);
        Call3 memory calli;
        for (uint256 i = 0; i < length;) {
            Call3Result memory result = returnData[i];
            calli = calls[i];
            (result.success, result.returnData) = calli.target.call{value: calli.value}(calli.callData);
            assembly {
                // Revert if the call fails and failure is not allowed
                // `allowFailure := calldataload(add(calli, 0x20))` and `success := mload(result)`
                if iszero(or(calldataload(add(calli, 0x20)), mload(result))) {
                    // set "Error(string)" signature: bytes32(bytes4(keccak256("Error(string)")))
                    mstore(0x00, 0x08c379a000000000000000000000000000000000000000000000000000000000)
                    // set data offset
                    mstore(0x04, 0x0000000000000000000000000000000000000000000000000000000000000020)
                    // set length of revert string
                    mstore(0x24, 0x0000000000000000000000000000000000000000000000000000000000000017)
                    // set revert string: bytes32(abi.encodePacked("Multicall3: call failed"))
                    mstore(0x44, 0x4d756c746963616c6c333a2063616c6c206661696c6564000000000000000000)
                    revert(0x00, 0x64)
                }
            }
            unchecked {
                ++i;
            }
        }
    }
}
