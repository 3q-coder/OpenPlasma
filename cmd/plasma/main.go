package main

import (
	"github.com/DryginAlexander/OpenPlasma/plasma/models"
	"github.com/DryginAlexander/OpenPlasma/plasma/operator"
	"github.com/DryginAlexander/OpenPlasma/plasma/web"
)

func main() {
	// log.Println("connecting to db")
	stor := models.NewStorage()
	defer stor.CloseDB()

	// log.Println("applying migration if needed")
	_ = stor.MigrateDB()

	oper := operator.NewOperator(&stor)
	web.Init(&stor, &oper)
}
