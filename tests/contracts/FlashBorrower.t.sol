// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import { Test } from "forge-std/Test.sol";
import { MockERC20 } from "solmate/test/utils/mocks/MockERC20.sol";

import { FlashBorrower } from "contracts/FlashBorrower.sol";

import { IERC20 } from "contracts/interfaces/IERC20.sol";
import { IERC3156FlashLender } from "contracts/interfaces/IERC3156FlashLender.sol";
import { IERC3156FlashBorrower } from "contracts/interfaces/IERC3156FlashBorrower.sol";


/// @notice Example Flash Lender
/// @notice Conforms to the EIP-3156: Flash Loans
contract FlashLender is IERC3156FlashLender {

    bytes32 public constant CALLBACK_SUCCESS = keccak256("ERC3156FlashBorrower.onFlashLoan");

    event FlashLoan(address indexed receiver, address token, uint256 amount, uint256 fee);

    error FlashLoanCallbackFailed();

    /// @dev The amount of currency available to be lent.
    /// @param token The loan currency.
    // @return The amount of `token` that can be borrowed.
    function maxFlashLoan(address token) external view returns (uint256) {
        return IERC20(token).balanceOf(address(this));
    }

    /// @dev The fee to be charged for a given loan.
    /// @param token The loan currency.
    /// @param amount The amount of tokens lent.
    /// @return The amount of `token` to be charged for the loan, on top of the returned principal.
    function flashFee(address token, uint256 amount) external view returns (uint256) {
        // The flash fee is 1% of the loan amount
        return amount / 100;
    }

    /// @dev Initiate a flash loan.
    /// @param receiver The receiver of the tokens in the loan, and the receiver of the callback.
    /// @param token The loan currency.
    /// @param amount The amount of tokens lent.
    /// @param data Arbitrary data structure, intended to contain user-defined parameters.
    function flashLoan(
        IERC3156FlashBorrower receiver,
        address token,
        uint256 amount,
        bytes calldata data
    ) external returns (bool) {
        uint256 fee = amount / 100;

        // Transfer the token to the receiver
        IERC20(token).transfer(address(receiver), amount);
        emit FlashLoan(address(receiver), token, amount, fee);

        // Call the receiver flashloan callback
        if (receiver.onFlashLoan(msg.sender, token, amount, fee, data) != CALLBACK_SUCCESS) {
            revert FlashLoanCallbackFailed();
        }

        // Transfer the token back to this lender contract
        IERC20(token).transferFrom(address(receiver), address(this), amount + fee);

        // Wowwee, great success!
        return true;
    }
}

contract FlashBorrowerTest is Test {
    FlashBorrower public instance;
    FlashLender public lender;

    MockERC20 public token;

    /// @notice Use a constant owner
    address constant owner = address(0xBA5EBA11BAD);

    function setUp() public {
        lender = new FlashLender();
        instance = new FlashBorrower(lender, owner);
        token = new MockERC20("Mock", "MCK", 18);
        token.mint(address(instance), 1000);
        token.mint(address(lender), 1000);
    }

    function testMetadata() public {
        address configured_owner = instance.owner();
        assertEq(configured_owner, owner);
        address configured_lender = address(instance.lender());
        assertEq(configured_lender, address(lender));
    }

    function testFlashLoan(address leonardo) public {
        vm.assume(leonardo != owner);

        // Verify token balances
        assertEq(token.balanceOf(address(instance)), 1000);
        assertEq(token.balanceOf(address(lender)), 1000);

        FlashBorrower.Call3[] memory no_calls;

        // A random address can't flash loan
        vm.startPrank(leonardo);
        vm.expectRevert(abi.encodeWithSignature("Unauthorized()"));
        instance.flashBorrow(address(token), 1000, no_calls);
        vm.stopPrank();

        // Owner can flash loan
        vm.prank(owner);
        instance.flashBorrow(address(token), 1000, no_calls);
        assertEq(token.balanceOf(address(instance)), 990);
        assertEq(token.balanceOf(address(lender)), 1010);
    }
}