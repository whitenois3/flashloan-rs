// SPDX-License Identifier: MIT
pragma solidity 0.8.17;

import "contracts/interfaces/IERC20.sol";
import "contracts/interfaces/IERC3156FlashBorrower.sol";
import "contracts/interfaces/IERC3156FlashLender.sol";

/// @title FlashBorrower
/// @author asnared <https://github.com/abigger87>
/// @notice A Minimal, Multicallable Flashloan Receiver
contract FlashBorrower is IERC3156FlashBorrower {
    enum Action {NORMAL, OTHER}

    IERC3156FlashLender lender;

    constructor (
        IERC3156FlashLender lender_
    ) public {
        lender = lender_;
    }

    /// @dev ERC-3156 Flash loan callback
    function onFlashLoan(
        address initiator,
        address token,
        uint256 amount,
        uint256 fee,
        bytes calldata data
    ) external override returns (bytes32) {
        require(
            msg.sender == address(lender),
            "FlashBorrower: Untrusted lender"
        );
        require(
            initiator == address(this),
            "FlashBorrower: Untrusted loan initiator"
        );
        (Action action) = abi.decode(data, (Action));
        if (action == Action.NORMAL) {
            require(IERC20(token).balanceOf(address(this)) >= amount);
            // make a profitable trade here
            IERC20(token).transfer(initiator, amount + fee);
        } else if (action == Action.OTHER) {
            // do another
        }
        return keccak256("ERC3156FlashBorrower.onFlashLoan");
    }

    /// @dev Initiate a flash loan
    function flashBorrow(
        address token,
        uint256 amount
    ) public {
        bytes memory data = abi.encode(Action.NORMAL);
        uint256 _allowance = IERC20(token).allowance(address(this), address(lender));
        uint256 _fee = lender.flashFee(token, amount);
        uint256 _repayment = amount + _fee;
        IERC20(token).approve(address(lender), _allowance + _repayment);
        lender.flashLoan(this, token, amount, data);
    }
}