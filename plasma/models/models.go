package models

import (
	"fmt"

	"github.com/DryginAlexander/OpenPlasma/plasma"
	"github.com/DryginAlexander/OpenPlasma/plasma/settings"
	"github.com/jinzhu/gorm"
	_ "github.com/jinzhu/gorm/dialects/postgres"
	_ "github.com/jinzhu/gorm/dialects/sqlite"
	gormigrate "gopkg.in/gormigrate.v1"
)

type Storage struct {
	db *gorm.DB
}

func NewStorage() Storage {
	var DBConnStr string
	var dialect string
	switch settings.DBDialect {
	case "sqlite":
		DBConnStr = settings.DBName
		dialect = "sqlite3"
	case "postgresql":
		DBConnStr = fmt.Sprintf(
			"host=%s port=%s user=%s password=%s dbname=%s sslmode=disable",
			settings.DBHost,
			settings.DBPort,
			settings.DBUser,
			settings.DBPw,
			settings.DBName,
		)
		dialect = "postgres"
		// default:
		// 	return errors.New(fmt.Sprintf("unknown DBDialect %s", settings.DBDialect))
	}

	db, _ := gorm.Open(dialect, DBConnStr)
	// DB.SetLogger(logger)
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
