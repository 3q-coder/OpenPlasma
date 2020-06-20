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
	CreateUser(addr string) (*User, error)
	UserById(id int) (*User, error)
	UserByAddress(addr string) (*User, error)
	// deposit
	CreateDeposit(dep *Deposit) error
	DepositsByUserId(id int) ([]Deposit, error)
	// transfer
	CreateTransfer(trans *Transfer) error
	TransfersByUserId(id int) ([]Transfer, error)
	// onchain withdraw
	CreateOnchainWithdraw(withd *OnchainWithdrawal) error
	OnchainWithdrawByUserId(id int) ([]OnchainWithdrawal, error)
	// offchain withdraw
	CreateOffchainWithdraw(withd *OffchainWithdrawal) error
	OffchainWithdrawByUserId(id int) ([]OffchainWithdrawal, error)
}

// TODO make handler
type Operator interface {
	ExecuteDeposits() error
	ExecuteTransfers() error
	ExecuteOnchainWithdrawals() error
	ExecuteOffchainWithdrawals() error
}
