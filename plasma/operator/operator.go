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

func (o *Operator) CreateTransfer(trans plasma.Transfer) error {
	// TODO properly handle concurent requests

	user, _ := o.storage.UserByAddress(trans.From)
	trans.UserId = user.ID

	_ = o.storage.ReduceBalance(trans.UserId, trans.Value)

	user, _ = o.storage.UserByAddress(trans.To)
	_ = o.storage.IncreaseBalance(user.ID, trans.Value)

	_ = o.storage.CreateTransfer(&trans)

	return nil
}

func (o *Operator) CreateOffchainWithdraw(from string, withd plasma.OffchainWithdrawal) error {
	// TODO properly handle concurent requests

	user, _ := o.storage.UserByAddress(from)
	withd.UserId = user.ID

	_ = o.storage.ReduceBalance(withd.UserId, withd.Value)

	_ = o.storage.CreateOffchainWithdraw(&withd)

	return nil
}
