package plasma

import "time"

type User struct {
	Idn      int
	Username string
	Password string
	Address  string
	Balance  int
}

type Deposit struct {
	UserId int
	Value  int
}

type Transfer struct {
	UserId    int
	From      string
	To        string
	Value     int
	Nonce     int
	Signature string
}

type OnchainWithdrawal struct {
	UserId  int
	Value   int
	Address string
}

type OffchainWithdrawal struct {
	UserId    int
	Value     int
	Address   string
	Nonce     int
	Signature string
}

type Storage interface {
	// user
	IsUsernameAvailable(username string) bool
	IsUserValid(username, password string) bool
	CreateUser(user *User) error
	GetUsersCount() int
	UserById(id int) (*User, error)
	UserByAddress(addr string) (*User, error)
	ReduceBalance(user_id int, value int) error
	IncreaseBalance(user_id int, value int) error
	// deposit
	CreateDeposit(dep *Deposit) error
	DepositsByUserId(id int) ([]Deposit, error)
	// transfer
	CreateTransfer(trans *Transfer) error
	TransfersByUserId(id int) ([]Transfer, error)
	// onchain withdraw
	CreateOnchainWithdraw(withd *OnchainWithdrawal) error
	OnchainWithdrawalsByUserId(id int) ([]OnchainWithdrawal, error)
	// offchain withdraw
	CreateOffchainWithdraw(withd *OffchainWithdrawal) error
	OffchainWithdrawalsByUserId(id int) ([]OffchainWithdrawal, error)
}

type Operator interface {
	// web
	RegisterUser(username, password, addr string) (*User, error)
	CreateTransfer(trans Transfer) error
	CreateOffchainWithdraw(from string, withd OffchainWithdrawal) error
	// plasma blocks
	ExecuteDeposits() error
	ExecuteTransfers() error
	ExecuteOnchainWithdrawals() error
	ExecuteOffchainWithdrawals() error
}

type HotConfig interface {
	DepositPeriod() (time.Duration, error)
	TransferPeriod() (time.Duration, error)
	OnchainWithdrawalPeriod() (time.Duration, error)
	OffchainWithdrawalPeriod() (time.Duration, error)
}
