package scheduler

import (
	"fmt"
	"time"

	"github.com/DryginAlexander/OpenPlasma/plasma"
)

func RunDeposits(hotConfig plasma.HotConfig, oper plasma.Operator) {
	for {
		// TODO stop though chanel

		per, _ := hotConfig.DepositPeriod()
		time.Sleep(time.Duration(per) * time.Second)

		oper.ExecuteDeposits()

		fmt.Println("RunDeposits every ", per)
	}
	// logger.Printf("[listener] - SHUTDOWN\n")
}

func RunTransfers(hotConfig plasma.HotConfig, oper plasma.Operator) {
	for {
		// TODO stop though chanel

		per, _ := hotConfig.TransferPeriod()
		time.Sleep(time.Duration(per) * time.Second)

		oper.ExecuteTransfers()

		fmt.Println("RunTransfers every ", per)
	}
	// logger.Printf("[listener] - SHUTDOWN\n")
}

func RunOffchainWithdrawals(hotConfig plasma.HotConfig, oper plasma.Operator) {
	for {
		// TODO stop though chanel

		per, _ := hotConfig.OffchainWithdrawalPeriod()
		time.Sleep(time.Duration(per) * time.Second)

		oper.ExecuteOffchainWithdrawals()

		fmt.Println("RunOffchainWithdrawals every ", per)
	}
	// logger.Printf("[listener] - SHUTDOWN\n")
}

func RunOnchainWithdrawals(hotConfig plasma.HotConfig, oper plasma.Operator) {
	for {
		// TODO stop though chanel

		per, _ := hotConfig.OnchainWithdrawalPeriod()
		time.Sleep(time.Duration(per) * time.Second)

		oper.ExecuteOnchainWithdrawals()

		fmt.Println("RunOnchainWithdrawals every ", per)
	}
	// logger.Printf("[listener] - SHUTDOWN\n")
}
