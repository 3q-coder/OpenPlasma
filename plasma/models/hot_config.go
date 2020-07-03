package models

import (
	"os"
	"strconv"
	"time"

	"github.com/jinzhu/gorm"
	"github.com/joho/godotenv"
)

type HotConfig struct {
	gorm.Model
	DepositPeriod            int
	TransferPeriod           int
	OnchainWithdrawalPeriod  int
	OffchainWithdrawalPeriod int
}

func (s *Storage) InitHotConfig(dotenvFileName string) error {
	var count int
	s.db.Model(&HotConfig{}).Count(&count)

	// config has been initialized ------------------------
	if count > 0 {
		return nil
	}

	// initialize config ----------------------------------
	if dotenvFileName != "" {
		err := godotenv.Load(dotenvFileName)
		if err != nil {
			return err
		}
	}

	depositPeriod, _ := strconv.Atoi(os.Getenv("DEPOSIT_PERIOD"))
	transferPeriod, _ := strconv.Atoi(os.Getenv("TRANSFER_PERIOD"))
	onchainWithdrawalPeriod, _ := strconv.Atoi(os.Getenv("ONCHAIN_WITHDRAWAL_PERIOD"))
	offchainWithdrawalPeriod, _ := strconv.Atoi(os.Getenv("OFFCHAIN_WITHDRAWAL_PERIOD"))

	config := HotConfig{
		DepositPeriod:            depositPeriod,
		TransferPeriod:           transferPeriod,
		OnchainWithdrawalPeriod:  onchainWithdrawalPeriod,
		OffchainWithdrawalPeriod: offchainWithdrawalPeriod,
	}

	return s.db.Create(&config).Error
}

// Getters ----------------------------------------------------------
// ------------------------------------------------------------------

func (s *Storage) DepositPeriod() (time.Duration, error) {
	conf := HotConfig{}
	err := s.db.Set("gorm:auto_preload", true).
		Where("id = ?", 1).Find(&conf).Error
	return time.Duration(conf.DepositPeriod) * time.Second, err
}

func (s *Storage) TransferPeriod() (time.Duration, error) {
	conf := HotConfig{}
	err := s.db.Set("gorm:auto_preload", true).
		Where("id = ?", 1).Find(&conf).Error
	return time.Duration(conf.TransferPeriod) * time.Second, err
}

func (s *Storage) OnchainWithdrawalPeriod() (time.Duration, error) {
	conf := HotConfig{}
	err := s.db.Set("gorm:auto_preload", true).
		Where("id = ?", 1).Find(&conf).Error
	return time.Duration(conf.OnchainWithdrawalPeriod) * time.Second, err
}

func (s *Storage) OffchainWithdrawalPeriod() (time.Duration, error) {
	conf := HotConfig{}
	err := s.db.Set("gorm:auto_preload", true).
		Where("id = ?", 1).Find(&conf).Error
	return time.Duration(conf.OffchainWithdrawalPeriod) * time.Second, err
}
