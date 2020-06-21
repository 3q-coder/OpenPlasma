package operator

import (
	"errors"
	"strings"

	"github.com/DryginAlexander/OpenPlasma/plasma"
)

type Operator struct {
	storage plasma.Storage
}

func NewOperator(stor plasma.Storage) Operator {
	return Operator{stor}
}

func (o *Operator) RegisterUser(username, password, addr string) (*plasma.User, error) {
	if strings.TrimSpace(password) == "" {
		return nil, errors.New("The password can't be empty")
	} else if !o.storage.IsUsernameAvailable(username) {
		return nil, errors.New("The username isn't available")
	}

	id := o.storage.GetUsersCount()
	user := plasma.User{
		ID:       id,
		Username: username,
		Password: password,
		Address:  addr,
	}
	o.storage.CreateUser(&user)
	return &user, nil
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
