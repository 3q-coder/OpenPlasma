package models

import (
	"github.com/DryginAlexander/OpenPlasma/plasma"
	"github.com/jinzhu/gorm"
)

type Transfer struct {
	gorm.Model
	plasma.Transfer
}

func (s *Storage) CreateTransfer(_trans *plasma.Transfer) error {
	trans := Transfer{Transfer: *_trans}

	if err := s.db.Create(&trans).Error; err != nil {
		return err
	}
	return nil
}

func (s *Storage) TransfersByUserId(id int) ([]plasma.Transfer, error) {
	transfers := []*Transfer{}
	err := s.db.Set("gorm:auto_preload", true).
		Where("account_ID = ?", id).Find(&transfers).Error
	var plasma_transfers []plasma.Transfer
	for _, trans := range transfers {
		plasma_transfers = append(plasma_transfers, trans.Transfer)
	}
	return plasma_transfers, err
}
