package main

import (
	"context"
	"fmt"

	"github.com/DryginAlexander/OpenPlasma/plasma/models"
	"github.com/DryginAlexander/OpenPlasma/plasma/operator"
	"github.com/DryginAlexander/OpenPlasma/plasma/scheduler"
	"github.com/DryginAlexander/OpenPlasma/plasma/settings"
	"github.com/DryginAlexander/OpenPlasma/plasma/web"
)

func main() {

	fmt.Println("init settings")
	_ = settings.Init("./env/dev.env")

	fmt.Println("connecting to db")
	stor := models.NewStorage()
	defer stor.CloseDB()

	fmt.Println("applying migration if needed")
	_ = stor.MigrateDB()

	fmt.Println("init hot config if needed")
	_ = stor.InitHotConfig("./env/dev.env")

	ctx, finish := context.WithCancel(context.Background())
	defer finish()

	// fmt.Println("init settings")

	// go blockchain.RunListener()
	// go blockchain.RunSender()

	oper := operator.NewOperator(&stor)

	go scheduler.RunDeposits(ctx, &stor, &oper)
	go scheduler.RunTransfers(ctx, &stor, &oper)
	go scheduler.RunOnchainWithdrawals(ctx, &stor, &oper)
	go scheduler.RunOffchainWithdrawals(ctx, &stor, &oper)

	web.Init(&stor, &stor, &oper)
}
