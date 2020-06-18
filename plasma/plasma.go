package plasma

type User struct {
	ID      int
	Address string
}

type Deposit struct {
	UserId int
	Value  int
}

type Transfer struct {
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

type Operator interface {
	CreateUser(user *User) error
	AddDeposit(dep *Deposit) error
	AddTransfer(trans *Transfer) error
	AddOnchainWithdrawal(withd *OnchainWithdrawal) error
	AddOffchainWithdrawal(withd *OffchainWithdrawal) error
	ExecuteDeposits() error
	ExecuteTransfers() error
	ExecuteOnchainWithdrawals() error
	ExecuteOffchainWithdrawals() error
}
