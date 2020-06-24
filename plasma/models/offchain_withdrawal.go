package models

import (
	"github.com/DryginAlexander/OpenPlasma/plasma"
	"github.com/jinzhu/gorm"
)

type OffchainWithdrawal struct {
	gorm.Model
	plasma.OffchainWithdrawal
}

func (s *Storage) CreateOffchainWithdraw(_withd *plasma.OffchainWithdrawal) error {
	withd := OffchainWithdrawal{OffchainWithdrawal: *_withd}

	if err := s.db.Create(&withd).Error; err != nil {
		return err
	}
	return nil
}

func (s *Storage) OffchainWithdrawalsByUserId(id int) ([]plasma.OffchainWithdrawal, error) {
	withds := []*OffchainWithdrawal{}
	err := s.db.Set("gorm:auto_preload", true).
		Where("account_ID = ?", id).Find(&withds).Error
	var plasma_withds []plasma.OffchainWithdrawal
	for _, withd := range withds {
		plasma_withds = append(plasma_withds, withd.OffchainWithdrawal)
	}
	return plasma_withds, err
}
