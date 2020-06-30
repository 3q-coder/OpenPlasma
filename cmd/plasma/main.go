package main

import (
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

	// fmt.Println("init settings")

	// go blockchain.RunListener()
	// go blockchain.RunSender()

	oper := operator.NewOperator(&stor)

	go scheduler.RunDeposits(&stor, &oper)
	go scheduler.RunTransfers(&stor, &oper)
	go scheduler.RunOnchainWithdrawals(&stor, &oper)
	go scheduler.RunOffchainWithdrawals(&stor, &oper)

	web.Init(&stor, &stor, &oper)
}
