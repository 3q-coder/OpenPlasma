package scheduler

import (
	"context"
	"fmt"
	"time"

	"github.com/DryginAlexander/OpenPlasma/plasma"
)

func RunDeposits(ctx context.Context, hotConfig plasma.HotConfig, oper plasma.Operator) {
	for {
		waitTime, _ := hotConfig.DepositPeriod()

		select {
		case <-ctx.Done():
			fmt.Println("[depositWorker] - SHUTDOWN")
			return
		case <-time.After(waitTime):
			oper.ExecuteDeposits()
			fmt.Println("RunDeposits every ", waitTime)
		}
	}
}

func RunTransfers(ctx context.Context, hotConfig plasma.HotConfig, oper plasma.Operator) {
	for {
		waitTime, _ := hotConfig.TransferPeriod()

		select {
		case <-ctx.Done():
			fmt.Println("[transferWorker] - SHUTDOWN")
			return
		case <-time.After(waitTime):
			oper.ExecuteTransfers()
			fmt.Println("RunTransfers every ", waitTime)
		}
	}
}

func RunOffchainWithdrawals(ctx context.Context, hotConfig plasma.HotConfig, oper plasma.Operator) {
	for {
		waitTime, _ := hotConfig.OffchainWithdrawalPeriod()

		select {
		case <-ctx.Done():
			fmt.Println("[offchainWithdrawalWorker] - SHUTDOWN")
			return
		case <-time.After(waitTime):
			oper.ExecuteOffchainWithdrawals()
			fmt.Println("RunOffchainWithdrawals every ", waitTime)
		}
	}
}

func RunOnchainWithdrawals(ctx context.Context, hotConfig plasma.HotConfig, oper plasma.Operator) {
	for {
		waitTime, _ := hotConfig.OnchainWithdrawalPeriod()

		select {
		case <-ctx.Done():
			fmt.Println("[onchainWithdrawalWorker] - SHUTDOWN")
			return
		case <-time.After(waitTime):
			oper.ExecuteOnchainWithdrawals()
			fmt.Println("RunOnchainWithdrawals every ", waitTime)
		}
	}
}
