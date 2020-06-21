package postgres

import (
	"github.com/DryginAlexander/OpenPlasma/plasma"
)

type Storage struct {
	balances            map[int]int
	users               []plasma.User
	deposits            []plasma.Deposit
	transfers           []plasma.Transfer
	onchainWithdrawals  []plasma.OnchainWithdrawal
	offchainWithdrawals []plasma.OffchainWithdrawal
}

func NewStorage() Storage {
	stor := Storage{
		balances: make(map[int]int),
	}

	stor.CreateUser(
		&plasma.User{
			ID:       0,
			Username: "user0",
			Password: "pass0",
			Address:  "0x0",
		},
	)
	stor.CreateUser(
		&plasma.User{
			ID:       1,
			Username: "user1",
			Password: "pass1",
			Address:  "0x1",
		},
	)

	stor.CreateDeposit(
		&plasma.Deposit{
			UserId: 0,
			Value:  100,
		},
	)

	stor.CreateDeposit(
		&plasma.Deposit{
			UserId: 1,
			Value:  200,
		},
	)

	stor.balances[0] = 100
	stor.balances[1] = 200

	return stor
}

func (s *Storage) IsUsernameAvailable(username string) bool {
	for _, u := range s.users {
		if u.Username == username {
			return false
		}
	}
	return true
}

func (s *Storage) IsUserValid(username, password string) bool {
	for _, u := range s.users {
		if u.Username == username && u.Password == password {
			return true
		}
	}
	return false
}

func (s *Storage) CreateUser(user *plasma.User) error {
	s.users = append(s.users, *user)
	return nil
}

func (s *Storage) GetUsersCount() int {
	return len(s.users)
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

func (s *Storage) OnchainWithdrawalsByUserId(id int) ([]plasma.OnchainWithdrawal, error) {
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

func (s *Storage) OffchainWithdrawalsByUserId(id int) ([]plasma.OffchainWithdrawal, error) {
	var ans []plasma.OffchainWithdrawal

	for _, withd := range s.offchainWithdrawals {
		if withd.UserId == id {
			ans = append(ans, withd)
		}
	}
	return ans, nil
}

func (s *Storage) ReduceBalance(user_id int, value int) error {
	s.balances[user_id] -= value
	return nil
}

func (s *Storage) IncreaseBalance(user_id int, value int) error {
	s.balances[user_id] += value
	return nil
}
