package postges

import (
	"github.com/DryginAlexander/OpenPlasma/plasma"
)

type Storage struct {
	users               []plasma.User
	deposits            []plasma.Deposit
	transfers           []plasma.Transfer
	onchainWithdrawals  []plasma.OnchainWithdrawal
	offchainWithdrawals []plasma.OffchainWithdrawal
}

func (s *Storage) CreateUser(addr string) (*plasma.User, error) {
	id := len(s.users)
	user := plasma.User{
		ID:      id,
		Address: addr,
	}
	s.users = append(s.users, user)
	return &user, nil
}

func (s *Storage) UserById(id int) (*plasma.User, error) {
	if id < len(s.users) {
		return &s.users[id], nil
	}
	// TODO handle errors
	return nil, nil
}

func (s *Storage) UserByAddress(addr string) (*plasma.User, error) {
	for _, user := range s.users {
		if user.Address == addr {
			return &user, nil
		}
	}
	// TODO handle errors
	return nil, nil
}

func (s *Storage) CreateDeposit(dep *plasma.Deposit) error {
	s.deposits = append(s.deposits, *dep)
	return nil
}

func (s *Storage) DepositsByUserId(id int) ([]plasma.Deposit, error) {
	var ans []plasma.Deposit

	for _, dep := range s.deposits {
		if dep.UserId == id {
			ans = append(ans, dep)
		}
	}
	return ans, nil
}

func (s *Storage) CreateTransfer(trans *plasma.Transfer) error {
	s.transfers = append(s.transfers, *trans)
	return nil
}

func (s *Storage) TransfersByUserId(id int) ([]plasma.Transfer, error) {
	var ans []plasma.Transfer

	for _, trans := range s.transfers {
		if trans.UserId == id {
			ans = append(ans, trans)
		}
	}
	return ans, nil
}

func (s *Storage) CreateOnchainWithdraw(withd *plasma.OnchainWithdrawal) error {
	s.onchainWithdrawals = append(s.onchainWithdrawals, *withd)
	return nil
}

func (s *Storage) OnchainWithdrawByUserId(id int) ([]plasma.OnchainWithdrawal, error) {
	var ans []plasma.OnchainWithdrawal

	for _, withd := range s.onchainWithdrawals {
		if withd.UserId == id {
			ans = append(ans, withd)
		}
	}
	return ans, nil
}

func (s *Storage) CreateOffchainWithdraw(withd *plasma.OffchainWithdrawal) error {
	s.offchainWithdrawals = append(s.offchainWithdrawals, *withd)
	return nil
}

func (s *Storage) OffchainWithdrawByUserId(id int) ([]plasma.OffchainWithdrawal, error) {
	var ans []plasma.OffchainWithdrawal

	for _, withd := range s.offchainWithdrawals {
		if withd.UserId == id {
			ans = append(ans, withd)
		}
	}
	return ans, nil
}
