package main

import (
	"fmt"

	"github.com/DryginAlexander/OpenPlasma/plasma/models"
	"github.com/DryginAlexander/OpenPlasma/plasma/operator"
	"github.com/DryginAlexander/OpenPlasma/plasma/web"
)

func main() {
	fmt.Println("connecting to db")
	stor := models.NewStorage()
	defer stor.CloseDB()

	fmt.Println("applying migration if needed")
	_ = stor.MigrateDB()

	fmt.Println("init hot config if needed")
	_ = stor.InitHotConfig("./env/dev.env")

	// fmt.Println("init settings")

	oper := operator.NewOperator(&stor)
	web.Init(&stor, &stor, &oper)
}
