package models

import (
	"github.com/DryginAlexander/OpenPlasma/plasma"
	"github.com/jinzhu/gorm"
	_ "github.com/jinzhu/gorm/dialects/postgres"
	_ "github.com/jinzhu/gorm/dialects/sqlite"
	gormigrate "gopkg.in/gormigrate.v1"
)

type Storage struct {
	db *gorm.DB
}

func NewStorage() Storage {
	db, _ := gorm.Open("sqlite3", "admin.db")
	return Storage{
		db: db,
	}
}

func (s *Storage) CloseDB() {
	s.db.Close()
}

func (s *Storage) MigrateDB() error {
	m := gormigrate.New(s.db, gormigrate.DefaultOptions, []*gormigrate.Migration{
		// inital migration
		{
			ID: "202006242355",
			Migrate: func(tx *gorm.DB) error {
				type User struct {
					gorm.Model
					plasma.User
				}
				type Deposit struct {
					gorm.Model
					plasma.Deposit
				}
				type Transfer struct {
					gorm.Model
					plasma.Transfer
				}
				type OnchainWithdrawal struct {
					gorm.Model
					plasma.OnchainWithdrawal
				}
				type OffchainWithdrawal struct {
					gorm.Model
					plasma.OffchainWithdrawal
				}
				type HotConfig struct {
					gorm.Model
					DepositPeriod            int
					TransferPeriod           int
					OnchainWithdrawalPeriod  int
					OffchainWithdrawalPeriod int
				}
				return tx.AutoMigrate(&User{}, &Deposit{}, &Transfer{},
					&OnchainWithdrawal{}, &OffchainWithdrawal{}, &HotConfig{}).Error
			},
			Rollback: func(tx *gorm.DB) error {
				return tx.DropTableIfExists("users", "deposits", "transfers",
					"onchain_withdrawals", "offchain_withdrawals", "hot_configs").Error
			},
		},
		// future migrations ...
	})
	return m.Migrate()
}

// func BeginDBTransaction() (*gorm.DB, error) {
// 	tx := DB.Begin()
// 	return tx, tx.Error
// }
