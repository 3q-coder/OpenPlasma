package models

import (
	"github.com/DryginAlexander/OpenPlasma/plasma"
	"github.com/jinzhu/gorm"
)

type OnchainWithdrawal struct {
	gorm.Model
	plasma.OnchainWithdrawal
}

func (s *Storage) CreateOnchainWithdraw(_withd *plasma.OnchainWithdrawal) error {
	withd := OnchainWithdrawal{OnchainWithdrawal: *_withd}

	if err := s.db.Create(&withd).Error; err != nil {
		return err
	}
	return nil
}

func (s *Storage) OnchainWithdrawalsByUserId(id int) ([]plasma.OnchainWithdrawal, error) {
	withds := []*OnchainWithdrawal{}
	err := s.db.Set("gorm:auto_preload", true).
		Where("user_ID = ?", id).Find(&withds).Error
	var plasma_withds []plasma.OnchainWithdrawal
	for _, withd := range withds {
		plasma_withds = append(plasma_withds, withd.OnchainWithdrawal)
	}
	return plasma_withds, err
}
