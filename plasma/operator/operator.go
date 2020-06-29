package operator

import (
	"github.com/DryginAlexander/OpenPlasma/plasma"
)

type Operator struct {
	storage plasma.Storage
}

func NewOperator(stor plasma.Storage) Operator {
	return Operator{stor}
}
