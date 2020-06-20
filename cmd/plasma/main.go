package main

import (
	"github.com/DryginAlexander/OpenPlasma/plasma/operator"
	"github.com/DryginAlexander/OpenPlasma/plasma/postgres"
	"github.com/DryginAlexander/OpenPlasma/plasma/web"
)

func main() {
	stor := postgres.NewStorage()
	oper := operator.NewOperator(&stor)
	web.Init(&stor, &oper)
}
